package main

// product-css — derive the npm-consumable product surface from the pipeline's
// demo-oriented artifacts. Ported from tools/product-css.mjs.
//
//	dist/shadless-core.css        theme vars + @theme + custom variants +
//	                              @utility helpers + keyframes (NO docs-site
//	                              chrome, NO demo @source, NO demo body pad)
//	dist/css/<name>.css           per-component @layer block (@apply source,
//	                              written by pipeline/demo.go — READ here)
//	dist/shadless.product.css     tokens + fixes + all parts (tailwind input)
//	dist/shadless.full[.min].css  compiled by the demo chain (zero-build use)
//
// Why extraction instead of shipping globals.css wholesale: probes/h4/
// globals.css was captured to render the shadcn ORACLE site correctly, so it
// carries site chrome (rehype-pretty-code prose, steps, typeset, dialog-ring,
// style-* packs, a:active dimming, overscroll resets) that must NOT leak into
// consumer pages. The keep-list is deliberately narrow; the verify gate proves
// the extracted set still compiles every component rule.
//
//	pipeline product-css                                write tokens + product entry
//	go test -C pipeline -run '^TestProductVerify$'      compiled full.css ⊇ component rules
//
// SHADLESS_CSS_FIXES is lifted out of src/docs/theme-prepaint.mjs by regex —
// as TEXT, the way its JS predecessor did. That is a read, not a second
// implementation: the definition still lives in exactly one place.

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var (
	reImportLine   = regexp.MustCompile(`(?m)^@import .+;$`)
	reDarkVariant  = regexp.MustCompile(`(?m)^@custom-variant dark [^\n]+$`)
	reBlockOpen    = regexp.MustCompile(`@theme[^{]*\{|(^|\n)[ \t]*(:root|\.dark)\s*\{`)
	reBorderReset  = regexp.MustCompile(`\n[ \t]*\*[ \t]*\{[^}]*@apply border-border[^}]*\}`)
	reStrayImport  = regexp.MustCompile(`(?m)^@import ("[^"]+"|url\([^)]*\));?$`)
	reSlotSelector = regexp.MustCompile(`\[data-slot="([^"]+)"\]`)
	reStandalone   = regexp.MustCompile(`(?m)^  \.(\S+) \{$`)
	reCSSEscape    = regexp.MustCompile(`\\(.)`)
	reDataAttr     = regexp.MustCompile(`\[data-[^\]]*\]`)
	reCSSFixes     = regexp.MustCompile("export const SHADLESS_CSS_FIXES = `([^`]*)`")
)

const shadcnMarkerEnd = "/* === end inlined shadcn/tailwind.css === */"

// takeBlock pulls a balanced block (at-rule or selector) starting at the index
// of its opening brace.
func takeBlock(text string, openIdx int) (string, error) {
	depth := 0
	for i := openIdx; i < len(text); i++ {
		switch text[i] {
		case '{':
			depth++
		case '}':
			depth--
			if depth == 0 {
				return text[openIdx : i+1], nil
			}
		}
	}
	return "", fmt.Errorf("unbalanced block")
}

// headerOf returns the at-rule/selector text from line start up to the brace.
func headerOf(text string, braceIdx int) string {
	start := strings.LastIndex(text[:braceIdx], "\n") + 1
	return strings.TrimSpace(text[start:braceIdx])
}

// extractTokens is the keep-list: the narrow product-relevant subset of a
// globals.css that mixes library config with oracle-site chrome.
func extractTokens(globals string) (string, error) {
	var keep []string
	// 1. @import lines (tailwindcss + tw-animate-css)
	keep = append(keep, reImportLine.FindAllString(globals, -1)...)
	// 2. the inlined shadcn library tailwind.css (marker-bounded)
	begin := strings.Index(globals, "/* === begin inlined shadcn/tailwind.css === */")
	end := strings.Index(globals, shadcnMarkerEnd)
	if begin < 0 || end < 0 {
		return "", fmt.Errorf("inlined shadcn/tailwind.css markers missing")
	}
	keep = append(keep, globals[begin:end+len(shadcnMarkerEnd)])
	// 3. dark-mode custom variant (line-scoped exact grab)
	if dark := reDarkVariant.FindString(globals); dark != "" {
		keep = append(keep, dark)
	}
	// 4. walk top-level-ish at-rule/selector blocks with balanced braces;
	//    keep only the product-relevant ones
	for _, loc := range reBlockOpen.FindAllStringIndex(globals, -1) {
		braceIdx := loc[1] - 1
		header := headerOf(globals, braceIdx)
		body, err := takeBlock(globals, braceIdx)
		if err != nil {
			return "", err
		}
		switch {
		case strings.HasPrefix(header, "@theme") && strings.Contains(body, "--color-background:"):
			keep = append(keep, header+body)
		case (header == ":root" || header == ".dark") && strings.Contains(body, "--background:"):
			keep = append(keep, header+" "+body)
		}
	}
	// 5. the base border/outline reset rule (indented inside @layer base, so
	//    the line-anchored walk above would miss it; flat [^}] body is safe)
	if star := reBorderReset.FindString(globals); star != "" {
		keep = append(keep, "@layer base {"+strings.TrimSpace(star)+"\n}")
	}
	return strings.Join(keep, "\n\n") + "\n", nil
}

