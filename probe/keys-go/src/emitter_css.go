package main

// emitter_css.go — the Go port of src/emitter/css.mjs. Faithful per branch;
// every comment about a shipped bug stays, since each one cost a debugging
// session to learn.
//
// The IR is read from generated/ir/<name>.json with json.Decoder and
// case-tolerant struct tags — JSON keys are exactly Go field names here.

import (
	"bytes"
	"encoding/json"
	"fmt"
	"regexp"
	"sort"
	"strings"

	"shadless/pipeline/internal/twmerge"
)

// IR shape as css.mjs + the emitter consume it (superset of tiers.json's).
type cssIrComponent struct {
	Name         string            `json:"name"`
	Tier         string            `json:"tier"`
	Imports      []string          `json:"imports"`
	Cva          orderedCva        `json:"cva"`
	Components   []irFn            `json:"components"`
	Conditionals []irCond          `json:"conditionals"`
	CvaRefs      []irCvaRef        `json:"cvaRefs"`
	TagHints     map[string]string `json:"tagHints"`
}

type irFn struct {
	Fn       string `json:"fn"`
	Export   bool   `json:"export"`
	Elements []irEl `json:"elements"`
}

type irEl struct {
	Tag      string   `json:"tag"`
	Slot     string   `json:"slot"`
	Classes  []string `json:"classes"`
	Spread   bool     `json:"spread"`
	Children []string `json:"children"`
}

type irCond struct {
	Kind string      `json:"kind"`
	Fn   string      `json:"fn"`
	Slot *string     `json:"slot"`
	Then string      `json:"then"`
	Else string      `json:"else"`
	Test *irCondTest `json:"test"`
}

type irCondTest struct {
	Name    string `json:"name"`
	Op      string `json:"op"`
	Value   string `json:"value"`
	Default any    `json:"default"`
}

type irCvaRef struct {
	Slot     string            `json:"slot"`
	Ref      string            `json:"ref"`
	Table    cvaTable          `json:"table"`
	Dyn      []irCvaDyn        `json:"dyn"`
	DynAxes  []string          `json:"dynAxes"`
	Defaults map[string]string `json:"defaults"`
}

type irCvaDyn struct {
	Attr    string `json:"attr"`
	When    string `json:"when"`
	Classes string `json:"classes"`
}

// cvaTable keeps the JSON's insertion order for axes AND values: JS's
// Object.entries iterates in source order, and the emitted rule ORDER
// (which axis's default block comes first) is observable in dist output.
type cvaTable struct {
	Base       string
	axisOrder  []string // axis keys in JSON order
	variants   map[string]map[string]string
	valueOrder map[string][]string // per-axis value keys in JSON order
	defaults   map[string]string
}

func (t *cvaTable) UnmarshalJSON(b []byte) error {
	var raw struct {
		Base     string            `json:"base"`
		Variants json.RawMessage   `json:"variants"`
		Defaults map[string]string `json:"defaults"`
	}
	if err := json.Unmarshal(b, &raw); err != nil {
		return err
	}
	t.Base = raw.Base
	t.defaults = raw.Defaults
	t.variants = map[string]map[string]string{}
	t.valueOrder = map[string][]string{}
	axes, err := decodeOrderedObject(raw.Variants)
	if err != nil {
		return err
	}
	for _, ax := range axes.keys {
		vals, err := decodeOrderedObject(axes.raw[ax])
		if err != nil {
			return err
		}
		m := map[string]string{}
		for _, vk := range vals.keys {
			var cls string
			if err := json.Unmarshal(vals.raw[vk], &cls); err != nil {
				return err
			}
			m[vk] = cls
		}
		t.axisOrder = append(t.axisOrder, ax)
		t.variants[ax] = m
		t.valueOrder[ax] = vals.keys
	}
	return nil
}

// orderedObject is a JSON object that remembers its key order.
type orderedObject struct {
	keys []string
	raw  map[string]json.RawMessage
}

