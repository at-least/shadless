package main

// example-fixture — INTERACTIVE pages for dialog-family kernel examples,
// ported from tools/example-fixture.mjs.
//
// Generation per example: oracle-render the CLOSED state, then CLICK the
// trigger and harvest the MOUNTED overlay/content radix appended to <body>
// — assemble a fixture page: closed markup + <template id="d1-portal"> +
// vendored kernel + the component's glue.
//
// Self-verifying: write to the final path, click through (open → content
// present → dismiss → closed); on any failure the page is DELETED and the
// example reported. --check regenerates and byte-compares.
//
// The in-page halves (id remapping via learn/remap, template harvesting,
// the tabs DOM rebuild, the shadless.get API walk) live as embedded .js
// files — they are browser JS by nature; Go owns the orchestration.

import (
	_ "embed"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"

	"github.com/evanw/esbuild/pkg/api"
)

//go:embed ef_harvest_layer.js
var efHarvestLayer string

//go:embed ef_menu_ids.js
var efMenuIds string

//go:embed ef_nav_ids.js
var efNavIds string

//go:embed ef_tabs.js
var efTabsDriver string

//go:embed ef_api.js
var efApiDriver string

const (
	efTmp      = "build/example-fixture"
	efSelftest = "build/fixture"
)

type efDef struct {
	Name         string `json:"name"`
	Imports      string `json:"imports"`
	Usage        string `json:"usage"`
	Open         string `json:"open"`
	ShadlessPage string `json:"shadlessPage"`
	OracleCss    string `json:"oracleCss"`
}

type efTarget struct {
	name        string
	families    []string
	unsupported []string
	trivial     []string
	def         *efDef // contracts mode
}

// buildContractOracleGo — tools/contracts/oracle-build.mjs in Go: bundle a
// contract def's React usage tree into OUT/oracle.{js,html}.
func buildContractOracleGo(def efDef, out, recorder string) error {
	entry := "\nimport React from \"react\";\n" +
		"import { createRoot } from \"react-dom/client\";\n" +
		def.Imports + "\n" +
		recorder + "\n" +
		"window.__open = true;\n" +
		"const root = createRoot(document.getElementById(\"root\"));\n" +
		"const render = () => root.render((" + def.Usage + "));\n" +
		"window.__setOpen = (o) => { window.__open = o; render(); };\n" +
		"render();\n"
	cache := oracleCacheDir()
	for _, d := range []string{out, cache} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			return err
		}
	}
	base := filepath.Base(out)
	entryFile := filepath.Join(cache, ".contract-entry-"+base+".mjs")
	bundle := absOrDie(filepath.Join(cache, "contract-"+base+".js"))
	if err := os.WriteFile(entryFile, []byte(entry), 0o644); err != nil {
		return err
	}
	res := api.Build(api.BuildOptions{
		EntryPoints: []string{entryFile},
		Bundle:      true,
		Format:      api.FormatIIFE,
		Outfile:     bundle,
		Write:       true, // the Go API's zero value is false — no file on disk
		LogLevel:    api.LogLevelError,
		Alias: map[string]string{
			"@":                            absOrDie(".upstream/shadcn-ui/apps/v4"),
			"@/registry/bases/radix/ui":    absOrDie("build/resolved-ui/ui"),
			"@/registry/bases/radix/lib":   absOrDie("build/resolved-ui/lib"),
			"@/registry/bases/radix/hooks": absOrDie("build/resolved-ui/hooks"),
			// route-group indirection + subtree cut: ui components import the
			// demo-app icon switcher, which pulls next/navigation + nuqs
			// into the oracle bundle — stub it
			"@/app/(create)/components/icon-placeholder": absOrDie("tools/contracts/stubs/icon-placeholder.jsx"),
		},
		Loader: map[string]api.Loader{".tsx": api.LoaderTSX},
		JSX:    api.JSXAutomatic,
	})
	if len(res.Errors) > 0 {
		return fmt.Errorf("esbuild: %s", res.Errors[0].Text)
	}
	styleTag := ""
	if def.OracleCss != "" {
		styleTag = "<style>" + def.OracleCss + "</style>"
	}
	rel, err := filepath.Rel(absOrDie(out), bundle)
	if err != nil {
		return err
	}
	html := "<!doctype html><html><head>" + styleTag + "</head><body><div id=\"root\"></div><script src=\"" + rel + "\"></script></body></html>"
	return os.WriteFile(filepath.Join(out, "oracle.html"), []byte(html), 0o644)
}

// ---- radix id stabilization (Go halves of learn/remap/stripRadixIds) ----

var (
	efReStripRadixIds = regexp.MustCompile(`\s(?:id|aria-controls|aria-labelledby|aria-describedby)="radix-[^"]*"`)
	efReStripHidden   = regexp.MustCompile(`\s(?:aria-hidden|data-aria-hidden)="true"`)
	efReRadixTok      = regexp.MustCompile(`radix-[\w:-]*`)
	efReIdAttr        = regexp.MustCompile(`\sid="(radix-[^"]*)"`)
	efReLabelledBy    = regexp.MustCompile(`aria-labelledby="(radix-[^"]*)"`)
	efReIdInTag       = regexp.MustCompile(`\sid="(radix-[^"]*)"`)
	// "k0s1" → "k0": a submenu layer's stable id carries its parent's plus a
	// sub index. The head is REQUIRED: a select instance's own prefix is "s0",
	// and `s\d+$` on its own turned that into "" — nine fixture pages shipped
	// aria-labelledby="-trigger".
	efReTrailingSub = regexp.MustCompile(`^(.+?)s\d+$`)
	efReWordTrigger = regexp.MustCompile(`^(\w+)-trigger$`)
	efReHasIdAttr   = regexp.MustCompile(`\sid="`)
)

