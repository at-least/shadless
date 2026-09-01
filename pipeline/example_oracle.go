package main

// example-oracle, ported from tools/example-oracle.mjs — upstream
// examples/*.tsx → shadless demo HTML, 1:1 by construction. BUILD renders
// the real React + registry sources in chromium (the oracle), extracts
// #root.innerHTML as the page body; --check re-renders and byte-diffs the
// normalized DOM against the emitted page.
//
// All-or-nothing: a render failure writes NOTHING (pages or manifests) —
// a partial emit once shrank the owned set to zero pages while every
// downstream gate stayed green.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const (
	oraExamplesDir = ".upstream/shadcn-ui/apps/v4/examples/radix"
	oraManifest    = "docs/example-oracle.json"
	oraFixTargets  = "docs/example-fixture-targets.json"
)

var fixtureFamilySel = map[string]string{
	"alert-dialog":     `[data-slot="alert-dialog-trigger"]`,
	"dialog":           `[data-slot="dialog-trigger"]`,
	"sheet":            `[data-slot="sheet-trigger"]`,
	"popover":          `[data-slot="popover-trigger"]`,
	"tooltip":          `[data-slot="tooltip-trigger"]`,
	"hover-card":       `[data-slot="hover-card-trigger"]`,
	"dropdown-menu":    `[data-slot="dropdown-menu-trigger"]`,
	"context-menu":     `[data-slot="context-menu-trigger"]`,
	"menubar":          `[data-slot="menubar"]`,
	"select":           `[data-slot="select-trigger"]`,
	"tabs":             `[data-slot="tabs"]`,
	"slider":           `[data-slot="slider"]`,
	"scroll-area":      `[data-slot="scroll-area"]`,
	"carousel":         `[data-slot="carousel"]`,
	"navigation-menu":  `[data-slot="navigation-menu"]`,
}

type oraTarget struct {
	name      string
	out       string
	bodyStyle string
}

func oraLoadTargets() (targets []oraTarget, skipped []string) {
	catalogB, _ := os.ReadFile("docs/catalog.json")
	var catalog struct {
		Previews []struct {
			Name   string `json:"name"`
			Status string `json:"status"`
		} `json:"previews"`
	}
	json.Unmarshal(catalogB, &catalog)
	tiersB, _ := os.ReadFile("src/registry/tiers.json")
	var tiers map[string]struct {
		Tier string `json:"tier"`
	}
	json.Unmarshal(tiersB, &tiers)
	isKernelDemo := func(demoName string) bool {
		m := regexp.MustCompile(`^(.+)-demo$`).FindStringSubmatch(demoName)
		return m != nil && tiers[m[1]].Tier == "kernel"
	}
	// alert-demo replaces the retired build-demo hand emitter (dist target +
	// padding:1rem body — the docs host page historically framed it that way)
	targets = append(targets, oraTarget{name: "alert-demo", out: "dist/components/alert-demo.html", bodyStyle: "padding:1rem"})
	seen := map[string]bool{"alert-demo": true}
	for _, p := range catalog.Previews {
		if !fileExists(filepath.Join(oraExamplesDir, p.Name+".tsx")) {
			skipped = append(skipped, p.Name)
			continue
		}
		if p.Status != "authored" && p.Status != "existing-dist" {
			continue
		}
		if seen[p.Name] {
			continue
		}
		if isKernelDemo(p.Name) {
			skipped = append(skipped, p.Name)
			continue
		}
		targets = append(targets, oraTarget{name: p.Name, out: "docs/demos/" + p.Name + ".html"})
		seen[p.Name] = true
	}
	return targets, skipped
}

// oraScriptsHead: trivial-tier families become INTERACTIVE by loading the
// runtime — scripts deferred in <head> so <body> stays byte-equal to the
// oracle render (that is what --check compares).
func oraScriptsHead(trivial []string) string {
	if len(trivial) == 0 {
		return ""
	}
	s := "\n<script defer src=\"../shadless.js\"></script>"
	for _, c := range trivial {
		s += "\n<script defer src=\"../js/" + c + ".js\"></script>"
	}
	return s
}

func oraPageHtml(name, body, bodyAttr string, trivial []string) string {
	attr := bodyAttr
	if attr == "" {
		attr = `class="p-8"`
	}
	return "<!doctype html>\n<html><head><meta charset=\"utf-8\"><title>shadless " + name +
		"</title>\n<link rel=\"stylesheet\" href=\"../out.css\">" + injectPrePaint("") + oraScriptsHead(trivial) +
		"</head>\n<body " + attr + ">\n" + body + "\n</body></html>"
}