// decodeOrderedObject decodes one JSON object preserving key order via a
// token stream.
func decodeOrderedObject(b []byte) (orderedObject, error) {
	out := orderedObject{raw: map[string]json.RawMessage{}}
	if len(bytes.TrimSpace(b)) == 0 {
		return out, nil
	}
	dec := json.NewDecoder(bytes.NewReader(b))
	tok, err := dec.Token()
	if err != nil {
		return out, err
	}
	if d, ok := tok.(json.Delim); !ok || d != '{' {
		return out, fmt.Errorf("expected object, got %v", tok)
	}
	for dec.More() {
		kt, err := dec.Token()
		if err != nil {
			return out, err
		}
		key, ok := kt.(string)
		if !ok {
			return out, fmt.Errorf("object key not a string: %v", kt)
		}
		var val json.RawMessage
		if err := dec.Decode(&val); err != nil {
			return out, err
		}
		if _, dup := out.raw[key]; !dup {
			out.keys = append(out.keys, key)
		}
		out.raw[key] = val
	}
	_, _ = dec.Token() // closing }
	return out, nil
}

// orderedCva is ir.cva with table order preserved (Object.entries order).
type orderedCva struct {
	keys   []string
	tables map[string]cvaTable
}

func (o *orderedCva) UnmarshalJSON(b []byte) error {
	obj, err := decodeOrderedObject(b)
	if err != nil {
		return err
	}
	o.tables = map[string]cvaTable{}
	for _, k := range obj.keys {
		var t cvaTable
		if err := json.Unmarshal(obj.raw[k], &t); err != nil {
			return err
		}
		o.keys = append(o.keys, k)
		o.tables[k] = t
	}
	return nil
}

// ---- from css.mjs, verbatim in intent --------------------------------------

var reCssEscape = regexp.MustCompile(`[^A-Za-z0-9_-]`)

func cssEscape(t string) string {
	// JS replace with a function: CSS escaping is identity-but-prefix for
	// every character outside [A-Za-z0-9_-].
	return reCssEscape.ReplaceAllStringFunc(t, func(ch string) string { return "\\" + ch })
}

var reResidueText = regexp.MustCompile(`^text-(xs|sm|base|lg|xl|\dxl)$`)
var reResidueSize = regexp.MustCompile(`^size-`)
var reResidueTextLeading = regexp.MustCompile(`^(leading-|text-(xs|sm|base|lg|xl|\dxl)$)`)
var reResidueSizeW = regexp.MustCompile(`^(w-|size-)`)
var reResidueSizeH = regexp.MustCompile(`^(h-|size-)`)

// residueResets mirrors css.mjs:39. When twMerge dropped a base token whose
// extra properties (line-height on text-sm, height on size-4) are not
// restated by the value, the value rule gets an explicit reset.
func residueResets(base, value string) []string {
	merged := map[string]bool{}
	for _, t := range strings.Fields(twmerge.Merge(base + " " + value)) {
		merged[t] = true
	}
	var out []string
	valueToks := strings.Fields(value)
	for _, tok := range strings.Fields(base) {
		if merged[tok] {
			continue
		}
		switch {
		case reResidueText.MatchString(tok):
			if !someMatch(valueToks, reResidueTextLeading) {
				out = append(out, "leading-[inherit]")
			}
		case reResidueSize.MatchString(tok):
			if !someMatch(valueToks, reResidueSizeW) {
				out = append(out, "w-auto")
			}
			if !someMatch(valueToks, reResidueSizeH) {
				out = append(out, "h-auto")
			}
		}
	}
	return dedup(out)
}

// residueRuleParts computes the (optional) residue-reset rule and the main
// `[data-slot=slot]:where(...)` rule for one cva axis value — widened to also
// match the unset/empty attribute when val is the axis default. apply is the
// value's own classes, before any reset tokens (from residueResets) are
// appended into the main rule's @apply list; causes are read from that
// original apply so the reset rule's selector names the utility that actually
// needs the reset, never a reset token appended here. hasReset reports
// whether resetRule is populated. Local cva table rules and cross-file
// cvaRefs rules order the two returned rules differently to match their
// respective JS sources, so the caller appends them, not this function.
func residueRuleParts(slot, axis, val, def, baseApply, apply string) (resetRule string, hasReset bool, mainRule string) {
	full := apply
	resets := residueResets(baseApply, apply)
	if len(resets) > 0 {
		full = apply + " " + strings.Join(resets, " ")
		causes := filterToks(apply, func(vt string) bool {
			return len(residueResets(baseApply, vt)) > 0
		})
		if len(causes) > 0 {
			sel := "." + cssEscape(causes[0])
			for _, c := range causes[1:] {
				sel += ", ." + cssEscape(c)
			}
			resetRule = fmt.Sprintf("  [data-slot=\"%s\"]:where(%s) { @apply %s; }", slot, sel, strings.Join(resets, " "))
			hasReset = true
		}
	}
	mainRule = fmt.Sprintf("  [data-slot=\"%s\"]:where([data-%s=\"%s\"]) { @apply %s; }", slot, axis, val, full)
	if val == def {
		mainRule = fmt.Sprintf("  [data-slot=\"%s\"]:where(:not([data-%s]), [data-%s=\"%s\"], [data-%s=\"\"]) { @apply %s; }",
			slot, axis, axis, val, axis, full)
	}
	return resetRule, hasReset, mainRule
}

