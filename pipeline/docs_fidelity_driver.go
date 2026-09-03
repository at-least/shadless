package main

// docs-fidelity driver, ported from tools/docs-fidelity.mjs — mdx↔markdown
// page fidelity over EVERY page in docs/content-map.json.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

func runDocsFidelity() int {
	const demos = "docs/public/demos"
	guideBySlug := map[string]guide{}
	for _, g := range guides {
		guideBySlug[g.slug] = g
	}
	pagePath := func(name string) string {
		if _, ok := guideBySlug[name]; ok {
			return "docs/guides/" + name + ".md"
		}
		return "docs/components/" + name + ".md"
	}
	if !fileExists("docs/index.md") {
		fmt.Fprintln(os.Stderr, "FAIL  docs-fidelity: the markdown pages are not built — run the docs chain first (make docs)")
		return 1
	}

	cmB, err := os.ReadFile("docs/content-map.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-fidelity:", err)
		return 1
	}
	// ordered read: pages iteration order must match the JS Object.entries
	cmOrdered, err := decodeOrderedObject(extractRawField(cmB, "pages"))
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-fidelity: content-map:", err)
		return 1
	}

	type issue struct{ page, kind, detail string }
	var issues []issue
	pages := 0
	reCompSource := regexp.MustCompile(`/components/radix/`)
	reVariantHref := regexp.MustCompile(`/components/(base|aria)/|-(base|aria)\.html$`)

	for _, name := range cmOrdered.keys {
		if name == "index" {
			continue
		}
		var meta struct {
			Source string `json:"source"`
		}
		if err := json.Unmarshal(cmOrdered.raw[name], &meta); err != nil {
			continue
		}
		mdPath := pagePath(name)
		if !fileExists(mdPath) {
			issues = append(issues, issue{name, "missing-page", "content-map page has no built markdown"})
			continue
		}
		pages++
		isComponent := reCompSource.MatchString(meta.Source)
		g := guideBySlug[name]

		if meta.Source == "" || !fileExists(meta.Source) {
			issues = append(issues, issue{name, "source-missing", "source " + meta.Source + " unreadable"})
			continue
		}
		rawB, err := os.ReadFile(meta.Source)
		if err != nil {
			issues = append(issues, issue{name, "source-missing", "source " + meta.Source + " unreadable"})
			continue
		}
		raw := string(rawB)
		base := filepath.Base(meta.Source)
		adjusted, err := applyTextAdjustments(base, raw)
		if err != nil {
			issues = append(issues, issue{name, "text-adjustment", err.Error()})
			continue
		}
		if g.util != "" {
			adjusted, err = rewriteUtilityJsxFences(g.slug, adjusted)
			if err != nil {
				issues = append(issues, issue{name, "utility-jsx-fences", err.Error()})
				continue
			}
		}
		fixLeakedJsx := g.util == ""
		M, err := mdxPageFacts(name, adjusted, isComponent, g.installSection, g.rtlMigrate, isComponent, isComponent, isComponent, isComponent, fixLeakedJsx)
		if err != nil {
			issues = append(issues, issue{name, "leaked-jsx", err.Error()})
			continue
		}
		mdB, _ := os.ReadFile(mdPath)
		H := mdPageFacts(string(mdB))
		fidelityRawMDX = adjusted

		expected := ""
		if isComponent {
			expected = "dist/components/" + name + ".html"
		}
		for _, d := range comparePage(M, H, name, isComponent, expected) {
			kind := d
			detail := ""
			if i := strings.Index(d, ": "); i >= 0 {
				kind, detail = d[:i], d[i+2:]
			}
			issues = append(issues, issue{name, kind, detail})
		}

		// disk-existence checks: VitePress follows page links itself; the
		// demo iframes are raw html it does not check
		for _, src := range H.iframes {
			if !fileExists(filepath.Join(demos, filepath.Base(src))) {
				issues = append(issues, issue{name, "iframe-404", src})
			}
		}
		if dh := docsHrefsOf(string(mdB)); len(dh) > 0 {
			issues = append(issues, issue{name, "docs-href", "unrewritten: " + strings.Join(dedup(dh), ", ")})
		}

		// variant retirement (2026-08-26): base/aria mirror is GONE
		var variantHrefs []string
		for _, h := range allHrefsOf(string(mdB)) {
			if reVariantHref.MatchString(h) {
				variantHrefs = append(variantHrefs, h)
			}
		}
		if len(variantHrefs) > 0 {
			issues = append(issues, issue{name, "variant-href", "links to retired variant pages: " + strings.Join(dedup(variantHrefs), ", ")})
		}

		// retired prose: declared TEXT_ADJUSTMENTS must not survive
		for _, adj := range textAdjustments {
			if !containsTok(adj.files, base) {
				continue
			}
			for _, op := range adj.ops {
				if strings.Contains(H.text, op.find) {
					n := 60
					if len(op.find) < n {
						n = len(op.find)
					}
					issues = append(issues, issue{name, "retired-prose",
						fmt.Sprintf("%s: %q survives in the built page", adj.id, op.find[:n])})
				}
			}
		}
	}

	// report
	byKind := map[string][]issue{}
	var kinds []string
	for _, i := range issues {
		if _, ok := byKind[i.kind]; !ok {
			kinds = append(kinds, i.kind)
		}
		byKind[i.kind] = append(byKind[i.kind], i)
	}
	sort.Strings(kinds)
	for _, k := range kinds {
		list := byKind[k]
		fmt.Fprintf(os.Stderr, "FAIL  %s (%d):\n", k, len(list))
		n := 8
		if len(list) < n {
			n = len(list)
		}
		for _, i := range list[:n] {
			fmt.Fprintf(os.Stderr, "  - [%s] %s\n", i.page, i.detail)
		}
		if len(list) > 8 {
			fmt.Fprintf(os.Stderr, "  … +%d more\n", len(list)-8)
		}
	}
	fmt.Printf("docs fidelity: %d pages compared against mdx sources — issues: %d\n", pages, len(issues))
	if len(issues) > 0 {
		fmt.Fprintln(os.Stderr, "FAIL  docs fidelity (built pages drift from their mdx sources)")
		return 1
	}
	fmt.Println("PASS  docs fidelity (every page matches its mdx source: headings/previews/fences/links)")
	return 0
}

