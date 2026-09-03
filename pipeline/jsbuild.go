package main

// The JS product surface — ported from tools/build-js.mjs. One base plus one
// file per component, mirroring the CSS surface (core + shadless/<name>.css).
//
//	dist/shadless.js       vendored radix kernel + src/runtime/core.js
//	                       (delegation engine, registry, theme, auto-init)
//	dist/shadless.min.js   the same, minified
//	dist/js/<name>.js      src/runtime/components/<name>.js — registers the
//	                       component's behavior with the base; carousel
//	                       bundles the vendored embla engine in front
//
//	dist/esm/shadless.min.mjs  the same, minified (the `import` condition of
//	                       shadless/js.min)
//	dist/esm/shadless.mjs  the base as an ES module: the same IIFE body
//	                       (window.shadless is still set — the component files
//	                       address it by that global) followed by
//	                       `export default shadless` + named exports
//	dist/esm/<name>.mjs    `import "./shadless.mjs"` + the component file, so
//	                       a bundler evaluates the base first no matter how a
//	                       consumer orders its imports
//	dist/esm/*.d.ts        src/runtime/shadless.d.ts beside the base; each
//	                       component module is typed as a side-effect module
//
// Static components have no file; the docs say so per component.
//
// This is the one tool where a Go dependency was worth taking. esbuild IS Go —
// the npm package ships this same implementation — so github.com/evanw/esbuild
// can be pinned to the exact version package.json carries, and the minified
// output is byte-identical to what the JS produced. That matters because
// dist/shadless.min.js is committed and `reproducible` compares it byte for
// byte; a minifier that merely produced *valid* output would fail that gate
// forever. (The same reasoning is why the browser gates did NOT move: the only
// usable playwright-go pins a different chromium than the JS side runs.)

// NOTE: this file is jsbuild.go, not build_js.go. Go gives any file ending in
// _<goos>.go an implicit build constraint, and `js` is a GOOS — build_js.go
// compiles only for GOOS=js and is silently dropped from every normal build.

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/evanw/esbuild/pkg/api"
)

// namedExports are the members of window.shadless re-exported by name. Kept in
// step with core.js's `global.shadless = {…}`; TestUnitRuntimeNamedExports
// asserts the set against the source.
var namedExports = []string{
	"init", "initAll", "destroy", "refresh", "start", "stop",
	"register", "get", "instances", "h", "theme",
}

// iifeBase joins the two IIFEs. The `;` is load-bearing: `})(window)` followed
// by `(function () {` would otherwise parse as a CALL.
func iifeBase(kernel, core string) string {
	return kernel + "\n;\n" + core
}

func esmBase(kernel, core string) string {
	return iifeBase(kernel, core) +
		"\n;\nconst shadless = globalThis.shadless\nexport default shadless\nexport const { " +
		strings.Join(namedExports, ", ") + " } = shadless\n"
}

func esmComponent(src string) string { return "import \"./shadless.mjs\"\n;\n" + src }

// minify runs esbuild's transform with the same options the JS passed.
func minify(src string, format api.Format) (string, error) {
	r := api.Transform(src, api.TransformOptions{
		MinifyWhitespace:  true,
		MinifyIdentifiers: true,
		MinifySyntax:      true,
		Target:            api.ES2017,
		Format:            format,
	})
	if len(r.Errors) > 0 {
		return "", fmt.Errorf("esbuild: %s", r.Errors[0].Text)
	}
	return string(r.Code), nil
}

// buildJs writes the whole JS surface under dist and returns the component
// names it emitted.
func buildJs(root, dist string) ([]string, error) {
	read := func(rel string) (string, error) {
		b, err := os.ReadFile(filepath.Join(root, rel))
		return string(b), err
	}
	write := func(rel, content string) error {
		p := filepath.Join(root, dist, rel)
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			return err
		}
		return os.WriteFile(p, []byte(content), 0o644)
	}

	kernel, err := read("vendor/radix-kernel.iife.js")
	if err != nil {
		return nil, err
	}
	core, err := read("src/runtime/core.js")
	if err != nil {
		return nil, err
	}
	base := iifeBase(kernel, core)
	if err := write("shadless.js", base); err != nil {
		return nil, err
	}
	minBase, err := minify(base, api.FormatDefault)
	if err != nil {
		return nil, err
	}
	if err := write("shadless.min.js", minBase); err != nil {
		return nil, err
	}

	// dist/js and dist/esm are rebuilt from scratch: a component deleted
	// upstream must not leave its file behind, where the pack gate would keep
	// shipping it.
	for _, d := range []string{"js", "esm"} {
		if err := os.RemoveAll(filepath.Join(root, dist, d)); err != nil {
			return nil, err
		}
		if err := os.MkdirAll(filepath.Join(root, dist, d), 0o755); err != nil {
			return nil, err
		}
	}

	esm := esmBase(kernel, core)
	if err := write("esm/shadless.mjs", esm); err != nil {
		return nil, err
	}
	// the `import` condition of shadless/js.min — the IIFE min has no
	// export statement, so an ESM consumer of the min entry got undefined
	minESM, err := minify(esm, api.FormatESModule)
	if err != nil {
		return nil, err
	}
	if err := write("esm/shadless.min.mjs", minESM); err != nil {
		return nil, err
	}
	dts, err := read("src/runtime/shadless.d.ts")
	if err != nil {
		return nil, err
	}
	if err := write("esm/shadless.d.ts", dts); err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(filepath.Join(root, "src/runtime/components"))
	if err != nil {
		return nil, err
	}
	var files []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".js") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)

	var names []string
	for _, f := range files {
		name := strings.TrimSuffix(f, ".js")
		src, err := read("src/runtime/components/" + f)
		if err != nil {
			return nil, err
		}
		if name == "carousel" {
			embla, err := read("vendor/embla-carousel.iife.js")
			if err != nil {
				return nil, err
			}
			src = embla + "\n;\n" + src
		}
		if err := write("js/"+f, src); err != nil {
			return nil, err
		}
		if err := write("esm/"+name+".mjs", esmComponent(src)); err != nil {
			return nil, err
		}
		if err := write("esm/"+name+".d.ts",
			"// registers the "+name+" behavior with the base (side-effect module)\n"+
				"import \"./shadless.mjs\"\nexport {}\n"); err != nil {
			return nil, err
		}
		names = append(names, name)
	}
	return names, nil
}

func runBuildJs() int {
	root, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	names, err := buildJs(root, "dist")
	if err != nil {
		fmt.Fprintln(os.Stderr, "build-js:", err)
		return 1
	}
	fmt.Printf("build-js: dist/shadless.js (base) + %d component files in dist/js/ (+ dist/esm/ mirrors)\n",
		len(names))
	return 0
}