func efStripRadixIds(h string) string {
	h = efReStripRadixIds.ReplaceAllString(h, "")
	return efReStripHidden.ReplaceAllString(h, "")
}

func efRemap(h string, idMap map[string]string) string {
	return efReRadixTok.ReplaceAllStringFunc(h, func(id string) string {
		if v, ok := idMap[id]; ok {
			return v
		}
		return id
	})
}

type efSlotStable struct{ slot, stable string }

// efLearn maps radix auto ids to stable fixture ids (insertion-ordered
// slotToStable, exactly like the JS Object.entries walk).
func efLearn(html string, slotToStable []efSlotStable, idMap map[string]string) {
	base := ""
	for _, e := range slotToStable {
		// attribute order is radix's, not ours: find the tag, then its id
		var tag string
		re := regexp.MustCompile(`<[^>]*data-slot="` + regexp.QuoteMeta(e.slot) + `"[^>]*>`)
		if m := re.FindString(html); m != "" {
			tag = m
		}
		if tag != "" {
			if m := efReIdInTag.FindStringSubmatch(tag); m != nil {
				idMap[m[1]] = e.stable
			}
		}
		if base == "" {
			base = e.stable
		}
	}
	// every other radix id inside the layer gets a stable derived id so
	// the references between them survive
	n := 0
	if base == "" {
		base = "x"
	}
	for _, m := range efReIdAttr.FindAllStringSubmatch(html, -1) {
		if _, ok := idMap[m[1]]; !ok {
			idMap[m[1]] = base + "-e" + strconv.Itoa(n)
			n++
		}
	}
	// a reference to an id that exists nowhere in the layer is radix's
	// internal id for the TRIGGER: point it at the stable trigger id
	triggerStable := ""
	if base != "x" {
		triggerStable = efReTrailingSub.ReplaceAllString(base, "$1") + "-trigger"
	}
	for _, m := range efReLabelledBy.FindAllStringSubmatch(html, -1) {
		if _, ok := idMap[m[1]]; !ok && triggerStable != "" {
			idMap[m[1]] = triggerStable
		}
	}
}

// efEnsureContentId — on the FIRST tag carrying the data-slot, insert id
// before ">" unless the tag already has one. The JS this was ported from
// used a lookahead that only saw the text AFTER the data-slot attribute;
// radix renders `id` BEFORE `data-slot` (`role="dialog" id="k0"
// data-slot="popover-content"`), so the check missed the id that efRemap
// had just written and spliced a second id="k0" onto the same element —
// every popover-family fixture page shipped a tag with two id attributes.
// The whole tag is inspected now, from its "<" to its ">".
func efEnsureContentId(h, comp, id string) string {
	attr := `data-slot="` + comp + `-content"`
	idx := strings.Index(h, attr)
	if idx < 0 {
		return h
	}
	end := strings.IndexByte(h[idx:], '>')
	if end < 0 {
		return h
	}
	end += idx
	start := strings.LastIndexByte(h[:idx], '<')
	if start < 0 {
		start = 0
	}
	if efReHasIdAttr.MatchString(h[start:end]) {
		return h
	}
	return h[:end] + ` id="` + id + `">` + h[end+1:]
}

// efTriggerWithId — dialog's closed-markup trigger annotation: on the
// first `<… data-slot="<comp>-trigger"…>` tag, append id="d1-trigger"
// unless the tag already carries any id.
func efTriggerWithId(bodyHtml, comp, id string) string {
	re := regexp.MustCompile(`(<[^>]*data-slot="` + regexp.QuoteMeta(comp) + `-trigger"[^>]*?)>`)
	loc := re.FindStringSubmatchIndex(bodyHtml)
	if loc == nil {
		return bodyHtml
	}
	open := bodyHtml[loc[2]:loc[3]]
	if efReHasIdAttr.MatchString(open) {
		return bodyHtml
	}
	return bodyHtml[:loc[3]] + ` id="` + id + `">` + bodyHtml[loc[1]:]
}

// efRetargetJs — spliced into every place a trigger is renamed: whatever in
// the root pointed at the OLD id (a <label for>, aria-labelledby,
// aria-describedby, aria-controls, aria-owns) now points at the new one.
// Upstream authors ids on triggers (<SelectTrigger id="checkout-exp-month-
// ts6"> with a matching <FieldLabel htmlFor>); renaming the trigger to
// s0-trigger without this left every such label orphaned.
const efRetargetJs = `if (o && o !== t.id) { const A = ["for", "aria-labelledby", "aria-describedby", "aria-controls", "aria-owns"]; document.querySelectorAll("#root " + A.map((a) => "[" + a + "]").join(",")).forEach((e) => { for (const a of A) { const v = e.getAttribute(a); if (v == null) continue; const ts = v.split(/\s+/); if (ts.includes(o)) e.setAttribute(a, ts.map((x) => (x === o ? t.id : x)).join(" ")) } }) }`

// ---- the runner ----

