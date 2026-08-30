package main

// docs-catalog — enumerate every ComponentPreview + ComponentSource name
// referenced by the docs mirror set and write docs/catalog.json. Ported from
// tools/docs-catalog.mjs.
//
// Extraction is tag-scoped (fence-stripped, multiline attrs, any attribute
// order) — line-based tooling over-counts (gate v2 lesson: sed on rg -U output
// emits one artifact line per physical line of a multiline match).
//
// Status rules (recorded in PLAN §Wave F):
//
//	preview "X-demo" + dist/components/X.html exists -> existing-dist (primary
//	demos reuse dist/, per scope decision v2); every other name -> to-author
//	(docs/demos/<name>.html, authored in FT7 waves).
//	ComponentSource name=X -> dist/components/X.html (existing-dist | no-dist).
//	Previews whose component is tombstoned (no implementation in shadless) get
//	status="tombstoned" instead of "to-author" — they can never be authored.
//	Mirror of GREY_COMPONENTS in tools/docs-build.mjs; lists must stay in sync
//	(cross-checked at every build).

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

const (
	radixDir     = ".upstream/shadcn-ui/apps/v4/content/docs/components/radix"
	distComps    = "dist/components"
	catalogOut   = "docs/catalog.json"
	guidesSetDir = "docs/content/ + UP/..."
)

var (
	reFence      = regexp.MustCompile("(?s)```.*?```")
	reDocsTag    = regexp.MustCompile(`<(ComponentPreview|ComponentSource)\b([^>]*)>`)
	rePreviewTag = regexp.MustCompile(`<ComponentPreview\b([^>]*)>`)
	rePrimary    = regexp.MustCompile(`^([a-z0-9-]+)-demo$`)
	reHooks      = regexp.MustCompile(`\b(useState|useEffect|useRef|useContext|useMemo|useCallback|useReducer|useLayoutEffect|useImperativeHandle|useId|useTransition|useDeferredValue|useSyncExternalStore|useInsertionEffect)\b`)
)

// stripFences blanks code fences: they are prose, not rendered tags. Replaced
// with spaces (not removed) so offsets and line numbers stay stable.
func stripFences(text string) string {
	return reFence.ReplaceAllStringFunc(text, func(m string) string {
		b := []byte(m)
		for i := range b {
			if b[i] != '\n' {
				b[i] = ' '
			}
		}
		return string(b)
	})
}

// attrOf reads one attribute out of a tag's attribute text. The (^|\s) anchor
// is deliberate: a bare \b would let data-name= match name=.
func attrOf(attrs, name string) (string, bool) {
	re := regexp.MustCompile(`(?:^|\s)` + regexp.QuoteMeta(name) + `="([^"]*)"`)
	if m := re.FindStringSubmatch(attrs); m != nil {
		return m[1], true
	}
	return "", false
}

// optStr models a JS optional: a key may be absent entirely (Present false —
// what `attr(x)` yields when the attribute is missing) or present with a null
// value (what `attr(x) || null` yields).
type optStr struct {
	Present bool
	Null    bool
	Val     string
}

func attrOrNull(attrs, name string) optStr { // radix scan: `attr(x) || null`
	v, ok := attrOf(attrs, name)
	if !ok || v == "" {
		return optStr{Present: true, Null: true}
	}
	return optStr{Present: true, Val: v}
}

func attrOrAbsent(attrs, name string) optStr { // guide scan: bare `attr(x)`
	v, ok := attrOf(attrs, name)
	if !ok {
		return optStr{}
	}
	return optStr{Present: true, Val: v}
}

type previewRec struct {
	Name        string
	Component   string
	StyleName   optStr
	Description optStr
	HostPages   []string

	Status   string
	DemoPath string // "" with DemoPathNull means an explicit null
	DemoNull bool
	Quality  string
}

type sourceRec struct {
	Name      string
	Component string
	HostPages []string
	Status    string
	DemoPath  string
	DemoNull  bool
}

type flagRec struct {
	File, Kind, Reason string
	Src                optStr
}

type scanResult struct {
	Key, Dir                  string
	Files                     int
	PreviewTags, MultilineTag int
	Previews                  []previewRec
	Sources                   []sourceRec
	Flags                     []flagRec
}

func lineCount(s string) int { return strings.Count(s, "\n") + 1 }

