#!/usr/bin/env bash
# Golden harness — live A/B comparison of the Go binary and the Rust port.
#
# Four layers, from weakest to strongest:
#   1. CLI surface matrix: plan/list/status/inputs over every target, flag
#      combination and error path — stdout, stderr and exit code must match.
#   2. Key parity: goprobe (the real Go keyer, probe/keys-go/setup.sh) vs the
#      Rust __keys subcommand, over the real tree.
#   3. A discriminating fixture: a minimal tree stamped with the GO side's
#      keys, so `status` can only report `fresh` if the Rust keyer reproduces
#      the Go keys exactly — a port bug flips verdicts, which layer 1 on the
#      stale real tree cannot see.
#   4. Runner semantics on stamped fixtures: fresh-skip, --force execution,
#      stamp removal on failure, blocked-node accounting and the --keep-going
#      run report — compared Go vs Rust with timings normalized (the one thing
#      a port cannot reproduce).
#
# Writes replayable goldens under tests/golden/ (layer 1, real tree only).
# Exit non-zero on any mismatch.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
RS_ROOT="$(cd "$HERE/.." && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$RS_ROOT/../shadless}"
GO_BIN="$SHADLESS_ROOT/build/pipeline"
RS_BIN="$RS_ROOT/target/release/pipeline"
KEYS_BIN="$RS_ROOT/probe/keys-go/goprobe"
GOLDEN="$RS_ROOT/tests/golden"
FIXTURE="/tmp/shadless-rs-runner-fixture"

fail=0
pass=0

echo "== building binaries =="
(cd "$SHADLESS_ROOT" && go build -C pipeline -o ../build/pipeline .) || exit 1
(cd "$RS_ROOT" && cargo build --release -q) || exit 1
"$RS_ROOT/probe/keys-go/setup.sh" >/dev/null || exit 1

slug() { printf '%s' "$*" | tr -c 'A-Za-z0-9._=-' '_'; }

