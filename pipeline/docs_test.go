package main

import (
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// ---- fenceShadow / locators (ported from tools/unit/transforms.mjs) ----

func TestUnitFenceShadow(t *testing.T) {
	src := "```js\nabc\n```\nX marks the spot"
	shadow := fenceShadow(src)
	if len(shadow) != len(src) {
		t.Errorf("length preserved: %d != %d", len(shadow), len(src))
	}
	if !strings.Contains(shadow, "X marks the spot") {
		t.Error("prose untouched")
	}
	if strings.Contains(shadow, "abc") {
		t.Error("fence content blanked")
	}
}

const transformsRAW = "intro prose\n<CodeTabs>\n<TabsList><TabsTrigger value=\"cli\">Command</TabsTrigger></TabsList>\n<TabsContent value=\"cli\">\n```bash\nnpx shadcn@latest add x\n```\n</TabsContent>\n<TabsContent value=\"manual\">\nnpm install radix-ui\n</TabsContent>\n</CodeTabs>\n## Usage\n<CodeTabs><TabsContent value=\"manual\">second</TabsContent></CodeTabs>\ntail"

func TestUnitLocateCodeTabs(t *testing.T) {
	spans := locateCodeTabsSpans(fenceShadow(transformsRAW))
	if len(spans) != 2 {
		t.Fatalf("both code-tabs blocks found (fenced look-alike safe): %d", len(spans))
	}
	if !strings.Contains(transformsRAW[spans[0].start:spans[0].end], "npx shadcn@latest add x") {
		t.Error("span 0 wraps the shadcn CLI fence")
	}
	if !strings.Contains(transformsRAW[spans[0].start:spans[0].end], "npm install radix-ui") {
		t.Error("span 0 also wraps the manual tab")
	}
	if !strings.Contains(transformsRAW[spans[1].start:spans[1].end], "second") {
		t.Error("span 1 wraps second block")
	}
}

const rtlRAW = "intro\n## Get Started\nbody\n\n## Migrating existing components\n\n<Steps>\n\n```bash\nnpx shadcn@latest migrate rtl\n```\n\n</Steps>"

func TestUnitRtlMigrate(t *testing.T) {
	s := locateRtlMigrateSpan(fenceShadow(rtlRAW))
	if s == nil || rtlRAW[s.start:s.start+2] != "##" {
		t.Fatal("span located at the migrate heading")
	}
	if got := rtlRAW[s.end-len("</Steps>") : s.end]; got != "</Steps>" {
		t.Errorf("end after </Steps>: %q", got)
	}
	gated := withoutRtlMigrate(rtlRAW)
	if strings.Contains(gated, "migrate rtl") {
		t.Error("gate drops the CLI fence")
	}
	if !strings.Contains(gated, "## Get Started") {
		t.Error("earlier sections survive")
	}
	if locateRtlMigrateSpan(fenceShadow("nope")) != nil {
		t.Error("absent section → nil")
	}
}

// SINGLE-SOURCE CONTRACT: builder replace == gate drop.
func TestUnitBuilderGateContract(t *testing.T) {
	raw := strings.Join(strings.Split(transformsRAW, "\n")[:12], "\n") + "\n## Usage\nbody"
	spans := locateCodeTabsSpans(fenceShadow(raw))
	if len(spans) != 1 {
		t.Fatalf("single span: %d", len(spans))
	}
	built := replaceSpan(raw, spans[0], "REPLACEMENT")
	gated := withoutCodeTabs(raw)
	if strings.Replace(built, "REPLACEMENT", "\n", 1) != gated {
		t.Error("builder span == gate drop")
	}
}

const guideRAW = "intro\n## Installation\n\n```bash\nnpm i\n```\n\n## Usage\nbody"

func TestUnitInstallSection(t *testing.T) {
	s := locateInstallSection(fenceShadow(guideRAW))
	if s == nil || guideRAW[s.start:s.start+2] != "##" {
		t.Fatal("span located")
	}
	if got := guideRAW[s.end : s.end+8]; got != "## Usage" {
		t.Errorf("end at Usage heading: %q", got)
	}
	built := replaceSpan(guideRAW, *s, "## Installation\n\nvanilla truth")
	gated := withoutInstallSection(guideRAW)
	if !strings.Contains(built, "## Installation") || !strings.Contains(gated, "## Installation") {
		t.Error("both views carry the Installation heading")
	}
	if strings.Contains(built, "npm i") || strings.Contains(gated, "npm i") {
		t.Error("both views drop the npm fence")
	}
	if locateInstallSection(fenceShadow("no sections")) != nil || withoutInstallSection("no sections") != "no sections" {
		t.Error("install absent → nil/unchanged")
	}
}

func TestUnitApplyTextAdjustments(t *testing.T) {
	raw := "Set `cursor: pointer` yourself. You can also enable this during project setup with `npx shadcn@latest init --pointer`.\n"
	out, err := applyTextAdjustments("button.mdx", raw)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out, "there is no flag that adds it for you") {
		t.Error("false claim rewritten")
	}
	if strings.Contains(out, "npx shadcn@latest init --pointer") {
		t.Error("original claim gone")
	}
	if untouched, _ := applyTextAdjustments("badge.mdx", raw); untouched != raw {
		t.Error("unlisted file untouched")
	}
	if _, err := applyTextAdjustments("button.mdx", "unrelated prose"); err == nil ||
		!strings.Contains(err.Error(), "text adjustment button-pointer-cli-prose") {
		t.Errorf("missing find throws (re-anchor required): %v", err)
	}
	for _, adj := range textAdjustments {
		if adj.id == "" || len(adj.files) < 1 || len(adj.ops) < 1 {
			t.Errorf("descriptor incomplete: %+v", adj)
		}
		for _, op := range adj.ops {
			if len(op.find) < 10 || len(op.replace) == 0 || op.find == op.replace {
				t.Errorf("op malformed in %s", adj.id)
			}
		}
	}
}

