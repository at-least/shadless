// M0 probe — capture golden esbuild Transform outputs from the Go API so the
// pinned CLI binary (what Rust will spawn) can be diffed against them.
//
// The two functions below are verbatim copies of the call sites under test:
//   esbuildTsx  <- shadless/pipeline/convert.go
//   minify      <- shadless/pipeline/jsbuild.go
// Any drift between this file and those call sites invalidates the probe.
package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/evanw/esbuild/pkg/api"
)

const root = "/home/newlix/github/at-least/shadless"
const outGo = "/home/newlix/github/at-least/shadless-rs/probe/m0/go-out"
const outGoMin = "/home/newlix/github/at-least/shadless-rs/probe/m0/go-out-min"

func die(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, "probe:", err)
		os.Exit(1)
	}
}

func esbuildTsx(src string) (string, error) {
	res := api.Transform(src, api.TransformOptions{
		Loader:      api.LoaderTSX,
		JSX:         api.JSXTransform,
		JSXFactory:  "React.createElement",
		JSXFragment: "React.Fragment",
		Format:      api.FormatESModule,
		Charset:     api.CharsetUTF8,
	})
	if len(res.Errors) > 0 {
		return "", fmt.Errorf("esbuild: %s", res.Errors[0].Text)
	}
	return string(res.Code), nil
}

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

func write(dir, name, content string) {
	die(os.MkdirAll(dir, 0o755))
	die(os.WriteFile(filepath.Join(dir, name), []byte(content), 0o644))
}

func main() {
	ents, err := os.ReadDir(filepath.Join(root, "build/resolved-ui/ui"))
	die(err)
	names := []string{}
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".tsx") {
			names = append(names, strings.TrimSuffix(e.Name(), ".tsx"))
		}
	}
	sort.Strings(names)

	for _, name := range names {
		b, err := os.ReadFile(filepath.Join(root, "build/resolved-ui/ui", name+".tsx"))
		die(err)
		out, err := esbuildTsx(string(b))
		die(err)
		write(outGo, name+".js", out)
	}
	fmt.Printf("probe: wrote %d transform outputs to %s\n", len(names), outGo)

	kernel, err := os.ReadFile(filepath.Join(root, "vendor/radix-kernel.iife.js"))
	die(err)
	core, err := os.ReadFile(filepath.Join(root, "src/runtime/core.js"))
	die(err)
	base := string(kernel) + "\n;\n" + string(core) // iifeBase, jsbuild.go
	minified, err := minify(base, api.FormatDefault)
	die(err)
	write(outGoMin, "shadless.min.js", minified)
	fmt.Printf("probe: wrote minify output (%d bytes) to %s\n", len(minified), outGoMin)
}
