#!/usr/bin/env bash
# IR byte-identity harness (M4 acceptance).
#
# The Rust convert must reproduce the committed generated/ir byte-for-byte.
# Go's convert was verified deterministic (two runs + committed tree all
# identical, 2026-09-07), so the committed tree IS the oracle. Safety: the
# pre-run state is snapshotted and restored on any mismatch.
#
# Stages: during the staged port, `pipeline convert` runs whatever is ported
# and this harness reports per-file pass/fail with first-divergence context —
# a failing file localizes itself in seconds.
set -u
RS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$RS_ROOT/..}"
RS_BIN="$(node "$RS_ROOT/../tools/pipeline-bin.mjs")"

cd "$SHADLESS_ROOT" || exit 2

if git status --short generated/ | grep -q .; then
  echo "ABORT: committed generated/ is dirty — restore it before measuring"
  git status --short generated/ | head -10
  exit 2
fi

# The snapshot lives in a private scratch dir, and generated/ir is restored
# from it however the script ends (unless --keep): a fixed /tmp name
# outlived every run, and an interrupted run left the converted tree in
# place with the only pristine copy in /tmp.
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/shadless-ir-diff-XXXXXX")" || exit 2
BEFORE="$SCRATCH/ir-before"
trap 'trap "" INT TERM; rm -rf "$SCRATCH"' EXIT # until the snapshot is whole
cp -r generated/ir "$BEFORE" || exit 2
if [ "${1:-}" != "--keep" ]; then
  trap 'trap "" INT TERM; rm -rf generated/ir && cp -r "$BEFORE" generated/ir; git checkout -q generated/ 2>/dev/null; rm -rf "$SCRATCH"' EXIT
fi

"$RS_BIN" convert
rc=$?

total=0; pass=0; fail=0
while IFS= read -r f; do
  rel="${f#generated/ir/}"
  total=$((total+1))
  if [ -f "$BEFORE/$rel" ] && cmp -s "$BEFORE/$rel" "$f"; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    if [ $fail -le 8 ]; then
      echo "DIFF: $rel"
      if [ -f "$BEFORE/$rel" ] && [ -f "$f" ]; then
        cmp "$BEFORE/$rel" "$f" | head -2
        # first divergent byte with context from both sides
        off="$(cmp "$BEFORE/$rel" "$f" 2>/dev/null | sed -E 's/.*byte ([0-9]+).*/\1/')"
        if [ -n "$off" ]; then
          echo "  go: $(dd if="$BEFORE/$rel" bs=1 skip=$((off>40?off-40:0)) count=80 2>/dev/null)"
          echo "  rs: $(dd if="$f" bs=1 skip=$((off>40?off-40:0)) count=80 2>/dev/null)"
        fi
      fi
    fi
  fi
done < <(find generated/ir -name '*.json' | sort)

echo "== IR: $pass/$total byte-identical, $fail differ (convert exit=$rc) =="
# the EXIT trap restores the committed tree — the measurement must not
# mutate it — unless --keep asked for the converted one
[ "${1:-}" = "--keep" ] && echo "(--keep: leaving the rust-converted tree in place)"
[ "$fail" = 0 ] && [ "$rc" = 0 ]