// ---- dropReactImportFences / stripImportsFromMixedFences ----

func TestUnitDropReactImportFences(t *testing.T) {
	// pure fence: dropped, ONE newline consumed after the closing fence
	pure := "frame.\n\n```tsx\nimport { X } from \"@/components/ui/x\"\n```\n\n```tsx\nconst { x } = useX()\n```\nafter"
	got := dropReactImportFences(pure)
	want := "frame.\n\n\n```tsx\nconst { x } = useX()\n```\nafter"
	if got != want {
		t.Errorf("pure fence:\n got %q\nwant %q", got, want)
	}
	// mixed fence: untouched by the drop pass (stripMixed handles it)
	mixed := "state.\n\n```tsx showLineNumbers\nimport * as React from \"react\"\n\nexport function Example() {}\n```\n\n## Next"
	if got := dropReactImportFences(mixed); got != mixed {
		t.Errorf("mixed fence must be identity, got %q", got)
	}
	// import spanning toward a fence: no match, fence kept
	spanning := "```tsx\nimport {\n A,\n```\nrest"
	if got := dropReactImportFences(spanning); got != spanning {
		t.Errorf("spanning fence kept, got %q", got)
	}
}

func TestUnitStripImportsFromMixedFences(t *testing.T) {
	// The JS loop is GREEDY for every import shape: after the import line it
	// keeps consuming until a from-line or EOF, so a single-line import
	// followed by JSX loses the WHOLE body (bubble.mdx relies on this).
	// These assertions pin that behaviour, not intuition.
	mixed := "```tsx\nimport { A } from \"@/a\"\n\nexport function E() {}\n```"
	if got := stripImportsFromMixedFences(mixed); got != "" {
		t.Errorf("mixed strip (greedy, drops body): %q", got)
	}
	renum := "```tsx {2,4-5}\nimport { A } from \"@/a\"\nconst x = 1\nconst y = 2\n```"
	if got := stripImportsFromMixedFences(renum); got != "" {
		t.Errorf("renumber (body consumed): %q", got)
	}
	only := "```tsx\nimport { A } from \"@/a\"\n```"
	if got := stripImportsFromMixedFences(only); got != "" {
		t.Errorf("imports-only drops: %q", got)
	}
	// no import at all: fence passes through untouched
	plain := "```tsx\nconst x = 1\n```"
	if got := stripImportsFromMixedFences(plain); got != plain {
		t.Errorf("no-import fence untouched: %q", got)
	}
}

