package main

// docs-guides, ported from tools/docs-guides.mjs — FT4 keep-list (decision
// record), the shared /docs/ route resolver, guide preview enumeration and
// the content-map writer. Merged here rather than into docs_catalog.go: the
// catalog is radix-only by design and lives on its own cadence; the guide
// table travels with the docs build that consumes it.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const guidesUp = ".upstream/shadcn-ui/apps/v4/content/docs"

type guide struct {
	slug           string
	route          string
	title          string
	source         string
	disposition    string
	notes          string
	installSection bool
	util           string
	rtlMigrate     bool
	pinned         bool
}

var guides = []guide{
	{slug: "introduction", route: "/docs/introduction", title: "Introduction", source: "docs/content/introduction.mdx", disposition: "adapted",
		notes:          "FT8: vanilla rewrite of upstream (root)/index.mdx; pure prose + 1 Accordion FAQ — was wrongly pruned as \"framework content\" but contains no React/CLI specifics",
		installSection: false, pinned: true},
	{slug: "installation", route: "/docs/installation", title: "Installation", source: "docs/content/installation.mdx", disposition: "adapted",
		notes:          "authored vanilla rewrite; replaces upstream installation/index.mdx (CLI/create/framework cards — React toolchain) and installation/manual.mdx (Tailwind+aliases+components.json setup); copy-files story over dist/ artifacts",
		installSection: false},
	{slug: "dark-mode", route: "/docs/dark-mode", title: "Dark Mode", source: "docs/content/dark-mode.mdx", disposition: "adapted",
		notes:          "authored rewrite; upstream dark-mode/index.mdx is framework cards only (next/vite/astro/remix/tanstack-start pruned); .dark theme variables ship precompiled in dist/out.css; mode-toggle preview to-author (FT7)",
		installSection: false},
	{slug: "rtl", route: "/docs/rtl", title: "RTL", source: guidesUp + "/rtl/index.mdx", disposition: "mirror",
		notes:          "load-bearing (56 radix pages link /docs/rtl); framework sub-pages (rtl/next|vite|start) pruned → their LinkedCard links greyed; card-rtl preview to-author (FT7); shadcn-CLI migrate section rewritten (rtlMigrate)",
		installSection: false, rtlMigrate: true},
	{slug: "shimmer", route: "/docs/utils/shimmer", title: "shimmer", source: guidesUp + "/utils/shimmer.mdx", disposition: "adapted",
		notes:          "mirrored; Installation section replaced (utilities ship precompiled in dist/out.css, no npm install); all 9 previews base-style (base-rhea) → unavailable",
		installSection: true, util: "shimmer"},
	{slug: "scroll-fade", route: "/docs/utils/scroll-fade", title: "scroll-fade", source: guidesUp + "/utils/scroll-fade.mdx", disposition: "adapted",
		notes:          "mirrored; Installation section replaced (utilities ship precompiled in dist/out.css); all 7 previews base-style (6 base-rhea + 1 base-nova) → unavailable",
		installSection: true, util: "scroll-fade"},
	{slug: "ai-sdk", route: "/docs/helpers/ai-sdk", title: "AI SDK", source: guidesUp + "/helpers/ai-sdk.mdx", disposition: "mirror",
		notes:          "kept per keep-list; @shadcn/helpers/ai-sdk is a React useChat package — mirrored as reference (fences stay verbatim, same policy as radix pages); ai-sdk-helper-demo base-style → unavailable",
		installSection: false},
	{slug: "tanstack-ai", route: "/docs/helpers/tanstack-ai", title: "TanStack AI", source: guidesUp + "/helpers/tanstack-ai.mdx", disposition: "mirror",
		notes:          "kept per keep-list; @shadcn/helpers/tanstack-ai is a React package — mirrored as reference; tanstack-ai-helper-demo base-style → unavailable",
		installSection: false},
	{slug: "typography", route: "/docs/typography", title: "Typography", source: "docs/content/typography.mdx", disposition: "adapted",
		notes:          "FT8: vanilla rewrite — upstream typography.mdx demos a <Typography> component shadless does not (and should not) ship; this guide maps the same typographic roles to plain Tailwind utilities already in dist/out.css",
		installSection: false},
}

