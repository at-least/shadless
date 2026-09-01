package main

// demo-parity, ported from gates/demo-parity.mjs — every shipped demo page,
// styled by our CSS, computes what the SAME DOM computes under upstream's
// own stylesheet. Same DOM on both sides, so every difference is the
// emitted CSS. Cells ratcheted in gates/demo-parity-baseline.json.

import (
	_ "embed"
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"strings"
)

const demoParityBaseline = "gates/demo-parity-baseline.json"

var demoParityProps = []string{
	"color", "background-color", "border-color", "border-top-width", "border-radius", "padding-top",
	"padding-right", "padding-bottom", "padding-left", "margin-top", "margin-left", "width", "height",
	"min-width", "max-width", "font-size", "font-weight", "line-height", "display", "flex-direction",
	"align-items", "justify-content", "gap", "position", "opacity", "box-shadow", "text-align", "overflow",
}

// harness shell pinned on both sides: neither body convention is the component
const demoParityFreeze = "*,*::before,*::after{transition:none!important;animation:none!important} body{padding:0!important;margin:0;color:var(--foreground);background:var(--background)}"

// collect runs in the page: all four theme×dir cells for every [data-slot].
//go:embed demo_parity_collect.js
var demoParityCollect string



var reParityNum = regexp.MustCompile(`-?\d*\.?\d+(?:e[-+]?\d+)?`)
var reParityOklab = regexp.MustCompile(`oklab\((-?[\d.]+) 0 0\)`)

// demoParityNorm rounds every number to 2dp and canonicalises axis-only
// oklab() to oklch() (Chrome serialises the same colour both ways).
func demoParityNorm(v string) string {
	v = reParityNum.ReplaceAllStringFunc(v, func(n string) string {
		r := parseFloat2dp(n)
		return jsNumberString(r)
	})
	return reParityOklab.ReplaceAllString(v, "oklch($1 0 0)")
}

func runDemoParity(record, details bool) int {
	var owned []struct {
		Name string `json:"name"`
		Out  string `json:"out"`
	}
	ob, err := os.ReadFile("docs/example-oracle.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo-parity:", err)
		return 1
	}
	json.Unmarshal(ob, &owned)
	oracleCss, _ := os.ReadFile("build/gates/oracle.css")
	outCss, _ := os.ReadFile("dist/out.css")

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo-parity:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "demo-parity:", err)
		return 1
	}
	page, _ := shell.newPage(false)
	page.routeAbortExternal()

	var cells []parityCell
	pages, compared := 0, 0
	reBody := regexp.MustCompile(`(?s)<body([^>]*)>(.*)</body>`)
	reScript := regexp.MustCompile(`(?s)<script.*?</script>`)
	for _, t := range owned {
		if !strings.HasPrefix(t.Out, "docs/demos/") {
			continue
		}
		html, err := os.ReadFile(t.Out)
		if err != nil {
			continue
		}
		m := reBody.FindStringSubmatch(string(html))
		if m == nil {
			continue
		}
		bare := reScript.ReplaceAllString(m[2], "")
		doc := func(css, root string) string {
			return "<!doctype html><html class=\"" + root + "\"><head><style>" + css + "</style><style>" + demoParityFreeze + "</style></head><body" + m[1] + ">" + bare + "</body></html>"
		}
		collectOnce := func() map[string]map[string]string {
			v, err := page.evaluateFnArg(demoParityCollect, demoParityProps)
			if err != nil {
				return nil
			}
			out := map[string]map[string]string{}
			if obj, ok := v.(map[string]any); ok {
				for k, sv := range obj {
					if sm, ok := sv.(map[string]any); ok {
						cells := map[string]string{}
						for p, val := range sm {
							cells[p], _ = val.(string)
						}
						out[k] = cells
					}
				}
			}
			return out
		}
		page.evaluateFnArg(`(html) => { document.open(); document.write(html); document.close(); return true }`, doc(string(outCss), ""))
		page.waitForTimeout(30)
		ours := collectOnce()
		page.evaluateFnArg(`(html) => { document.open(); document.write(html); document.close(); return true }`, doc(string(oracleCss), "style-nova"))
		page.waitForTimeout(30)
		theirs := collectOnce()
		pages++
		for k, ref := range theirs {
			got, ok := ours[k]
			if !ok {
				continue
			}
			compared++
			parts := strings.Split(k, "@")
			slotKey, theme, dir := parts[0], parts[1], parts[2]
			for _, p := range demoParityProps {
				a := demoParityNorm(ref[p])
				b := demoParityNorm(got[p])
				if a != b {
					cells = append(cells, parityCell{t.Name + "/" + slotKey + "/" + p + "@" + theme + "@" + dir, a, b})
				}
			}
		}
	}

	actual, order := cellMap(cells)
	if details {
		for _, id := range order {
			if strings.HasSuffix(id, "@light@ltr") {
				fmt.Printf("%s: %s\n", id, showCell(actual[id]))
			}
		}
	}
	if _, err := os.Stat(demoParityBaseline); record || err != nil {
		if err := writeParityBaseline(demoParityBaseline,
			"shipped demo DOM under our css vs the same DOM under upstream css; may only shrink, and a recorded cell's VALUES are pinned too",
			nil, actual); err != nil {
			fmt.Fprintln(os.Stderr, "demo-parity:", err)
			return 1
		}
		fmt.Printf("demo-parity: baseline recorded (%d cells over %d pages, %d element×theme×dir comparisons)\n",
			len(actual), pages, compared)
		return 0
	}
	_, recorded, err := loadParityBaseline(demoParityBaseline)
	if err != nil {
		fmt.Fprintln(os.Stderr, "demo-parity:", err)
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
		fmt.Fprintf(os.Stderr, "FAIL  demo-parity (%d NEW cells where a shipped demo ≠ upstream css)\n  %s\n",
			len(d.appeared), strings.Join(parts, "\n  "))
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
		fmt.Fprintf(os.Stderr, "FAIL  demo-parity (%d recorded cells still differ, but by a DIFFERENT amount — re-look, then re-record: ./build/pipeline demo-parity --record)\n  %s\n",
			len(d.changed), strings.Join(parts, "\n  "))
		return 1
	}
	if len(d.fixed) > 0 {
		n := 20
		if len(d.fixed) < n {
			n = len(d.fixed)
		}
		fmt.Fprintf(os.Stderr, "FAIL  demo-parity (%d recorded cells no longer differ — record the win: ./build/pipeline demo-parity --record && ./build/pipeline ledger --record)\n  %s\n",
			len(d.fixed), strings.Join(d.fixed[:n], "\n  "))
		return 1
	}
	fmt.Printf("PASS  demo-parity (%d pages, %d comparisons, %d cells at the recorded baseline incl. their values; --strict is the end state)\n",
		pages, compared, len(actual))
	return 0
}

func parseFloat2dp(s string) float64 {
	var f float64
	fmt.Sscanf(s, "%g", &f)
	r := f*100 + 0.5*sign(f)
	// JS: Math.round(x*100)/100 || 0
	r = float64(int64(f*100+0.5*sign(f))) / 100
	if r == 0 {
		return 0
	}
	return r
}

func sign(f float64) float64 {
	if f < 0 {
		return -1
	}
	return 1
}