// ---- frontmatter (ported from tools/unit/frontmatter.mjs) ----

func TestUnitParseFrontmatter(t *testing.T) {
	if got := parseFrontmatter("---\ntitle: Hi\ndesc: x\n---\nbody"); len(got) != 2 ||
		fmString(got, "title") != "Hi" || fmString(got, "desc") != "x" {
		t.Errorf("basic: %v", got)
	}
	if got := parseFrontmatter("# no frontmatter"); len(got) != 0 {
		t.Errorf("none: %v", got)
	}
	if got := parseFrontmatter("---\r\ntitle: Hi\r\n---\r\nbody"); fmString(got, "title") != "Hi" {
		t.Errorf("CRLF tolerated: %v", got)
	}
	if got := parseFrontmatter("---\ntitle: \"Hi\"\n---"); fmString(got, "title") != "Hi" {
		t.Errorf("matched-quote pair stripped: %v", got)
	}
	if got := parseFrontmatter("---\ntitle: years'\n---"); fmString(got, "title") != "years'" {
		t.Errorf("trailing apostrophe kept: %v", got)
	}
	if got := parseFrontmatter("---\ntop:\n  sub: 1\n---"); got["top"] == nil {
		t.Errorf("nested one level: %v", got)
	}
	got := parseFrontmatter("---\na: true\nb: false\nc: 42\n---")
	if got["a"] != true || got["b"] != false || got["c"] != 42 {
		t.Errorf("booleans/numbers coerced: %v", got)
	}
}

func TestUnitStripImports(t *testing.T) {
	if got := stripImports("import \"a\";\n\n# Heading\n\nPara.\n\nimport { x } from \"b\";\n"); got != "\n\n# Heading\n\nPara.\n\n\n" {
		t.Errorf("bare import does not swallow prose: %q", got)
	}
	if got := stripImports("import {\n  A,\n  B,\n} from \"pkg\";\nrest"); got != "\nrest" {
		t.Errorf("multiline named import: %q", got)
	}
	if got := stripImports("import x from \"p\";\nimport * as ns from \"q\";\ntext"); got != "\n\ntext" {
		t.Errorf("default + namespace: %q", got)
	}
}

// ---- fixture families: Go table == JS table (drift pin until
// example-fixture ports and the .mjs goes away) ----

func TestUnitFixtureFamiliesGolden(t *testing.T) {
	// the JS original (tools/fixture-families.mjs) is gone; the golden is
	// the snapshot its last run produced — the table it pinned is the one
	// docs_families.go must keep serving byte-identical
	gb, err := os.ReadFile(filepath.Join("testdata", "fixture-families-golden.json"))
	if err != nil {
		t.Fatal(err)
	}
	var golden map[string]struct {
		Protocol string `json:"protocol"`
		Trivial  string `json:"trivial"`
		Api0     string `json:"api0"`
		Api2     string `json:"api2"`
	}
	if err := json.Unmarshal(gb, &golden); err != nil {
		t.Fatal(err)
	}
	for name, g := range golden {
		if got := protocolMdx(name); got != g.Protocol {
			t.Errorf("%s: protocolMdx diverges\n got: %q\nwant: %q", name, got, g.Protocol)
		}
		if got := trivialMdx(name); got != g.Trivial {
			t.Errorf("%s: trivialMdx diverges", name)
		}
		if got := apiReferenceMdx(name, nil, nil, "", false); got != g.Api0 {
			t.Errorf("%s: apiReferenceMdx(0 slots) diverges", name)
		}
		if got := apiReferenceMdx(name, []string{"s1", "s2"}, nil, "", false); got != g.Api2 {
			t.Errorf("%s: apiReferenceMdx(2 slots) diverges", name)
		}
	}
}

