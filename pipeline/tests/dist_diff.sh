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
# The snapshot lives in a private scratch dir, and dist/ is restored from it
# however the script ends: a fixed /tmp name outlived every run, and an
# interrupted run left the engine's dist/ in place with the only pristine
# copy in /tmp.
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/shadless-dist-diff-XXXXXX")" || exit 2
BEFORE="$SCRATCH/dist-before"
trap 'trap "" INT TERM; rm -rf "$SCRATCH"' EXIT # until the snapshot is whole
cp -r dist "$BEFORE" || exit 2
trap 'trap "" INT TERM; rm -rf dist && cp -r "$BEFORE" dist; git checkout -q dist/ 2>/dev/null; rm -rf "$SCRATCH"' EXIT

CMD="${1:-build-js}"
shift 2>/dev/null || true
"$RS_BIN" "$CMD" "$@"
rc=$?

total=0; pass=0; fail=0
# the union of both trees: walking only the post-run tree never sees a file
# the run DELETED, and the harness's whole job is exactly that comparison
while IFS= read -r rel; do
  total=$((total+1))
  if [ -e "$BEFORE/$rel" ] && [ -e "dist/$rel" ] && cmp -s "$BEFORE/$rel" "dist/$rel"; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    if [ $fail -le 8 ]; then
      if [ -e "$BEFORE/$rel" ] && [ ! -e "dist/$rel" ]; then
        echo "DIFF: dist/$rel (deleted by the run)"
      elif [ ! -e "$BEFORE/$rel" ]; then
        echo "DIFF: dist/$rel (not in the committed tree)"
      else
        echo "DIFF: dist/$rel"
        cmp "$BEFORE/$rel" "dist/$rel" 2>/dev/null | head -1
      fi
    fi
  fi
done < <(
  { (cd dist && find . -type f -printf '%P\n')
    (cd "$BEFORE" && find . -type f -printf '%P\n')
  } | sort -u
)

echo "== dist: $pass/$total byte-identical, $fail differ ($CMD exit=$rc) =="
[ "$fail" = 0 ] && [ "$rc" = 0 ]