func someMatch(toks []string, re *regexp.Regexp) bool {
	for _, t := range toks {
		if re.MatchString(t) {
			return true
		}
	}
	return false
}

func dedup(ss []string) []string {
	seen := map[string]bool{}
	var out []string
	for _, s := range ss {
		if !seen[s] {
			seen[s] = true
			out = append(out, s)
		}
	}
	return out
}

var markerRe = regexp.MustCompile(`^(group|peer)(/[\w-]+)?$`)

var deadUtilities = map[string]bool{"origin-top-center": true}

type splitMarkersOut struct {
	apply   string
	markers []string
}

func splitMarkers(str string) splitMarkersOut {
	var applyToks, markers []string
	for _, t := range strings.Fields(str) {
		switch {
		case markerRe.MatchString(t), skinData.Allowlist[t], deadUtilities[t]:
			markers = append(markers, t)
		default:
			applyToks = append(applyToks, t)
		}
	}
	return splitMarkersOut{apply: strings.Join(applyToks, " "), markers: markers}
}

// cvaSlot: naming convention (buttonVariants → Button) else a single fn
// whose name starts with the stem. No slot → no rule (inputGroupButtonVariants
// → input-group ROOT pollution was the bug the old ||-fallback let through).
func cvaSlot(ir cssIrComponent) map[string]struct {
	table cvaTable
	slot  string
} {
	out := map[string]struct {
		table cvaTable
		slot  string
	}{}
	for _, varName := range ir.Cva.keys {
		table := ir.Cva.tables[varName]
		stem := strings.TrimSuffix(varName, "Variants")
		var fn *irFn
		for i := range ir.Components {
			if strings.EqualFold(ir.Components[i].Fn, stem) {
				fn = &ir.Components[i]
				break
			}
		}
		if fn == nil {
			for i := range ir.Components {
				if strings.HasPrefix(strings.ToLower(ir.Components[i].Fn), strings.ToLower(stem)) {
					fn = &ir.Components[i]
					break
				}
			}
		}
		slot := ""
		if fn != nil {
			for _, e := range fn.Elements {
				if e.Slot != "" {
					slot = e.Slot
					break
				}
			}
		}
		out[varName] = struct {
			table cvaTable
			slot  string
		}{table, slot}
	}
	return out
}

var reCleanTagPrefix = regexp.MustCompile(`^<ternary:[^/]+/`)
var reCleanTagSuffix = regexp.MustCompile(`>$`)

func cleanTag(t string) string {
	return kebab(reCleanTagSuffix.ReplaceAllString(reCleanTagPrefix.ReplaceAllString(t, ""), ""))
}

// ---- componentCss -----------------------------------------------------------

type componentCssOut struct {
	rules         []string
	markers       map[string][]string
	anchors       map[string]string
	anchorMarkers map[string][]string
	unlayered     []string
}