// ---- docs-build parity: the Go rebuild leaves the committed docs tree ----
// byte-identical (the committed pages are the JS build's record).
//
// docs-build WIPES and regenerates docs/components, docs/guides and the
// public assets, and its output folds in docs/demos and dist — so this test
// only runs when both trees are git-clean. Anything uncommitted there means
// the comparison is meaningless AND rebuilding over it would destroy the
// uncommitted work. When the parity fails anyway, the rebuild is rolled back
// before the failure is raised: a red gate must not leave a damaged tree.
func TestUnitDocsBuildParity(t *testing.T) {
	root := repoRoot(t)
	// the surface docs-build reads AND writes: generated/ir and the pin move
	// the pages' content, so a dirty one would make the rebuild differ and
	// the parity red for a reason the tree did not cause
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "docs", "dist", "generated", "src/registry/pin.json", "package.json", "package-lock.json").Output(); len(out) != 0 {
		t.Skipf("docs inputs dirty, skip parity: %s", strings.SplitN(string(out), "\n", 2)[0])
	}
	if !fileExists(root + "/docs/index.md") {
		t.Skip("docs not built yet")
	}
	pipelineBin := filepath.Join(t.TempDir(), "pipeline")
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "docs-build")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		// docs-build wipes docs/components and docs/guides before it can
		// fail mid-rebuild (prettier, page errors, counts) — never leave
		// that state behind
		_ = exec.Command("git", "-C", root, "checkout", "--", "docs").Run()
		t.Fatalf("docs-build: %v\n%s", err, out)
	}
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "docs").Output(); len(out) != 0 {
		_ = exec.Command("git", "-C", root, "checkout", "--", "docs").Run()
		t.Fatalf("Go docs-build changed the committed tree:\n%s", out)
	}
}

// resolveDocsRoute (ported from tools/unit/routes.mjs — assertions unchanged).
func TestUnitResolveDocsRoute(t *testing.T) {
	members := map[string]bool{"alert": true, "button": true}
	type c struct {
		href string
		file string
		frag string
		grey bool
		nil_ bool
	}
	for _, tc := range []c{
		{href: "/docs/components/alert", file: "alert.html"},
		{href: "/docs/components/radix/alert", file: "alert.html"},
		{href: "/docs/components/base/alert", grey: true},
		{href: "/docs/components/aria/alert#x", grey: true},
		{href: "/docs/components/nope", grey: true},
		{href: "/docs/introduction", file: "introduction.html"},
		{href: "/docs/react", grey: true},
		{href: "alert.html", nil_: true},
	} {
		got := resolveDocsRoute(tc.href, members)
		if tc.nil_ {
			if got != nil {
				t.Errorf("%s: want nil, got %+v", tc.href, got)
			}
			continue
		}
		if got == nil {
			t.Errorf("%s: nil", tc.href)
			continue
		}
		if got.grey != tc.grey || got.file != tc.file || got.frag != tc.frag {
			t.Errorf("%s: got %+v want {file:%q frag:%q grey:%v}", tc.href, got, tc.file, tc.frag, tc.grey)
		}
	}
	if len(guides) < 7 {
		t.Errorf("GUIDES list sane: %d", len(guides))
	}
	for _, g := range guides {
		if g.slug == "" || g.route == "" || g.source == "" {
			t.Errorf("guide entry incomplete: %+v", g)
		}
	}
}

// The react-prop gate must fire on markup that carries a JSX prop, and stay
// quiet on markup that is already ported. It reads only the built page, so
// it is the one check a bug inside the shared rewriter cannot hide from.
func TestUnitReactPropsInMarkup(t *testing.T) {
	cases := []struct {
		fence string
		want  []string
	}{
		{`<button data-slot="button" variant="ghost" size="icon">x</button>`, []string{"size", "variant"}},
		{`<div data-slot="item" asChild>`, []string{"asChild"}},
		{`<label data-slot="field-label" htmlFor="email">Email</label>`, []string{"htmlFor"}},
		{`<button data-slot="button" data-variant="ghost" data-size="icon" aria-label="Thumbs up and down">x</button>`, nil},
		{`<a data-slot="marker" href="/files" class="x" id="m1">Explored 4 files</a>`, nil},
		{`<div class="not-a-slot" variant="ghost">`, nil},
	}
	for _, tc := range cases {
		got := reactPropsInMarkup(tc.fence)
		if strings.Join(got, ",") != strings.Join(tc.want, ",") {
			t.Errorf("reactPropsInMarkup(%q) = %v, want %v", tc.fence, got, tc.want)
		}
	}
}

