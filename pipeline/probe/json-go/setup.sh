#!/usr/bin/env bash
# Build the jsonorder fixture probe: the REAL pipeline/jsonorder.go plus a
# fixture main, producing golden.txt for the Rust serializer's unit test.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$HERE/../../../shadless}"
cp "$SHADLESS_ROOT/pipeline/jsonorder.go" "$HERE/jsonorder.go"
cd "$HERE"
[ -f go.mod ] || printf 'module jsonorder\n\ngo 1.24\n' > go.mod
go run . 
echo "golden.txt: $(wc -c < golden.txt) bytes"
