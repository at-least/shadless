package main

// T5 emitter, ported from src/emitter/index.mjs: IR (tier=static) → semantic
// usage HTML + slot-keyed CSS. Gates: static file count, no non-anchor
// class= in HTML, slot-tree vs IR (exact tags + nesting, via x/net/html —
// the jsdom gate's Go twin), no literal PascalCase tags.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"golang.org/x/net/html"
	"shadless/pipeline/internal/twmerge"
)

// treeNode is the resolved element tree buildTree produces.
type treeNode struct {
	tag     string
	slot    string
	anchor  string
	anchorM []string
	kids    []*treeNode
}

var reSketch = regexp.MustCompile(`^<([^ >]+)(?: slot=([^ >]+))?`)

// buildTree resolves a fn's element tree from the flat walk-order elements
// + sketches. Every tag (root AND children) goes through normalizeTag — the
// old code coerced every non-native root to <button>.
type buildTreeOpts struct {
	claimed       map[int]bool
	anchors       map[string]string
	anchorMarkers map[string][]string
}

func buildTree(ir cssIrComponent, fn irFn, o buildTreeOpts) (*treeNode, error) {
	type elIdx struct {
		e irEl
		i int
	}
	byKey := make([]elIdx, len(fn.Elements))
	for i, e := range fn.Elements {
		byKey[i] = elIdx{e, i}
	}
	var resolve func(el irEl, i int) (*treeNode, error)
	resolve = func(el irEl, i int) (*treeNode, error) {
		var kids []*treeNode
		for _, sk := range el.Children {
			m := reSketch.FindStringSubmatch(sk)
			if m == nil {
				continue // text/{children}/OPT?/expr → nothing structural
			}
			var hit *elIdx
			// raw-tag match on both sides (sketch carries the raw JSX name)
			for idx := range byKey {
				cand := byKey[idx]
				if _, used := o.claimedIdx(cand.i); used {
					continue
				}
				if m[2] != "" {
					if cand.e.Slot == m[2] && cand.e.Tag == m[1] {
						hit = &cand
						break
					}
				} else if cand.e.Tag == m[1] {
					hit = &cand
					break
				}
			}
			if hit != nil {
				o.markUsed(hit.i)
				kid, err := resolve(hit.e, hit.i)
				if err != nil {
					return nil, err
				}
				kids = append(kids, kid)
				continue
			}
			// unresolvable sketch: icon → svg; native → bare; else skip
			tag, ok := normalizeTag(m[1], ir.TagHints)
			if !ok {
				tag = m[1]
			}
			if nativeTags[tag] {
				slot := ""
				if m[2] != "" {
					slot = m[2]
				}
				kids = append(kids, &treeNode{tag: tag, slot: slot})
			}
		}
		tag, ok := normalizeTag(el.Tag, ir.TagHints)
		if !ok {
			return nil, fmt.Errorf("[%s] unresolvable tag in %s: %s", ir.Name, fn.Fn, el.Tag)
		}
		return &treeNode{
			tag:     tag,
			slot:    el.Slot,
			anchor:  o.anchors[fmt.Sprintf("%s#%d", fn.Fn, i)],
			anchorM: o.anchorMarkers[fmt.Sprintf("%s#%d", fn.Fn, i)],
			kids:    kids,
		}, nil
	}
	if len(fn.Elements) == 0 {
		return nil, fmt.Errorf("[%s] fn %s has no elements", ir.Name, fn.Fn)
	}
	rootIdx := 0
	o.markUsed(rootIdx)
	return resolve(fn.Elements[0], rootIdx)
}

// claimed/used helpers on buildTreeOpts (kept small so buildTree's body
// reads like the JS it mirrors).
func (o buildTreeOpts) claimedIdx(i int) (struct{}, bool) {
	if o.claimed == nil {
		return struct{}{}, false
	}
	if o.claimed[i] {
		return struct{}{}, true
	}
	return struct{}{}, false
}

func (o *buildTreeOpts) markUsed(i int) {
	if o.claimed == nil {
		o.claimed = map[int]bool{}
	}
	o.claimed[i] = true
}