// findCalloutOpen must survive a JSX attribute whose value is an expression
// containing ">". native-select.mdx is the real case; the old
// `<Callout\b[^>]*>` stopped inside `/>` and printed a bare "}>" as the first
// line of the callout.
func TestUnitFindCalloutOpen(t *testing.T) {
	for _, c := range []struct {
		name, in, want string
	}{
		{"plain", `<Callout>body`, `<Callout>`},
		{"attrs", `<Callout variant="warning" title="x">body`, `<Callout variant="warning" title="x">`},
		{"jsx expression with a > inside",
			`<Callout variant="info" icon={<InfoIcon className="translate-y-[3px]!" />}>body`,
			`<Callout variant="info" icon={<InfoIcon className="translate-y-[3px]!" />}>`},
		{"nested braces", `<Callout icon={f({a: <X/>})}>body`, `<Callout icon={f({a: <X/>})}>`},
		{"a > inside a quoted value", `<Callout title="a > b">body`, `<Callout title="a > b">`},
	} {
		s, e := findCalloutOpen(c.in)
		if s < 0 {
			t.Errorf("%s: no match", c.name)
			continue
		}
		if got := c.in[s:e]; got != c.want {
			t.Errorf("%s: got %q want %q", c.name, got, c.want)
		}
	}
	// negatives: the scanner must not claim a tag that is not there
	for _, c := range []string{"no callout here", "<CalloutBox>", "<Callout unterminated"} {
		if s, _ := findCalloutOpen(c); s >= 0 {
			t.Errorf("%q: matched but should not have", c)
		}
	}
}

// Heading facts must carry a LEVEL and the heading's real text. depth was
// m[2]-m[1] — the group start minus the whole match's end, i.e. a negative
// byte length — so "## Foo" scored -6 and "### Foo" -7, differing only
// because the line got one character longer. stripInlineCode expanded "$1"
// against a group-less regex, deleting the code span, so "## `size`" and
// "## `side`" both collapsed to the same text and the gate could not see one
// change into the other.
func TestUnitHeadingFacts(t *testing.T) {
	h := mdPageFacts("# T\n\n## Foo\n\n### Bar\n\n#### Baz\n").headings
	want := []headingEnt{{2, "Foo"}, {3, "Bar"}, {4, "Baz"}}
	if len(h) != len(want) {
		t.Fatalf("got %v want %v", h, want)
	}
	for i := range want {
		if h[i] != want[i] {
			t.Errorf("heading %d: got %v want %v", i, h[i], want[i])
		}
	}
	a := mdPageFacts("# T\n\n## `size`\n").headings
	b := mdPageFacts("# T\n\n## `side`\n").headings
	if len(a) != 1 || len(b) != 1 {
		t.Fatalf("got %v / %v", a, b)
	}
	if a[0] == b[0] {
		t.Errorf("two different code-only headings compare equal: %v", a[0])
	}
	if a[0].text != "size" {
		t.Errorf("inline code stripped away the heading text: %q", a[0].text)
	}
}

// resolveImport mirrors the JS gate's package.json exports resolution:
// bare "shadless"/"shadless/" -> exports['.'], an exact subkey, an
// object-form export needing import/default, and a "./*" wildcard fallback.
func TestUnitResolveImport(t *testing.T) {
	exports := map[string]any{
		".":                  "./dist/shadless-core.css",
		"./accordion.css":    "./dist/css/accordion.css",
		"./js":               map[string]any{"import": "./dist/esm/shadless.mjs", "default": "./dist/shadless.js"},
		"./only-default.css": map[string]any{"default": "./dist/css/only-default.css"},
		"./*":                "./dist/css/*",
	}
	cases := []struct{ spec, want string }{
		{"shadless", "dist/shadless-core.css"},
		{"shadless/", "dist/shadless-core.css"},
		{"shadless/accordion.css", "dist/css/accordion.css"},
		{"shadless/js", "dist/esm/shadless.mjs"},
		{"shadless/only-default.css", "dist/css/only-default.css"},
		{"shadless/whatever.css", "dist/css/whatever.css"}, // wildcard fallback
	}
	for _, c := range cases {
		if got := resolveImport(exports, c.spec); got != c.want {
			t.Errorf("resolveImport(%q) = %q, want %q", c.spec, got, c.want)
		}
	}
	// no exact key and no wildcard at all: resolves to nothing
	if got := resolveImport(map[string]any{".": "./dist/shadless-core.css"}, "shadless/nope.css"); got != "" {
		t.Errorf("no rule resolves it: got %q, want \"\"", got)
	}
}