func componentCss(ir cssIrComponent) (componentCssOut, error) {
	loadSkin()
	var rules []string
	cvaMap := cvaSlot(ir)
	cvaSlots := map[string]bool{}
	for _, v := range cvaMap {
		if v.slot != "" {
			cvaSlots[v.slot] = true
		}
	}
	markers := map[string][]string{}
	anchors := map[string]string{}
	anchorsOrder := []string{}
	anchorMarkers := map[string][]string{}
	var lateAnchorRules []string
	usedTokens := map[string]bool{}
	token := func(t string) string {
		out, n := t, 1
		for usedTokens[out] {
			n++
			out = fmt.Sprintf("%s-%d", t, n)
		}
		usedTokens[out] = true
		return out
	}

	// pass 1: group elements — slotted (by slot) / slotless (anchors)
	type bySlotItem struct {
		el  irEl
		key string
	}
	bySlot := map[string][]bySlotItem{}
	bySlotOrder := []string{}
	for _, c := range ir.Components {
		for idx, el := range c.Elements {
			if len(el.Classes) == 0 {
				continue
			}
			key := fmt.Sprintf("%s#%d", c.Fn, idx)
			if el.Slot != "" && !cvaSlots[el.Slot] {
				if _, seen := bySlot[el.Slot]; !seen {
					bySlotOrder = append(bySlotOrder, el.Slot)
				}
				bySlot[el.Slot] = append(bySlot[el.Slot], bySlotItem{el, key})
			} else if el.Slot == "" {
				base := ""
				if len(c.Elements) > 0 && c.Elements[0].Tag == el.Tag && idx == 0 {
					base = kebab(c.Fn)
				} else {
					base = kebab(c.Fn) + "-" + cleanTag(el.Tag)
				}
				anchors[key] = token(base)
				anchorsOrder = append(anchorsOrder, key)
			}
		}
	}

	// conditional class branches (class-cond with a readable test)
	type condBranch struct {
		then, elseCls string
		test          *irCondTest
	}
	condBranches := map[string][]condBranch{}
	rootSlot := ""
	for _, c := range ir.Components {
		for _, e := range c.Elements {
			if e.Slot != "" {
				rootSlot = e.Slot
				break
			}
		}
		if rootSlot != "" {
			break
		}
	}
	for _, cond := range ir.Conditionals {
		if cond.Kind != "class-cond" || cond.Test == nil {
			continue
		}
		c := findFn(ir, cond.Fn)
		if c == nil {
			continue
		}
		for idx, el := range c.Elements {
			if containsTok(el.Classes, cond.Then) && containsTok(el.Classes, cond.Else) {
				key := fmt.Sprintf("%s#%d", cond.Fn, idx)
				condBranches[key] = append(condBranches[key], condBranch{cond.Then, cond.Else, cond.Test})
			}
		}
	}

	stripBranches := func(el irEl, key string) string {
		var keep []string
		for _, c := range el.Classes {
			branch := false
			for _, cd := range condBranches[key] {
				if c == cd.then || c == cd.elseCls {
					branch = true
					break
				}
			}
			if !branch {
				keep = append(keep, c)
			}
		}
		return strings.Join(keep, " ")
	}

	// branchRules emits the twin :is()/absent-default rule pair per
	// conditional on this element.
	branchRules := func(selector, key string) []string {
		var out []string
		for _, cond := range condBranches[key] {
			t := cond.test
			attr := "data-" + kebab(t.Name)
			v := t.Value
			explicitTrue := cond.then
			explicitFalse := cond.elseCls
			if t.Op != "===" {
				explicitTrue, explicitFalse = cond.elseCls, cond.then
			}
			var absentIsTrue bool
			switch d := t.Default.(type) {
			case nil:
				absentIsTrue = true
			case string:
				absentIsTrue = (d == v) == (t.Op == "===")
			default:
				absentIsTrue = true
			}
			ctx, ctxOther := "", ""
			if rootSlot != "" {
				ctx = fmt.Sprintf(`, [data-slot="%s"][%s="%s"] *`, rootSlot, attr, v)
				ctxOther = fmt.Sprintf(`, [data-slot="%s"][%s]:not([%s="%s"]) *`, rootSlot, attr, attr, v)
			}
			isV := fmt.Sprintf(":is([%s=\"%s\"]%s)", attr, v, ctx)
			isOther := fmt.Sprintf(":is([%s]:not([%s=\"%s\"])%s)", attr, attr, v, ctxOther)
			tm := splitMarkers(explicitTrue)
			fm := splitMarkers(explicitFalse)
			notInline := branchNotInline(tm.apply, fm.apply)
			trueSel := selector + ":where("
			if absentIsTrue {
				trueSel += ":not(" + isOther[4:len(isOther)-1] + ")"
			} else {
				trueSel += isV
			}
			trueSel += notInline + ")"
			falseSel := selector + ":where("
			if absentIsTrue {
				falseSel += isOther
			} else {
				falseSel += ":not(" + isV[4:len(isV)-1] + ")"
			}
			falseSel += notInline + ")"
			if tm.apply != "" {
				out = append(out, fmt.Sprintf("  %s { @apply %s; }", trueSel, tm.apply))
			}
			if fm.apply != "" {
				out = append(out, fmt.Sprintf("  %s { @apply %s; }", falseSel, fm.apply))
			}
		}
		return out
	}

	// anchor rules — in INSERTION order (JS Map). anchors appended by the
	// conflicting-sig pass below ride at the end via lateAnchorRules and are
	// registered into anchors before the cva block.
	for _, key := range anchorsOrder {
		t := anchors[key]
		fnName, idx := splitAnchorKey(key)
		c := findFn(ir, fnName)
		if c == nil || idx >= len(c.Elements) {
			continue
		}
		el := c.Elements[idx]
		s := splitMarkers(stripBranches(el, key))
		if len(s.markers) > 0 {
			anchorMarkers[key] = s.markers
		}
		if s.apply != "" {
			rules = append(rules, fmt.Sprintf("  .%s { @apply %s; }", t, s.apply))
		}
		rules = append(rules, branchRules("."+t, key)...)
	}

	// plain-class slot rules — INSERTION order (JS Map ordering); the
	// slot key first appears is the slot's birth order
	for _, slot := range bySlotOrder {
		items := bySlot[slot]
		type sigInfo struct {
			tags    map[string]bool
			markers []string
		}
		sigs := map[string]*sigInfo{}
		var sigOrder []string
		for _, item := range items {
			s := splitMarkers(stripBranches(item.el, item.key))
			info, ok := sigs[s.apply]
			if !ok {
				info = &sigInfo{tags: map[string]bool{}}
				sigs[s.apply] = info
				sigOrder = append(sigOrder, s.apply)
			}
			tag, ok := normalizeTag(item.el.Tag, ir.TagHints)
			if !ok {
				tag = "?"
			}
			info.tags[tag] = true
			info.markers = append(info.markers, s.markers...)
		}
		var allMarkers []string
		for _, it := range items {
			s := splitMarkers(strings.Join(it.el.Classes, " "))
			allMarkers = append(allMarkers, s.markers...)
		}
		if len(allMarkers) > 0 {
			markers[slot] = dedup(allMarkers)
		}
		if len(sigOrder) == 1 {
			apply := sigOrder[0]
			if apply != "" {
				rules = append(rules, fmt.Sprintf("  [data-slot=\"%s\"] { @apply %s; }", slot, apply))
			}
			for _, it := range items {
				rules = append(rules, branchRules(fmt.Sprintf("[data-slot=\"%s\"]", slot), it.key)...)
			}
			continue
		}
		// conflicting class sets on one slot
		tagOverlap := false
		for _, info := range sigs {
			for tag := range info.tags {
				if tag == "?" {
					tagOverlap = true
					break
				}
				for _, other := range sigs {
					if other != info && other.tags[tag] {
						tagOverlap = true
						break
					}
				}
			}
		}
		if !tagOverlap {
			for _, apply := range sigOrder {
				info := sigs[apply]
				var tag string
				for t := range info.tags {
					tag = t
					break
				}
				if apply != "" {
					rules = append(rules, fmt.Sprintf("  [data-slot=\"%s\"]:is(%s) { @apply %s; }", slot, tag, apply))
				}
			}
			continue
		}
		var tokenLists [][]string
		for _, apply := range sigOrder {
			toks := strings.Fields(apply)
			if len(toks) == 0 {
				tokenLists = nil
				break
			}
			tokenLists = append(tokenLists, toks)
		}
		if tokenLists == nil {
			continue
		}
		common := intersectStrings(tokenLists)
		if len(common) > 0 {
			rules = append(rules, fmt.Sprintf("  [data-slot=\"%s\"] { @apply %s; }", slot, strings.Join(common, " ")))
		}
		for _, it := range items {
			own := strings.Fields(splitMarkers(strings.Join(it.el.Classes, " ")).apply)
			var rest []string
			for _, t := range own {
				if !containsTok(common, t) {
					rest = append(rest, t)
				}
			}
			if len(rest) == 0 {
				continue
			}
			tk := token(slot)
			anchors[it.key] = tk
			anchorsOrder = append(anchorsOrder, it.key)
			lateAnchorRules = append(lateAnchorRules, fmt.Sprintf("  .%s { @apply %s; }", tk, strings.Join(rest, " ")))
		}
	}

	// cva rules — variants and axes iterate in JSON insertion order
	// (JS Object.entries on a plain object), not sort order. The
	// badge/button/field divergence was sort order; the shapes are equal.
	rules = append(rules, lateAnchorRules...)
	cvaNames := ir.Cva.keys
	for _, varName := range cvaNames {
		cv := cvaMap[varName]
		if cv.slot == "" {
			continue
		}
		s := splitMarkers(cv.table.Base)
		if len(s.markers) > 0 {
			markers[cv.slot] = s.markers
		}
		if s.apply != "" {
			rules = append(rules, fmt.Sprintf("  [data-slot=\"%s\"] { @apply %s; }", cv.slot, s.apply))
		}
		baseApply := s.apply
		axes := cv.table.axisOrder
		for _, axis := range axes {
			def := cv.table.defaults[axis]
			vals := cv.table.variants[axis]
			for _, val := range cv.table.valueOrder[axis] {
				cls := vals[val]
				if cls == "" {
					continue
				}
				t := splitMarkers(cls)
				if len(t.markers) > 0 {
					markers[cv.slot] = append(markers[cv.slot], t.markers...)
				}
				if t.apply == "" {
					continue
				}
				resetRule, hasReset, mainRule := residueRuleParts(cv.slot, axis, val, def, baseApply, t.apply)
				if hasReset {
					rules = append(rules, resetRule)
				}
				rules = append(rules, mainRule)
			}
		}
	}

	// cross-file cva refs
	for _, r := range ir.CvaRefs {
		for _, d := range r.Dyn {
			rules = append(rules, fmt.Sprintf("  [data-slot=\"%s\"][%s=\"%s\"] { @apply %s; }", r.Slot, d.Attr, d.When, d.Classes))
		}
		for _, axis := range r.DynAxes {
			vals := r.Table.variants[axis]
			def, hasDef := r.Defaults[axis]
			baseApply := splitMarkers(r.Table.Base).apply
			for _, val := range r.Table.valueOrder[axis] {
				cls := vals[val]
				if cls == "" {
					continue
				}
				t := splitMarkers(cls)
				if t.apply == "" {
					continue
				}
				if !hasDef {
					rules = append(rules, fmt.Sprintf("  [data-slot=\"%s\"][data-%s=\"%s\"] { @apply %s; }", r.Slot, axis, val, t.apply))
					continue
				}
				resetRule, hasReset, mainRule := residueRuleParts(r.Slot, axis, val, def, baseApply, t.apply)
				rules = append(rules, mainRule)
				if hasReset {
					rules = append(rules, resetRule)
				}
			}
		}
	}

	// allowlisted skin markers that DO have a skin body must emit their own
	// class-anchored rule (.cn-menu-translucent found by gates/overlay.mjs
	// dissolve check)
	usedAllowlist := map[string]bool{}
	for _, ms := range markers {
		for _, t := range ms {
			if skinData.Allowlist[t] {
				if _, has := skinData.Map[t]; has {
					usedAllowlist[t] = true
				}
			}
		}
	}
	for _, ms := range anchorMarkers {
		for _, t := range ms {
			if skinData.Allowlist[t] {
				if _, has := skinData.Map[t]; has {
					usedAllowlist[t] = true
				}
			}
		}
	}
	var unlayered []string
	ul := make([]string, 0, len(usedAllowlist))
	for t := range usedAllowlist {
		ul = append(ul, t)
	}
	sort.Strings(ul)
	for _, t := range ul {
		unlayered = append(unlayered, fmt.Sprintf(".%s { @apply %s; }", t, skinData.Map[t]))
	}

	return componentCssOut{
		rules:         reApplyMerge(rules),
		markers:       markers,
		anchors:       anchors,
		anchorMarkers: anchorMarkers,
		unlayered:     reApplyMerge(unlayered),
	}, nil
}

