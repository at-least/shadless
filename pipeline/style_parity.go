package main

// style-parity, ported from tools/style-parity.mjs — COMPUTED STYLE parity
// between the React oracle and the shadless fixture, per contract component.
// Pair by data-slot sequence, compare getComputedStyle over a property list;
// deterministic, computed values rounded to 2dp.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const styleParityDir = "tools/contracts/out"

var styleParityProps = []string{
	"color", "background-color", "border-color", "border-top-width", "border-radius",
	"display", "position", "flex-direction", "align-items", "justify-content",
	"gap", "padding-top", "padding-right", "padding-bottom", "padding-left",
	"margin-top", "margin-right", "margin-bottom", "margin-left",
	"width", "height", "min-width", "max-width", "font-size", "font-weight",
	"line-height", "text-align", "opacity", "z-index", "overflow", "visibility",
	"box-shadow", "transform", "inset", "flex-wrap", "grid-template-columns",
}

// runtime-measured / animation noise both sides carry differently
var styleParitySkip = map[string]bool{
	"transform": true, "transition": true, "transition-duration": true, "animation": true,
}

// Measurements were being taken MID-ANIMATION: freeze both sides first.
const styleParityFreeze = `*, *::before, *::after {
  transition: none !important; animation: none !important;
  animation-duration: 0s !important; transition-duration: 0s !important;
}`

// Harness shell, identical on both sides (neither convention is the component).
const styleParityShell = "padding:0;margin:0;color:var(--foreground);background:var(--background)"

const styleParityCollect = `(props) => {
  const out = []
  const walk = (el) => {
    if (el.tagName === "SCRIPT" || el.tagName === "TEMPLATE") return
    if (el.hasAttribute("data-slot")) {
      const cs = getComputedStyle(el)
      const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out.push({ slot: el.getAttribute("data-slot"), tag: el.tagName, style })
    }
    for (const c of el.children) walk(c)
  }
  for (const root of document.body.children) walk(root)
  return out
}`

var reStyleCalc = regexp.MustCompile(`calc\([^)]*\)`)

func styleParityNorm(v string) string {
	if v == "" {
		return v
	}
	v = reParityNum.ReplaceAllStringFunc(v, func(n string) string {
		r := parseFloat2dp(n)
		if r == 0 {
			return "0" // Object.is(-0) guard
		}
		return jsNumberString(r)
	})
	v = reParityOklab.ReplaceAllString(v, "oklch($1 0 0)")
	return reStyleCalc.ReplaceAllString(v, "calc(…)")
}

type spCellEnt struct {
	component string
	key       string
	prop      string
	oracle    string
	shadless  string
}

