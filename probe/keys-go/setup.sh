#!/usr/bin/env bash
# Build goprobe: the shadless pipeline package with main() swapped for a key
# printer. This is the KEY-PARITY ORACLE — the real Go keyer code, unmodified,
# so any drift in the Rust port shows up as a diff. Re-run to refresh after
# upstream pipeline changes.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
SHADLESS_ROOT="${SHADLESS_ROOT:-$HERE/../../../shadless}"

rm -rf "$HERE/src"
mkdir -p "$HERE/src"
# the whole package compiles together; tests and the stamps store are not code
(cd "$SHADLESS_ROOT/pipeline" && tar --exclude='*_test.go' --exclude='./stamps' --exclude='./build' -cf - .) | tar -xf - -C "$HERE/src"

# strip everything from `func main() {` to EOF, drop now-unused imports, and
# append the printer
python3 - "$HERE/src/main.go" <<'EOF'
import re
import sys
path = sys.argv[1]
src = open(path).read()
cut = src.index("func main() {")
src = src[:cut]
for imp in ['"runtime"', '"strconv"', '"time"']:
    src = src.replace("\t" + imp + "\n", "")
src += '''
func main() {
	root, err := os.Getwd()
	die(err)
	g, err := LoadGraphAt(root)
	die(err)
	k := NewKeyer(root, g)
	for _, id := range g.IDs() {
		key, ok, err := k.Key(id)
		die(err)
		if !ok {
			fmt.Printf("%s\\t0\\t\\n", id)
			continue
		}
		n, _ := g.Node(id)
		fmt.Printf("%s\\t1\\t%s\\n", id, stampValue(root, n, key))
	}
}
'''
open(path, "w").write(src)
EOF

cd "$HERE/src"
GOPROXY=off GOFLAGS=-mod=mod go build -o "$HERE/goprobe" .
echo "goprobe built: $HERE/goprobe"
