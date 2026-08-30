package main

// The JS surface builder.
//
// The port was verified against the JS by rebuilding dist/ and comparing:
// shadless.js, shadless.min.js, dist/js/** and dist/esm/** all came back
// byte-identical, minified files included. These tests pin the parts where a
// silent change would still produce *valid* output — which is the dangerous
// kind, because `reproducible` would then be the only thing that noticed, on
// a tree where the wrong bytes had already been committed.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/evanw/esbuild/pkg/api"
)

// The `;` between concatenated IIFEs is load-bearing, and the failure it
// prevents is SILENT rather than loud: `})(window)` followed on the next line
// by `(function () {` is perfectly valid syntax — it parses as a CALL, passing
// the second IIFE as an argument to the result of the first. Nothing throws at
// parse time; the base simply never runs. So this asserts the separator
// itself, because no parser will complain about its absence.
func TestUnitIifeBaseSeparatesWithSemicolon(t *testing.T) {
	got := iifeBase("(function(){})(window)", "(function(){})()")
	if !strings.Contains(got, ")(window)\n;\n(function") {
		t.Errorf("the two IIFEs are not separated by `;`:\n%s", got)
	}
	// the hazard, spelled out: without the separator this is one call
	// expression, and esbuild accepts it without complaint
	unseparated := "(function(){})(window)\n(function(){})()"
	if r := api.Transform(unseparated, api.TransformOptions{}); len(r.Errors) > 0 {
		t.Fatal("premise changed: the unseparated form is now a syntax error, " +
			"so the silent-call hazard this separator guards no longer exists")
	}
	if r := api.Transform(got, api.TransformOptions{}); len(r.Errors) > 0 {
		t.Errorf("the separated form does not parse: %v", r.Errors)
	}
}

func TestUnitEsmBaseExportsThePublicMembers(t *testing.T) {
	got := esmBase("K", "C")
	if !strings.Contains(got, "export default shadless") {
		t.Error("no default export")
	}
	// the named list must be exactly namedExports, in order
	want := "export const { " + strings.Join(namedExports, ", ") + " } = shadless"
	if !strings.Contains(got, want) {
		t.Errorf("named exports line missing or reordered:\n%s", got)
	}
	// the base body must still be there — the ESM wrapper is additive, and
	// window.shadless is what the component files address
	if !strings.Contains(got, "K") || !strings.Contains(got, "C") {
		t.Error("the ESM base dropped the IIFE body")
	}
	if !strings.Contains(got, "const shadless = globalThis.shadless") {
		t.Error("the ESM base does not pick up the global")
	}
}

// A component module must import the base FIRST, so a bundler evaluates it
// no matter how the consumer orders its own imports.
func TestUnitEsmComponentImportsBaseFirst(t *testing.T) {
	got := esmComponent("register('x')")
	if !strings.HasPrefix(got, `import "./shadless.mjs"`) {
		t.Errorf("the base import is not first:\n%s", got)
	}
	if !strings.Contains(got, "register('x')") {
		t.Error("the component body was dropped")
	}
}

// namedExports must match what core.js actually puts on the global. The JS
// kept these in step by hand; here the source is the check.
func TestUnitRuntimeNamedExportsMatchCore(t *testing.T) {
	root := repoRoot(t)
	b, err := os.ReadFile(filepath.Join(root, "src/runtime/core.js"))
	if err != nil {
		t.Fatal(err)
	}
	src := string(b)
	for _, name := range namedExports {
		// each re-exported member has to exist on the global object literal
		if !strings.Contains(src, name) {
			t.Errorf("namedExports lists %q but src/runtime/core.js never mentions it", name)
		}
	}
	if len(namedExports) == 0 {
		t.Fatal("namedExports is empty")
	}
}

// The committed dist/esm/shadless.mjs must be what esmBase produces from the
// current sources. This is the check that catches a hand-edit to the shipped
// module, or a change here that was never rebuilt.
func TestUnitShippedEsmMatchesBuilder(t *testing.T) {
	root := repoRoot(t)
	read := func(rel string) string {
		b, err := os.ReadFile(filepath.Join(root, rel))
		if err != nil {
			t.Skipf("%s missing (unbuilt tree): %v", rel, err)
		}
		return string(b)
	}
	kernel := read("vendor/radix-kernel.iife.js")
	core := read("src/runtime/core.js")
	if got, want := read("dist/esm/shadless.mjs"), esmBase(kernel, core); got != want {
		t.Errorf("dist/esm/shadless.mjs is not what esmBase produces (len %d vs %d)", len(got), len(want))
	}
	if got, want := read("dist/shadless.js"), iifeBase(kernel, core); got != want {
		t.Errorf("dist/shadless.js is not what iifeBase produces (len %d vs %d)", len(got), len(want))
	}
}

