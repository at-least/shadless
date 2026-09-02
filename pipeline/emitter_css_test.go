package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// Golden: pipeline/emitter_css.go's componentCss against src/emitter/css.mjs's,
// on every IR in generated/ir. Any byte difference in any component's CSS
// is a porting bug.
func TestUnitEmitterCssGolden(t *testing.T) {
	root := "/home/newlix/github/at-least/shadless"
	ents, err := os.ReadDir(filepath.Join(root, "generated/ir"))
	if err != nil {
		t.Skip(err)
	}
	goldenDir := t.TempDir()
	var names []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".json") {
			names = append(names, strings.TrimSuffix(e.Name(), ".json"))
		}
	}
	// JS reference output: run css.mjs's componentCss once per IR and dump
	dumpScript := filepath.Join(goldenDir, "dump.mjs")
	dump := `import { readFileSync, writeFileSync } from "node:fs"
process.chdir("/home/newlix/github/at-least/shadless")
const { componentCss, wrapComponentCss } = await import("/home/newlix/github/at-least/shadless/src/emitter/css.mjs")
const outIdx = process.argv.indexOf("--out")
const out = {}
	// slice AFTER --out: argv is [node, script, --out, path, name…]; names start at outIdx+2
	for (const name of process.argv.slice(outIdx + 2)) {
  const ir = JSON.parse(readFileSync("generated/ir/" + name + ".json", "utf8"))
  const css = componentCss(ir)
  out[name] = { rules: css.rules, unlayered: css.unlayered, markers: css.markers,
    anchors: Object.fromEntries(css.anchors), anchorMarkers: Object.fromEntries(css.anchorMarkers),
    wrapped: wrapComponentCss(ir.name, css) }
}
writeFileSync(process.argv[outIdx + 1], JSON.stringify(out))
`
	if err := os.WriteFile(dumpScript, []byte(dump), 0o644); err != nil {
		t.Fatal(err)
	}
	jsOut := filepath.Join(goldenDir, "css-golden.json")
	// jsOut comes second: process.argv[process.argv.indexOf("--out")+1]
	args := []string{dumpScript, "--out", jsOut}
	args = append(args, names...)
	if out, err := exec.Command("node", args...).CombinedOutput(); err != nil {
		t.Fatalf("css.mjs dump: %v\n%s", err, out)
	}
	var golden map[string]struct {
		Rules         []string            `json:"rules"`
		Unlayered     []string            `json:"unlayered"`
		Markers       map[string][]string `json:"markers"`
		Anchors       map[string]string   `json:"anchors"`
		AnchorMarkers map[string][]string `json:"anchorMarkers"`
		Wrapped       string              `json:"wrapped"`
	}
	gb, _ := os.ReadFile(jsOut)
	if len(gb) == 0 || string(gb) == "{}" {
		t.Fatalf("css.mjs dump produced nothing (%d bytes) — names were not passed where the script expects them", len(gb))
	}
	if err := json.Unmarshal(gb, &golden); err != nil {
		t.Fatal(err)
	}

	if err := os.Chdir(root); err != nil {
		t.Fatalf("chdir %s: %v", root, err)
	}
	defer func() { os.Chdir("/home/newlix/github/at-least/shadless/pipeline") }()
	loadSkin()
	failed := 0
	for _, name := range names {
		irb, _ := os.ReadFile(filepath.Join(root, "generated/ir", name+".json"))
		var ir cssIrComponent
		if err := json.Unmarshal(irb, &ir); err != nil {
			t.Errorf("%s: ir parse: %v", name, err)
			continue
		}
		out, err := componentCss(ir)
		if err != nil {
			t.Errorf("%s: %v", name, err)
			continue
		}
		g := golden[name]
		var diff string
		gr := strings.Join(g.Rules, "\n")
		grules := strings.Join(out.rules, "\n")
		if grules != gr {
			diff = firstDiff(grules, gr)
			t.Errorf("%s: rules differ at %s\n--- go ---\n%s\n--- js ---\n%s", name, diff, grules, gr)
			failed++
			continue
		}
		if strings.Join(out.unlayered, "\n") != strings.Join(g.Unlayered, "\n") {
			t.Errorf("%s: unlayered differ", name)
			failed++
			continue
		}
	}
	if failed > 0 {
		t.Fatalf("%d components diverge", failed)
	}
}

func TestUnitEmitterCssWraps(t *testing.T) {
	// wrapComponentCss shape (from css.mjs) — pinned textually
	got := wrapComponentCss("widget", componentCssOut{rules: []string{"  /* r */"}})
	want := "/* widget */\n@layer components {\n  /* r */\n}"
	if got != want {
		t.Errorf("wrap: %q", got)
	}
}

// firstDiff returns line context around the first differing line between a
// and b.
func firstDiff(a, b string) string {
	al, bl := strings.Split(a, "\n"), strings.Split(b, "\n")
	n := len(al)
	if len(bl) > n {
		n = len(bl)
	}
	for i := 0; i < n; i++ {
		x, y := "", ""
		if i < len(al) {
			x = al[i]
		}
		if i < len(bl) {
			y = bl[i]
		}
		if x != y {
			return fmt.Sprintf("line %d:\n  go: %q\n  js: %q", i, x, y)
		}
	}
	return "same"
}
