package main

// demo build, ported from tools/demo.mjs: unified slot-keyed CSS for all
// emitted components + a browsable demo page per component in
// dist/components/. Static pages come from the emitter; kernel pages reuse
// the verified src/kernel fixtures; trivial-js pages reuse probes/t7;
// carousel from probes/t8; menubar/navigation-menu from src/kernel; field's
// presentational fixture is inlined below (as it was in demo.mjs).
//
// Paths are rewritten to dist-relative assets and a single dist/out.css
// styles every page (compiled later by the tw node).

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// rewritePaths — ported from tools/demo-lib.mjs. The bare `out.css` form is
// a SEPARATE replace from the `[^"]*-out\.css` form: the dash is mandatory
// in the first pattern, so it can never match plain "out.css". Deleting the
// "duplicate" was the 2026-08-25 incident this ordering now pins.
var (
	reLinkCompCss = regexp.MustCompile(`(<link[^>]*href=")[^"]*-out\.css(")`)
	reLinkBareCss = regexp.MustCompile(`(<link[^>]*href=")out\.css(")`)
	reLinkDistCss = regexp.MustCompile(`(<link[^>]*href=")\.\./\.\./dist/out\.css(")`)
	reScriptBase  = regexp.MustCompile(`(<script[^>]*src=")\.\./\.\./dist/shadless\.js(")`)
	reScriptComp  = regexp.MustCompile(`(<script[^>]*src=")\.\./\.\./dist/js/([\w-]+\.js)(")`)
)

func rewritePaths(html string) string {
	out := reLinkCompCss.ReplaceAllString(html, `${1}../out.css$2`)
	out = reLinkBareCss.ReplaceAllString(out, `${1}../out.css$2`)
	out = reLinkDistCss.ReplaceAllString(out, `${1}../out.css$2`)
	out = reScriptBase.ReplaceAllString(out, `${1}../shadless.js$2`)
	out = reScriptComp.ReplaceAllString(out, `${1}../js/$2$3`)
	return out
}

var reHasOutCss = regexp.MustCompile(`<link[^>]*out\.css`)

// ensureLink: t7 fixtures ship without a stylesheet link.
func ensureLink(html string) string {
	if reHasOutCss.MatchString(html) {
		return html
	}
	return strings.Replace(html, "<head>", "<head>\n<link rel=\"stylesheet\" href=\"../out.css\">", 1)
}

var kernelT6 = []string{"alert-dialog", "context-menu", "dropdown-menu", "hover-card",
	"popover", "scroll-area", "select", "sheet", "slider", "tabs", "tooltip"}

var trivialT7 = []string{"accordion", "aspect-ratio", "avatar", "checkbox", "collapsible",
	"label", "progress", "radio-group", "separator", "switch", "toggle", "toggle-group"}

// out.css's content scan is EXPLICIT (source(none)); this list == the
// `demo-css` inputs in pipeline/nodes.go — keep them in step. ./js because
// the runtime injects utility classes at wire time; ../generated/ir
// because shipped pages carry HTML-escaped '>' variants the scanner cannot
// read from markup — still unfixed on the consumer side.
var demoSources = []string{"./components", "./js", "../docs/demos", "../docs/content", "../src/kernel",
	"../tools/contracts/out", "../generated/ir", "../probes/t7", "../probes/t8"}

func fieldDemoHtml() string {
	return `<!doctype html>
<html><head><meta charset="utf-8"><title>shadless field</title>
<link rel="stylesheet" href="../out.css"></head>
<body>
  <fieldset data-slot="field-set">
    <legend data-slot="field-legend" data-variant="legend">Login</legend>

    <div data-slot="field-group">
      <div data-slot="field" class="group/field" data-orientation="vertical">
        <div data-slot="field-content" class="group/field-content">
          <label data-slot="field-label" class="group/field-label peer/field-label" for="email">Email</label>
          <input data-slot="input" id="email" type="email" placeholder="m@example.com">
          <p data-slot="field-description">We'll never share your email.</p>
        </div>
      </div>

      <div data-slot="field" class="group/field" data-orientation="vertical" data-invalid="true">
        <div data-slot="field-content" class="group/field-content">
          <label data-slot="field-label" class="group/field-label peer/field-label" for="pw">Password</label>
          <input data-slot="input" id="pw" type="password" aria-invalid="true">
          <div data-slot="field-error">Password must be at least 8 characters.</div>
        </div>
      </div>

      <div data-slot="field-separator" class="group/field-group">
        <div data-slot="separator" class="absolute inset-0 top-1/2"></div>
        <span data-slot="field-separator-content">or</span>
      </div>
    </div>
  </fieldset>
</body></html>`
}

