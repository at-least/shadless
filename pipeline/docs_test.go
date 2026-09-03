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
	if got := rtlRAW[s.end-len("</Steps>"):s.end]; got != "</Steps>" {
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
	raw := "The `AvatarImage` component displays the avatar image. It accepts all Radix UI Avatar Image props.\nThe `AvatarFallback` component displays a fallback when the image fails to load. It accepts all Radix UI Avatar Fallback props.\n"
	out, err := applyTextAdjustments("avatar.mdx", raw)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out, "plain `<img data-slot=\"avatar-image\">`") {
		t.Error("false claim rewritten")
	}
	if strings.Contains(out, "It accepts all Radix UI Avatar Image props.") {
		t.Error("original claim gone")
	}
	if untouched, _ := applyTextAdjustments("badge.mdx", raw); untouched != raw {
		t.Error("unlisted file untouched")
	}
	if _, err := applyTextAdjustments("avatar.mdx", "unrelated prose"); err == nil ||
		!strings.Contains(err.Error(), "text adjustment avatar-props-prose") {
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
func TestUnitDocsBuildParity(t *testing.T) {
	root := "/home/newlix/github/at-least/shadless"
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "docs/components", "docs/guides", "docs/index.md", "docs/content-map.json").Output(); len(out) != 0 {
		t.Skipf("docs dirty, skip parity: %s", strings.SplitN(string(out), "\n", 2)[0])
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
		t.Fatalf("docs-build: %v\n%s", err, out)
	}
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "docs/components", "docs/guides", "docs/index.md", "docs/content-map.json").Output(); len(out) != 0 {
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
	if len(guides) < 9 {
		t.Errorf("GUIDES list sane: %d", len(guides))
	}
	for _, g := range guides {
		if g.slug == "" || g.route == "" || g.source == "" {
			t.Errorf("guide entry incomplete: %+v", g)
		}
	}
}
