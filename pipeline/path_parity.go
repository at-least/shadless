package main

// path-parity, ported from gates/path-parity.mjs — every consume path
// (css-import / full-css) computes what React computes for every slotted
// element, in both themes and both directions. The non-browser half
// (cvaSlot, splitMarkers, normalizeTag, twMerge, cva-table composition) is
// the emitter's own code — already Go.

import (
	_ "embed"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"regexp"
	"sort"
	"strings"

	"shadless/pipeline/internal/twmerge"
)

const (
	ppSim      = "build/gates/path-parity"
	ppBaseline = "gates/path-parity-baseline.json"
)

var ppProps = []string{
	"color", "background-color", "border-color", "border-top-width", "border-bottom-width",
	"border-left-width", "border-right-width", "border-radius", "padding-top", "padding-right",
	"padding-bottom", "padding-left", "margin-top", "margin-left", "margin-right", "width",
	"min-width", "max-width", "height", "min-height", "row-gap", "column-gap", "font-size",
	"font-weight", "line-height", "letter-spacing", "text-align", "display", "flex-direction",
	"align-items", "justify-content", "position", "top", "left", "right", "opacity", "box-shadow",
	"outline-width", "overflow", "white-space", "text-decoration-line", "transform", "translate", "scale", "visibility",
}

var ppVoid = map[string]bool{"input": true, "img": true, "br": true, "hr": true}

// upstream @custom-variant bodies
var ppShorthand = map[string][2]string{
	"data-open": {"data-state", "open"}, "data-closed": {"data-state", "closed"},
	"data-checked": {"data-state", "checked"}, "data-unchecked": {"data-state", "unchecked"},
	"data-active": {"data-state", "active"}, "data-selected": {"data-selected", "true"},
	"data-disabled": {"data-disabled", "true"}, "data-horizontal": {"data-orientation", "horizontal"},
	"data-vertical": {"data-orientation", "vertical"},
}

var ppCvaAxes = map[string]bool{"variant": true, "size": true}

var (
	rePPChildSlot = regexp.MustCompile(`data-\[slot=([-\w]+)\]`)
	rePPCombinator = regexp.MustCompile(`^(group|peer|has|in)-`)
	rePPAttr      = regexp.MustCompile(`^(data|aria)-\[([\w-]+)(?:=([\w-]+))?\]$`)
	rePPAria      = regexp.MustCompile(`^aria-(expanded|invalid|checked|disabled|pressed|selected|current)$`)
	rePPDataBare  = regexp.MustCompile(`^data-(inset|highlighted|empty|pressed|autoscrolling|popup-open)$`)
)

func ppChildSlots(cls string) []string {
	seen := map[string]bool{}
	var out []string
	for _, m := range rePPChildSlot.FindAllStringSubmatch(cls, -1) {
		if !seen[m[1]] {
			seen[m[1]] = true
			out = append(out, m[1])
		}
	}
	return out
}

// ppInlineClasses: React evaluates a conditional; keep the branch the
// default selects, then twMerge the surviving list.
func ppInlineClasses(ir cssIrComponent, fn string, el irEl) string {
	drop := map[string]bool{}
	for _, c := range ir.Conditionals {
		if c.Kind != "class-cond" || c.Fn != fn || c.Test == nil {
			continue
		}
		if !containsTok(el.Classes, c.Then) || !containsTok(el.Classes, c.Else) {
			continue
		}
		var truthy bool
		switch d := c.Test.Default.(type) {
		case nil:
			truthy = true
		case string:
			truthy = (d == c.Test.Value) == (c.Test.Op == "===")
		default:
			truthy = true
		}
		if truthy {
			drop[c.Else] = true
		} else {
			drop[c.Then] = true
		}
	}
	var keep []string
	for _, c := range el.Classes {
		if !drop[c] {
			keep = append(keep, c)
		}
	}
	return twmerge.Merge(strings.Join(keep, " "))
}