func buildProductEntry(tokensCSS, fixesCSS, partsCSS string) string {
	return tokensCSS + "\n" + fixesCSS + "\n" + partsCSS + "\n"
}

// slotSet returns the [data-slot="…"] names in first-occurrence order, which
// is what the JS Set iteration produced and what the failure messages list.
func slotSet(css string) ([]string, map[string]bool) {
	var order []string
	seen := map[string]bool{}
	for _, m := range reSlotSelector.FindAllStringSubmatch(css, -1) {
		if !seen[m[1]] {
			seen[m[1]] = true
			order = append(order, m[1])
		}
	}
	return order, seen
}

type productReport struct{ Missing, DemoDropped, Chrome, Tokens, Stray []string }

// verifyProduct: every [data-slot="…"] selector in the per-component PARTS
// must survive compilation in BOTH chains (a missing custom variant /
// @utility / token silently drops rules). out.css legitimately carries extra
// docs-site slots (layout/copy-button/docs chrome) — those must NOT appear in
// the product build, which is what the chrome check enforces. The stray-class
// check enforces hermeticity: every standalone class rule in the product build
// must trace back to the product source text (unescaped), else content
// scanning leaked into the build (see pipeline/tw.go).
func verifyProduct(fullCSS, outCSS, partsCSS, productSource string) productReport {
	expected, _ := slotSet(partsCSS)
	_, fullSlots := slotSet(fullCSS)
	_, outSlots := slotSet(outCSS)
	var r productReport
	for _, s := range expected {
		if !fullSlots[s] {
			r.Missing = append(r.Missing, s)
		}
		if !outSlots[s] {
			r.DemoDropped = append(r.DemoDropped, s)
		}
	}
	for _, needle := range []string{"rehype", "typeset", "dialog-ring", "style-vega", "data-wrapper",
		`[data-slot="docs"]`, `[data-slot="layout"]`, `[data-slot="copy-button"]`} {
		if strings.Contains(fullCSS, needle) {
			r.Chrome = append(r.Chrome, needle)
		}
	}
	// runtime vars the compiled output must carry. NOTE: --color-* aliases are
	// compile-time only under `@theme inline` (inlined at use sites), so they
	// never appear literally in a correct build — do not "fix" this by adding
	// them back.
	for _, tok := range []string{"--background:", "--radius:"} {
		if !strings.Contains(fullCSS, tok) {
			r.Tokens = append(r.Tokens, tok)
		}
	}
	// compiled @apply of VARIANT-qualified utilities emits the variant into the
	// selector — a legitimate compile artifact of a product-source rule, so
	// compare the base class too before calling it stray
	for _, m := range reStandalone.FindAllStringSubmatch(fullCSS, -1) {
		cls := reCSSEscape.ReplaceAllString(m[1], "$1")
		base := reDataAttr.ReplaceAllString(cls, "")
		if !strings.Contains(productSource, cls) && !strings.Contains(productSource, base) {
			r.Stray = append(r.Stray, cls)
		}
	}
	return r
}

// partFiles lists dist/css/*.css minus the aggregate, sorted.
func partFiles(root string) ([]string, error) {
	entries, err := os.ReadDir(filepath.Join(root, "dist/css"))
	if err != nil {
		return nil, err
	}
	var out []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".css") && e.Name() != "shadless.css" {
			out = append(out, e.Name())
		}
	}
	return out, nil
}

func gateProductVerify(root string) error {
	read := func(p string) (string, error) {
		b, err := os.ReadFile(filepath.Join(root, p))
		return string(b), err
	}
	full, err := read("dist/shadless.full.css")
	if err != nil {
		return fmt.Errorf("FAIL  product-css --verify: %v", err)
	}
	out, err := read("dist/out.css")
	if err != nil {
		return fmt.Errorf("FAIL  product-css --verify: %v", err)
	}
	names, err := partFiles(root)
	if err != nil {
		return fmt.Errorf("FAIL  product-css --verify: %v", err)
	}
	var parts []string
	for _, n := range names {
		s, err := read("dist/css/" + n)
		if err != nil {
			return fmt.Errorf("FAIL  product-css --verify: %v", err)
		}
		parts = append(parts, s)
	}
	partsCSS := strings.Join(parts, "\n")
	productSource, err := read("dist/shadless.product.css")
	if err != nil {
		return fmt.Errorf("FAIL  product-css --verify: %v", err)
	}

	r := verifyProduct(full, out, partsCSS, productSource)
	var problems []string
	add := func(label string, xs []string) {
		if len(xs) > 0 {
			problems = append(problems, label+strings.Join(xs, ", "))
		}
	}
	add("slot rules missing from product build: ", r.Missing)
	add("slot rules missing from DEMO build (both chains disagree): ", r.DemoDropped)
	add("docs chrome leaked into product build: ", r.Chrome)
	add("tokens missing from product build: ", r.Tokens)
	add("standalone classes with no origin in product source (content-scan leak?): ", r.Stray)
	if len(problems) > 0 {
		return fmt.Errorf("FAIL  product-css --verify\n  %s", strings.Join(problems, "\n  "))
	}
	slots, _ := slotSet(partsCSS)
	fmt.Printf("PASS  product-css --verify (%d slot rules in both chains, no docs chrome, no stray classes)\n", len(slots))
	return nil
}

