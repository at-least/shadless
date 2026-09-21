#!/usr/bin/env bash
# dist byte-identity harness: whatever the Rust build-js/emit chain writes
# under dist/ must equal the committed tree byte-for-byte. Snapshot before,
# run, diff, restore.
set -u
RS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$RS_ROOT/..}"
RS_BIN="$(node "$RS_ROOT/../tools/pipeline-bin.mjs")"

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
# the union of both trees: walking only the post-run tree never sees a file
# the run DELETED, and the harness's whole job is exactly that comparison
while IFS= read -r rel; do
  total=$((total+1))
  if [ -e "/tmp/dist-before/$rel" ] && [ -e "dist/$rel" ] && cmp -s "/tmp/dist-before/$rel" "dist/$rel"; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    if [ $fail -le 8 ]; then
      if [ -e "/tmp/dist-before/$rel" ] && [ ! -e "dist/$rel" ]; then
        echo "DIFF: dist/$rel (deleted by the run)"
      elif [ ! -e "/tmp/dist-before/$rel" ]; then
        echo "DIFF: dist/$rel (not in the committed tree)"
      else
        echo "DIFF: dist/$rel"
        cmp "/tmp/dist-before/$rel" "dist/$rel" 2>/dev/null | head -1
      fi
    fi
  fi
done < <(
  { (cd dist && find . -type f -printf '%P\n')
    (cd /tmp/dist-before && find . -type f -printf '%P\n')
  } | sort -u
)

echo "== dist: $pass/$total byte-identical, $fail differ ($CMD exit=$rc) =="
rm -rf dist && cp -r /tmp/dist-before dist
git checkout -q dist/ 2>/dev/null
[ "$fail" = 0 ] && [ "$rc" = 0 ]
