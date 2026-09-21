#!/usr/bin/env bash
# Golden harness — SELF-VERIFICATION: records this engine's own behavior as
# replayable goldens. No Go anywhere: no go build, no goprobe, no Go binary.
#
# What changed vs the pre-2026-09-10 dual-engine harness (tag go-parity-final
# in git history): that harness ran every case under BOTH engines and
# live-compared — the byte-parity oracle that governed the port. The port is
# complete and its parity vs Go is a proven historical claim (the go-parity-final
# tag holds the last dual-engine state; gate_parity.rs, the opt-in Go
# cross-check, was deleted when the Go engine was removed). From here on the acceptance is:
#   - this script records the engine's behavior,
#   - tests/golden.rs replays it under `cargo test`,
#   - the gates themselves (run all / cargo test true-tree tests) stay the
#     tree's authority.
#
# Layers (mirroring the old numbering):
#   1. CLI surface matrix: plan/list/status/inputs over every target, flag
#      combination and error path — stdout/stderr/exit recorded.
#   2. Key folding: the Rust __keys output over the real tree, recorded.
#   3. A discriminating fixture: a minimal tree stamped with this engine's
#      keys — `status` may only report `fresh` if the keyer/stamp round-trip
#      is intact (this layer is self-referential by nature now; keyer drift
#      is caught by layer 2's golden, not here).
#   4. Runner semantics on stamped fixtures: fresh-skip, --force execution,
#      stamp removal on failure, blocked-node accounting, --keep-going
#      report — asserted structurally. The mirror table's `pin` node runs
#      `go test`, so layer 4 shadows `go` with a stub that exits 127 and
#      logs: any go invocation DURING LAYER 4 lands in that log (layers 1-3
#      run recording verbs that never spawn).
#
# Recorded under the go-mirror TABLE (pure authored data in nodes.rs, no Go
# execution for these verbs): it is fingerprint-free, so the goldens do not
# churn on every engine edit, and plan/list/status/inputs/__keys under it
# never spawn anything. The harness never `run`s on the real tree — `run`
# happens only on the fixture, where the Go-verbatim commands fail by
# absence of ./build/pipeline (exactly the designed determinism, still
# without any Go).
#
# Exit non-zero on any assertion failure.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
RS_ROOT="$(cd "$HERE/.." && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$RS_ROOT/..}"
export SHADLESS_GRAPH=go-mirror
RS_BIN="$(node "$RS_ROOT/../tools/pipeline-bin.mjs")"
GOLDEN="$RS_ROOT/tests/golden"
FIXTURE="/tmp/shadless-rs-runner-fixture"
STUB_BIN="/tmp/shadless-rs-go-stub"

fail=0
pass=0
recorded=0

echo "== building the Rust binary =="
(cd "$RS_ROOT" && cargo build --release -q) || exit 1

slug() { printf '%s' "$*" | tr -c 'A-Za-z0-9._=-' '_'; }

# record_case <cwd> <args...>: run the engine, record out/err/exit under
# $GOLDEN_DIR (default tests/golden). Verification is golden.rs's replay.
GOLDEN_DIR="$GOLDEN"
record_case() {
  local cwd="$1"; shift
  local s
  s="$(slug "$@")"
  local d="$GOLDEN_DIR/$s"
  mkdir -p "$GOLDEN_DIR"
  (cd "$cwd" && "$RS_BIN" "$@") > "$d.rs.out" 2> "$d.rs.err" </dev/null
  local rs_code=$?
  echo "$rs_code" > "$d.rs.code"
  printf '%s\n' "$*" > "$d.cmd"
  recorded=$((recorded+1))
}

# check <label> <condition...>: a structural self-assertion
check() {
  local label="$1"; shift
  if "$@"; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    echo "FAIL: $label"
  fi
}

echo "== layer 1: CLI matrix on the real tree (record) =="
rm -rf "$GOLDEN"; mkdir -p "$GOLDEN"

for view in plan list status; do
  for target in fast full builds all; do
    record_case "$SHADLESS_ROOT" "$view" "$target"
  done
  record_case "$SHADLESS_ROOT" "$view" --gates-only fast
  record_case "$SHADLESS_ROOT" "$view" --builds-only full
  record_case "$SHADLESS_ROOT" "$view" --force fast
done

# every node in the graph (incl. contracts:* shards) through every view
mapfile -t ALL_IDS < <(cd "$SHADLESS_ROOT" && "$RS_BIN" plan all)
for id in "${ALL_IDS[@]}"; do
  record_case "$SHADLESS_ROOT" plan "$id"
  record_case "$SHADLESS_ROOT" status "$id"
  record_case "$SHADLESS_ROOT" list "$id"
  record_case "$SHADLESS_ROOT" inputs "$id"