func runProductCSS() int {
	wd, _ := os.Getwd()
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "product-css:", err)
		return 1
	}
	fail := func(err error) int { fmt.Fprintln(os.Stderr, "product-css:", err); return 1 }
	read := func(p string) (string, error) {
		b, err := os.ReadFile(filepath.Join(root, p))
		return string(b), err
	}

	// dist/globals.css (composed by the demo chain: probe base + fixes + nova
	// skin utilities + slot rules) when present; the probe capture alone
	// predates the skin and cannot supply the @utility cn-* defs the product
	// slot rules @apply.
	globalsPath := "dist/globals.css"
	if _, err := os.Stat(filepath.Join(root, globalsPath)); err != nil {
		globalsPath = "probes/h4/globals.css"
	}
	globals, err := read(globalsPath)
	if err != nil {
		return fail(err)
	}
	// the demo entry turns tailwind's automatic content detection off
	// (source(none) + explicit @source, see pipeline/demo.go); the CONSUMER's
	// build must keep detection on — their pasted markup is what it scans
	globals = strings.Replace(globals, `@import "tailwindcss" source(none);`, `@import "tailwindcss";`, 1)

	if err := os.MkdirAll(filepath.Join(root, "dist/css"), 0o755); err != nil {
		return fail(err)
	}
	// tw-animate-css is INLINED into the product surface: the consumer story is
	// "two @imports + your tailwind build" — an extra npm package (or a
	// resolvable node_modules) for the animate layer would break that. The demo
	// globals keep the real @import (it resolves inside this repo).
	animate, err := read("node_modules/tw-animate-css/dist/tw-animate.css")
	if err != nil {
		return fail(err)
	}
	tokens, err := extractTokens(globals)
	if err != nil {
		return fail(err)
	}
	tokens = strings.Replace(tokens, `@import "tw-animate-css";`,
		"/* === begin inlined tw-animate-css (self-contained product surface) === */\n"+
			strings.TrimSpace(animate)+"\n/* === end inlined tw-animate-css === */", 1)
	if strings.Contains(tokens, `@import "tw-animate-css";`) {
		return fail(fmt.Errorf("tw-animate-css import not replaced"))
	}
	var strayImports []string
	for _, m := range reStrayImport.FindAllStringSubmatch(tokens, -1) {
		if m[1] != `"tailwindcss"` {
			strayImports = append(strayImports, m[1])
		}
	}
	if len(strayImports) > 0 {
		return fail(fmt.Errorf("shadless.css not self-contained — unresolved @import(s): %s",
			strings.Join(strayImports, ", ")))
	}
	core := "/* shadless theme — extracted from probes/h4/globals.css by pipeline/product_css.go.\n" +
		"   Product surface ONLY: theme vars, @theme, custom variants, @utility helpers,\n" +
		"   keyframes. Deliberately excluded: docs-site chrome (prose/steps/packs),\n" +
		"   demo @source and demo body padding. The only @import left is \"tailwindcss\"\n" +
		"   itself — the animate layer is inlined so consumers need nothing else. */\n" + tokens
	if err := os.WriteFile(filepath.Join(root, "dist/shadless-core.css"), []byte(core), 0o644); err != nil {
		return fail(err)
	}

	// product entry = tokens + fixes + per-component parts (written by demo.mjs)
	prepaint, err := read("src/docs/theme-prepaint.mjs")
	if err != nil {
		return fail(err)
	}
	m := reCSSFixes.FindStringSubmatch(prepaint)
	if m == nil {
		return fail(fmt.Errorf("SHADLESS_CSS_FIXES not found in src/docs/theme-prepaint.mjs"))
	}
	names, err := partFiles(root)
	if err != nil {
		return fail(err)
	}
	var parts []string
	for _, n := range names {
		s, err := read("dist/css/" + n)
		if err != nil {
			return fail(err)
		}
		parts = append(parts, strings.TrimSpace(s))
	}
	if len(parts) == 0 {
		return fail(fmt.Errorf("dist/css has no per-component files — run the demo chain first"))
	}
	entry := buildProductEntry(core, m[1], strings.Join(parts, "\n\n"))
	if err := os.WriteFile(filepath.Join(root, "dist/shadless.product.css"), []byte(entry), 0o644); err != nil {
		return fail(err)
	}
	fmt.Printf("product-css: shadless-core.css + shadless.product.css (%d component parts)\n", len(parts))
	return 0
}