# run_case <cwd> <args...>: run both binaries, compare out/err/code, record
# golden under $GOLDEN_DIR (default tests/golden; fixture cases go elsewhere).
GOLDEN_DIR="$GOLDEN"
run_case() {
  local cwd="$1"; shift
  local s
  s="$(slug "$@")"
  local d="$GOLDEN_DIR/$s"
  mkdir -p "$GOLDEN_DIR"
  (cd "$cwd" && "$GO_BIN" "$@") > "$d.go.out" 2> "$d.go.err" </dev/null
  local go_code=$?
  (cd "$cwd" && "$RS_BIN" "$@") > "$d.rs.out" 2> "$d.rs.err" </dev/null
  local rs_code=$?
  echo "$go_code" > "$d.go.code"
  echo "$rs_code" > "$d.rs.code"
  printf '%s\n' "$*" > "$d.cmd"
  local ok=1
  cmp -s "$d.go.out" "$d.rs.out" || ok=0
  cmp -s "$d.go.err" "$d.rs.err" || ok=0
  [ "$go_code" = "$rs_code" ] || ok=0
  if [ $ok = 1 ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    echo "FAIL: (in $cwd) pipeline $*"
    diff "$d.go.out" "$d.rs.out" | head -6
    diff "$d.go.err" "$d.rs.err" | head -6
    [ "$go_code" = "$rs_code" ] || echo "  exit code: go=$go_code rs=$rs_code"
  fi
}

# compare_normalized <label> <go-file> <rs-file>: byte-diff with timings masked.
# Lines are SORTED before diffing: at -j1 the dispatch still hands the single
# job slot to whichever worker's goroutine reaches it first, so the Go
# original's execution ORDER is genuinely nondeterministic — only the set of
# lines, the counts and the exit code are port-relevant.
compare_normalized() {
  local label="$1" gf="$2" rf="$3"
  local norm='s/in [0-9]+\.[0-9]s/in Ts/g; s/\([0-9]+\.[0-9]+s\)/(Ts)/g'
  if diff <(sed -E "$norm" "$gf" | sort) <(sed -E "$norm" "$rf" | sort) > /tmp/cmp.diff; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    echo "FAIL: $label"
    head -12 /tmp/cmp.diff
  fi
}

# mk_fixture: a minimal tree the pipeline can load, stamped with the GO side's
# keys, outputs made present. Only a byte-exact Rust keyer can then reproduce
# `fresh` verdicts.
mk_fixture() {
  rm -rf "$FIXTURE"
  mkdir -p "$FIXTURE/tools/contracts/components" "$FIXTURE/src/registry"
  cat > "$FIXTURE/src/registry/tiers.json" <<'JSON'
{"alpha":{"tier":"static"},"beta":{"tier":"kernel"},"gamma":{"tier":"interactive","emit":true}}
JSON
  (cd "$FIXTURE" && "$KEYS_BIN") | while IFS=$'\t' read -r id ok value; do
    [ "$ok" = "1" ] || continue
    mkdir -p "$FIXTURE/pipeline/stamps"
    printf '%s\n' "$value" > "$FIXTURE/pipeline/stamps/${id//:/__}"
  done
  for _ in 1 2 3 4 5 6; do
    (cd "$FIXTURE" && "$GO_BIN" status all) | grep 'output missing' | sed -E 's/.*output missing: (.*)\)/\1/' | while read -r p; do
      mkdir -p "$FIXTURE/$p"
    done
  done
}

echo "== layer 1: CLI matrix on the real tree =="
rm -rf "$GOLDEN"; mkdir -p "$GOLDEN"

for view in plan list status; do
  for target in fast full builds all; do
    run_case "$SHADLESS_ROOT" "$view" "$target"
  done
  run_case "$SHADLESS_ROOT" "$view" --gates-only fast
  run_case "$SHADLESS_ROOT" "$view" --builds-only full
  run_case "$SHADLESS_ROOT" "$view" --force fast
done

# every node in the graph (incl. contracts:* shards) through every view
mapfile -t ALL_IDS < <(cd "$SHADLESS_ROOT" && "$GO_BIN" plan all)
for id in "${ALL_IDS[@]}"; do
  run_case "$SHADLESS_ROOT" plan "$id"
  run_case "$SHADLESS_ROOT" status "$id"
  run_case "$SHADLESS_ROOT" list "$id"
  run_case "$SHADLESS_ROOT" inputs "$id"
done
for id in emit demo contracts coverage pack reproducible; do
  run_case "$SHADLESS_ROOT" inputs "$id" --produces
done

# multi-target combinations and flag interactions
run_case "$SHADLESS_ROOT" plan pin emit
run_case "$SHADLESS_ROOT" plan convert demo-css
run_case "$SHADLESS_ROOT" plan contracts:button style-parity
run_case "$SHADLESS_ROOT" list pin emit
run_case "$SHADLESS_ROOT" plan --gates-only --builds-only fast

# error paths: usage, unknown command, unknown node, unknown flag
run_case "$SHADLESS_ROOT" plan
run_case "$SHADLESS_ROOT" list
run_case "$SHADLESS_ROOT" status
run_case "$SHADLESS_ROOT" plan nope
run_case "$SHADLESS_ROOT" list nope
run_case "$SHADLESS_ROOT" status nope
run_case "$SHADLESS_ROOT" plan --bogus
run_case "$SHADLESS_ROOT" inputs
run_case "$SHADLESS_ROOT" inputs --produces
run_case "$SHADLESS_ROOT" inputs nope
run_case "$SHADLESS_ROOT" bogus
run_case "$SHADLESS_ROOT"

echo "== layer 2: key parity on the real tree =="
(cd "$SHADLESS_ROOT" && "$KEYS_BIN") > /tmp/keys.go 2>/tmp/keys.go.err || { echo "goprobe failed"; exit 1; }
(cd "$SHADLESS_ROOT" && "$RS_BIN" __keys) > /tmp/keys.rs 2>/tmp/keys.rs.err
if cmp -s /tmp/keys.go /tmp/keys.rs; then
  pass=$((pass+1)); echo "keys: $(wc -l < /tmp/keys.go) nodes byte-identical"
else
  fail=$((fail+1)); echo "FAIL: keys differ"; diff /tmp/keys.go /tmp/keys.rs | head -10
fi

echo "== layer 3: discriminating fixture (stamped by the GO side) =="
mk_fixture
if cmp -s <(cd "$FIXTURE" && "$GO_BIN" status all) <(cd "$FIXTURE" && "$RS_BIN" status all); then
  pass=$((pass+1)); echo "fixture status all: identical"
else
  fail=$((fail+1)); echo "FAIL: fixture status all"; diff <(cd "$FIXTURE" && "$GO_BIN" status all) <(cd "$FIXTURE" && "$RS_BIN" status all) | head -12
fi
go_status="$(cd "$FIXTURE" && "$GO_BIN" status all)"
go_fresh=$(printf '%s\n' "$go_status" | grep -c ' fresh$')
go_stale=$(printf '%s\n' "$go_status" | grep -c 'STALE')
if [ "$go_fresh" -lt 39 ] || [ "$go_stale" != "0" ]; then
  fail=$((fail+1)); echo "FAIL: fixture not fully stamped ($go_fresh fresh, $go_stale stale) — not discriminating"
else
  pass=$((pass+1)); echo "fixture freshness discriminating: $go_fresh fresh, 1 NEVER-FRESH, 0 stale"
fi
GOLDEN_DIR="$RS_ROOT/tests/golden-fixture"
for target in fast full builds; do
  run_case "$FIXTURE" status "$target"
  run_case "$FIXTURE" plan "$target"
done
GOLDEN_DIR="$GOLDEN"

# layer 4: runner semantics on stamped fixtures. Parameterized by -j so the
# flakiness sweep can replay it at several concurrency levels; each scenario
# is a fresh pair of fixtures, Go runs one, Rust the other.
layer4() {
  local jobs="$1"
  local J="PIPELINE_PARALLEL=$jobs"

  # a) fresh-skip: everything stamped+present -> all skipped, nothing runs
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$GO_BIN" run builds) > /tmp/r.go.out 2> /tmp/r.go.err
  go_code=$?
  (cd "$FIXTURE" && env "$J" "$RS_BIN" run builds) > /tmp/r.rs.out 2> /tmp/r.rs.err
  rs_code=$?
  compare_normalized "fresh-skip stdout (j$jobs)" /tmp/r.go.out /tmp/r.rs.out
  if [ "$go_code" = "$rs_code" ] && [ "$go_code" = "0" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): fresh-skip exit codes: go=$go_code rs=$rs_code (want 0/0)"
  fi
  # 17 build nodes + pin (pulled in as convert/rtl-dict/docs-upstream-mirror's need)
  grep -q "ran 0, skipped 18 " /tmp/r.go.out && pass=$((pass+1)) || { fail=$((fail+1)); echo "FAIL(j$jobs): fresh-skip summary"; grep "ran " /tmp/r.go.out; }
  # no stamp was rewritten and no stamp removed: status still all fresh
  if [ "$(cd "$FIXTURE" && "$GO_BIN" status builds | grep -c ' fresh$')" = "18" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): fresh-skip disturbed the stamps"
  fi

  # b) --force on builds: every command fails instantly on the fixture (there is
  #    no ./build/pipeline binary, no go module); failed nodes lose their stamps
  mk_fixture
  local had_stamp=0; [ -f "$FIXTURE/pipeline/stamps/build-js" ] && had_stamp=1
  (cd "$FIXTURE" && env "$J" "$GO_BIN" run --force builds) > /tmp/r.go.out 2> /tmp/r.go.err
  go_code=$?
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$RS_BIN" run --force builds) > /tmp/r.rs.out 2> /tmp/r.rs.err
  rs_code=$?
  compare_normalized "--force builds stdout (j$jobs)" /tmp/r.go.out /tmp/r.rs.out
  compare_normalized "--force builds stderr (j$jobs)" /tmp/r.go.err /tmp/r.rs.err
  if [ "$go_code" = "$rs_code" ] && [ "$go_code" = "1" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): --force exit codes: go=$go_code rs=$rs_code (want 1/1)"
  fi
  if [ "$had_stamp" = "1" ] && [ ! -f "$FIXTURE/pipeline/stamps/build-js" ]; then
    pass=$((pass+1)); echo "stamp-on-failure (j$jobs): failed node's stamp removed"
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): stamp-on-failure (had=$had_stamp)"
  fi

  # c) --keep-going with failures: the run report (cmd+tail only, no timing)
  #    must be byte-identical, and blocked-node accounting must match
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$GO_BIN" run --keep-going --force builds) > /dev/null 2>&1
  cp "$FIXTURE/build/gates/run-report.json" /tmp/report.go.json
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$RS_BIN" run --keep-going --force builds) > /dev/null 2>&1
  if [ ! -f "$FIXTURE/build/gates/run-report.json" ]; then
    fail=$((fail+1)); echo "FAIL(j$jobs): rust wrote no run report"
  elif cmp -s /tmp/report.go.json "$FIXTURE/build/gates/run-report.json"; then
    pass=$((pass+1)); echo "run-report.json (j$jobs): byte-identical ($(wc -c < /tmp/report.go.json) bytes)"
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): run-report.json differs"
    diff /tmp/report.go.json "$FIXTURE/build/gates/run-report.json" | head -12
  fi

  # d) --keep-going with NO failures: the empty report — Go marshals the
  #    initialized empty map as {} and empty slices as []; the port must too
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$GO_BIN" run --keep-going builds) > /dev/null 2>&1
  cp "$FIXTURE/build/gates/run-report.json" /tmp/report.go.empty
  mk_fixture
  (cd "$FIXTURE" && env "$J" "$RS_BIN" run --keep-going builds) > /dev/null 2>&1
  if cmp -s /tmp/report.go.empty "$FIXTURE/build/gates/run-report.json"; then
    pass=$((pass+1)); echo "empty run-report.json (j$jobs): byte-identical"
  else
    fail=$((fail+1)); echo "FAIL(j$jobs): empty run-report.json differs"
    diff /tmp/report.go.empty "$FIXTURE/build/gates/run-report.json" | head -8
  fi
}

echo "== layer 4: runner semantics on stamped fixtures (flakiness sweep) =="
for jobs in 1 1 1 1 1 1 1 1 1 1 4 4 4 8; do
  layer4 "$jobs"
done

echo
echo "== $pass passed, $fail failed =="
[ "$fail" = 0 ]