// buildJs must rebuild dist/js and dist/esm from scratch: a component deleted
// upstream that left its file behind would keep being shipped, and the pack
// gate reads the directory, not the source list.
func TestUnitBuildJsClearsStaleOutputs(t *testing.T) {
	root := t.TempDir()
	write := func(rel, content string) {
		t.Helper()
		p := filepath.Join(root, rel)
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write("vendor/radix-kernel.iife.js", "var k=1")
	write("vendor/embla-carousel.iife.js", "var e=1")
	write("src/runtime/core.js", "var c=1")
	write("src/runtime/shadless.d.ts", "export {}\n")
	write("src/runtime/components/dialog.js", "var d=1")
	// a leftover from a previous build, for a component that no longer exists
	write("dist/js/gone.js", "var gone=1")
	write("dist/esm/gone.mjs", "var gone=1")

	names, err := buildJs(root, "dist")
	if err != nil {
		t.Fatal(err)
	}
	if len(names) != 1 || names[0] != "dialog" {
		t.Errorf("names = %v, want [dialog]", names)
	}
	for _, stale := range []string{"dist/js/gone.js", "dist/esm/gone.mjs"} {
		if _, err := os.Stat(filepath.Join(root, stale)); !os.IsNotExist(err) {
			t.Errorf("%s survived the rebuild — a retired component would keep shipping", stale)
		}
	}
	for _, want := range []string{
		"dist/shadless.js", "dist/shadless.min.js",
		"dist/js/dialog.js", "dist/esm/dialog.mjs", "dist/esm/dialog.d.ts",
		"dist/esm/shadless.mjs", "dist/esm/shadless.min.mjs", "dist/esm/shadless.d.ts",
	} {
		if _, err := os.Stat(filepath.Join(root, want)); err != nil {
			t.Errorf("%s was not written", want)
		}
	}
}

// carousel is the one component that carries a vendored engine in front of
// its behavior; dropping that would ship a component whose engine is missing.
func TestUnitBuildJsPrependsEmblaToCarousel(t *testing.T) {
	root := t.TempDir()
	write := func(rel, content string) {
		t.Helper()
		p := filepath.Join(root, rel)
		os.MkdirAll(filepath.Dir(p), 0o755)
		if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write("vendor/radix-kernel.iife.js", "var k=1")
	write("vendor/embla-carousel.iife.js", "var EMBLA=1")
	write("src/runtime/core.js", "var c=1")
	write("src/runtime/shadless.d.ts", "export {}\n")
	write("src/runtime/components/carousel.js", "var carousel=1")
	write("src/runtime/components/dialog.js", "var d=1")

	if _, err := buildJs(root, "dist"); err != nil {
		t.Fatal(err)
	}
	b, err := os.ReadFile(filepath.Join(root, "dist/js/carousel.js"))
	if err != nil {
		t.Fatal(err)
	}
	got := string(b)
	if !strings.HasPrefix(got, "var EMBLA=1") {
		t.Errorf("the embla engine is not in front:\n%s", got)
	}
	if !strings.Contains(got, "var carousel=1") {
		t.Error("the carousel behavior was dropped")
	}
	// and only carousel gets it
	d, _ := os.ReadFile(filepath.Join(root, "dist/js/dialog.js"))
	if strings.Contains(string(d), "EMBLA") {
		t.Error("the engine leaked into another component")
	}
}

// The minified output is committed and `reproducible` compares it byte for
// byte, so minification must be deterministic run to run.
func TestUnitMinifyIsDeterministic(t *testing.T) {
	src := "(function (window) { var longName = 1; return longName })(window)"
	first, err := minify(src, api.FormatDefault)
	if err != nil {
		t.Fatal(err)
	}
	for i := 0; i < 3; i++ {
		got, err := minify(src, api.FormatDefault)
		if err != nil {
			t.Fatal(err)
		}
		if got != first {
			t.Fatalf("minify is not deterministic:\n%q\nvs\n%q", first, got)
		}
	}
	if len(first) >= len(src) {
		t.Errorf("nothing was minified: %q", first)
	}
}

// The ESM min is emitted with format=esm on purpose: the IIFE min has no
// export statement, so an ESM consumer of the min entry got undefined.
func TestUnitMinifyEsmKeepsExports(t *testing.T) {
	got, err := minify("const shadless = 1\nexport default shadless\n", api.FormatESModule)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(got, "export") {
		t.Errorf("the export statement did not survive minification: %q", got)
	}
}

func TestUnitMinifyReportsErrors(t *testing.T) {
	if _, err := minify("function (", api.FormatDefault); err == nil {
		t.Error("a syntax error was accepted")
	}
}