// dedupePreviews/dedupeSources merge by name, first-seen metadata wins,
// hostPages accumulate in first-seen order with no duplicates.
func TestUnitDedupePreviews(t *testing.T) {
	in := []previewRec{
		{Name: "accordion-demo", Component: "accordion",
			StyleName: optStr{Present: true, Val: "radix-nova"}, Description: optStr{Present: true, Val: "first"}},
		{Name: "card-rtl", Component: "card"},
		{Name: "accordion-demo", Component: "rtl", Description: optStr{Present: true, Val: "second (must not win)"}},
		{Name: "accordion-demo", Component: "accordion"}, // repeat host page: must not duplicate
	}
	out := dedupePreviews(in)
	if len(out) != 2 {
		t.Fatalf("2 unique names, got %d: %+v", len(out), out)
	}
	if out[0].Name != "accordion-demo" || out[1].Name != "card-rtl" {
		t.Fatalf("first-seen name order: %+v", out)
	}
	a := out[0]
	if a.StyleName.Val != "radix-nova" {
		t.Errorf("first entry's StyleName wins: %+v", a.StyleName)
	}
	if a.Description.Val != "first" {
		t.Errorf("first entry's Description wins: %+v", a.Description)
	}
	if got := strings.Join(a.HostPages, ","); got != "accordion,rtl" {
		t.Errorf("hostPages accumulate in first-seen order, no dup: %q", got)
	}
}

func TestUnitDedupeSources(t *testing.T) {
	in := []sourceRec{
		{Name: "button", Component: "button"},
		{Name: "select", Component: "select"},
		{Name: "button", Component: "button-group"},
		{Name: "button", Component: "button"}, // repeat: must not duplicate
	}
	out := dedupeSources(in)
	if len(out) != 2 {
		t.Fatalf("2 unique names, got %d: %+v", len(out), out)
	}
	if out[0].Name != "button" || out[1].Name != "select" {
		t.Fatalf("first-seen name order: %+v", out)
	}
	if got := strings.Join(out[0].HostPages, ","); got != "button,button-group" {
		t.Errorf("hostPages accumulate in first-seen order, no dup: %q", got)
	}
}

// isTombstoneName pins the exact-vs-prefix boundary the comment above
// tombstonePrefixList documents a real historical bug in: menubar and
// navigation-menu were once wrongly matched by prefix drift and must not be
// again now that they are contract-tested glue, not grey.
func TestUnitIsTombstoneName(t *testing.T) {
	cases := []struct {
		name string
		want bool
	}{
		{"calendar", true},         // exact member of greyComponents
		{"date-picker-demo", true}, // genuine prefix match: "date-picker-"
		{"menubar", false},         // historical bug: no longer grey, must not prefix-match
		{"navigation-menu", false}, // historical bug: no longer grey, must not prefix-match
		{"date", false},            // shares letters with "date-picker" but no "-" boundary
		{"forms", false},           // shares letters with "form" but no "-" boundary
	}
	for _, c := range cases {
		if got := isTombstoneName(c.name); got != c.want {
			t.Errorf("isTombstoneName(%q) = %v, want %v", c.name, got, c.want)
		}
	}
}

// kebabIconName matches lucide's own kebab-case icon ids.
func TestUnitKebabIconName(t *testing.T) {
	cases := map[string]string{
		"ChevronLeftIcon": "chevron-left",
		"XIcon":           "x",
	}
	for in, want := range cases {
		if got := kebabIconName(in); got != want {
			t.Errorf("kebabIconName(%q) = %q, want %q", in, got, want)
		}
	}
}