func scanSet(root, key, dir string) (scanResult, error) {
	r := scanResult{Key: key, Dir: dir}
	entries, err := os.ReadDir(filepath.Join(root, dir))
	if err != nil {
		return r, err
	}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".mdx") {
			continue
		}
		r.Files++
		component := strings.TrimSuffix(e.Name(), ".mdx")
		b, err := os.ReadFile(filepath.Join(root, dir, e.Name()))
		if err != nil {
			return r, err
		}
		text := stripFences(string(b))
		for _, m := range reDocsTag.FindAllStringSubmatch(text, -1) {
			tag, attrs := m[1], m[2]
			if tag == "ComponentPreview" {
				r.PreviewTags++
				if strings.Contains(attrs, "\n") {
					r.MultilineTag++
				}
				name, ok := attrOf(attrs, "name")
				if !ok || name == "" {
					r.Flags = append(r.Flags, flagRec{File: e.Name(), Kind: tag, Reason: "no name= attr"})
					continue
				}
				r.Previews = append(r.Previews, previewRec{
					Name: name, Component: component,
					StyleName:   attrOrNull(attrs, "styleName"),
					Description: attrOrNull(attrs, "description"),
				})
				continue
			}
			name, ok := attrOf(attrs, "name")
			if !ok || name == "" {
				r.Flags = append(r.Flags, flagRec{File: e.Name(), Kind: tag, Reason: "no name= attr",
					Src: attrOrNull(attrs, "src")})
				continue
			}
			r.Sources = append(r.Sources, sourceRec{Name: name, Component: component})
		}
	}
	return r, nil
}

// FT8: guide previews (mode-toggle, card-rtl, …) live on guide pages, not in
// the radix subtree. Same tag-scope, same dedupe, same dist-match; the guide
// slug plays the part of the component.
var guideSources = []struct{ Slug, Path string }{
	{"installation", "docs/content/installation.mdx"},
	{"dark-mode", "docs/content/dark-mode.mdx"},
	{"rtl", ".upstream/shadcn-ui/apps/v4/content/docs/rtl/index.mdx"},
	{"shimmer", ".upstream/shadcn-ui/apps/v4/content/docs/utils/shimmer.mdx"},
	{"scroll-fade", ".upstream/shadcn-ui/apps/v4/content/docs/utils/scroll-fade.mdx"},
	{"ai-sdk", ".upstream/shadcn-ui/apps/v4/content/docs/helpers/ai-sdk.mdx"},
	{"tanstack-ai", ".upstream/shadcn-ui/apps/v4/content/docs/helpers/tanstack-ai.mdx"},
}

func scanGuides(root string) (scanResult, error) {
	r := scanResult{Key: "guides", Dir: guidesSetDir}
	for _, g := range guideSources {
		b, err := os.ReadFile(filepath.Join(root, g.Path))
		if err != nil {
			continue
		}
		r.Files++
		text := stripFences(string(b))
		for _, m := range rePreviewTag.FindAllStringSubmatch(text, -1) {
			attrs := m[1]
			r.PreviewTags++
			if strings.Contains(attrs, "\n") {
				r.MultilineTag++
			}
			name, ok := attrOf(attrs, "name")
			if !ok || name == "" {
				r.Flags = append(r.Flags, flagRec{File: g.Slug, Kind: "ComponentPreview", Reason: "no name= attr"})
				continue
			}
			r.Previews = append(r.Previews, previewRec{
				Name: name, Component: g.Slug,
				StyleName:   attrOrAbsent(attrs, "styleName"),
				Description: attrOrAbsent(attrs, "description"),
			})
		}
	}
	return r, nil
}

// dedupePreviews merges by name: a name may be referenced from several pages,
// FT7 authors it once. hostPages keeps every referencing page; the defining
// page's metadata wins.
func dedupePreviews(records []previewRec) []previewRec {
	var order []string
	byName := map[string]*previewRec{}
	for _, r := range records {
		e, ok := byName[r.Name]
		if !ok {
			cp := r
			cp.HostPages = nil
			byName[r.Name] = &cp
			order = append(order, r.Name)
			e = &cp
		}
		if !contains(e.HostPages, r.Component) {
			e.HostPages = append(e.HostPages, r.Component)
		}
	}
	out := make([]previewRec, len(order))
	for i, n := range order {
		out[i] = *byName[n]
	}
	return out
}