func runExampleFixture(args []string) int {
	contracts := has(args, "--contracts")
	check := has(args, "--check")

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "example-fixture:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "example-fixture:", err)
		return 1
	}
	page, err := shell.newPageErrorsOnly()
	if err != nil {
		fmt.Fprintln(os.Stderr, "example-fixture:", err)
		return 1
	}
	if err := page.routeAbortExternal(); err != nil {
		fmt.Fprintln(os.Stderr, "example-fixture:", err)
		return 1
	}

	var targets []efTarget
	if contracts {
		ents, err := os.ReadDir("tools/contracts/components")
		if err != nil {
			fmt.Fprintln(os.Stderr, "example-fixture:", err)
			return 1
		}
		var files []string
		for _, e := range ents {
			if strings.HasSuffix(e.Name(), ".mjs") {
				files = append(files, e.Name())
			}
		}
		sort.Strings(files)
		for _, f := range files {
			res, err := shell.call(map[string]any{
				"op":   "loadContractDef",
				"file": "file://" + absOrDie(filepath.Join("tools/contracts/components", f)),
			})
			if err != nil {
				fmt.Fprintln(os.Stderr, "example-fixture:", err)
				return 1
			}
			var def efDef
			b, _ := json.Marshal(res["def"])
			json.Unmarshal(b, &def)
			comp := strings.TrimSuffix(strings.TrimSuffix(f, ".mjs"), "-multiple")
			if _, ok := family[comp]; !ok || !strings.HasPrefix(def.ShadlessPage, "src/kernel/") {
				continue
			}
			name := strings.TrimSuffix(f, ".mjs")
			d := def
			targets = append(targets, efTarget{name: name, families: []string{comp}, def: &d})
		}
	} else {
		tb, err := os.ReadFile("docs/example-fixture-targets.json")
		if err != nil {
			fmt.Fprintln(os.Stderr, "example-fixture:", err)
			return 1
		}
		var raw []struct {
			Name     string   `json:"name"`
			Families []string `json:"families"`
			Trivial  []string `json:"trivial"`
		}
		if err := json.Unmarshal(tb, &raw); err != nil {
			fmt.Fprintln(os.Stderr, "example-fixture:", err)
			return 1
		}
		for _, t := range raw {
			var known, unsupported []string
			for _, f := range t.Families {
				if _, ok := family[f]; ok {
					known = append(known, f)
				} else {
					unsupported = append(unsupported, f)
				}
			}
			// dialog kinds annotate a closed-markup COPY and must run last
			// so the copy already carries the other families' annotations
			sort.SliceStable(known, func(i, j int) bool {
				return family[known[i]].kind != "dialog" && family[known[j]].kind == "dialog"
			})
			targets = append(targets, efTarget{name: t.Name, families: known, unsupported: unsupported, trivial: t.Trivial})
		}
	}

	// the self-test renders from a scratch tree under build/ assembled from
	// what this build just produced (docs/demos carries no vendored assets)
	for _, d := range []string{efSelftest + "/pages", efSelftest + "/js"} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "example-fixture:", err)
			return 1
		}
	}
	jsEnts, _ := os.ReadDir("dist/js")
	for _, e := range jsEnts {
		b, err := os.ReadFile(filepath.Join("dist/js", e.Name()))
		if err != nil {
			continue
		}
		os.WriteFile(filepath.Join(efSelftest, "js", e.Name()), b, 0o644)
	}
	if b, err := os.ReadFile("dist/shadless.js"); err == nil {
		os.WriteFile(filepath.Join(efSelftest, "shadless.js"), b, 0o644)
	}
	// absent only in a tree that has never been built — the page then
	// renders unstyled, exactly as it did when docs/site/out.css was
	// missing
	if b, err := os.ReadFile("dist/out.css"); err == nil {
		os.WriteFile(filepath.Join(efSelftest, "out.css"), b, 0o644)
	}

	emitted := 0
	var failures, unsupportedPages []string

	strOf := func(v any) string {
		s, _ := v.(string)
		return s
	}
	intOf := func(v any) int {
		f, _ := v.(float64)
		return int(f)
	}
	rootHtml := func() (string, error) {
		v, err := page.evaluateFn(`() => document.querySelector("#root").innerHTML`)
		if err != nil {
			return "", err
		}
		return strOf(v), nil
	}
	harvestAdded := func(before int) (string, error) {
		v, err := page.evaluateFnArg(`(n) => {
      const added = [...document.body.children].slice(n)
        .filter((el) => el.tagName !== "SCRIPT" && !el.hasAttribute("data-radix-focus-guard"))
      return added.length ? added.map((el) => el.outerHTML).join("\n") : null
    }`, before)
		if err != nil {
			return "", err
		}
		return strOf(v), nil
	}

	for _, target := range targets {
		name := target.name
		if len(target.families) == 0 {
			unsupportedPages = append(unsupportedPages, name+": "+strings.Join(target.unsupported, ", "))
			continue
		}
		if len(target.unsupported) > 0 {
			unsupportedPages = append(unsupportedPages, name+": "+strings.Join(target.unsupported, ", ")+" (page wired for "+strings.Join(target.families, ", ")+" only)")
		}
		outPath := filepath.Join("docs", "demos", name+".html")
		css, base, jsdir := "../out.css", "../shadless.js", "../js/"
		if contracts {
			outPath = target.def.ShadlessPage
			css, base, jsdir = "../../dist/out.css", "../../dist/shadless.js", "../../dist/js/"
		}
		// the self-test page: the regeneration's bytes, written into the
		// scratch tree at the DEPTH the target sits at (default pages point
		// their asset tags one level up, contracts pages at the repo root
		// two levels up), renamed over the committed page only after the
		// self-test passes. The page must keep its .html name: chromium
		// refuses file:// navigation to anything else (a hidden path or an
		// unknown extension renders nothing), and the self-test would time
		// out on a blank document.
		scratchPath := filepath.Join(efSelftest, "pages", name+".html")
		if contracts {
			scratchPath = filepath.Join(efSelftest, name+".html")
		}
		err := func() (err error) {
			defer func() {
				if r := recover(); r != nil {
					err = fmt.Errorf("harness: %v", r)
				}
			}()
			if contracts {
				out := filepath.Join(efTmp, "contracts", name)
				if err := buildContractOracleGo(*target.def, out, ""); err != nil {
					return err
				}
				if err := page.gotoURL("file://" + absOrDie(filepath.Join(out, "oracle.html"))); err != nil {
					return err
				}
				page.waitForTimeout(600)
				// controlled-open trees mount their content at first render — close
				page.evaluateFn(`() => { if (typeof window.__setOpen === "function" && window.__open) window.__setOpen(false) }`)
				page.waitForTimeout(400)
			} else {
				htmlFile, err := buildOracleGo(name, efTmp)
				if err != nil {
					return err
				}
				if err := awaitOracleGo(page, htmlFile); err != nil {
					return err
				}
			}
			dirV, _ := page.evaluate(`document.documentElement.getAttribute("dir") || "ltr"`)
			dir := strOf(dirV)
			if dir == "" {
				dir = "ltr"
			}
			// several families on one page: each contributes templates and
			// its glue; the first family's self-test proves the page
			var jsFiles []string
			seen := map[string]bool{}
			for _, f := range target.families {
				j := family[f].js
				if j != "" && !seen[j] {
					seen[j] = true
					jsFiles = append(jsFiles, j)
				}
			}
			var allTemplates []string
			var selfTests []func() error
			var bodyHtml string

			for _, comp := range target.families {
				fam := family[comp]
				templates := ""
				var selfTest func() error
				idMap := map[string]string{}

				switch fam.kind {
				case "dialog":
					closedHtml, err := rootHtml()
					if err != nil {
						return err
					}
					beforeV, _ := page.evaluateFn(`() => document.body.children.length`)
					before := intOf(beforeV)
					if err := page.locClick("", `[data-slot$="-trigger"]`, 0, "left"); err != nil {
						return err
					}
					page.waitForTimeout(600)
					portalHtml, err := harvestAdded(before)
					if err != nil {
						return err
					}
					if portalHtml == "" {
						return fmt.Errorf("no mounted overlay/content after trigger click")
					}
					efLearn(portalHtml, []efSlotStable{
						{comp + "-content", "d1"},
						{comp + "-title", "d1-title"},
						{comp + "-description", "d1-desc"},
					}, idMap)
					trigV, _ := page.evaluateFnArg(`(sel) => document.querySelector("#root " + sel)?.id`, `[data-slot="`+comp+`-trigger"]`)
					if trigOrig := strOf(trigV); strings.HasPrefix(trigOrig, "radix-") {
						idMap[trigOrig] = "d1-trigger"
					}
					fixed := efStripRadixIds(efRemap(portalHtml, idMap))
					bodyHtml = efTriggerWithId(efStripRadixIds(efRemap(closedHtml, idMap)), comp, "d1-trigger")
					templates = "<template id=\"d1-portal\">\n" + fixed + "\n</template>"
					selfTest = func() error {
						if err := page.locClick("", "#d1-trigger", 0, "left"); err != nil {
							return err
						}
						page.waitForTimeout(500)
						mine := `[data-slot="` + comp + `-content"]`
						openV, _ := page.evaluateFnArg(`(sel) => !!document.querySelector(sel)`, mine)
						if openV != true {
							return fmt.Errorf("did not open")
						}
						// dismissal mirrors what the glue wires: content's
						// direct-child X, then action/cancel, then Escape
						page.evaluateFn(`() => {
              const btn = document.querySelector('[data-slot$="-content"] > button')
                ?? document.querySelector('[data-slot$="-action"], [data-slot$="-cancel"]')
              if (btn) btn.click()
              else document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }))
            }`)
						page.waitForTimeout(500)
						closedV, _ := page.evaluateFnArg(`(sel) => !document.querySelector(sel)`, mine)
						if closedV != true {
							return fmt.Errorf("did not close")
						}
						return nil
					}

				case "portal":
					// one template per trigger instance: open each in the
					// live React page, harvest what radix appended to <body>
					triggerSel := `[data-slot="` + comp + `-trigger"]`
					contentSel := `[data-slot="` + comp + `-content"]`
					countV, _ := page.evaluateFnArg(`(sel) => document.querySelectorAll("#root " + sel).length`, triggerSel)
					count := intOf(countV)
					if count == 0 {
						return fmt.Errorf("no trigger in the oracle render")
					}
					var parts []string
					mounted := func() (string, error) {
						v, err := page.evaluateFnArg(`(sel) => {
              const c = document.querySelector(sel + ":not([data-ef-harvested])")
              if (!c) return null
              const top = c.closest("body > *") || c
              const html = top.outerHTML
              c.setAttribute("data-ef-harvested", "")
              return html
            }`, contentSel)
						if err != nil {
							return "", err
						}
						return strOf(v), nil
					}
					origV, _ := page.evaluateFnArg(`(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id)`, triggerSel)
					var origIds []string
					if arr, ok := origV.([]any); ok {
						for _, e := range arr {
							origIds = append(origIds, strOf(e))
						}
					}
					prefixOf := func(i int) string {
						if m := efReWordTrigger.FindStringSubmatch(origIds[i]); m != nil {
							return m[1]
						}
						return "k" + strconv.Itoa(i)
					}
					refs := make([]bool, count)
					for i := 0; i < count; i++ {
						act := func() error {
							if fam.open == "hover" {
								// approach from BELOW the row: a horizontal path
								// from the page corner crosses sibling triggers
								if err := page.mouseMove(2, 2, 1); err != nil {
									return err
								}
								page.waitForTimeout(400)
								box, err := page.locBox("", "#root "+triggerSel, i)
								if err != nil {
									return err
								}
								if box == nil {
									return fmt.Errorf("instance %d: no trigger box", i)
								}
								if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height+60, 1); err != nil {
									return err
								}
								page.waitForTimeout(300)
								if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height/2, 6); err != nil {
									return err
								}
								page.waitForTimeout(1100) // radix TooltipProvider / HoverCard open delays (700ms)
								return nil
							}
							if err := page.locClick("", "#root "+triggerSel, i, "left"); err != nil {
								return err
							}
							page.waitForTimeout(500)
							return nil
						}
						if err := act(); err != nil {
							return err
						}
						portalHtml, err := mounted()
						if err != nil {
							return err
						}
						if portalHtml == "" {
							if err := act(); err != nil {
								return err
							}
							portalHtml, err = mounted()
							if err != nil {
								return err
							}
						}
						if portalHtml == "" {
							return fmt.Errorf("instance %d: nothing mounted after %s", i, fam.open)
						}
						efLearn(portalHtml, []efSlotStable{{comp + "-content", prefixOf(i)}}, idMap)
						if strings.HasPrefix(origIds[i], "radix-") {
							idMap[origIds[i]] = prefixOf(i) + "-trigger"
						}
						// does the open trigger point at its content? Then the
						// glue will publish content.id and the content root
						// needs one — read WHILE OPEN (aria-describedby is
						// added on open)
						refV, _ := page.evaluateFnArg(`(a) => { const t = document.querySelectorAll("#root " + a.sel)[a.i]; return t.hasAttribute("aria-describedby") || t.hasAttribute("aria-controls") }`,
							map[string]any{"sel": triggerSel, "i": i})
						refs[i] = refV == true
						parts = append(parts, efReHarvestMark.ReplaceAllString(portalHtml, ""))
						page.keyPress("Escape")
						page.mouseMove(0, 0, 1)
						page.waitForTimeout(700)
					}
					// ids by instance order: k<i>-trigger ↔ k<i>-portal
					ids := make([]string, count)
					prefixes := make([]string, count)
					for i := range ids {
						ids[i] = prefixOf(i) + "-trigger"
						prefixes[i] = prefixOf(i)
					}
					page.evaluateFnArg(`({ sel, ids }) => document.querySelectorAll("#root " + sel).forEach((t, i) => { const o = t.id; t.id = ids[i]; `+efRetargetJs+` })`,
						map[string]any{"sel": triggerSel, "ids": ids})
					rh, err := rootHtml()
					if err != nil {
						return err
					}
					bodyHtml = efStripRadixIds(efRemap(rh, idMap))
					var tpls []string
					for k, p := range parts {
						content := efStripRadixIds(efRemap(p, idMap))
						if refs[k] {
							content = efEnsureContentId(content, comp, prefixes[k])
						}
						tpls = append(tpls, "<template id=\""+prefixes[k]+"-portal\">\n"+content+"\n</template>")
					}
					templates = strings.Join(tpls, "\n")
					selfTest = func() error {
						sel := "#" + prefixes[0] + "-trigger"
						if fam.open == "hover" {
							box, err := page.locBox("", sel, 0)
							if err != nil {
								return err
							}
							if box == nil {
								return fmt.Errorf("no trigger box")
							}
							if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height+60, 1); err != nil {
								return err
							}
							page.waitForTimeout(300)
							if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height/2, 6); err != nil {
								return err
							}
							page.waitForTimeout(1100)
						} else {
							if err := page.locClick("", sel, 0, "left"); err != nil {
								return err
							}
							page.waitForTimeout(500)
						}
						openV, _ := page.evaluateFnArg(`(c) => !!document.querySelector("[data-slot=\"" + c + "-content\"]")`, comp)
						if openV != true {
							return fmt.Errorf("did not open")
						}
						page.keyPress("Escape")
						page.mouseMove(0, 0, 1)
						page.waitForTimeout(700)
						closedV, _ := page.evaluateFnArg(`(c) => !document.querySelector("[data-slot=\"" + c + "-content\"]")`, comp)
						if closedV != true {
							return fmt.Errorf("did not close")
						}
						return nil
					}

				case "menu", "select":
					isSelect := fam.kind == "select"
					triggerSel := `[data-slot="` + comp + `-trigger"]`
					contentSel := `[data-slot="` + comp + `-content"]`
					subTriggerSel := `[data-slot="` + comp + `-sub-trigger"]`
					// disabled triggers open nothing: skip them
					enabledV, _ := page.evaluateFnArg(`(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => !t.disabled && t.getAttribute("aria-disabled") !== "true" && !t.hasAttribute("data-disabled"))`, triggerSel)
					var enabled []bool
					if arr, ok := enabledV.([]any); ok {
						for _, e := range arr {
							enabled = append(enabled, e == true)
						}
					}
					count := len(enabled)
					if count == 0 {
						return fmt.Errorf("no trigger in the oracle render")
					}
					type layer struct{ layerId, html string }
					var templates_ []layer
					mountedContent := func(sel string) bool {
						v, _ := page.evaluateFnArg(`(sel) => !!document.querySelector(sel + ":not([data-ef-harvested])")`, sel)
						return v == true
					}
					var harvestLayer func(layerId, sel string) error
					harvestLayer = func(layerId, sel string) error {
						if !mountedContent(sel) {
							return fmt.Errorf("layer %s: nothing mounted", layerId)
						}
						v, err := page.evaluateFnArg(efHarvestLayer, map[string]any{"sel": sel, "sub": subTriggerSel, "layerId": layerId})
						if err != nil {
							return err
						}
						m, _ := v.(map[string]any)
						if m == nil {
							return fmt.Errorf("layer %s: harvest failed", layerId)
						}
						html := strOf(m["html"])
						subCount := intOf(m["subCount"])
						efLearn(html, []efSlotStable{
							{comp + "-content", layerId},
							{comp + "-sub-content", layerId},
						}, idMap)
						templates_ = append(templates_, layer{layerId, efReHarvestMark.ReplaceAllString(html, "")})
						for j := 0; j < subCount; j++ {
							st := "#" + layerId + "s" + strconv.Itoa(j) + "-trigger"
							box, err := page.locBox("", st, 0)
							if err != nil {
								return err
							}
							if box == nil {
								return fmt.Errorf("layer %s: no sub-trigger box", layerId)
							}
							// radix opens a sub menu on pointer movement over
							// its trigger, or on ArrowRight from the focused
							// trigger; pointer first
							if err := page.mouseMove(box.X+4, box.Y+box.Height/2, 3); err != nil {
								return err
							}
							if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height/2, 6); err != nil {
								return err
							}
							page.waitForTimeout(600)
							subSel := `[data-slot="` + comp + `-sub-content"]`
							if !mountedContent(subSel) {
								page.evaluateFnArg(`(sel) => document.querySelector(sel).focus()`, st)
								page.keyPress("ArrowRight")
								page.waitForTimeout(500)
							}
							if err := harvestLayer(layerId+"s"+strconv.Itoa(j), subSel); err != nil {
								return err
							}
							// back to the parent layer: point away
							if err := page.mouseMove(box.X+box.Width/2, box.Y-40, 4); err != nil {
								return err
							}
							page.waitForTimeout(300)
						}
						return nil
					}
					existingV, _ := page.evaluateFnArg(`(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => /^(\w+)-trigger$/.exec(t.id)?.[1] ?? null)`, triggerSel)
					menuOrigV, _ := page.evaluateFnArg(`(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id)`, triggerSel)
					var existing []string
					var menuOrig []string
					if arr, ok := existingV.([]any); ok {
						for _, e := range arr {
							existing = append(existing, strOf(e)) // nil → ""
						}
					}
					if arr, ok := menuOrigV.([]any); ok {
						for _, e := range arr {
							menuOrig = append(menuOrig, strOf(e))
						}
					}
					idOf := func(i int) string {
						if existing[i] != "" {
							return existing[i]
						}
						if isSelect {
							return "s" + strconv.Itoa(i)
						}
						return "m" + strconv.Itoa(i)
					}
					for i, o := range menuOrig {
						if strings.HasPrefix(o, "radix-") {
							idMap[o] = idOf(i) + "-trigger"
						}
					}
					for i := 0; i < count; i++ {
						if !enabled[i] {
							continue
						}
						if fam.open == "contextmenu" {
							box, err := page.locBox("", "#root "+triggerSel, i)
							if err != nil {
								return err
							}
							if box == nil {
								return fmt.Errorf("instance %d: no trigger box", i)
							}
							if err := page.mouseClickButton(box.X+box.Width/2, box.Y+box.Height/2, "right"); err != nil {
								return err
							}
						} else {
							if err := page.locClick("", "#root "+triggerSel, i, "left"); err != nil {
								return err
							}
						}
						page.waitForTimeout(500)
						if err := harvestLayer(idOf(i), contentSel); err != nil {
							return err
						}
						// close everything (Escape per open layer)
						for k := 0; k < 4; k++ {
							page.keyPress("Escape")
							page.waitForTimeout(150)
						}
						page.waitForTimeout(400)
					}
					ids := make([]string, count)
					for i := range ids {
						ids[i] = idOf(i)
					}
					page.evaluateFnArg(efMenuIds, map[string]any{"sel": triggerSel, "isSelect": isSelect, "attr": fam.attr, "ids": ids})
					rh, err := rootHtml()
					if err != nil {
						return err
					}
					bodyHtml = efStripRadixIds(efRemap(rh, idMap))
					// sub-trigger original ids (recorded in-page) map to
					// their stable ids
					for _, t := range templates_ {
						for _, m := range efReOrigAndId.FindAllStringSubmatch(t.html, -1) {
							idMap[m[1]] = m[2]
						}
					}
					var tpls []string
					for _, t := range templates_ {
						tpls = append(tpls, "<template id=\""+t.layerId+"-tpl\">\n"+efReOrigAttr.ReplaceAllString(efStripRadixIds(efRemap(t.html, idMap)), "")+"\n</template>")
					}
					templates = strings.Join(tpls, "\n")
					firstEnabled := -1
					for i, e := range enabled {
						if e {
							firstEnabled = i
							break
						}
					}
					selfTest = func() error {
						if firstEnabled < 0 {
							return nil // nothing openable by design
						}
						sel := "#" + idOf(firstEnabled) + "-trigger"
						if fam.open == "contextmenu" {
							box, err := page.locBox("", sel, 0)
							if err != nil {
								return err
							}
							if box == nil {
								return fmt.Errorf("no trigger box")
							}
							if err := page.mouseClickButton(box.X+box.Width/2, box.Y+box.Height/2, "right"); err != nil {
								return err
							}
						} else {
							if err := page.locClick("", sel, 0, "left"); err != nil {
								return err
							}
						}
						page.waitForTimeout(500)
						openV, _ := page.evaluateFnArg(`(sel) => !!document.querySelector(sel + "[data-state=open]")`, contentSel)
						if openV != true {
							return fmt.Errorf("did not open")
						}
						// the first sub menu must open from its trigger too
						n, _ := page.locCount("", subTriggerSel)
						if n > 0 {
							box, err := page.locBox("", subTriggerSel, 0)
							if err != nil {
								return err
							}
							if box != nil {
								if err := page.mouseMove(box.X+4, box.Y+box.Height/2, 3); err != nil {
									return err
								}
								if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height/2, 6); err != nil {
									return err
								}
								page.waitForTimeout(600)
								subV, _ := page.evaluateFnArg(`(c) => !!document.querySelector("[data-slot=\"" + c + "-sub-content\"]")`, comp)
								if subV != true {
									return fmt.Errorf("sub menu did not open")
								}
								page.keyPress("Escape")
								page.waitForTimeout(300)
							}
						}
						page.keyPress("Escape")
						page.waitForTimeout(500)
						closedV, _ := page.evaluateFnArg(`(sel) => !!document.querySelector(sel + "[data-state=open]")`, contentSel)
						if closedV == true {
							return fmt.Errorf("did not close")
						}
						return nil
					}

				case "nav":
					triggerSel := `[data-slot="navigation-menu-trigger"]`
					contentSel := `[data-slot="navigation-menu-content"]`
					countV, _ := page.evaluateFnArg(`(sel) => document.querySelectorAll("#root " + sel).length`, triggerSel)
					count := intOf(countV)
					if count == 0 {
						return fmt.Errorf("no trigger in the oracle render")
					}
					var parts []string
					navOrigV, _ := page.evaluateFnArg(`(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id)`, triggerSel)
					var navOrig []string
					if arr, ok := navOrigV.([]any); ok {
						for _, e := range arr {
							navOrig = append(navOrig, strOf(e))
						}
					}
					navIdOf := func(i int) string {
						if m := efReWordTrigger.FindStringSubmatch(navOrig[i]); m != nil {
							return m[1]
						}
						return "n" + strconv.Itoa(i)
					}
					for i := 0; i < count; i++ {
						if err := page.locClick("", "#root "+triggerSel, i, "left"); err != nil {
							return err
						}
						page.waitForTimeout(500)
						v, err := page.evaluateFnArg(`(sel) => {
              const c = document.querySelector(sel + ":not([data-ef-harvested])")
              if (!c) return null
              const html = c.outerHTML; c.setAttribute("data-ef-harvested", ""); return html
            }`, contentSel)
						if err != nil {
							return err
						}
						html := strOf(v)
						if html == "" {
							return fmt.Errorf("instance %d: no content mounted after click", i)
						}
						efLearn(html, []efSlotStable{{"navigation-menu-content", navIdOf(i) + "-content"}}, idMap)
						if strings.HasPrefix(navOrig[i], "radix-") {
							idMap[navOrig[i]] = navIdOf(i) + "-trigger"
						}
						parts = append(parts, efReHarvestMark.ReplaceAllString(html, ""))
						page.keyPress("Escape")
						page.waitForTimeout(400)
					}
					page.evaluateFnArg(efNavIds, triggerSel)
					rh, err := rootHtml()
					if err != nil {
						return err
					}
					bodyHtml = efStripRadixIds(efRemap(rh, idMap))
					navIdsV, _ := page.evaluateFnArg(`(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id.replace(/-trigger$/, ""))`, triggerSel)
					var navIds []string
					if arr, ok := navIdsV.([]any); ok {
						for _, e := range arr {
							navIds = append(navIds, strOf(e))
						}
					}
					var tpls []string
					for k, p := range parts {
						tpls = append(tpls, "<template id=\""+navIds[k]+"-content-tpl\">\n"+efStripRadixIds(efRemap(p, idMap))+"\n</template>")
					}
					templates = strings.Join(tpls, "\n")
					selfTest = func() error {
						if err := page.locClick("", "#"+navIds[0]+"-trigger", 0, "left"); err != nil {
							return err
						}
						page.waitForTimeout(500)
						openV, _ := page.evaluateFnArg(`(sel) => !!document.querySelector(sel)`, contentSel)
						if openV != true {
							return fmt.Errorf("did not open")
						}
						page.keyPress("Escape")
						page.waitForTimeout(500)
						closedV, _ := page.evaluateFnArg(`(sel) => !!document.querySelector(sel + "[data-state=open]")`, contentSel)
						if closedV == true {
							return fmt.Errorf("did not close")
						}
						return nil
					}

				case "inline":
					// tabs: radix mounts only the active panel; activate each
					// trigger and collect its panel, then rebuild every root
					v, err := page.evaluateFn(efTabsDriver)
					if err != nil {
						return err
					}
					bodyHtml = strOf(v)
					selfTest = func() error {
						idxV, _ := page.evaluateFn(`() => [...document.querySelectorAll("[data-slot=tabs-trigger]")]
              .findIndex((t) => !t.disabled && t.getAttribute("data-state") !== "active")`)
						idx := intOf(idxV)
						if idx < 0 {
							return nil
						}
						if err := page.locClick("", "[data-slot=tabs-trigger]", idx, "left"); err != nil {
							return err
						}
						page.waitForTimeout(300)
						okV, _ := page.evaluateFnArg(`(i) => {
              const t = document.querySelectorAll("[data-slot=tabs-trigger]")[i]
              const p = document.getElementById(t.getAttribute("aria-controls"))
              // panel-less lists: the trigger state is the whole story
              return t.getAttribute("data-state") === "active" && (!p || !p.hasAttribute("hidden"))
            }`, idx)
						if okV != true {
							return fmt.Errorf("tab %d did not activate", idx)
						}
						return nil
					}

				default: // "none"
					if contracts && target.def.Open != "" {
						if err := page.driver(target.def.Open); err != nil {
							return err
						}
						page.waitForTimeout(500)
					}
					rh, err := rootHtml()
					if err != nil {
						return err
					}
					bodyHtml = efStripRadixIds(rh)
					selfTest = func() error {
						switch comp {
						case "slider":
							n, _ := page.locCount("", "[data-slot=slider-thumb]")
							if n == 0 {
								return nil
							}
							before, _, _ := page.locAttr("", "[data-slot=slider-thumb]", "aria-valuenow")
							page.evaluateFnArg(`(sel) => document.querySelector(sel).focus()`, "[data-slot=slider-thumb]")
							page.keyPress("ArrowRight")
							page.waitForTimeout(200)
							after, _, _ := page.locAttr("", "[data-slot=slider-thumb]", "aria-valuenow")
							if before == after {
								disV, _ := page.evaluateFnArg(`(sel) => { const t = document.querySelector(sel); return t.getAttribute("aria-disabled") === "true" || !!t.closest("[data-disabled]") }`, "[data-slot=slider-thumb]")
								if disV != true {
									return fmt.Errorf("slider thumb did not move on ArrowRight")
								}
							}
						case "carousel":
							n, _ := page.locCount("", "[data-slot=carousel-next]:not([disabled])")
							if n == 0 {
								return nil
							}
							beforeV, _ := page.evaluateFn(`() => [...document.querySelectorAll("[data-slot=carousel-previous]")].map((b) => b.disabled).join()`)
							if err := page.locClick("", "[data-slot=carousel-next]:not([disabled])", 0, "left"); err != nil {
								return err
							}
							page.waitForTimeout(500)
							afterV, _ := page.evaluateFn(`() => [...document.querySelectorAll("[data-slot=carousel-previous]")].map((b) => b.disabled).join()`)
							if strOf(beforeV) == strOf(afterV) {
								return fmt.Errorf("carousel did not scroll on next (previous button state unchanged)")
							}
						case "scroll-area":
							wiredV, _ := page.evaluateFn(`() => [...document.querySelectorAll("[data-slot=scroll-area-scrollbar]")].every((b) => b.hasAttribute("data-state") || b.style.length > 0 || b.querySelector("[data-slot=scroll-area-thumb]")?.style.length > 0)`)
							if wiredV != true {
								return fmt.Errorf("scroll-area scrollbars not wired")
							}
						}
						return nil
					}
				}

				if templates != "" {
					allTemplates = append(allTemplates, templates)
				}
				if selfTest != nil {
					selfTests = append(selfTests, selfTest)
				}
			}
			if contracts {
				bodyHtml = "<div>\n" + bodyHtml + "\n</div>"
			}
			inner := bodyHtml
			if len(allTemplates) > 0 {
				inner = bodyHtml + "\n" + strings.Join(allTemplates, "\n")
			}
			dirAttr := ""
			if dir == "rtl" {
				dirAttr = ` dir="rtl"`
			}
			var jsTags []string
			for _, g := range jsFiles {
				jsTags = append(jsTags, "<script src=\""+jsdir+g+".js\"></script>")
			}
			var trivTags []string
			for _, c := range target.trivial {
				trivTags = append(trivTags, "<script src=\""+jsdir+c+".js\"></script>")
			}
			html := "<!doctype html>\n<html" + dirAttr + "><head><meta charset=\"utf-8\"><title>shadless " + name + "</title>\n" +
				"<link rel=\"stylesheet\" href=\"" + css + "\">" + ThemePrePaintScript + "</head>\n" +
				"<body class=\"p-8\">\n" + inner + "\n" +
				"<script src=\"" + base + "\"></script>\n" +
				strings.Join(jsTags, "\n") + "\n" +
				strings.Join(trivTags, "\n") + "\n" +
				"</body></html>\n"

			// prove the page interactive BEFORE it lands: a red node keeps
			// previous contents instead of deleting or half-updating them
			// (the same policy example-oracle states)
			if err := os.WriteFile(scratchPath, []byte(html), 0o644); err != nil {
				return err
			}
			ev0, _ := page.events()
			errBase := len(ev0)
			testPath := scratchPath
			if err := page.gotoURL("file://" + absOrDie(testPath)); err != nil {
				return err
			}
			page.waitForTimeout(400)
			for _, st := range selfTests {
				if err := st(); err != nil {
					ev1, _ := page.events()
					errs := efTruncateAll(ev1[errBase:])
					joined := strings.Join(errs, " | ")
					if joined == "" {
						joined = "no page errors"
					}
					return fmt.Errorf("self-test: %s (%s)", err.Error(), joined)
				}
			}
			// the programmatic handles: every openable instance must open and
			// close through shadless.get(trigger) too, and announce it
			var fams []string
			for _, f := range target.families {
				switch family[f].kind {
				case "dialog", "portal", "menu", "select", "nav":
					fams = append(fams, f)
				}
			}
			if len(fams) > 0 {
				apiV, err := page.evaluateFnArg(efApiDriver, fams)
				if err != nil {
					return err
				}
				if s := strOf(apiV); s != "" {
					return fmt.Errorf("self-test (api): %s", s)
				}
			}
			ev1, _ := page.events()
			if n := len(ev1) - errBase; n > 0 {
				return fmt.Errorf("self-test: page errors — %s", strings.Join(efTruncateAll(ev1[errBase:]), " | "))
			}
			// compare against the committed page BEFORE the rename: after it,
			// outPath is the regeneration and the comparison is vacuous
			if check {
				committed, err := os.ReadFile(outPath)
				if err != nil {
					return err
				}
				if string(committed) != html {
					return fmt.Errorf("committed page drifted from regeneration")
				}
			}
			if err := os.Rename(testPath, outPath); err != nil {
				return err
			}
			return nil
		}()
		if err != nil {
			// the committed page was never touched; the scratch page only
			// survives for EF_KEEP debugging
			if os.Getenv("EF_KEEP") == "" {
				os.Remove(scratchPath)
			}
			failures = append(failures, name+": "+firstLine(err.Error()))
			continue
		}
		emitted++
	}

	if len(failures) > 0 {
		fmt.Fprintf(os.Stderr, "FAIL  example-fixture\n  %s\n", strings.Join(failures, "\n  "))
		return 1
	}
	if len(unsupportedPages) > 0 {
		fmt.Printf("example-fixture: %d pages carry families without a protocol yet:\n  %s\n", len(unsupportedPages), strings.Join(unsupportedPages, "\n  "))
	}
	suffix := " emitted"
	if check {
		suffix = " == committed"
	}
	fmt.Printf("PASS  example-fixture (%d interactive pages%s, open/close self-verified)\n", emitted, suffix)
	return 0
}

var efReHarvestMark = regexp.MustCompile(`\sdata-ef-harvested=""`)
var efReOrigAttr = regexp.MustCompile(`\sdata-ef-orig="[^"]*"`)
var efReOrigAndId = regexp.MustCompile(`data-ef-orig="(radix-[^"]*)"[^>]*\sid="([^"]+)"`)

func efTruncateAll(in []string) []string {
	out := make([]string, len(in))
	for i, s := range in {
		if len(s) > 400 {
			s = s[:400]
		}
		out[i] = s
	}
	return out
}