// rewriteJsxAttrs: aria-*/data-* and plain HTML attrs pass through, a
// declared cva axis becomes its data attribute, an undeclared axis value
// and an unrelated prop both fail loud (the cue for a docs_overrides.go
// entry) rather than shipping markup no stylesheet selects.
func TestUnitRewriteJsxAttrs(t *testing.T) {
	info := jsxTagInfo{tag: "button", slot: "button", axes: map[string][]string{"variant": {"default", "ghost"}}}

	if got, err := rewriteJsxAttrs("Button", `aria-label="Close" data-foo="bar"`, info); err != nil || got != ` aria-label="Close" data-foo="bar"` {
		t.Errorf("aria-/data- passthrough: got %q err %v", got, err)
	}
	if got, err := rewriteJsxAttrs("Button", `id="x"`, info); err != nil || got != ` id="x"` {
		t.Errorf("plain html attr passthrough: got %q err %v", got, err)
	}
	if got, err := rewriteJsxAttrs("Button", `variant="ghost"`, info); err != nil || got != ` data-variant="ghost"` {
		t.Errorf("declared axis value rewritten: got %q err %v", got, err)
	}
	if _, err := rewriteJsxAttrs("Button", `variant="fancy"`, info); err == nil || !strings.Contains(err.Error(), "not a declared value") {
		t.Errorf("undeclared axis value must fail loud: %v", err)
	}
	if _, err := rewriteJsxAttrs("Button", `asChild`, info); err == nil || !strings.Contains(err.Error(), "has no markup form") {
		t.Errorf("unrelated prop must fail loud: %v", err)
	}
}

// shiftHighlightRefs/atoiSafe: TestUnitStripImportsFromMixedFences's existing
// cases all use a single-line import, whose greedy inner loop (documented at
// docs_transforms.go:338) swallows the whole fence body before EOF, so
// removed always equals len(lines) and shiftHighlightRefs is never called. A
// multi-line import whose closing line matches reImportFrom before EOF stops
// early instead, leaving real code behind to renumber.
func TestUnitStripImportsFromMixedFencesRenumber(t *testing.T) {
	// 3 lines removed (the multi-line import), 2 lines of real code survive.
	// Highlight refs: "1" is inside the removed range (dropped), "4-" has a
	// blank endpoint (kept verbatim, whatever it referred to), "5-6" is past
	// the removed range (shifted by -3 to "2-3").
	in := "```tsx {1,4-,5-6}\nimport {\n  A,\n} from \"@/a\"\nconst x = 1\nconst y = 2\n```"
	want := "```tsx {4-,2-3}\nconst x = 1\nconst y = 2\n```"
	if got := stripImportsFromMixedFences(in); got != want {
		t.Errorf("multi-line import renumber:\n got %q\nwant %q", got, want)
	}
}

// normalizeBlankLines: collapse repeated blank lines outside fences, leave
// fence content (blank lines and "#"-looking lines alike) untouched, and
// insert a blank line above a heading that does not already have one.
func TestUnitNormalizeBlankLines(t *testing.T) {
	in := strings.Join([]string{
		"prose line",
		"## Heading After Prose", // no blank above -> one inserted
		"text",
		"",
		"## Heading Already Spaced", // already preceded by blank -> unchanged
		"para",
		"",
		"", // double blank OUTSIDE a fence -> collapses to one
		"after double blank",
		"```text",
		"",
		"",                                // double blank INSIDE a fence -> survives untouched
		"## not a heading, fence content", // "#" line inside a fence -> left alone
		"```",
		"closing prose",
	}, "\n")
	want := strings.Join([]string{
		"prose line",
		"",
		"## Heading After Prose",
		"text",
		"",
		"## Heading Already Spaced",
		"para",
		"",
		"after double blank",
		"```text",
		"",
		"",
		"## not a heading, fence content",
		"```",
		"closing prose",
	}, "\n")
	if got := normalizeBlankLines(in); got != want {
		t.Errorf("normalizeBlankLines:\n got %q\nwant %q", got, want)
	}
}