// renderTree renders an element tree. defaultInner REPLACES the root's
// content (hand-authored compositions win over the sketch tree);
// defaultBySlot fills EMPTY leaves.
func renderTree(node *treeNode, markers map[string][]string, defaultInner string, defaultBySlot map[string]string, isRoot bool) string {
	var classes []string
	if node.slot != "" && len(markers[node.slot]) > 0 {
		classes = append(classes, dedup(markers[node.slot])...)
	}
	if node.anchor != "" {
		classes = append(classes, node.anchor)
	}
	classes = append(classes, node.anchorM...)
	cls := ""
	if len(classes) > 0 {
		cls = ` class="` + strings.Join(classes, " ") + `"`
	}
	slot := ""
	if node.slot != "" {
		slot = ` data-slot="` + node.slot + `"`
	}
	open := "<" + node.tag + slot + cls + ">"
	if voidTags[node.tag] {
		return open
	}
	var inner string
	switch {
	case isRoot && defaultInner != "":
		inner = defaultInner
	case len(node.kids) > 0:
		parts := make([]string, len(node.kids))
		for i, k := range node.kids {
			parts[i] = renderTree(k, markers, "", defaultBySlot, false)
		}
		inner = strings.Join(parts, "")
	default:
		inner = defaultBySlot[node.slot]
	}
	return open + inner + "</" + node.tag + ">"
}

// tableWrap: stray table-parts get dropped by HTML parsers at body level —
// wrap ancestors.
var tableWrap = map[string]string{
	"thead": "table", "tbody": "table", "tfoot": "table",
	"caption": "table", "colgroup": "table", "tr": "table", "th": "table", "td": "table",
}

func renderFn(tree *treeNode, markers map[string][]string, defaultInner string, defaultBySlot map[string]string) string {
	h := renderTree(tree, markers, defaultInner, defaultBySlot, true)
	tag := tree.tag
	for tableWrap[tag] != "" {
		h = "<" + tableWrap[tag] + ">" + h + "</" + tableWrap[tag] + ">"
		tag = tableWrap[tag]
	}
	return h
}

// resolveDefault: string → escaped text; {Inner,Attrs,Children} composed.
func resolveDefault(ir cssIrComponent, fn irFn) (inner string, attrs map[string]string, children map[string]string, ok bool) {
	entry, present := DEFAULT_CONTENT[ir.Name][fn.Fn]
	if !present || !entry.Set && entry.Inner == "" && entry.Attrs == nil && entry.Children == nil {
		// present-and-null → explicitly no default (ok=false)
		return "", nil, nil, false
	}
	return entry.Inner, entry.Attrs, entry.Children, entry.Inner != "" || entry.Attrs != nil || entry.Children != nil
}

// mergeRootAttrs applies extra attrs to the root open tag — quote-aware scan
// for the end of the first open tag (a plain [^>]* breaks on ">" inside
// attribute values).
func mergeRootAttrs(h string, attrs map[string]string) string {
	var parts []string
	for k, v := range attrs {
		parts = append(parts, fmt.Sprintf(`%s="%s"`, k, escHtml(v)))
	}
	if len(parts) == 0 {
		return h
	}
	extra := strings.Join(parts, " ")
	m := regexp.MustCompile(`^<([a-zA-Z][\w-]*)`).FindStringSubmatch(h)
	if m == nil {
		return h
	}
	i := len(m[0])
	var quote byte
	for i < len(h) {
		ch := h[i]
		if quote != 0 {
			if ch == quote {
				quote = 0
			}
		} else if ch == '"' || ch == '\'' {
			quote = ch
		} else if ch == '>' {
			break
		}
		i++
	}
	return h[:i] + " " + extra + h[i:]
}

// validateDefaultContent: stale keys previously survived silently.
func validateDefaultContent(statics []cssIrComponent) []string {
	var errs []string
	names := map[string]bool{}
	byName := map[string]cssIrComponent{}
	for _, ir := range statics {
		names[ir.Name] = true
		byName[ir.Name] = ir
	}
	// deterministic iteration for stable error output
	var comps []string
	for c := range DEFAULT_CONTENT {
		comps = append(comps, c)
	}
	sort.Strings(comps)
	for _, comp := range comps {
		if !names[comp] {
			errs = append(errs, "unknown component key: "+comp)
			continue
		}
		ir := byName[comp]
		var fns []string
		for f := range DEFAULT_CONTENT[comp] {
			fns = append(fns, f)
		}
		sort.Strings(fns)
		for _, fn := range fns {
			exported := false
			for _, c := range ir.Components {
				if c.Export && c.Fn == fn {
					exported = true
					break
				}
			}
			if !exported {
				errs = append(errs, fmt.Sprintf("[%s] unknown fn key: %s", comp, fn))
			}
		}
	}
	return errs
}