func dedupeSources(records []sourceRec) []sourceRec {
	var order []string
	byName := map[string]*sourceRec{}
	for _, r := range records {
		e, ok := byName[r.Name]
		if !ok {
			cp := r
			cp.HostPages = nil
			byName[r.Name] = &cp
			order = append(order, r.Name)
			e = &cp
		}
		if !contains(e.HostPages, r.Component) {
			e.HostPages = append(e.HostPages, r.Component)
		}
	}
	out := make([]sourceRec, len(order))
	for i, n := range order {
		out[i] = *byName[n]
	}
	return out
}

func contains(xs []string, x string) bool {
	for _, v := range xs {
		if v == x {
			return true
		}
	}
	return false
}

// Preview names with these component prefixes cannot be authored (no shadless
// implementation exists). Matched by exact name or "<prefix>-" so 'data-table'
// beats 'data' and 'date-picker' beats 'date'.
var tombstonePrefixes = []string{
	"calendar", "chart", "combobox", "command", "data-table", "date-picker",
	"drawer", "form", "input-otp", "menubar", "navigation-menu", "questionnaire",
	"resizable", "sidebar", "sonner", "toast", "typography",
}

func isTombstoneName(name string) bool {
	for _, p := range tombstonePrefixes {
		if name == p || strings.HasPrefix(name, p+"-") {
			return true
		}
	}
	return false
}

func optJSON(o optStr) (any, bool) {
	if !o.Present {
		return nil, false
	}
	if o.Null {
		return jsonNull{}, true
	}
	return o.Val, true
}

