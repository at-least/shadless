package main

// oracle-css — a stylesheet for the React oracle that owes nothing to
// src/emitter. Ported from gates/oracle-css.mjs.
//
// Until this existed the React oracle in style-parity was styled with
// dist/out.css — OUR compiled output. That made "computed styles match the
// oracle" a tautology for every bug in the CSS emitter: the cva
// default-variant cascade (0dd7391, 30 broken cells) and the skin marker
// rules (cn-menu-translucent, five opaque menus) were both invisible to it.
//
// build/gates/oracle.css is built from upstream's own inputs only:
//
//	apps/v4/app/globals.css                   custom variants, @theme, :root/.dark
//	packages/shadcn/src/tailwind.css          what @import "shadcn/tailwind.css" resolves to
//	apps/v4/registry/styles/style-nova.css    the skin (.style-nova .cn-* rules)
//	@source build/resolved-ui                 the resolved registry the oracle bundle
//	                                          renders (cn-* expanded at the source,
//	                                          exactly what the oracle DOM carries)
//
// compiled with the same tailwindcss the product uses. Nothing under src/ is
// read — that is the whole point, and the reason this one is worth porting:
// it has no shared definition to duplicate.

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

const (
	upstreamDir  = ".upstream/shadcn-ui"
	oracleOutDir = "build/gates"
)

var skinRule = regexp.MustCompile(`\.style-nova\s+\.cn-[\w-]+`)

func runOracleCSS() int {
	wd, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	abs := func(p string) string { return filepath.Join(root, p) }
	read := func(p string) (string, error) {
		b, err := os.ReadFile(abs(p))
		return string(b), err
	}

	app, err := read(upstreamDir + "/apps/v4/app/globals.css")
	if err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	shadcnTw, err := read(upstreamDir + "/packages/shadcn/src/tailwind.css")
	if err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	skin, err := read(upstreamDir + "/apps/v4/registry/styles/style-nova.css")
	if err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	legacy := abs(upstreamDir + "/apps/v4/app/legacy-themes.css")

	var lines []string
	for _, line := range strings.Split(app, "\n") {
		switch {
		case strings.Contains(line, `"shadcn/tailwind.css"`):
			lines = append(lines, "/* shadcn/tailwind.css (inlined from packages/shadcn/src) */", shadcnTw)
		case strings.Contains(line, `"./legacy-themes.css"`):
			if _, err := os.Stat(legacy); err == nil {
				lines = append(lines, fmt.Sprintf("@import %q;", legacy))
			}
		case strings.HasPrefix(line, "@source "):
			// the app's own style dirs; replaced below
		default:
			lines = append(lines, line)
		}
	}
	lines = append(lines,
		fmt.Sprintf("@source %q;", abs("build/resolved-ui")),
		// usage trees carry example classes
		fmt.Sprintf("@source %q;", abs("tools/contracts/components")),
		// the examples' own utilities (max-w-lg on an accordion demo, …): the
		// demo pages carry them inline, so the oracle stylesheet must define them
		fmt.Sprintf("@source %q;", abs(upstreamDir+"/apps/v4/examples")),
		"/* === style-nova.css (the pinned skin, verbatim) === */", skin)

	if err := os.MkdirAll(abs(oracleOutDir), 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	entry := oracleOutDir + "/oracle.entry.css"
	if err := os.WriteFile(abs(entry), []byte(strings.Join(lines, "\n")), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}

	out := oracleOutDir + "/oracle.css"
	// through the same wrapper the product uses, compiled with build/gates as
	// the content-scan cwd exactly as the JS gate did
	if err := twCompile(root, entry, out, oracleOutDir, false, false); err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	css, err := read(out)
	if err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	fmt.Printf("oracle-css: %s (%.0fKB, %d skin rules, zero bytes from src/)\n",
		out, float64(len(css))/1024, len(skinRule.FindAllString(css, -1)))
	return 0
}
