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

// buildOracleEntryCSS applies the app's own globals.css line-by-line: the
// shadcn/tailwind.css import is inlined verbatim, the legacy-themes.css
// import is spliced back in (as an @import of legacyImportPath) only when
// hasLegacy says the file exists, every @source line the app declared is
// dropped (the oracle scans its own resolved tree instead, appended below),
// and every other line passes through unchanged. sourceDirs become the
// oracle's own @source list, and skinCSS (the pinned skin, verbatim) is
// appended last.
func buildOracleEntryCSS(appCSS, shadcnTailwindCSS, skinCSS, legacyImportPath string, hasLegacy bool, sourceDirs []string) string {
	var lines []string
	for _, line := range strings.Split(appCSS, "\n") {
		switch {
		case strings.Contains(line, `"shadcn/tailwind.css"`):
			lines = append(lines, "/* shadcn/tailwind.css (inlined from packages/shadcn/src) */", shadcnTailwindCSS)
		case strings.Contains(line, `"./legacy-themes.css"`):
			if hasLegacy {
				lines = append(lines, fmt.Sprintf("@import %q;", legacyImportPath))
			}
		case strings.HasPrefix(line, "@source "):
			// the app's own style dirs; replaced below
		default:
			lines = append(lines, line)
		}
	}
	for _, d := range sourceDirs {
		lines = append(lines, fmt.Sprintf("@source %q;", d))
	}
	lines = append(lines, "/* === style-nova.css (the pinned skin, verbatim) === */", skinCSS)
	return strings.Join(lines, "\n")
}

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
	_, legacyErr := os.Stat(legacy)

	entryCSS := buildOracleEntryCSS(app, shadcnTw, skin, legacy, legacyErr == nil, []string{
		abs("build/resolved-ui"),
		// usage trees carry example classes
		abs("tools/contracts/components"),
		// the examples' own utilities (max-w-lg on an accordion demo, …): the
		// demo pages carry them inline, so the oracle stylesheet must define them
		abs(upstreamDir + "/apps/v4/examples"),
	})

	if err := os.MkdirAll(abs(oracleOutDir), 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}
	entry := oracleOutDir + "/oracle.entry.css"
	if err := os.WriteFile(abs(entry), []byte(entryCSS), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "oracle-css:", err)
		return 1
	}

	out := oracleOutDir + "/oracle.css"
	// Through the same wrapper the product uses, and with NO compile cwd: an
	// empty scratch dir, so the only content scanned is the @source list above
	// — all of it absolute, all of it upstream.
	//
	// It used to pass oracleOutDir "exactly as the JS gate did", which made
	// tailwind scan build/gates. That directory is the pipeline's junk drawer:
	// gate logs, an upstream report, a stale ir-before/ copy of the orphan IR
	// 54cf18c deleted — and oracle.css itself. Three consequences, all of them
	// found by diffing this step's output against a container's:
	//
	//   - rules leaked in from files no source produces. `.static` came from
	//     chain-*.log; the field component's data-[error=true] rule came from
	//     build/gates/ir-before/form.json, a copy of a file that no longer
	//     exists in the tree.
	//   - the gate's whole reason for existing was weakened. Its `Why` says it
	//     "reads nothing under src/, so style-parity is no longer circular",
	//     and the logs in that directory are full of src/-originated class
	//     names (full-B.log alone carries 20 data-slot occurrences).
	//   - it ratcheted. Scanning its own previous output meant a class, once
	//     admitted, could never leave.
	//
	// And none of it was reproducible: a fresh clone has an empty build/gates
	// and would have produced a different stylesheet.
	if err := twCompile(root, entry, out, "", false, false); err != nil {
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