// wrapComponentCss mirrors css.mjs's wrapComponentCss — one layer block plus
// an optional unlayered skin-marker block, used identically by the static
// emitter and the demo builder.
func wrapComponentCss(name string, css componentCssOut) string {
	parts := []string{"/* " + name + " */\n@layer components {\n" + strings.Join(css.rules, "\n") + "\n}"}
	if len(css.unlayered) > 0 {
		parts = append(parts, "/* "+name+": skin markers (unlayered, as upstream ships them) */\n"+strings.Join(css.unlayered, "\n"))
	}
	return strings.Join(parts, "\n")
}

func reApplyMerge(rules []string) []string {
	out := make([]string, 0, len(rules))
	for _, r := range rules {
		out = append(out, applyMergeRe.ReplaceAllStringFunc(r, func(m string) string {
			body := applyMergeRe.FindStringSubmatch(m)[1]
			return "@apply " + twmerge.Merge(body) + ";"
		}))
	}
	return out
}

var applyMergeRe = regexp.MustCompile(`@apply ([^;]+);`)

// ---- helpers ----------------------------------------------------------------

func findFn(ir cssIrComponent, name string) *irFn {
	for i := range ir.Components {
		if ir.Components[i].Fn == name {
			return &ir.Components[i]
		}
	}
	return nil
}

