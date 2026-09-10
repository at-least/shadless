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
RS_BIN="$RS_ROOT/target/release/pipeline"

cd "$SHADLESS_ROOT" || exit 2

if git status --short generated/ | grep -q .; then
  echo "ABORT: committed generated/ is dirty — restore it before measuring"
  git status --short generated/ | head -10
  exit 2
fi

rm -rf /tmp/ir-before && cp -r generated/ir /tmp/ir-before

"$RS_BIN" convert
rc=$?

total=0; pass=0; fail=0
while IFS= read -r f; do
  rel="${f#generated/ir/}"
  total=$((total+1))
  if [ -f "/tmp/ir-before/$rel" ] && cmp -s "/tmp/ir-before/$rel" "$f"; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    if [ $fail -le 8 ]; then
      echo "DIFF: $rel"
      if [ -f "/tmp/ir-before/$rel" ] && [ -f "$f" ]; then
        cmp "/tmp/ir-before/$rel" "$f" | head -2
        # first divergent byte with context from both sides
        off="$(cmp "/tmp/ir-before/$rel" "$f" 2>/dev/null | sed -E 's/.*byte ([0-9]+).*/\1/')"
        if [ -n "$off" ]; then
          echo "  go: $(dd if="/tmp/ir-before/$rel" bs=1 skip=$((off>40?off-40:0)) count=80 2>/dev/null)"
          echo "  rs: $(dd if="$f" bs=1 skip=$((off>40?off-40:0)) count=80 2>/dev/null)"
        fi
      fi
    fi
  fi
done < <(find generated/ir -name '*.json' | sort)

echo "== IR: $pass/$total byte-identical, $fail differ (convert exit=$rc) =="
# restore the committed tree either way — the measurement must not mutate it
rm -rf generated/ir && cp -r /tmp/ir-before generated/ir
if [ "${1:-}" = "--keep" ]; then
  echo "(--keep: leaving the rust-converted tree in place)"
else
  git checkout -q generated/ 2>/dev/null
fi
[ "$fail" = 0 ] && [ "$rc" = 0 ]