var prunedGuides = map[string]struct{ Source, Reason string }{
	"forms":                {guidesUp + "/forms/", "React-only (react-hook-form / tanstack-form / formisch guides); field.mdx links ×4 → greyed spans"},
	"react":                {guidesUp + "/react/", "React-only component recipes (message-scroller, questionnaire); radix links ×6 → greyed spans"},
	"registry":             {guidesUp + "/registry/", "shadcn CLI registry system (json schema, MCP, namespaces) — no vanilla equivalent"},
	"changelog":            {guidesUp + "/changelog/", "shadcn release notes — not shadless content"},
	"(root)":               {guidesUp + "/(root)/", "shadcn-site root pages (theming, cli, components.json…) — React/CLI specific"},
	"framework sub-pages":  {guidesUp + "/{installation,dark-mode,rtl}/* (non-index)", "per-React-framework setup guides (next/vite/astro/remix/tanstack/laravel/gatsby…) — installation/dark-mode/rtl index pages kept instead"},
}

// routeTarget is resolveDocsRoute's result: routable (file+frag) or grey.
type routeTarget struct {
	file string
	frag string
	grey bool
}

var reCompRoute = regexp.MustCompile(`^/docs/components/(?:radix/)?([a-z0-9-]+)$`)

func resolveDocsRoute(href string, members map[string]bool) *routeTarget {
	if !strings.HasPrefix(href, "/") || strings.HasPrefix(href, "//") {
		return nil
	}
	path := href
	frag := ""
	if i := strings.IndexByte(href, '#'); i >= 0 {
		path, frag = href[:i], href[i+1:]
	}
	if m := reCompRoute.FindStringSubmatch(path); m != nil {
		if members[m[1]] {
			return &routeTarget{file: m[1] + ".html", frag: frag}
		}
		return &routeTarget{grey: true}
	}
	for _, g := range guides {
		if g.route == path {
			return &routeTarget{file: g.slug + ".html", frag: frag}
		}
	}
	return &routeTarget{grey: true}
}

// scanGuidePreviews enumerates guide ComponentPreview names with the same
// fence-stripped tag-scoped discipline as the catalog.
var (
	reGuidePreviewTag = regexp.MustCompile(`<ComponentPreview\b([^>]*)>`)
	reGuideFences     = regexp.MustCompile("(?s)```.*?```")
)

func scanGuidePreviews(catalogPreviewStatus map[string]string) (map[string]guidePreviewInfo, []string) {
	out := map[string]guidePreviewInfo{}
	var order []string
	for _, g := range guides {
		b, err := os.ReadFile(g.source)
		if err != nil {
			continue
		}
		text := reGuideFences.ReplaceAllStringFunc(string(b), func(m string) string {
			return strings.Map(func(r rune) rune {
				if r == '\n' {
					return '\n'
				}
				return ' '
			}, m)
		})
		for _, m := range reGuidePreviewTag.FindAllStringSubmatch(text, -1) {
			name := attrOfJS(m[1], "name")
			if name == "" {
				continue
			}
			if e, seen := out[name]; seen {
				e.hostPages = append(e.hostPages, g.slug)
				out[name] = e
				continue
			}
			styleName := attrOfJS(m[1], "styleName")
			var disposition, reason string
			if st, ok := catalogPreviewStatus[name]; ok {
				disposition = st
				reason = "already cataloged in the radix set"
			} else if strings.HasPrefix(styleName, "base-") {
				disposition = "unavailable"
				reason = "base-line demo (" + styleName + ") — shadless implements the radix line only"
			} else {
				disposition = "to-author"
				reason = "authored in an FT7 wave"
			}
			out[name] = guidePreviewInfo{hostPages: []string{g.slug}, styleName: styleName, disposition: disposition, reason: reason}
			order = append(order, name)
		}
	}
	return out, order
}

type guidePreviewInfo struct {
	hostPages   []string
	styleName   string
	disposition string
	reason      string
}

// attrOfJS: (^|\s)name="value" — a bare \b would let data-name= match name=.
func attrOfJS(attrs, name string) string {
	re := regexp.MustCompile(`(?:^|\s)` + regexp.QuoteMeta(name) + `="([^"]*)"`)
	if m := re.FindStringSubmatch(attrs); m != nil {
		return m[1]
	}
	return ""
}