func ppStateConfigs(cls string) [][2]string {
	out := map[string][2]string{}
	var order []string
	for _, tok := range strings.Fields(cls) {
		segs := strings.Split(tok, ":")
		segs = segs[:len(segs)-1]
		skip := false
		for _, v := range segs {
			if v == "*" || v == "**" || rePPCombinator.MatchString(v) {
				skip = true
				break
			}
		}
		if skip {
			continue
		}
		for _, v := range segs {
			bare := regexp.MustCompile(`^not-.*$`).ReplaceAllString(v, "")
			if bare == "" || strings.HasPrefix(bare, "data-[slot=") {
				continue
			}
			if sh, ok := ppShorthand[bare]; ok {
				k := sh[0] + "=" + sh[1]
				if _, seen := out[k]; !seen {
					out[k] = sh
					order = append(order, k)
				}
				continue
			}
			if m := rePPAttr.FindStringSubmatch(bare); m != nil {
				if ppCvaAxes[m[2]] {
					continue
				}
				val := m[3]
				if val == "" {
					val = "true"
				}
				k := m[1] + "-" + m[2] + "=" + val
				if _, seen := out[k]; !seen {
					out[k] = [2]string{m[1] + "-" + m[2], val}
					order = append(order, k)
				}
				continue
			}
			if m := rePPAria.FindStringSubmatch(bare); m != nil {
				k := "aria-" + m[1] + "=true"
				if _, seen := out[k]; !seen {
					out[k] = [2]string{"aria-" + m[1], "true"}
					order = append(order, k)
				}
				continue
			}
			if m := rePPDataBare.FindStringSubmatch(bare); m != nil {
				k := "data-" + m[1] + "="
				if _, seen := out[k]; !seen {
					out[k] = [2]string{"data-" + m[1], ""}
					order = append(order, k)
				}
			}
		}
	}
	var res [][2]string
	for _, k := range order {
		res = append(res, out[k])
	}
	return res
}

//go:embed pp_readall.js
var ppReadAll string



func ppNorm(v string) string {
	v = reParityNum.ReplaceAllStringFunc(v, func(n string) string {
		return jsNumberString(parseFloat2dp(n))
	})
	return reParityOklab.ReplaceAllString(v, "oklch($1 0 0)")
}

type ppItem struct {
	id       int
	label    string
	kids     []struct{ id, label string }
	slotHtml  string
	inlineHtml string
	state    bool
	variant  bool
}