func runDocsCatalog() int {
	wd, _ := os.Getwd()
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-catalog:", err)
		return 1
	}
	fail := func(err error) int { fmt.Fprintln(os.Stderr, "docs-catalog:", err); return 1 }
	abs := func(p string) string { return filepath.Join(root, p) }
	exists := func(p string) bool { _, err := os.Stat(abs(p)); return err == nil }

	pinRaw, err := os.ReadFile(abs("src/registry/pin.json"))
	if err != nil {
		return fail(err)
	}
	var pin struct {
		ShadcnUI struct{ Repo, Tag, Commit string } `json:"shadcn_ui"`
	}
	if err := json.Unmarshal(pinRaw, &pin); err != nil {
		return fail(err)
	}
	var tiers map[string]struct {
		Tier string `json:"tier"`
	}
	tiersRaw, err := os.ReadFile(abs("src/registry/tiers.json"))
	if err != nil {
		return fail(err)
	}
	if err := json.Unmarshal(tiersRaw, &tiers); err != nil {
		return fail(err)
	}

	radix, err := scanSet(root, "components/radix", radixDir)
	if err != nil {
		return fail(err)
	}
	guides, err := scanGuides(root)
	if err != nil {
		return fail(err)
	}
	sets := []scanResult{radix, guides}

	distFiles := map[string]bool{}
	if entries, err := os.ReadDir(abs(distComps)); err == nil {
		for _, e := range entries {
			distFiles[strings.TrimSuffix(e.Name(), ".html")] = true
		}
	}
	distPath := func(name string) string {
		if distFiles[name] {
			return distComps + "/" + name + ".html"
		}
		return ""
	}

	// Authored status derives from the FILE SYSTEM, not the previous catalog
	// (Wave H D5): docs/demos/<name>.html existing ⇒ authored. The old
	// prev-catalog-only preservation silently reset all 301 authored entries to
	// to-author after a test-clean baseline regen — 149 pages rendered
	// placeholders and the RTL selector vanished, with every gate green.
	prevAuthored := map[string]bool{}
	if b, err := os.ReadFile(abs(catalogOut)); err == nil {
		var prev struct {
			Previews []struct{ Name, Status string } `json:"previews"`
		}
		if json.Unmarshal(b, &prev) == nil {
			for _, p := range prev.Previews {
				if p.Status == "authored" {
					prevAuthored[p.Name] = true
				}
			}
		}
	}

	// A -demo is kernel iff the underlying component is. Kernel-tier -demo
	// previews keep the dist fixture (template + glue) path: the oracle's
	// static DOM has no Portal mount target, so opening popups would fail. The
	// fixture's <template id="d1-portal"> is the mount target the radix kernel
	// runtime needs.
	isKernel := func(name string) bool {
		key := name
		if m := regexp.MustCompile(`^(.+)-demo$`).FindStringSubmatch(name); m != nil {
			key = m[1]
		}
		return tiers[key].Tier == "kernel"
	}

	// Per-set dedupe (so each set's uniquePreviewNames count stays honest) and
	// a global dedupe across sets (so card-rtl on radix + guides collapses to
	// one catalog entry with both hostPages).
	radixUnique := dedupePreviews(radix.Previews)
	guidesUnique := dedupePreviews(guides.Previews)
	all := append(append([]previewRec{}, radix.Previews...), guides.Previews...)
	uniquePreviews := dedupePreviews(all)

	pExisting, pAuthor, pToAuthor, pTomb, unavail := 0, 0, 0, 0, 0
	previews := make([]previewRec, 0, len(uniquePreviews))
	for _, p := range uniquePreviews {
		var d string
		if m := rePrimary.FindStringSubmatch(p.Name); m != nil {
			d = distPath(m[1])
		}
		direct := distPath(p.Name)
		isGuideOnly := true
		for _, h := range p.HostPages {
			if exists(radixDir + "/" + h + ".mdx") {
				isGuideOnly = false
			}
		}
		isBaseStyle := p.StyleName.Present && !p.StyleName.Null && strings.HasPrefix(p.StyleName.Val, "base-")
		authoredFile := "docs/demos/" + p.Name + ".html"
		hasAuthoredFile := exists(authoredFile)

		switch {
		case hasAuthoredFile && !isKernel(p.Name):
			p.Status, p.DemoPath = "authored", authoredFile
			pAuthor++
		case hasAuthoredFile:
			// kernel-tier -demo: the authored file exists (the oracle ran), but
			// the iframe must point at the dist fixture, not the oracle static
			// DOM (which cannot open popups). Strip the oracle artifact so the
			// next oracle run sees the dist fixture as canonical.
			if err := os.Remove(abs(authoredFile)); err != nil {
				return fail(err)
			}
			switch {
			case direct != "":
				p.Status, p.DemoPath = "existing-dist", direct
				pExisting++
			case d != "":
				p.Status, p.DemoPath = "existing-dist", d
				pExisting++
			default:
				p.Status, p.DemoPath = "to-author", authoredFile
				pToAuthor++
			}
		case prevAuthored[p.Name]:
			// inconsistency: previously authored but the file is gone — loud,
			// not silent (a deleted demo must be a decision, not an accident)
			fmt.Fprintf(os.Stderr, "FAIL catalog: %s was authored but %s is missing\n", p.Name, authoredFile)
			return 1
		case direct != "":
			p.Status, p.DemoPath = "existing-dist", direct
			pExisting++
		case d != "":
			p.Status, p.DemoPath = "existing-dist", d
			pExisting++
		case isTombstoneName(p.Name):
			p.Status, p.DemoNull = "tombstoned", true
			pTomb++
		case isGuideOnly && isBaseStyle:
			// FT8: guide-only preview with a base-* styleName → unavailable
			// (base-line demo; shadless implements the radix line only)
			p.Status, p.DemoNull = "unavailable", true
			unavail++
		default:
			p.Status, p.DemoPath = "to-author", authoredFile
			pToAuthor++
		}
		previews = append(previews, p)
	}

	var sources []sourceRec
	var flags []flagRec
	setStats := jsonObj{}
	for _, s := range sets {
		for _, src := range dedupeSources(s.Sources) {
			if d := distPath(src.Name); d != "" {
				src.Status, src.DemoPath = "existing-dist", d
			} else {
				src.Status, src.DemoNull = "no-dist", true
			}
			sources = append(sources, src)
		}
		flags = append(flags, s.Flags...)
		unique := len(guidesUnique)
		if s.Key == "components/radix" {
			unique = len(radixUnique)
		}
		sourceFlags := 0
		previewFlags := 0
		for _, f := range s.Flags {
			if f.Kind == "ComponentSource" {
				sourceFlags++
			} else {
				previewFlags++
			}
		}
		setStats = setStats.add(s.Key, jsonObj{}.
			add("dir", s.Dir).
			add("mdxFiles", s.Files).
			add("previewTags", s.PreviewTags).
			add("multilinePreviewTags", s.MultilineTag).
			add("uniquePreviewNames", unique).
			add("sourceTags", len(s.Sources)+sourceFlags).
			add("namedSourceTags", len(s.Sources)))
		fmt.Printf("%s: %d mdx, %d preview tags (%d multiline), %d unique preview names, %d source tags (%d named, %d flagged)\n",
			s.Key, s.Files, s.PreviewTags, s.MultilineTag, unique,
			len(s.Sources)+len(s.Flags)-previewFlags, len(s.Sources), sourceFlags)
	}
	fmt.Printf("total previews: %d existing-dist, %d authored, %d to-author, %d tombstoned", pExisting, pAuthor, pToAuthor, pTomb)
	if unavail > 0 {
		fmt.Printf(" + %d unavailable (base-style)", unavail)
	}
	fmt.Println()

	// FT8: mark demos whose upstream mdx file uses React hooks as
	// "informational" — they were authored from React example-registry code in
	// FT7 batches, so their interactive semantics aren't validated against the
	// radix oracle the way primary demos are. PLAN FT4 evidence: 73/456 radix
	// demos. Heuristic: every preview in a file whose mdx uses React hooks is
	// marked informational (cheaper and more conservative than per-preview
	// scanning). Reproduces the documented 73 count within ~1.
	fileHasHooks := map[string]bool{}
	if entries, err := os.ReadDir(abs(radixDir)); err == nil {
		for _, e := range entries {
			if !strings.HasSuffix(e.Name(), ".mdx") {
				continue
			}
			b, err := os.ReadFile(abs(radixDir + "/" + e.Name()))
			if err != nil {
				continue
			}
			if reHooks.MatchString(stripFences(string(b))) {
				fileHasHooks[strings.TrimSuffix(e.Name(), ".mdx")] = true
			}
		}
	}
	marked := 0
	for i := range previews {
		for _, h := range previews[i].HostPages {
			if fileHasHooks[h] {
				previews[i].Quality = "informational"
				marked++
				break
			}
		}
	}
	fmt.Printf("quality: %d informational (host radix page uses React hooks; not contract-tested)\n", marked)

	previewJSON := make([]any, len(previews))
	for i, p := range previews {
		o := jsonObj{}.add("name", p.Name)
		if v, ok := optJSON(p.StyleName); ok {
			o = o.add("styleName", v)
		}
		if v, ok := optJSON(p.Description); ok {
			o = o.add("description", v)
		}
		o = o.add("hostPages", p.HostPages).add("status", p.Status)
		if p.DemoNull {
			o = o.add("demoPath", jsonNull{})
		} else {
			o = o.add("demoPath", p.DemoPath)
		}
		if p.Quality != "" {
			o = o.add("quality", p.Quality)
		}
		previewJSON[i] = o
	}
	sourceJSON := make([]any, len(sources))
	for i, s := range sources {
		o := jsonObj{}.add("name", s.Name).add("hostPages", s.HostPages).add("status", s.Status)
		if s.DemoNull {
			o = o.add("demoPath", jsonNull{})
		} else {
			o = o.add("demoPath", s.DemoPath)
		}
		sourceJSON[i] = o
	}
	flagJSON := make([]any, len(flags))
	for i, f := range flags {
		o := jsonObj{}.add("file", f.File).add("kind", f.Kind).add("reason", f.Reason)
		if v, ok := optJSON(f.Src); ok {
			o = o.add("src", v)
		}
		flagJSON[i] = o
	}

	catalog := jsonObj{}.
		add("version", 1).
		add("generatedFrom", jsonObj{}.
			add("repo", pin.ShadcnUI.Repo).
			add("tag", pin.ShadcnUI.Tag).
			add("commit", pin.ShadcnUI.Commit)).
		add("sets", setStats).
		add("previews", previewJSON).
		add("sources", sourceJSON).
		add("flags", flagJSON)

	if err := os.MkdirAll(abs("docs"), 0o755); err != nil {
		return fail(err)
	}
	if err := os.WriteFile(abs(catalogOut), []byte(marshalJS(catalog, "")+"\n"), 0o644); err != nil {
		return fail(err)
	}
	radixStats := setStats[0].V.(jsonObj)
	fmt.Printf("radix unique preview names: %v\n", radixStats[4].V)
	fmt.Printf("catalog: %s (%d previews, %d sources, %d flags)\n", catalogOut, len(previews), len(sources), len(flags))
	return 0
}