func runEmit() int {
	loadSkin()
	if err := os.MkdirAll("dist/components", 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "emit:", err)
		return 1
	}
	if err := os.MkdirAll("build/emit", 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "emit:", err)
		return 1
	}

	ents, err := os.ReadDir("src/registry/ir")
	if err != nil {
		fmt.Fprintln(os.Stderr, "emit:", err)
		return 1
	}
	var statics []cssIrComponent
	for _, e := range ents {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b, _ := os.ReadFile(filepath.Join("src/registry/ir", e.Name()))
		var ir cssIrComponent
		if err := json.Unmarshal(b, &ir); err != nil {
			fmt.Fprintln(os.Stderr, "emit: ir:", e.Name(), err)
			return 1
		}
		if ir.Tier == "static" {
			statics = append(statics, ir)
		}
	}
	tiersB, _ := os.ReadFile("src/registry/tiers.json")
	var tiers map[string]struct {
		Tier string `json:"tier"`
	}
	json.Unmarshal(tiersB, &tiers)
	wantStatic := 0
	for _, t := range tiers {
		if t.Tier == "static" {
			wantStatic++
		}
	}
	fail := false
	if len(statics) != wantStatic {
		fmt.Fprintf(os.Stderr, "FAIL expected %d static (from tiers.json), got %d\n", wantStatic, len(statics))
		return 1
	}
	for _, e := range validateDefaultContent(statics) {
		fmt.Fprintln(os.Stderr, "FAIL defaults:", e)
		fail = true
	}

	var cssParts []string
	allAnchors := map[string]bool{}
	type irTrees struct {
		ir    cssIrComponent
		trees []*treeNode
	}
	treesByIr := map[string]irTrees{}
	totalSlots := 0
	for _, ir := range statics {
		css, err := componentCss(ir)
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL css[%s]: %v\n", ir.Name, err)
			fail = true
			continue
		}
		for _, t := range css.anchors {
			allAnchors[t] = true
		}
		var trees []*treeNode
		var bodies []string
		for _, c := range ir.Components {
			if !c.Export {
				continue
			}
			tree, err := buildTree(ir, c, buildTreeOpts{
				claimed:       map[int]bool{},
				anchors:       css.anchors,
				anchorMarkers: css.anchorMarkers,
			})
			if err != nil {
				fmt.Fprintf(os.Stderr, "FAIL emit[%s.%s]: %v\n", ir.Name, c.Fn, err)
				fail = true
				bodies = append(bodies, "")
				continue
			}
			trees = append(trees, tree)
			inner, attrs, children, _ := resolveDefault(ir, c)
			if children == nil {
				children = map[string]string{}
			}
			h := renderFn(tree, css.markers, inner, children)
			if attrs != nil {
				h = mergeRootAttrs(h, attrs)
			}
			bodies = append(bodies, h)
		}
		treesByIr[ir.Name] = irTrees{ir, trees}
		page := "<!doctype html>\n<html><head><meta charset=\"utf-8\"><title>shadless " + ir.Name +
			"</title>\n<link rel=\"stylesheet\" href=\"../out.css\">" + ThemePrePaintScript + "</head>\n<body>\n" +
			strings.Join(bodies, "\n") + "\n</body></html>"
		if err := os.WriteFile("dist/components/"+ir.Name+".html", []byte(page), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "emit:", err)
			return 1
		}
		cssParts = append(cssParts, wrapComponentCss(ir.Name, css))
		totalSlots += strings.Count(page, `data-slot="`)
	}
	if err := os.WriteFile("dist/shadless.css", []byte(strings.Join(cssParts, "\n\n")), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "emit:", err)
		return 1
	}

	// CSS completeness gate: every class TOKEN must appear in some emitted
	// rule. Token-level, not string-level (same-slot complementary elements
	// legitimately split tokens across rules). A token the element's own
	// twMerge'd list drops is absent BY DESIGN, exactly as in React.
	{
		allCss := strings.Join(cssParts, "\n")
		for _, ir := range statics {
			for _, c := range ir.Components {
				for _, el := range c.Elements {
					var applyToks []string
					for _, x := range el.Classes {
						applyToks = append(applyToks, splitMarkers(x).apply)
					}
					kept := map[string]bool{}
					for _, t := range strings.Fields(twmerge.Merge(strings.Join(applyToks, " "))) {
						kept[t] = true
					}
					for _, cs := range el.Classes {
						var missing []string
						for _, t := range strings.Fields(splitMarkers(cs).apply) {
							if t != "" && kept[t] && !strings.Contains(allCss, t) {
								missing = append(missing, t)
							}
						}
						if len(missing) > 0 {
							n := 6
							if len(missing) < n {
								n = len(missing)
							}
							fmt.Fprintf(os.Stderr, "FAIL css[%s]: class tokens not in CSS: %q…\n", ir.Name, missing[:n])
							fail = true
						}
					}
				}
			}
		}
	}

	// globals (build/emit, NOT dist — the demo chain owns dist/globals.css)
	{
		gb, err := os.ReadFile("probes/h4/globals.css")
		if err != nil {
			fmt.Fprintln(os.Stderr, "emit:", err)
			return 1
		}
		g := strings.Replace(string(gb), "@source \"./demo.html\";\n", "", 1)
		out := g + "\n" + ShadlessCSSFixes + "\n" + strings.Join(cssParts, "\n\n")
		if err := os.WriteFile("build/emit/globals.css", []byte(out), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "emit:", err)
			return 1
		}
		var li strings.Builder
		li.WriteString("<!doctype html><html><head><meta charset=\"utf-8\">\n<link rel=\"stylesheet\" href=\"out.css\"></head><body>\n<ul>")
		for _, ir := range statics {
			fmt.Fprintf(&li, "<li><a href=\"components/%s.html\">%s</a></li>", ir.Name, ir.Name)
		}
		li.WriteString("</ul>\n</body></html>")
		if err := os.WriteFile("build/emit/demo-index.html", []byte(li.String()), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "emit:", err)
			return 1
		}
	}
	fmt.Printf("emit: %d pages, %d slots, shadless.css\n", len(statics), totalSlots)

	// ---- gates ----
	reClass := regexp.MustCompile(`class="([^"]*)"`)
	rePascal := regexp.MustCompile(`</?([A-Z][A-Za-z0-9]*(?:\.[A-Z][A-Za-z0-9]*)?)[\s>]`)

	// gate: no class= beyond markers/anchors/allowlist
	for _, ir := range statics {
		h, _ := os.ReadFile("dist/components/" + ir.Name + ".html")
		for _, m := range reClass.FindAllStringSubmatch(string(h), -1) {
			bad := false
			for _, t := range strings.Fields(m[1]) {
				if t == "" || markerRe.MatchString(t) || skinData.Allowlist[t] || allAnchors[t] {
					continue
				}
				bad = true
				break
			}
			if bad {
				fmt.Fprintf(os.Stderr, "FAIL [%s]: non-anchor class= in HTML: %s\n", ir.Name, m[1])
				fail = true
			}
		}
	}

	// gate: literal PascalCase / ternary tags
	for _, ir := range statics {
		h, _ := os.ReadFile("dist/components/" + ir.Name + ".html")
		if m := rePascal.FindStringSubmatch(string(h)); m != nil {
			fmt.Fprintf(os.Stderr, "FAIL [%s]: literal component tag in HTML: %s\n", ir.Name, m[1])
			fail = true
		}
	}

	// gate: slot-tree vs IR (exact tags + nesting) — x/net/html's parse is
	// the jsdom twin: same lowercased tags, same foster-parenting rules.
	for _, it := range treesByIr {
		h, _ := os.ReadFile("dist/components/" + it.ir.Name + ".html")
		doc, err := html.Parse(strings.NewReader(string(h)))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL [%s]: parse: %v\n", it.ir.Name, err)
			fail = true
			continue
		}
		treePairs, treeEdges := map[string]bool{}, map[string]bool{}
		var collect func(n *treeNode, parentSlot string)
		collect = func(n *treeNode, parentSlot string) {
			if n.slot != "" {
				treePairs[n.tag+"@"+n.slot] = true
				if parentSlot != "" {
					treeEdges[parentSlot+">"+n.slot] = true
				}
			}
			for _, k := range n.kids {
				collect(k, firstNonEmpty(n.slot, parentSlot))
			}
		}
		for _, t := range it.trees {
			collect(t, "")
		}

		// DEFAULT_CONTENT html chunks are sanctioned sources of pairs+edges
		defPairs, defEdges := map[string]bool{}, map[string]bool{}
		for _, c := range it.ir.Components {
			if !c.Export {
				continue
			}
			inner, _, _, has := resolveDefault(it.ir, c)
			if !has || inner == "" {
				continue
			}
			rootEl := c.Elements[0]
			rootTag, ok := normalizeTag(rootEl.Tag, it.ir.TagHints)
			if !ok {
				rootTag = "div"
			}
			scope := ""
			if tableWrap[rootTag] != "" {
				scope = "<table>"
			}
			wrapOpen := scope
			if rootEl.Slot != "" {
				wrapOpen += "<" + rootTag + ` data-slot="` + rootEl.Slot + `">`
			} else {
				wrapOpen += "<" + rootTag + ">"
			}
			closeTag := "</" + rootTag + ">" + scopeClose(scope)
			frag, err := html.Parse(strings.NewReader(wrapOpen + inner + closeTag))
			if err != nil {
				continue
			}
			walkSlotted(frag, func(e *html.Node, parentSlot string) {
				defPairs[e.Data+"@"+slotOf(e)] = true
				if parentSlot != "" {
					defEdges[parentSlot+">"+slotOf(e)] = true
				}
			})
		}

		// IR-side pairs
		irPairs := map[string]bool{}
		for _, c := range it.ir.Components {
			for _, el := range c.Elements {
				if el.Slot == "" {
					continue
				}
				tag, ok := normalizeTag(el.Tag, it.ir.TagHints)
				if !ok {
					tag = "?"
				}
				irPairs[tag+"@"+el.Slot] = true
			}
		}

		var domNodes []string
		domSet := map[string]bool{}
		domEdges := map[string]bool{}
		walkSlotted(doc, func(e *html.Node, parentSlot string) {
			p := e.Data + "@" + slotOf(e)
			domNodes = append(domNodes, p)
			domSet[p] = true
			if parentSlot != "" {
				domEdges[parentSlot+">"+slotOf(e)] = true
			}
		})

		sanctionedPairs := map[string]bool{}
		for k := range treePairs {
			sanctionedPairs[k] = true
		}
		for k := range irPairs {
			sanctionedPairs[k] = true
		}
		for k := range defPairs {
			sanctionedPairs[k] = true
		}
		sanctionedEdges := map[string]bool{}
		for k := range treeEdges {
			sanctionedEdges[k] = true
		}
		for k := range defEdges {
			sanctionedEdges[k] = true
		}

		// IR slot missing in DOM: any dom pair with the same slot suffix
		for p := range unionKeys(irPairs, treePairs) {
			slot := p[strings.Index(p, "@")+1:]
			found := false
			for d := range domSet {
				if strings.HasSuffix(d, "@"+slot) {
					found = true
					break
				}
			}
			if !found {
				fmt.Fprintf(os.Stderr, "FAIL [%s]: IR slot missing in DOM: %s\n", it.ir.Name, p)
				fail = true
			}
		}
		for _, d := range domNodes {
			if !sanctionedPairs[d] {
				fmt.Fprintf(os.Stderr, "FAIL [%s]: DOM slot not sanctioned (tree/IR/default): %s\n", it.ir.Name, d)
				fail = true
			}
		}
		for e := range domEdges {
			if !sanctionedEdges[e] {
				fmt.Fprintf(os.Stderr, "FAIL [%s]: DOM nesting not sanctioned: %s\n", it.ir.Name, e)
				fail = true
			}
		}
		if (len(irPairs) > 0 || len(treePairs) > 0) && len(domNodes) == 0 {
			fmt.Fprintf(os.Stderr, "FAIL [%s]: no slots rendered\n", it.ir.Name)
			fail = true
		}
	}

	if fail {
		fmt.Println("FAIL  emit gates")
		return 1
	}
	fmt.Printf("PASS  emit static gates (%d files, 0 class=, exact slot-tree, %d anchors)\n", len(statics), len(allAnchors))
	return 0
}

func scopeClose(scope string) string {
	if scope == "" {
		return ""
	}
	return "</table>"
}

func slotOf(e *html.Node) string {
	for _, a := range e.Attr {
		if a.Key == "data-slot" {
			return a.Val
		}
	}
	return ""
}

// walkSlotted visits every element carrying data-slot, with the nearest
// slotted ANCESTOR's slot.
func walkSlotted(n *html.Node, visit func(e *html.Node, parentSlot string)) {
	var walk func(n *html.Node, parentSlot string)
	walk = func(n *html.Node, parentSlot string) {
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			ps := parentSlot
			if c.Type == html.ElementNode {
				if s := slotOf(c); s != "" {
					visit(c, parentSlot)
					ps = s
				}
			}
			walk(c, ps)
		}
	}
	walk(n, "")
}

func firstNonEmpty(a, b string) string {
	if a != "" {
		return a
	}
	return b
}

func mapKeys(m map[string]bool) []string {
	var out []string
	for k := range m {
		out = append(out, k)
	}
	return out
}

func unionKeys(a, b map[string]bool) map[string]bool {
	out := map[string]bool{}
	for k := range a {
		out[k] = true
	}
	for k := range b {
		out[k] = true
	}
	return out
}