func runPathParity(record, details bool) int {
	root, _ := os.Getwd()
	os.RemoveAll(ppSim)
	os.MkdirAll(ppSim+"/node_modules", 0o755)
	if err := os.Symlink(root, ppSim+"/node_modules/shadless"); err != nil && !os.IsExist(err) {
		fmt.Fprintln(os.Stderr, "path-parity:", err)
		return 1
	}
	oracleCss, _ := os.ReadFile("build/gates/oracle.css")
	fullCss, _ := os.ReadFile("dist/shadless.full.css")

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "path-parity:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "path-parity:", err)
		return 1
	}

	var cells []parityCell
	compared, components, stateRenders, variantRenders := 0, 0, 0, 0
	ppShell := "body{margin:0;padding:0;color:var(--foreground);background:var(--background)} *{transition:none!important;animation:none!important}"
	doc := func(css, body, rootClass string) string {
		return "<!doctype html><html class=\"" + rootClass + "\"><head><style>" + css + "</style><style>" + ppShell + "</style></head><body>" + body + "</body></html>"
	}
	readAll := func(p *bpage, ids []string) map[string]map[string]string {
		v, err := p.evaluateFnArg(ppReadAll, map[string]any{"ids": ids, "props": ppProps})
		if err != nil {
			return nil
		}
		out := map[string]map[string]string{}
		if obj, ok := v.(map[string]any); ok {
			for k, sv := range obj {
				if sm, ok := sv.(map[string]any); ok {
					cellsM := map[string]string{}
					for p, val := range sm {
						cellsM[p], _ = val.(string)
					}
					out[k] = cellsM
				}
			}
		}
		return out
	}

	ents, _ := os.ReadDir("src/registry/ir")
	var files []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".json") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)

	for _, f := range files {
		name := strings.TrimSuffix(f, ".json")
		if !fileExists("dist/css/" + name + ".css") {
			continue
		}
		irb, _ := os.ReadFile("src/registry/ir/" + f)
		var ir cssIrComponent
		if json.Unmarshal(irb, &ir) != nil {
			continue
		}
		loadSkin()
		cva := cvaSlot(ir)
		cvaSlots := map[string]cvaTable{}
		for _, t := range cva {
			if t.slot != "" {
				cvaSlots[t.slot] = t.table
			}
		}
		var items []ppItem
		n := 0
		seen := map[string]bool{}
		elOfSlot := func(slot string) (*irFn, *irEl) {
			for ci := range ir.Components {
				for ei := range ir.Components[ci].Elements {
					if ir.Components[ci].Elements[ei].Slot == slot {
						return &ir.Components[ci], &ir.Components[ci].Elements[ei]
					}
				}
			}
			return nil, nil
		}
		tagOf := func(el irEl) string {
			t, ok := normalizeTag(el.Tag, ir.TagHints)
			if !ok {
				return "div"
			}
			return t
		}
		classesOf := func(c *irFn, el irEl, sel map[string]string) string {
			table, isCva := cvaSlots[el.Slot]
			if !isCva {
				var ctxDefaults []string
				for _, r := range ir.CvaRefs {
					if r.Slot != el.Slot {
						continue
					}
					for _, ax := range r.DynAxes {
						d, has := r.Defaults[ax]
						if !has {
							continue
						}
						key := sel[ax]
						if key == "" {
							key = d
						}
						if v := r.Table.variants[ax][key]; v != "" {
							ctxDefaults = append(ctxDefaults, v)
						}
					}
				}
				base := ppInlineClasses(ir, c.Fn, el)
				if len(ctxDefaults) > 0 {
					return twmerge.Merge(strings.Join(append([]string{base}, ctxDefaults...), " "))
				}
				return base
			}
			values := map[string]bool{}
			for _, axis := range table.axisOrder {
				for _, v := range table.valueOrder[axis] {
					values[table.variants[axis][v]] = true
				}
			}
			var extras []string
			for _, x := range el.Classes {
				if x != table.Base && !values[x] {
					extras = append(extras, x)
				}
			}
			list := []string{table.Base}
			for _, axis := range table.axisOrder {
				key := sel[axis]
				if key == "" {
					key = table.defaults[axis]
				}
				if v := table.variants[axis][key]; v != "" {
					list = append(list, v)
				}
			}
			list = append(list, extras...)
			return twmerge.Merge(strings.Join(list, " "))
		}
		wrapH := func(h string) string {
			return `<div style="position:relative;width:480px;height:160px;margin:8px 0">` + h + `</div>`
		}
		openTag := func(tag, attrs, inner string) string {
			if ppVoid[tag] {
				return "<" + tag + " " + attrs + ">"
			}
			return "<" + tag + " " + attrs + ">" + inner + "</" + tag + ">"
		}
		build := func(c *irFn, el irEl, sel map[string]string, attrs, label string, extraState, extraVariant bool) string {
			tag := tagOf(el)
			cls := classesOf(c, el, sel)
			markers := splitMarkers(cls).markers
			var kids []struct{ id, label string }
			slotKids, inlineKids := "", ""
			for _, k := range ppChildSlots(cls) {
				kc, ke := elOfSlot(k)
				if ke == nil {
					continue
				}
				ktag := tagOf(*ke)
				if regexp.MustCompile(`^[A-Z]`).MatchString(ktag) || ktag == "?" {
					continue
				}
				kcls := classesOf(kc, *ke, nil)
				km := strings.Join(splitMarkers(kcls).markers, " ")
				inert := ""
				for _, g := range ppChildSlots(kcls) {
					inert += `<div data-slot="` + g + `" style="display:none">x</div>`
				}
				kinner := ""
				if !ppVoid[ktag] && ktag != "svg" {
					kinner = "x" + inert
				}
				kid := len(kids)
				kids = append(kids, struct{ id, label string }{fmt.Sprintf("%d-%d", n, kid), label + ">" + k})
				slotKids += openTag(ktag, `data-slot="`+k+`" id="so-`+fmt.Sprint(n)+`-`+fmt.Sprint(kid)+`" class="`+km+`"`, kinner)
				inlineKids += openTag(ktag, `data-slot="`+k+`" id="in-`+fmt.Sprint(n)+`-`+fmt.Sprint(kid)+`" class="`+kcls+`"`, kinner)
			}
			inner := func(t, k string) string {
				if ppVoid[t] || t == "svg" {
					return ""
				}
				return "x" + k
			}
			idAttrs := fmt.Sprintf(`data-slot="%s" %s id="so-%d" class="%s"`, el.Slot, attrs, n, strings.Join(markers, " "))
			idAttrsIn := fmt.Sprintf(`data-slot="%s" %s id="in-%d" class="%s"`, el.Slot, attrs, n, cls)
			items = append(items, ppItem{
				id: n, label: label, kids: kids,
				slotHtml:   wrapH(openTag(tag, idAttrs, inner(tag, slotKids))),
				inlineHtml: wrapH(openTag(tag, idAttrsIn, inner(tag, inlineKids))),
				state:      extraState, variant: extraVariant,
			})
			n++
			return cls
		}
		for _, c := range ir.Components {
			for idx, el := range c.Elements {
				if el.Slot == "" || seen[el.Slot] {
					continue
				}
				tag := tagOf(el)
				if regexp.MustCompile(`^[A-Z]`).MatchString(tag) || tag == "?" {
					continue
				}
				seen[el.Slot] = true
				cc, elc := &ir.Components[func() int { for i := range ir.Components { if ir.Components[i].Fn == c.Fn { return i } }; return 0 }()], el
				cls := classesOf(cc, elc, nil)
				if strings.TrimSpace(cls) == "" {
					continue
				}
				build(cc, elc, map[string]string{}, "", fmt.Sprintf("%s#%d", el.Slot, idx), false, false)
				if table, ok := cvaSlots[el.Slot]; ok {
					for _, axis := range table.axisOrder {
						for _, v := range table.valueOrder[axis] {
							sel := map[string]string{axis: v}
							build(cc, elc, sel, fmt.Sprintf(`data-%s="%s"`, axis, v), fmt.Sprintf("%s#%d[%s=%s]", el.Slot, idx, axis, v), false, true)
						}
					}
				}
				for _, r := range ir.CvaRefs {
					if r.Slot != el.Slot {
						continue
					}
					for _, axis := range r.DynAxes {
						if _, has := r.Defaults[axis]; !has {
							continue
						}
						for _, v := range r.Table.valueOrder[axis] {
							clsV := r.Table.variants[axis][v]
							if clsV == "" {
								continue
							}
							inline := classesOf(cc, elc, map[string]string{axis: v})
							markers := splitMarkers(inline).markers
							tag := tagOf(el)
							items = append(items, ppItem{
								id: n, label: fmt.Sprintf("%s#%d[%s=%s]", el.Slot, idx, axis, v), variant: true,
								slotHtml:   wrapH(openTag(tag, fmt.Sprintf(`data-slot="%s" data-%s="%s" id="so-%d" class="%s"`, el.Slot, axis, v, n, strings.Join(markers, " ")), "x")),
								inlineHtml: wrapH(openTag(tag, fmt.Sprintf(`data-slot="%s" data-%s="%s" id="in-%d" class="%s"`, el.Slot, axis, v, n, inline), "x")),
							})
							n++
						}
					}
				}
				for _, av := range ppStateConfigs(cls) {
					build(cc, elc, map[string]string{}, fmt.Sprintf(`%s="%s"`, av[0], av[1]), fmt.Sprintf("%s#%d[%s=%s]", el.Slot, idx, av[0], av[1]), true, false)
				}
			}
		}
		if len(items) == 0 {
			continue
		}
		components++
		for _, it := range items {
			if it.state {
				stateRenders++
			}
			if it.variant {
				variantRenders++
			}
		}

		// (a) consumer build: core + this component's css
		os.WriteFile(ppSim+"/entry.css", []byte("@import \"shadless\";\n@import \"shadless/"+name+".css\";\n"), 0o644)
		tw := exec.Command("./build/pipeline", "tw", ppSim+"/entry.css", ppSim+"/out.css", "--cwd", ppSim)
		if out, err := tw.CombinedOutput(); err != nil {
			fmt.Fprintf(os.Stderr, "FAIL  path-parity: shadless/%s.css does not compile alone\n%s\n", name, out)
			return 1
		}
		consumerCss, _ := os.ReadFile(ppSim + "/out.css")
		var slotBody, inlineBody []string
		var ids, inIds []string
		for _, it := range items {
			slotBody = append(slotBody, it.slotHtml)
			inlineBody = append(inlineBody, it.inlineHtml)
			ids = append(ids, fmt.Sprintf("so-%d", it.id))
			for _, k := range it.kids {
				ids = append(ids, "so-"+k.id)
			}
			inIds = append(inIds, fmt.Sprintf("in-%d", it.id))
			for _, k := range it.kids {
				inIds = append(inIds, "in-"+k.id)
			}
		}
		if os.Getenv("PP_KEEP") != "" {
			os.WriteFile(ppSim+"/"+name+".html",
				[]byte(doc(string(consumerCss), strings.Join(slotBody, "\n"), "")+"\n<!-- ORACLE -->\n"+doc("", strings.Join(inlineBody, "\n"), "style-nova")), 0o644)
		}
		pA, _ := shell.newPage(false)
		pA.setContent(doc(string(consumerCss), strings.Join(slotBody, "\n"), ""))
		pA.waitForTimeout(50)
		pB, _ := shell.newPage(false)
		pB.setContent(doc(string(fullCss), strings.Join(slotBody, "\n"), ""))
		pB.waitForTimeout(50)
		pO, _ := shell.newPage(false)
		pO.setContent(doc(string(oracleCss), strings.Join(inlineBody, "\n"), "style-nova"))
		pO.waitForTimeout(50)
		a := readAll(pA, ids)
		b := readAll(pB, ids)
		o := readAll(pO, inIds)
		pA.close()
		pB.close()
		pO.close()
		for _, it := range items {
			nodes := []struct{ id int; label string }{{it.id, it.label}}
			for _, k := range it.kids {
				nodes = append(nodes, struct{ id int; label string }{-1, k.label})
				_ = k
			}
			_ = nodes
			themes := [][2]string{{"light", "ltr"}, {"dark", "ltr"}, {"light", "rtl"}, {"dark", "rtl"}}
			type nodeEnt struct{ idStr, label string }
			var nodeEnts []nodeEnt
			nodeEnts = append(nodeEnts, nodeEnt{fmt.Sprintf("so-%d", it.id), it.label})
			for _, k := range it.kids {
				nodeEnts = append(nodeEnts, nodeEnt{"so-" + k.id, k.label})
			}
			for _, theme := range themes {
				for _, nd := range nodeEnts {
					ref := o["in-"+nd.idStr[strings.Index(nd.idStr, "-")+1:]+"@"+theme[0]+"@"+theme[1]]
					refKey := strings.Replace(nd.idStr, "so-", "in-", 1)
					ref = o[refKey+"@"+theme[0]+"@"+theme[1]]
					for _, path := range []string{"css-import", "full-css"} {
						var side map[string]map[string]string
						if path == "css-import" {
							side = a
						} else {
							side = b
						}
						got := side[nd.idStr+"@"+theme[0]+"@"+theme[1]]
						if len(ref) == 0 || len(got) == 0 {
							continue
						}
						compared++
						for _, p := range ppProps {
							va := ppNorm(ref[p])
							vb := ppNorm(got[p])
							if va != vb {
								cells = append(cells, parityCell{
									fmt.Sprintf("%s/%s/%s@%s@%s@%s", name, nd.label, p, path, theme[0], theme[1]),
									va, vb,
								})
							}
						}
					}
				}
			}
		}
	}
	if os.Getenv("PP_KEEP") == "" {
		os.RemoveAll(ppSim)
	}

	actual, order := cellMap(cells)
	if details {
		for _, id := range order {
			if strings.HasSuffix(id, "@css-import@light@ltr") {
				fmt.Printf("%s: %s\n", id, showCell(actual[id]))
			}
		}
	}
	if record || !fileExists(ppBaseline) {
		if err := writeParityBaseline(ppBaseline,
			"slot-only markup via css-import / full-css vs React inline classes under upstream css; may only shrink, and a recorded cell's VALUES are pinned too",
			nil, actual); err != nil {
			fmt.Fprintln(os.Stderr, "path-parity:", err)
			return 1
		}
		fmt.Printf("path-parity: baseline recorded (%d cells over %d components, %d element×path×theme×dir comparisons incl. %d variant + %d state renders)\n",
			len(actual), components, compared, variantRenders, stateRenders)
		return 0
	}
	_, recorded, err := loadParityBaseline(ppBaseline)
	if err != nil {
		fmt.Fprintln(os.Stderr, "path-parity:", err)
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
		fmt.Fprintf(os.Stderr, "FAIL  path-parity (%d NEW cells where a consume path ≠ React under upstream css)\n  %s%s\n",
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
		fmt.Fprintf(os.Stderr, "FAIL  path-parity (%d recorded cells still differ, but by a DIFFERENT amount — re-look, then re-record: ./build/pipeline path-parity --record)\n  %s\n",
			len(d.changed), strings.Join(parts, "\n  "))
		return 1
	}
	if len(d.fixed) > 0 {
		n := 20
		if len(d.fixed) < n {
			n = len(d.fixed)
		}
		fmt.Fprintf(os.Stderr, "FAIL  path-parity (%d recorded cells no longer differ — record the win: ./build/pipeline path-parity --record && ./build/pipeline ledger --record)\n  %s\n",
			len(d.fixed), strings.Join(d.fixed[:n], "\n  "))
		return 1
	}
	fmt.Printf("PASS  path-parity (%d components, %d comparisons incl. %d state renders, %d cells at the recorded baseline incl. their values; --strict is the end state)\n",
		components, compared, stateRenders, len(actual))
	return 0
}