func runDemo() int {
	loadSkin()
	if err := os.MkdirAll("dist/components", 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}

	// tiers + shipped set (the four hardcoded tier sets are gone — dispatch
	// is tier-based, emit:true in tiers.json covers the exceptions)
	tiersB, err := os.ReadFile("src/registry/tiers.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}
	var regTiers map[string]struct {
		Tier string `json:"tier"`
		Emit bool   `json:"emit"`
	}
	if err := json.Unmarshal(tiersB, &regTiers); err != nil {
		fmt.Fprintln(os.Stderr, "demo: tiers:", err)
		return 1
	}
	shippedTier := func(tier string) bool {
		return tier == "static" || tier == "kernel" || tier == "trivial-js"
	}

	ents, err := os.ReadDir("generated/ir")
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}
	var names []string
	var fileOrder []string // ReadDir order: "alert-dialog.json" sorts BEFORE "alert.json" ('-' < '.')
	irAll := map[string]cssIrComponent{}
	for _, e := range ents {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b, _ := os.ReadFile(filepath.Join("generated/ir", e.Name()))
		var ir cssIrComponent
		if err := json.Unmarshal(b, &ir); err != nil {
			fmt.Fprintln(os.Stderr, "demo: ir:", e.Name(), err)
			return 1
		}
		if shippedTier(ir.Tier) || regTiers[ir.Name].Emit {
			names = append(names, ir.Name)
			fileOrder = append(fileOrder, ir.Name)
			irAll[ir.Name] = ir
		}
	}
	sort.Strings(names)
	// count assertion derives from the same predicate, so the two cannot disagree
	expected := 0
	for n, t := range regTiers {
		_ = n
		if shippedTier(t.Tier) || t.Emit {
			expected++
		}
	}
	if len(irAll) != expected {
		fmt.Fprintf(os.Stderr, "demo: expected %d emitted components (static/kernel/trivial-js + tiers.json emit:true), got %d\n", expected, len(irAll))
		return 1
	}

	// ---- 1. unified globals.css ----
	baseB, err := os.ReadFile("probes/h4/globals.css")
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}
	base := strings.Replace(string(baseB), "@source \"./demo.html\";\n", "", 1)
	if err := os.MkdirAll("dist/css", 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}
	var cssParts []string
	cssFiles := map[string]bool{}
	for _, name := range fileOrder { // JS iterated irAll = ReadDir order
		css, err := componentCss(irAll[name])
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL css[%s]: %v\n", name, err)
			return 1
		}
		if len(css.rules) == 0 {
			continue
		}
		part := wrapComponentCss(name, css)
		cssParts = append(cssParts, part)
		if err := os.WriteFile("dist/css/"+name+".css", []byte(part+"\n"), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "demo:", err)
			return 1
		}
		cssFiles[name+".css"] = true
	}
	// a component leaving the emitted set takes its file with it — an orphan
	// keeps shipping through shadless.product.css (the form.json bug class)
	distCssEnts, _ := os.ReadDir("dist/css")
	for _, f := range distCssEnts {
		if strings.HasSuffix(f.Name(), ".css") && !cssFiles[f.Name()] {
			os.Remove(filepath.Join("dist/css", f.Name()))
			fmt.Printf("demo: removed orphaned dist/css/%s (no longer an emitted component)\n", f.Name())
		}
	}
	var srcs []string
	for _, d := range demoSources {
		srcs = append(srcs, fmt.Sprintf("@source \"%s\";", d))
	}
	globals := strings.Replace(base, `@import "tailwindcss";`, `@import "tailwindcss" source(none);`, 1) +
		"\n" + strings.Join(srcs, "\n") + "\n\n" + ShadlessCSSFixes + "\n\n" + strings.Join(cssParts, "\n\n") +
		"\n@layer base { body { @apply bg-background text-foreground p-8; } }\n"
	if err := os.WriteFile("dist/globals.css", []byte(globals), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}

	// ---- 3. per-component demo pages ----
	emitted := 0
	readFixture := func(p string) (string, bool) {
		b, err := os.ReadFile(p)
		return string(b), err == nil
	}
	for _, name := range names {
		ir := irAll[name]
		var html string
		switch ir.Tier {
		case "static":
			if _, err := os.Stat("dist/components/" + name + ".html"); err != nil {
				fmt.Fprintf(os.Stderr, "demo: static page missing: %s (run the emit step first)\n", name)
				return 1
			}
			emitted++
			continue
		case "kernel":
			if name == "dialog" || containsTok(kernelT6, name) {
				src, ok := readFixture("src/kernel/" + name + ".html")
				if !ok {
					fmt.Fprintf(os.Stderr, "demo: no kernel fixture for %s\n", name)
					return 1
				}
				html = rewritePaths(src)
			} else {
				fmt.Fprintf(os.Stderr, "demo: no kernel fixture for %s\n", name)
				return 1
			}
		case "medium":
			if name == "menubar" || name == "navigation-menu" {
				src, ok := readFixture("src/kernel/" + name + ".html")
				if !ok {
					fmt.Fprintf(os.Stderr, "demo: no medium fixture for %s\n", name)
					return 1
				}
				html = ensureLink(rewritePaths(src))
			} else {
				fmt.Fprintf(os.Stderr, "demo: no medium fixture for %s\n", name)
				return 1
			}
		case "trivial-js":
			if !containsTok(trivialT7, name) {
				fmt.Fprintf(os.Stderr, "demo: no trivial fixture for %s\n", name)
				return 1
			}
			src, ok := readFixture("probes/t7/" + name + ".html")
			if !ok {
				fmt.Fprintf(os.Stderr, "demo: no trivial fixture for %s\n", name)
				return 1
			}
			html = ensureLink(rewritePaths(src))
		case "logic":
			if name == "field" {
				html = fieldDemoHtml()
			} else {
				fmt.Fprintf(os.Stderr, "demo: no presentational fixture for %s\n", name)
				return 1
			}
		case "external":
			if name == "carousel" {
				src, ok := readFixture("probes/t8/carousel.html")
				if !ok {
					fmt.Fprintln(os.Stderr, "demo: no carousel fixture")
					return 1
				}
				html = ensureLink(rewritePaths(src))
			} else {
				fmt.Fprintf(os.Stderr, "demo: no external fixture for %s\n", name)
				return 1
			}
		default:
			fmt.Fprintf(os.Stderr, "demo: unhandled tier %s\n", ir.Tier)
			return 1
		}
		if err := os.WriteFile("dist/components/"+name+".html", []byte(injectPrePaint(html)), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "demo:", err)
			return 1
		}
		emitted++
	}
	if emitted != expected {
		fmt.Fprintf(os.Stderr, "demo: emitted %d, expected %d\n", emitted, expected)
		return 1
	}

	// ---- 4. demo index ----
	groups := [][2]string{
		{"static", "Static (markup + CSS)"},
		{"kernel", "Kernel (base + per-component behavior)"},
		{"trivial-js", "Trivial (shadless runtime)"},
		{"logic", "Presentational logic (markup + CSS)"},
		{"external", "External (vanilla port)"},
	}
	var idx strings.Builder
	idx.WriteString("<!doctype html><html><head><meta charset=\"utf-8\"><title>shadless demo</title>\n<link rel=\"stylesheet\" href=\"out.css\"></head>\n<body>\n<h1>shadless demo</h1>\n")
	for _, g := range groups {
		var ns []string
		for _, name := range names {
			if irAll[name].Tier == g[0] || (g[0] == "kernel" && irAll[name].Tier == "medium") {
				ns = append(ns, name)
			}
		}
		fmt.Fprintf(&idx, "<h2>%s <small>(%d)</small></h2><ul>", g[1], len(ns))
		for _, n := range ns {
			fmt.Fprintf(&idx, "<li><a href=\"components/%s.html\">%s</a></li>", n, n)
		}
		idx.WriteString("</ul>\n")
	}
	idx.WriteString("</body></html>")
	if err := os.WriteFile("dist/demo-index.html", []byte(idx.String()), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "demo:", err)
		return 1
	}
	fmt.Printf("demo: %d pages, globals.css + assets written\n", emitted)
	return 0
}