func splitAnchorKey(key string) (string, int) {
	i := strings.LastIndex(key, "#")
	n := 0
	fmt.Sscanf(key[i+1:], "%d", &n)
	return key[:i], n
}

func containsTok(ss []string, s string) bool {
	for _, x := range ss {
		if x == s {
			return true
		}
	}
	return false
}

func intersectStrings(lists [][]string) []string {
	if len(lists) == 0 {
		return nil
	}
	var out []string
	for _, t := range lists[0] {
		everywhere := true
		for _, l := range lists[1:] {
			if !containsTok(l, t) {
				everywhere = false
				break
			}
		}
		if everywhere {
			out = append(out, t)
		}
	}
	return out
}

func filterToks(s string, keep func(string) bool) []string {
	var out []string
	for _, t := range strings.Fields(s) {
		if keep(t) {
			out = append(out, t)
		}
	}
	return out
}

// branchNotInline is the ":not(.token)…" chain that suppresses a branch rule
// when the element already carries either branch inline (or its logical twin).
func branchNotInline(tApply, fApply string) string {
	var toks []string
	for _, t := range dedup(append(strings.Fields(tApply), strings.Fields(fApply)...)) {
		toks = append(toks, twinsOf(t)...)
	}
	var parts []string
	seen := map[string]bool{}
	for _, t := range toks {
		if t == "" {
			continue
		}
		for _, sh := range shadowsOf(t) {
			if !seen[sh] {
				seen[sh] = true
				parts = append(parts, sh)
			}
		}
	}
	return strings.Join(parts, "")
}

// twinsOf: physical/logical spacing twins (pl-4 ↔ ps-4 in an RTL box).
func twinsOf(tok string) []string {
	m := reTwin.FindStringSubmatch(tok)
	if m == nil {
		return []string{tok}
	}
	alt := map[string]string{"l": "s", "r": "e", "s": "l", "e": "r"}[m[3]]
	return []string{tok, m[1] + m[2] + alt + "-" + m[4]}
}

var reTwin = regexp.MustCompile(`^(-?)(p|m|inset|scroll-p|scroll-m)(l|r|s|e)-(.+)$`)

// shadowsOf: a branch utility is suppressed when a utility from the same
// spacing/inset group is already inline (React's evaluated branch), so the
// :not() uses group-prefix shape, not exact-class shape.
func shadowsOf(tok string) []string {
	m := reShadow.FindStringSubmatch(tok)
	if m == nil {
		return []string{":not(." + cssEscape(tok) + ")"}
	}
	prefix := m[0]
	return []string{`:not([class^="` + prefix + `"])`, `:not([class*=" ` + prefix + `"])`}
}

var reShadow = regexp.MustCompile(`^(-?)(p|m|inset|top|right|bottom|left|start|end)([tblrsexy])?-`)