// writeContentMap emits docs/content-map.json byte-compatibly with the JS
// writer (JSON.stringify(_, null, 2) + "\n"; key order = insertion order).
func writeContentMap(componentPages []struct{ name, source string }, sections map[string][]string) error {
	catalogB, err := os.ReadFile("docs/catalog.json")
	if err != nil {
		return err
	}
	var catalog struct {
		Previews []struct {
			Name   string `json:"name"`
			Status string `json:"status"`
		} `json:"previews"`
	}
	if err := json.Unmarshal(catalogB, &catalog); err != nil {
		return err
	}
	status := map[string]string{}
	for _, p := range catalog.Previews {
		status[p.Name] = p.Status
	}
	gp, gpOrder := scanGuidePreviews(status)

	// pages: components (sorted), then guides, then index — the JS writer's
	// insertion order. sections land LAST inside each entry (the JS build
	// appended them in a second pass).
	var pagesKV []jsonKV
	for _, c := range componentPages {
		e := []jsonKV{
			{"source", c.source},
			{"disposition", "adapted"},
			{"notes", "radix mirror; installation Manual tab rewritten to the vanilla copy-files path (build-time transform over dist/ artifacts)"},
		}
		if sec := sections[c.name]; len(sec) > 0 {
			arr := make([]any, len(sec))
			for i, x := range sec {
				arr[i] = x
			}
			e = append(e, jsonKV{"sections", arr})
		}
		pagesKV = append(pagesKV, jsonKV{c.name, e})
	}
	for _, g := range guides {
		e := []jsonKV{
			{"source", g.source},
			{"disposition", g.disposition},
			{"notes", g.notes},
		}
		if sec := sections[g.slug]; len(sec) > 0 {
			arr := make([]any, len(sec))
			for i, x := range sec {
				arr[i] = x
			}
			e = append(e, jsonKV{"sections", arr})
		}
		pagesKV = append(pagesKV, jsonKV{g.slug, e})
	}
	pagesKV = append(pagesKV, jsonKV{"index", []jsonKV{
		{"source", "(generated)"},
		{"disposition", "generated"},
		{"notes", "components + guides index page"},
	}})

	// pruned in the recorded order
	pkeys := []string{"forms", "react", "registry", "changelog", "(root)", "framework sub-pages"}
	var prunedKV []jsonKV
	for _, k := range pkeys {
		p := prunedGuides[k]
		prunedKV = append(prunedKV, jsonKV{k, []jsonKV{
			{"source", p.Source},
			{"reason", p.Reason},
		}})
	}

	// guidePreviews in first-seen order
	var gpKV []jsonKV
	for _, name := range gpOrder {
		g := gp[name]
		var style any
		if g.styleName != "" {
			style = g.styleName
		}
		hosts := make([]any, len(g.hostPages))
		for i, h := range g.hostPages {
			hosts[i] = h
		}
		gpKV = append(gpKV, jsonKV{name, []jsonKV{
			{"hostPages", hosts},
			{"styleName", style},
			{"disposition", g.disposition},
			{"reason", g.reason},
		}})
	}

	root := []jsonKV{
		{"version", 1},
		{"generatedBy", "tools/docs-guides.mjs (FT4)"},
		{"pages", pagesKV},
		{"pruned", prunedKV},
		{"guidePreviews", gpKV},
	}
	if err := os.MkdirAll("docs", 0o755); err != nil {
		return err
	}
	return os.WriteFile("docs/content-map.json", []byte(stringifyJSON(root, 0)+"\n"), 0o644)
}

// runDocsGuides is the CLI entry (the old tools/docs-guides.mjs main).
func runDocsGuides() int {
	catalogB, err := os.ReadFile("docs/catalog.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-guides:", err)
		return 1
	}
	var catalog struct {
		Sources []struct {
			Name   string `json:"name"`
			Status string `json:"status"`
		} `json:"sources"`
	}
	if err := json.Unmarshal(catalogB, &catalog); err != nil {
		fmt.Fprintln(os.Stderr, "docs-guides:", err)
		return 1
	}
	var componentPages []struct{ name, source string }
	for _, s := range catalog.Sources {
		if s.Status != "existing-dist" {
			continue
		}
		p := filepath.Join(guidesUp+"/components/radix", s.Name+".mdx")
		if _, err := os.Stat(p); err != nil {
			continue
		}
		componentPages = append(componentPages, struct{ name, source string }{s.Name, p})
	}
	sort.Slice(componentPages, func(i, j int) bool {
		return componentPages[i].name < componentPages[j].name
	})
	if err := writeContentMap(componentPages, nil); err != nil {
		fmt.Fprintln(os.Stderr, "docs-guides:", err)
		return 1
	}
	nMirror, nAdapted := 0, 0
	for _, g := range guides {
		if g.disposition == "mirror" {
			nMirror++
		} else {
			nAdapted++
		}
	}
	fmt.Printf("guides keep-list: %d kept (%d mirror, %d adapted), %d pruned groups\n",
		len(guides), nMirror, nAdapted, len(prunedGuides))
	return 0
}
