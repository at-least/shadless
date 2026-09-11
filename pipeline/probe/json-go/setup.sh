#!/usr/bin/env bash
# HISTORICAL: regenerates golden.txt (which src/jsonorder.rs's test reads)
# by building the Go jsonorder.go. The Go engine was removed (tag
# go-engine-final holds pipeline/jsonorder.go); to regenerate, check out
# that tag in a second clone and point SHADLESS_ROOT at it.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:?point SHADLESS_ROOT at a go-engine-final checkout}"
cp "$SHADLESS_ROOT/pipeline/jsonorder.go" "$HERE/jsonorder.go"
cd "$HERE"
[ -f go.mod ] || printf 'module jsonorder\n\ngo 1.24\n' > go.mod
go run . 
echo "golden.txt: $(wc -c < golden.txt) bytes"