func runExampleOracle(check bool) int {
	// NO_ORACLE: examples whose golden exemption reason starts "external dep"
	// cannot be bundled; their pages stay hand-authored.
	var exemptions struct {
		Examples map[string]struct {
			Reason string `json:"reason"`
		} `json:"examples"`
	}
	if eb, err := os.ReadFile("src/registry/upstream-snapshot/exemptions.json"); err == nil {
		json.Unmarshal(eb, &exemptions)
	}
	noOracle := map[string]bool{}
	reExtDep := regexp.MustCompile(`^external dep `)
	for name, e := range exemptions.Examples {
		if reExtDep.MatchString(e.Reason) {
			noOracle[name] = true
		}
	}

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "example-oracle:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "example-oracle:", err)
		return 1
	}
	page, _ := shell.newPage(false)

	targets, skipped := oraLoadTargets()
	if len(skipped) > 0 {
		fmt.Printf("oracle: %d authored demos have no upstream example (kept hand-authored): %s\n",
			len(skipped), strings.Join(skipped, ", "))
	}

	if check {
		ownedB, err := os.ReadFile(oraManifest)
		if err != nil {
			fmt.Fprintln(os.Stderr, "example-oracle:", err)
			return 1
		}
		var owned []struct {
			Name string `json:"name"`
			Out  string `json:"out"`
		}
		json.Unmarshal(ownedB, &owned)
		drift := 0
		for _, t := range owned {
			htmlFile, err := buildOracleGo(t.Name, "build/example-oracle")
			if err != nil {
				fmt.Fprintf(os.Stderr, "DRIFT [%s]: oracle render failed (%s)\n", t.Name, firstLine(err.Error()))
				drift++
				continue
			}
			if err := awaitOracleGo(page, htmlFile); err != nil {
				fmt.Fprintf(os.Stderr, "DRIFT [%s]: oracle render failed (%s)\n", t.Name, firstLine(err.Error()))
				drift++
				os.Remove(htmlFile)
				continue
			}
			dom, err := oracleRootHtml(page)
			if err != nil {
				fmt.Fprintf(os.Stderr, "DRIFT [%s]: %v\n", t.Name, err)
				drift++
				continue
			}
			os.Remove(htmlFile)
			current, _ := os.ReadFile(t.Out)
			body := ""
			if m := regexp.MustCompile(`(?s)<body[^>]*>\n(.*)\n</body>`).FindStringSubmatch(string(current)); m != nil {
				body = m[1]
			}
			if oracleNorm(strings.TrimSpace(body)) != strings.TrimSpace(dom) {
				fmt.Fprintf(os.Stderr, "DRIFT [%s]: %s != oracle render\n", t.Name, t.Out)
				drift++
			}
		}
		if drift > 0 {
			fmt.Fprintf(os.Stderr, "FAIL  example-oracle check (%d/%d drifted)\n", drift, len(owned))
			return 1
		}
		fmt.Printf("PASS  example-oracle check (%d pages == oracle render)\n", len(owned))
		return 0
	}

	// trivial-js components with a behavior file; selector per component
	tiersB, _ := os.ReadFile("src/registry/tiers.json")
	var tiersAll map[string]struct {
		Tier string `json:"tier"`
	}
	json.Unmarshal(tiersB, &tiersAll)
	var trivialJS []string
	rtEnts, _ := os.ReadDir("src/runtime/components")
	for _, e := range rtEnts {
		c := strings.TrimSuffix(e.Name(), ".js")
		if tiersAll[c].Tier == "trivial-js" {
			trivialJS = append(trivialJS, c)
		}
	}
	trivialSel := map[string]string{}
	for _, c := range trivialJS {
		trivialSel[c] = `[data-slot="` + c + `"]`
	}

	type fixTarget struct {
		Name     string   `json:"name"`
		Families []string `json:"families"`
		Trivial  []string `json:"trivial"`
	}
	var fixtureTargets []fixTarget
	type renderedEnt struct {
		t       oraTarget
		dom     string
		trivial []string
	}
	var rendered []renderedEnt
	var failures, exempt []string

	// Phase 1 — render everything, write nothing (all-or-nothing)
	for _, t := range targets {
		tsx := filepath.Join(oraExamplesDir, t.name+".tsx")
		if !fileExists(tsx) {
			failures = append(failures, t.name+": "+tsx+" missing")
			continue
		}
		htmlFile, err := buildOracleGo(t.name, "build/example-oracle")
		if err != nil {
			if noOracle[t.name] {
				exempt = append(exempt, t.name)
				continue
			}
			failures = append(failures, t.name+": oracle render failed ("+firstLine(err.Error())+")")
			continue
		}
		if err := awaitOracleGo(page, htmlFile); err != nil {
			os.Remove(htmlFile)
			if noOracle[t.name] {
				exempt = append(exempt, t.name)
				continue
			}
			failures = append(failures, t.name+": oracle render failed ("+firstLine(err.Error())+")")
			continue
		}
		dom, err := oracleRootHtml(page)
		if err != nil {
			os.Remove(htmlFile)
			failures = append(failures, t.name+": "+firstLine(err.Error()))
			continue
		}
		// family + trivial detection: evaluate once with both maps
		detect := `(spec) => {
      const root = document.querySelector("#root")
      const fams = [], triv = []
      for (const [f, sel] of Object.entries(spec.families))
        if (root.querySelector(sel)) fams.push(f)
      for (const [c, sel] of Object.entries(spec.trivial))
        if (root.querySelector(sel)) triv.push(c)
      return { fams, triv }
    }`
		res, err := page.evaluateFnArg(detect, map[string]any{
			"families": fixtureFamilySel,
			"trivial":  trivialSel,
		})
		os.Remove(htmlFile)
		if err != nil {
			failures = append(failures, t.name+": "+firstLine(err.Error()))
			continue
		}
		m, _ := res.(map[string]any)
		var families, trivial []string
		for _, v := range sliceOf(m["fams"]) {
			families = append(families, v.(string))
		}
		for _, v := range sliceOf(m["triv"]) {
			trivial = append(trivial, v.(string))
		}
		sort.Strings(families)
		sort.Strings(trivial)
		if len(families) > 0 && strings.HasPrefix(t.out, "docs/demos/") {
			fixtureTargets = append(fixtureTargets, fixTarget{t.name, families, trivial})
			continue
		}
		rendered = append(rendered, renderedEnt{t, strings.TrimSpace(dom), trivial})
	}
	if len(exempt) > 0 {
		fmt.Printf("example-oracle: %d examples cannot be bundled (external deps, recorded in src/registry/upstream-snapshot/exemptions.json) — pages stay hand-authored: %s\n",
			len(exempt), strings.Join(exempt, ", "))
	}
	if len(failures) > 0 {
		for _, f := range failures {
			i := strings.Index(f, ": ")
			fmt.Fprintf(os.Stderr, "FAIL [%s]: %s\n", f[:i], f[i+2:])
		}
		fmt.Fprintf(os.Stderr, "FAIL  example-oracle (%d/%d examples did not render) — nothing written; %s and %s keep their previous contents\n",
			len(failures), len(targets), oraManifest, oraFixTargets)
		return 1
	}

	// Phase 2 — commit pages and manifests together
	for _, r := range rendered {
		attr := ""
		if r.t.bodyStyle != "" {
			attr = `style="` + r.t.bodyStyle + `"`
		}
		if err := os.WriteFile(r.t.out, []byte(oraPageHtml(r.t.name, r.dom, attr, r.trivial)), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "example-oracle:", err)
			return 1
		}
	}
	var manifestB strings.Builder
	manifestB.WriteString("[")
	for i, r := range rendered {
		if i > 0 {
			manifestB.WriteString(",")
		}
		manifestB.WriteString("\n {\n  \"name\": " + jsonString(r.t.name) + ",\n  \"out\": " + jsonString(r.t.out) + "\n }")
	}
	manifestB.WriteString("\n]\n")
	os.WriteFile(oraManifest, []byte(manifestB.String()), 0o644)

	sort.Slice(fixtureTargets, func(i, j int) bool { return fixtureTargets[i].Name < fixtureTargets[j].Name })
	var ftB strings.Builder
	ftB.WriteString("[")
	for i, ft := range fixtureTargets {
		if i > 0 {
			ftB.WriteString(",")
		}
		// JSON.stringify(_, null, 1) layout: array elements each on their
		// own line, one indent deeper than the key
		arr := func(items []string) string {
			if len(items) == 0 {
				return "[]"
			}
			var parts []string
			for _, f := range items {
				parts = append(parts, jsonString(f))
			}
			return "[\n   " + strings.Join(parts, ",\n   ") + "\n  ]"
		}
		ftB.WriteString("\n {\n  \"name\": " + jsonString(ft.Name) +
			",\n  \"families\": " + arr(ft.Families) +
			",\n  \"trivial\": " + arr(ft.Trivial) + "\n }")
	}
	ftB.WriteString("\n]\n")
	os.WriteFile(oraFixTargets, []byte(ftB.String()), 0o644)

	fmt.Printf("example-oracle: %d pages carry kernel families — handed to example-fixture (%s)\n",
		len(fixtureTargets), oraFixTargets)
	fmt.Printf("example-oracle: %d pages emitted from React oracle (%d targets, %d to example-fixture, %d exempt, 0 failures)\n",
		len(rendered), len(targets), len(fixtureTargets), len(exempt))
	return 0
}


func sliceOf(v any) []any {
	if a, ok := v.([]any); ok {
		return a
	}
	return nil
}