// extractRawField pulls a top-level field's raw JSON out of an object
// without a full decode (pages order matters).
func extractRawField(b []byte, field string) []byte {
	re := regexp.MustCompile(`"` + regexp.QuoteMeta(field) + `":`)
	m := re.FindIndex(b)
	if m == nil {
		return nil
	}
	i := m[1]
	for i < len(b) && (b[i] == ' ' || b[i] == '\n' || b[i] == '\t') {
		i++
	}
	// find the value's extent by brace matching (it is an object here)
	if i >= len(b) || b[i] != '{' {
		return nil
	}
	depth := 0
	j := i
	inStr := false
	for j < len(b) {
		c := b[j]
		if inStr {
			if c == '\\' {
				j += 2
				continue
			}
			if c == '"' {
				inStr = false
			}
		} else {
			switch c {
			case '"':
				inStr = true
			case '{':
				depth++
			case '}':
				depth--
				if depth == 0 {
					return b[i : j+1]
				}
			}
		}
		j++
	}
	return b[i:]
}

func docsHrefsOf(md string) []string {
	var out []string
	for _, m := range reDocsHref.FindAllStringSubmatch(fenceShadow(md), -1) {
		out = append(out, m[1])
	}
	return out
}

func allHrefsOf(md string) []string {
	var out []string
	for _, m := range reAllHref.FindAllStringSubmatch(fenceShadow(md), -1) {
		out = append(out, m[1])
	}
	return out
}