done
for id in emit demo contracts coverage pack reproducible; do
  record_case "$SHADLESS_ROOT" inputs "$id" --produces
done

# multi-target combinations and flag interactions
record_case "$SHADLESS_ROOT" plan pin emit
record_case "$SHADLESS_ROOT" plan convert demo-css
record_case "$SHADLESS_ROOT" plan contracts:button style-parity
record_case "$SHADLESS_ROOT" list pin emit
record_case "$SHADLESS_ROOT" plan --gates-only --builds-only fast

# error paths: usage, unknown command, unknown node, unknown flag
record_case "$SHADLESS_ROOT" plan
record_case "$SHADLESS_ROOT" list
record_case "$SHADLESS_ROOT" status
record_case "$SHADLESS_ROOT" plan nope
record_case "$SHADLESS_ROOT" list nope
record_case "$SHADLESS_ROOT" status nope
record_case "$SHADLESS_ROOT" plan --bogus
record_case "$SHADLESS_ROOT" inputs
record_case "$SHADLESS_ROOT" inputs --produces
record_case "$SHADLESS_ROOT" inputs nope
record_case "$SHADLESS_ROOT" bogus
record_case "$SHADLESS_ROOT"

# recording sanity: the recording itself must show a healthy engine --
# `plan all` exits 0 with real output, the unknown-command error path exits
# non-zero. Without these, a binary that panicked everywhere would record
# "green" and only fail later at replay time.
check "recorded plan all exits 0 with output" \
  [ "$(cat "$GOLDEN/plan_all.rs.code")" = "0" ] && [ -s "$GOLDEN/plan_all.rs.out" ]
check "recorded bogus command exits non-zero" \
  [ "$(cat "$GOLDEN/bogus.rs.code")" != "0" ]

echo "== layer 2: key folding on the real tree (record) =="
(cd "$SHADLESS_ROOT" && "$RS_BIN" __keys) > "$GOLDEN/keys.rs.txt" 2>/dev/null \
  || { echo "FAIL: __keys"; exit 1; }
key_lines=$(wc -l < "$GOLDEN/keys.rs.txt")
if [ -s "$GOLDEN/keys.rs.txt" ] && [ "$key_lines" -gt 50 ]; then
  pass=$((pass+1)); echo "keys recorded: $key_lines nodes"
else
  fail=$((fail+1)); echo "FAIL: keys golden suspiciously small ($key_lines lines)"
fi

# mk_fixture: a minimal tree the pipeline can load, stamped with this
# engine's keys, outputs made present. `status` may only report fresh.
mk_fixture() {
  rm -rf "$FIXTURE"
  mkdir -p "$FIXTURE/tools/contracts/components" "$FIXTURE/src/registry"
  cat > "$FIXTURE/src/registry/tiers.json" <<'JSON'
{"alpha":{"tier":"static"},"beta":{"tier":"kernel"},"gamma":{"tier":"interactive","emit":true}}
JSON
  (cd "$FIXTURE" && "$RS_BIN" __keys) | while IFS=$'\t' read -r id ok value; do
    [ "$ok" = "1" ] || continue
    mkdir -p "$FIXTURE/pipeline/stamps"
    printf '%s\n' "$value" > "$FIXTURE/pipeline/stamps/${id//:/__}"
  done
  for _ in 1 2 3 4 5 6; do
    (cd "$FIXTURE" && "$RS_BIN" status all) | grep 'output missing' | sed -E 's/.*output missing: (.*)\)/\1/' | while read -r p; do
      mkdir -p "$FIXTURE/$p"
    done
  done
}

echo "== layer 3: discriminating fixture (stamped by this engine) =="
mk_fixture
f_status="$(cd "$FIXTURE" && "$RS_BIN" status all)"
f_fresh=$(printf '%s\n' "$f_status" | grep -c ' fresh$')
f_stale=$(printf '%s\n' "$f_status" | grep -c 'STALE')
if [ "$f_fresh" -lt 39 ] || [ "$f_stale" != "0" ]; then
  fail=$((fail+1)); echo "FAIL: fixture not fully stamped ($f_fresh fresh, $f_stale stale) — not discriminating"
else
  pass=$((pass+1)); echo "fixture freshness discriminating: $f_fresh fresh, 1 NEVER-FRESH, 0 stale"
fi
# determinism: a second identical run must produce the identical report
if [ "$f_status" = "$(cd "$FIXTURE" && "$RS_BIN" status all)" ]; then
  pass=$((pass+1))
else
  fail=$((fail+1)); echo "FAIL: fixture status is nondeterministic across runs"
fi