func runStyleParity(strict, record bool) int {
	const baselinePath = "gates/style-parity-baseline.json"

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "style-parity:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "style-parity:", err)
		return 1
	}
	page, _ := shell.newPage(false)
	page.routeAbortExternal()

	var cells []spCellEnt
	var harnessErrors []string

	ents, _ := os.ReadDir("tools/contracts/components")
	var names []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".mjs") {
			names = append(names, strings.TrimSuffix(e.Name(), ".mjs"))
		}
	}
	sort.Strings(names)

	compared, components := 0, 0
	variants := []struct {
		id   string
		dark bool
		dir  string
	}{
		{"light@ltr", false, "ltr"}, {"dark@ltr", true, "ltr"},
		{"light@rtl", false, "rtl"}, {"dark@rtl", true, "rtl"},
	}

	collectMatrix := func() map[string][]map[string]any {
		out := map[string][]map[string]any{}
		for _, v := range variants {
			page.evaluateFnArg(`(v) => {
        document.documentElement.classList.toggle("dark", v.dark)
        document.documentElement.setAttribute("dir", v.dir)
      }`, map[string]any{"dark": v.dark, "dir": v.dir})
			page.waitForTimeout(60)
			res, err := page.evaluateFnArg(styleParityCollect, styleParityProps)
			if err != nil {
				return out
			}
			if arr, ok := res.([]any); ok {
				for _, e := range arr {
					if m, ok := e.(map[string]any); ok {
						out[v.id] = append(out[v.id], m)
					}
				}
			}
		}
		page.evaluate(`(function(){ document.documentElement.classList.remove("dark"); document.documentElement.setAttribute("dir","ltr") })()`)
		return out
	}
	freeze := func() {
		page.evaluateFnArg(`(shell) => { document.body.style.cssText += ";" + shell }`, styleParityShell)
		page.addStyleTag(styleParityFreeze)
		page.evaluate(`(function(){ document.getAnimations?.().forEach((a) => a.finish()) })()`)
		page.waitForTimeout(120)
	}

	for _, name := range names {
		dir := filepath.Join(styleParityDir, name)
		if !fileExists(filepath.Join(dir, "oracle.html")) || !fileExists(filepath.Join(dir, "shadless.html")) {
			continue
		}
		defRes, err := shell.call(map[string]any{
			"op":         "loadContractDef",
			"file":       "file://" + absOrDie(filepath.Join(cwd0(), "tools/contracts/components", name+".mjs")),
		})
		if err != nil {
			harnessErrors = append(harnessErrors, name+": harness error — "+firstLine(err.Error()))
			continue
		}
		var def struct {
			Open         string   `json:"open"`
			OpenShadless string   `json:"openShadless"`
			StyleIgnore  []string `json:"styleIgnore"`
		}
		b, _ := json.Marshal(defRes["def"])
		json.Unmarshal(b, &def)

		styleIgnore := map[string]bool{}
		for _, p := range def.StyleIgnore {
			styleIgnore[p] = true
		}
		for p := range styleParitySkip {
			styleIgnore[p] = true
		}
		var props []string
		for _, p := range styleParityProps {
			if !styleIgnore[p] {
				props = append(props, p)
			}
		}

		func() {
			defer func() {
				if r := recover(); r != nil {
					harnessErrors = append(harnessErrors, fmt.Sprintf("%s: harness error — %v", name, r))
				}
			}()
			// oracle side (styled by upstream's own oracle.css)
			absO, _ := filepath.Abs(filepath.Join(dir, "oracle.html"))
			if err := page.gotoURL("file://" + absO); err != nil {
				panic(err)
			}
			page.waitForTimeout(400)
			if def.Open != "" {
				if err := shell.callErr(map[string]any{"op": "driver", "pageId": page.id, "code": def.Open}); err != nil {
					panic(err)
				}
				page.waitForTimeout(400)
			}
			page.addStyleTagPath("build/gates/oracle.css")
			page.evaluate(`document.documentElement.classList.add("style-nova")`)
			freeze()
			oracleSides := collectMatrix()

			// shadless side (loads its own out.css via relative link)
			absS, _ := filepath.Abs(filepath.Join(dir, "shadless.html"))
			if err := page.gotoURL("file://" + absS); err != nil {
				panic(err)
			}
			page.addStyleTagPath("dist/out.css")
			page.waitForTimeout(400)
			open := def.OpenShadless
			if open == "" {
				open = def.Open
			}
			if open != "" {
				if err := shell.callErr(map[string]any{"op": "driver", "pageId": page.id, "code": open}); err != nil {
					panic(err)
				}
				page.waitForTimeout(400)
			}
			freeze()
			shadlessSides := collectMatrix()

			slotOf := func(e map[string]any) string { s, _ := e["slot"].(string); return s }
			for _, variant := range variants {
				seenA := map[string]int{}
				seenB := map[string]int{}
				keyOf := func(slot string, seen map[string]int) string {
					seen[slot]++
					return fmt.Sprintf("%s#%d", slot, seen[slot])
				}
				mapA := map[string]map[string]string{}
				var orderA []string
				for _, e := range oracleSides[variant.id] {
					k := keyOf(slotOf(e), seenA)
					style, _ := e["style"].(map[string]any)
					sm := map[string]string{}
					for p, v := range style {
						sm[p], _ = v.(string)
					}
					mapA[k] = sm
					orderA = append(orderA, k)
				}
				mapB := map[string]map[string]string{}
				var orderB []string
				for _, e := range shadlessSides[variant.id] {
					k := keyOf(slotOf(e), seenB)
					style, _ := e["style"].(map[string]any)
					sm := map[string]string{}
					for p, v := range style {
						sm[p], _ = v.(string)
					}
					mapB[k] = sm
					orderB = append(orderB, k)
				}
				push := func(k, prop, a, b string) {
					key := k
					if variant.id != "light@ltr" {
						key = k + "@" + variant.id
					}
					cells = append(cells, spCellEnt{name, key, prop, a, b})
				}
				for _, k := range orderA {
					a := mapA[k]
					b, ok := mapB[k]
					if !ok {
						push(k, "<presence>", "present", "missing")
						continue
					}
					for _, p := range props {
						va := styleParityNorm(a[p])
						vb := styleParityNorm(b[p])
						if va != vb {
							push(k, p, va, vb)
						}
					}
				}
				for _, k := range orderB {
					if _, ok := mapA[k]; !ok {
						push(k, "<presence>", "missing", "present")
					}
				}
				compared += len(mapA)
			}
			components++
		}()
	}

	if len(harnessErrors) > 0 {
		fmt.Fprintf(os.Stderr, "FAIL  style-parity (harness)\n  %s\n", strings.Join(harnessErrors, "\n  "))
		return 1
	}

	// `flaky` from the raw file (record must work pre-value format too)
	flaky := map[string]bool{}
	if fb, err := os.ReadFile(baselinePath); err == nil {
		var raw struct {
			Flaky []string `json:"flaky"`
		}
		json.Unmarshal(fb, &raw)
		for _, f := range raw.Flaky {
			flaky[f] = true
		}
	}

	var ratcheted []spCellEnt
	for _, c := range cells {
		if !flaky[c.component] {
			ratcheted = append(ratcheted, c)
		}
	}
	flakyCells := len(cells) - len(ratcheted)
	var actualCells []parityCell
	for _, c := range ratcheted {
		actualCells = append(actualCells, parityCell{c.component + "/" + c.key + "/" + c.prop, c.oracle, c.shadless})
	}
	actual, order := cellMap(actualCells)

	if record || !fileExists(baselinePath) {
		var fl []string
		for f := range flaky {
			fl = append(fl, f)
		}
		if err := writeParityBaseline(baselinePath,
			"Cells where the shadless fixture's computed style differs from the React oracle, "+
				"with the two values as recorded. This list may only shrink and the values are pinned; "+
				"see the ledger budget style-parity.dirty-cells.",
			fl, actual); err != nil {
			fmt.Fprintln(os.Stderr, "style-parity:", err)
			return 1
		}
		compSet := map[string]bool{}
		for _, c := range ratcheted {
			compSet[c.component] = true
		}
		fmt.Printf("style-parity: baseline recorded (%d cells across %d components, %d flaky components excluded)\n",
			len(actual), len(compSet), len(flaky))
		return 0
	}

	if strict && len(cells) > 0 {
		n := 10
		if len(cells) < n {
			n = len(cells)
		}
		var parts []string
		for _, c := range cells[:n] {
			parts = append(parts, c.component+"/"+c.key+"/"+c.prop+": oracle="+trunc60(c.oracle)+" shadless="+trunc60(c.shadless))
		}
		fmt.Fprintf(os.Stderr, "FAIL  style-parity --strict (%d differing cells)\n  %s\n", len(cells), strings.Join(parts, "\n  "))
		return 1
	}

	_, recorded, err := loadParityBaseline(baselinePath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "style-parity:", err)
		return 1
	}
	d := diffParityBaseline(recorded, actual, order)
	if len(d.appeared) > 0 {
		n := 40
		if len(d.appeared) < n {
			n = len(d.appeared)
		}
		var parts []string
		for _, id := range d.appeared[:n] {
			parts = append(parts, id+": "+showCell(actual[id]))
		}
		tail := ""
		if len(d.appeared) > 40 {
			tail = fmt.Sprintf("\n  … +%d more", len(d.appeared)-40)
		}
		fmt.Fprintf(os.Stderr, "FAIL  style-parity (%d NEW differing cells vs the React oracle)\n  %s%s\n",
			len(d.appeared), strings.Join(parts, "\n  "), tail)
		return 1
	}
	if len(d.changed) > 0 {
		n := 20
		if len(d.changed) < n {
			n = len(d.changed)
		}
		var parts []string
		for _, c := range d.changed[:n] {
			parts = append(parts, showChange(c))
		}
		tail := ""
		if len(d.changed) > 20 {
			tail = fmt.Sprintf("\n  … +%d more", len(d.changed)-20)
		}
		fmt.Fprintf(os.Stderr, "FAIL  style-parity (%d recorded cells still differ from the oracle, but by a DIFFERENT amount than what was recorded — look at them again, then re-record)\n  %s%s\n\n  ./build/pipeline style-parity --record\n",
			len(d.changed), strings.Join(parts, "\n  "), tail)
		return 1
	}
	if len(d.fixed) > 0 {
		n := 12
		if len(d.fixed) < n {
			n = len(d.fixed)
		}
		tail := ""
		if len(d.fixed) > 12 {
			tail = fmt.Sprintf("\n  … +%d more", len(d.fixed)-12)
		}
		fmt.Fprintf(os.Stderr, "FAIL  style-parity (%d recorded cells no longer differ — record the win so the slack cannot be re-spent)\n  %s%s\n\n  ./build/pipeline style-parity --record && ./build/pipeline ledger --record\n",
			len(d.fixed), strings.Join(d.fixed[:n], "\n  "), tail)
		return 1
	}
	fmt.Printf("PASS  style-parity (%d components, %d elements compared, %d cells at the recorded baseline incl. their values, %d cells in %d flaky components excluded; --strict is the end state)\n",
		components, compared, len(actual), flakyCells, len(flaky))
	return 0
}

func cwd0() string {
	wd, _ := os.Getwd()
	return wd
}

func absOrDie(p string) string {
	abs, err := filepath.Abs(p)
	if err != nil {
		return p
	}
	return abs
}
