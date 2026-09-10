#!/usr/bin/env bash
# dist byte-identity harness: whatever the Rust build-js/emit chain writes
# under dist/ must equal the committed tree byte-for-byte. Snapshot before,
# run, diff, restore.
set -u
RS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$RS_ROOT/..}"
RS_BIN="$RS_ROOT/target/release/pipeline"

cd "$SHADLESS_ROOT" || exit 2
if git status --short dist/ | grep -q .; then
  echo "ABORT: committed dist/ is dirty"; exit 2
fi
rm -rf /tmp/dist-before && cp -r dist /tmp/dist-before

CMD="${1:-build-js}"
shift 2>/dev/null || true
"$RS_BIN" "$CMD" "$@"
rc=$?

total=0; pass=0; fail=0
while IFS= read -r rel; do
  total=$((total+1))
  if [ -e "/tmp/dist-before/$rel" ] && cmp -s "/tmp/dist-before/$rel" "dist/$rel"; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    if [ $fail -le 8 ]; then
      echo "DIFF: dist/$rel"
      [ -f "/tmp/dist-before/$rel" ] && cmp "/tmp/dist-before/$rel" "dist/$rel" 2>/dev/null | head -1
    fi
  fi
done < <(cd dist && find . -type f -printf '%P\n' | sort)

echo "== dist: $pass/$total byte-identical, $fail differ ($CMD exit=$rc) =="
rm -rf dist && cp -r /tmp/dist-before dist
git checkout -q dist/ 2>/dev/null
[ "$fail" = 0 ] && [ "$rc" = 0 ]
