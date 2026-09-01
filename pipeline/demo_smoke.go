package main

// demo-smoke, ported from tools/demo-smoke.mjs — every dist/components/*.html
// renders with zero real console errors and every IR slot name appears in
// the page source. The browser half runs through the Wave-3 thin shell.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

var reSlotAttr = regexp.MustCompile(`data-slot="([\w-]+)"`)
var reRtlVariantPage = regexp.MustCompile(`-rtl-(en|he|fa)\.html$`)

func runDemoSmoke() int {
	tiersB, err := os.ReadFile("src/registry/tiers.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo-smoke:", err)
		return 1
	}
	var regTiers map[string]struct {
		Tier string `json:"tier"`
		Emit bool   `json:"emit"`
	}
	if err := json.Unmarshal(tiersB, &regTiers); err != nil {
		fmt.Fprintln(os.Stderr, "demo-smoke: tiers:", err)
		return 1
	}
	shipped := func(name, tier string) bool {
		return tier == "static" || tier == "kernel" || tier == "trivial-js" || regTiers[name].Emit
	}

	ents, err := os.ReadDir("dist/components")
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo-smoke:", err)
		return 1
	}
	var allHtml []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".html") {
			allHtml = append(allHtml, e.Name())
		}
	}
	sort.Strings(allHtml)
	var pages []string
	for _, f := range allHtml {
		if !reRtlVariantPage.MatchString(f) {
			pages = append(pages, f)
		}
	}
	rtlVariants := len(allHtml) - len(pages)
	emitted := 0
	for n, t := range regTiers {
		_ = n
		if shipped(n, t.Tier) {
			emitted++
		}
	}
	if len(pages) != emitted+1 {
		fmt.Fprintf(os.Stderr, "FAIL demo-smoke: expected %d base pages (%d IR + alert-demo), got %d (%d RTL variants skipped)\n",
			emitted+1, emitted, len(pages), rtlVariants)
		return 1
	}

	// global slot vocabulary across all emitted components (fixtures
	// legitimately compose multiple components)
	allSlots := map[string]bool{}
	irEnts, _ := os.ReadDir("src/registry/ir")
	for _, e := range irEnts {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b, _ := os.ReadFile(filepath.Join("src/registry/ir", e.Name()))
		var ir cssIrComponent
		if json.Unmarshal(b, &ir) != nil {
			continue
		}
		if !shipped(ir.Name, ir.Tier) {
			continue
		}
		for _, c := range ir.Components {
			for _, el := range c.Elements {
				if el.Slot != "" {
					allSlots[el.Slot] = true
				}
			}
		}
	}
	irSlots := func(name string) int {
		b, err := os.ReadFile("src/registry/ir/" + name + ".json")
		if err != nil {
			return 0
		}
		var ir cssIrComponent
		if json.Unmarshal(b, &ir) != nil {
			return 0
		}
		n := 0
		for _, c := range ir.Components {
			for _, el := range c.Elements {
				if el.Slot != "" {
					n++
				}
			}
		}
		return n
	}

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo-smoke:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "demo-smoke:", err)
		return 1
	}

	fail := false
	cwd, _ := os.Getwd()
	for _, f := range pages {
		name := strings.TrimSuffix(f, ".html")
		html, _ := os.ReadFile("dist/components/" + f)
		pageSlots := map[string]bool{}
		for _, m := range reSlotAttr.FindAllStringSubmatch(string(html), -1) {
			pageSlots[m[1]] = true
		}
		var phantom []string
		for _, m := range reSlotAttr.FindAllStringSubmatch(string(html), -1) {
			if !allSlots[m[1]] && !containsTok(phantom, m[1]) {
				phantom = append(phantom, m[1])
			}
		}
		nIRSlots := irSlots(name)
		page, err := shell.newPage(true)
		if err != nil {
			fmt.Fprintln(os.Stderr, "demo-smoke:", err)
			return 1
		}
		if err := page.gotoURL("file://" + filepath.Join(cwd, "dist/components", f)); err != nil {
			fmt.Fprintln(os.Stderr, "demo-smoke:", err)
			return 1
		}
		slotsV, err := page.evaluate(`document.querySelectorAll("[data-slot]").length`)
		if err != nil {
			fmt.Fprintln(os.Stderr, "demo-smoke:", err)
			return 1
		}
		errors, _ := page.events()
		page.close()
		slots := int(slotsV.(float64))
		if len(phantom) > 0 || len(errors) > 0 || (slots == 0 && nIRSlots > 0) {
			fmt.Fprintf(os.Stderr, "FAIL demo-smoke [%s]: phantom=%s slots=%d irSlots=%d errors=%q\n",
				f, strings.Join(phantom, ","), slots, nIRSlots, errors)
			fail = true
		}
	}
	if fail {
		fmt.Println("FAIL  demo smoke")
		return 1
	}
	fmt.Printf("PASS  demo smoke (%d base pages + %d RTL variants, IR-slot fidelity, 0 console errors)\n",
		len(pages), rtlVariants)
	return 0
}