# layer 4: runner semantics on stamped fixtures. Parameterized by -j for the
# flakiness sweep. The go STUB shadows any real go: the mirror table's pin
# node runs `go test`, which must fail deterministically (127) and never
# reach a real toolchain; the stub log is the proof (checked after layer 4).
layer4() {
  local jobs="$1"
  local J="PIPELINE_PARALLEL=$jobs"
  local P="PATH=$STUB_BIN:$PATH"

  # a) fresh-skip: everything stamped+present -> all skipped, nothing runs
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$P" "$RS_BIN" run builds) > /tmp/r.rs.out 2> /tmp/r.rs.err
  local rs_code=$?
  if [ "$rs_code" = "0" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): fresh-skip exit code: $rs_code (want 0)"
  fi
  # 17 build nodes + pin (pulled in as convert/rtl-dict/docs-upstream-mirror's need)
  grep -q "ran 0, skipped 18 " /tmp/r.rs.out \
    && pass=$((pass+1)) || { fail=$((fail+1)); echo "FAIL(j$jobs): fresh-skip summary"; grep "ran " /tmp/r.rs.out; }
  # no stamp was rewritten and no stamp removed: status still all fresh
  if [ "$(cd "$FIXTURE" && "$RS_BIN" status builds | grep -c ' fresh$')" = "18" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): fresh-skip disturbed the stamps"
  fi

  # b) --force on builds: every command fails instantly on the fixture (there
  #    is no ./build/pipeline binary, and `go` is the stub); failed nodes lose
  #    their stamps
  mk_fixture
  local had_stamp=0; [ -f "$FIXTURE/pipeline/stamps/build-js" ] && had_stamp=1
  (cd "$FIXTURE" && env "$J" "$P" "$RS_BIN" run --force builds) > /tmp/r.rs.out 2> /tmp/r.rs.err
  rs_code=$?
  if [ "$rs_code" = "1" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): --force exit code: $rs_code (want 1)"
  fi
  if [ "$had_stamp" = "1" ] && [ ! -f "$FIXTURE/pipeline/stamps/build-js" ]; then
    pass=$((pass+1)); echo "stamp-on-failure (j$jobs): failed node's stamp removed"
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): stamp-on-failure (had=$had_stamp)"
  fi

  # c) --keep-going with failures: a run report is written and the blocked
  #    accounting is present (structural; the report is machine-shaped, not
  #    byte-golden — its stderr embeds tool availability)
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$P" "$RS_BIN" run --keep-going --force builds) > /dev/null 2>&1
  if [ -f "$FIXTURE/build/gates/run-report.json" ] \
     && grep -q '"build-js"' "$FIXTURE/build/gates/run-report.json" \
     && ! grep -q '"blocked": \[\]' "$FIXTURE/build/gates/run-report.json"; then
    pass=$((pass+1)); echo "run-report.json (j$jobs): written with failure/blocked accounting"
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): run-report.json missing or vacuous"
  fi

  # d) --keep-going with NO failures: the empty report — the initialized
  #    empty map must marshal as {} and empty slices as []
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$P" "$RS_BIN" run --keep-going builds) > /dev/null 2>&1
  if grep -q '"failed": {}' "$FIXTURE/build/gates/run-report.json" \
     && grep -q '"blocked": \[\]' "$FIXTURE/build/gates/run-report.json"; then
    pass=$((pass+1)); echo "empty run-report.json (j$jobs): empty shape"
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): empty run-report.json unexpected shape"
    head -c 200 "$FIXTURE/build/gates/run-report.json"; echo
  fi
}

echo "== layer 4: runner semantics on stamped fixtures (flakiness sweep) =="
mkdir -p "$STUB_BIN"
printf '#!/bin/sh\necho "$*" >> /tmp/shadless-rs-go-stub.log\nexit 127\n' > "$STUB_BIN/go"
chmod +x "$STUB_BIN/go"
rm -f /tmp/shadless-rs-go-stub.log
for jobs in 1 1 1 1 1 1 1 1 1 1 4 4 4 8; do
  layer4 "$jobs"
done
if [ -f /tmp/shadless-rs-go-stub.log ]; then
  # The stub IS expected to be hit — the mirror table's pin node runs
  # `go test` and must fail deterministically. Every invocation must be
  # that shape; anything else means real logic tried to run go.
  if grep -qv '^test -C pipeline' /tmp/shadless-rs-go-stub.log; then
    fail=$((fail+1))
    echo "FAIL: unexpected go invocation(s):"
    grep -v '^test -C pipeline' /tmp/shadless-rs-go-stub.log | head -5
  else
    pass=$((pass+1))
    echo "go stub invocations all expected gate shapes: $(wc -l < /tmp/shadless-rs-go-stub.log) (zero real Go)"
  fi
else
  fail=$((fail+1)); echo "FAIL: go stub never invoked — pin should have failed through it"
fi

echo
echo "== $pass passed, $fail failed =="
[ "$fail" = 0 ]
