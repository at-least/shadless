package main

// contract — tools/contracts/run.mjs in Go: record facts from the shadcn
// oracle (React, real browser) and from the shadless page, diff after
// normalizing recorded differences.
//
// oracleOpen/shadlessOpen in result.json are the RAW recorder facts; the
// open-state diff below compares the NORMALIZED (normFact) projection of the
// same two objects. Two independent decodes of the same wire bytes, on
// purpose — rawToJsonable for the byte-identical persisted JSON, buildFact
// for the structured comparison.

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type cdef struct {
	Name           string              `json:"name"`
	Usage          string              `json:"usage"`
	Imports        string              `json:"imports"`
	Slots          []string            `json:"slots"`
	Open           string              `json:"open"`
	OpenShadless   string              `json:"openShadless"`
	MountedClasses *bool               `json:"mountedClasses"`
	MountedCheck   *bool               `json:"mountedCheck"`
	ShadlessPage   string              `json:"shadlessPage"`
	Scenarios      []string            `json:"scenarios"`
	TriggerSlot    string              `json:"triggerSlot"`
	StateProbe     string              `json:"stateProbe"`
	OracleCss      string              `json:"oracleCss"`
	IgnoreAttrs    map[string][]string `json:"ignoreAttrs"`
	CloseSelector  string              `json:"closeSelector"`
	OverlaySlot    string              `json:"overlaySlot"`
	ContentSlot    string              `json:"contentSlot"`
}

func cLoadDef(shell *browserShell, name string) (cdef, error) {
	res, err := shell.call(map[string]any{
		"op":   "loadContractDef",
		"file": "file://" + absOrDie(filepath.Join("tools/contracts/components", name+".mjs")),
	})
	if err != nil {
		return cdef{}, err
	}
	var def cdef
	b, err := json.Marshal(res["def"])
	if err != nil {
		return cdef{}, err
	}
	if err := json.Unmarshal(b, &def); err != nil {
		return cdef{}, err
	}
	return def, nil
}

// ---------- recorder (injected into both pages) -----------------------------

func cRecorderSrc(slots []string) string {
	parts := make([]string, len(slots))
	for i, s := range slots {
		parts[i] = jsonString(s)
	}
	slotsJSON := "[" + strings.Join(parts, ",") + "]"
	return `
window.__facts = function (tag) {
  var doc = document;
  function attrs(el) {
    if (!el) return null;
    var o = { tag: el.tagName.toLowerCase() };
    for (var a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a);
    o.text = (el.textContent || "").trim().replace(/\s+/g, "").slice(0, 24);
    return o;
  }
  var f = { step: tag };
  ` + slotsJSON + `.forEach(function (s) {
    var el = s.charAt(0) === "&"
      ? doc.querySelector(s.slice(1)) // "&<raw-css>" — full selector
      : doc.querySelector("[data-slot=" + s + "]");
    f[s] = attrs(el);
  });
  f.activeElement = doc.activeElement
    ? (doc.activeElement.getAttribute("data-slot") ||
       doc.activeElement.tagName.toLowerCase()) : null;
  f.scrollLock = {
    attr: doc.body.getAttribute("data-scroll-locked"),
    pointerEvents: doc.body.style.pointerEvents,
  };
  return f;
};
`
}

// ---------- shadless page (relative-path rewrite until emitter lands) -------

var reContractAttrPath = regexp.MustCompile(`(src|href)="([^"]+)"`)

func cRewriteRelativePaths(html, dir string) string {
	return reContractAttrPath.ReplaceAllStringFunc(html, func(m string) string {
		sub := reContractAttrPath.FindStringSubmatch(m)
		k, v := sub[1], sub[2]
		if strings.HasPrefix(v, "http:") || strings.HasPrefix(v, "https:") ||
			strings.HasPrefix(v, "file:") || strings.HasPrefix(v, "data:") ||
			strings.HasPrefix(v, "//") {
			return m
		}
		return k + `="` + dir + "/" + v + `"`
	})
}

// ---------- normalization of recorded differences ---------------------------

var (
	reContractAuto        = regexp.MustCompile(`^(radix-[\w:-]*|[a-z]+\d[\w-]*)$`)
	reContractOutlineNone = regexp.MustCompile(`^outline:\s*none$`)
	reContractPointerAuto = regexp.MustCompile(`^pointer-events:\s*auto$`)
	reContractWS          = regexp.MustCompile(`\s+`)
)

// normVal, ported from tools/contracts/run.mjs.
func cNormVal(v, key string) string {
	if key == "style" {
		var kept []string
		for _, s := range strings.Split(v, ";") {
			s = strings.TrimSpace(s)
			if s == "" || strings.HasPrefix(s, "--radix-") ||
				reContractOutlineNone.MatchString(s) || reContractPointerAuto.MatchString(s) {
				continue
			}
			kept = append(kept, s)
		}
		return strings.Join(kept, "; ")
	}
	if reContractAuto.MatchString(v) {
		return "<auto-id>"
	}
	parts := reContractWS.Split(v, -1)
	allAuto := v != ""
	for _, p := range parts {
		if p == "" {
			continue
		}
		if !reContractAuto.MatchString(p) {
			allAuto = false
			break
		}
	}
	if allAuto {
		out := make([]string, len(parts))
		for i := range parts {
			out[i] = "<auto-id>"
		}
		return strings.Join(out, " ")
	}
	return v
}

// cAttrObj is one slot's NORMALIZED element facts — normFact's rebuilt `el`.
type cAttrObj struct {
	keys []string
	val  map[string]string
}

func cIsNullRaw(raw json.RawMessage) bool {
	return bytes.Equal(bytes.TrimSpace(raw), []byte("null"))
}

// buildAttrObj is attrs()+normFact's per-slot rebuild, together: decode the
// wire object and apply normVal in the same pass.
func cBuildAttrObj(raw json.RawMessage) (*cAttrObj, error) {
	if cIsNullRaw(raw) {
		return nil, nil
	}
	obj, err := decodeOrderedObject(raw)
	if err != nil {
		return nil, err
	}
	out := &cAttrObj{val: map[string]string{}}
	for _, k := range obj.keys {
		if k == "data-radix-collection-item" {
			continue
		}
		var s string
		if err := json.Unmarshal(obj.raw[k], &s); err != nil {
			return nil, err
		}
		if k == "tag" {
			out.keys = append(out.keys, k)
			out.val[k] = s
			continue
		}
		nv := cNormVal(s, k)
		if k == "style" && nv == "" {
			continue
		}
		out.keys = append(out.keys, k)
		out.val[k] = nv
	}
	return out, nil
}

// cFact is the NORMALIZED projection of one side's recorder facts (normFact's
// output), keyed for the open-state diff.
type cFact struct {
	slots          map[string]*cAttrObj
	activeElement  *string
	scrollLockAttr *string
	scrollLockPE   string
}

func cBuildFact(raw json.RawMessage, slots []string) (*cFact, error) {
	top, err := decodeOrderedObject(raw)
	if err != nil {
		return nil, err
	}
	f := &cFact{slots: map[string]*cAttrObj{}}
	for _, s := range slots {
		r, ok := top.raw[s]
		if !ok {
			continue
		}
		el, err := cBuildAttrObj(r)
		if err != nil {
			return nil, err
		}
		f.slots[s] = el
	}
	if r, ok := top.raw["activeElement"]; ok && !cIsNullRaw(r) {
		var s string
		if err := json.Unmarshal(r, &s); err != nil {
			return nil, err
		}
		f.activeElement = &s
	}
	if r, ok := top.raw["scrollLock"]; ok {
		sl, err := decodeOrderedObject(r)
		if err != nil {
			return nil, err
		}
		if ar, ok := sl.raw["attr"]; ok && !cIsNullRaw(ar) {
			var s string
			if err := json.Unmarshal(ar, &s); err != nil {
				return nil, err
			}
			f.scrollLockAttr = &s
		}
		if pr, ok := sl.raw["pointerEvents"]; ok {
			json.Unmarshal(pr, &f.scrollLockPE)
		}
	}
	return f, nil
}

// rawToJsonable decodes wire bytes into the jsonorder.go value tree, keeping
// key order — the UNNORMALIZED facts persisted to result.json.
func cRawToJsonable(raw json.RawMessage) (any, error) {
	t := bytes.TrimSpace(raw)
	if len(t) == 0 || string(t) == "null" {
		return jsonNull{}, nil
	}
	if t[0] == '"' {
		var s string
		if err := json.Unmarshal(t, &s); err != nil {
			return nil, err
		}
		return s, nil
	}
	if t[0] == '{' {
		obj, err := decodeOrderedObject(t)
		if err != nil {
			return nil, err
		}
		out := jsonObj{}
		for _, k := range obj.keys {
			v, err := cRawToJsonable(obj.raw[k])
			if err != nil {
				return nil, err
			}
			out = out.add(k, v)
		}
		return out, nil
	}
	return nil, fmt.Errorf("cRawToJsonable: unexpected value %s", t)
}

func cUnionKeysOrdered(a, b *cAttrObj) []string {
	seen := map[string]bool{}
	var out []string
	add := func(el *cAttrObj) {
		if el == nil {
			return
		}
		for _, k := range el.keys {
			if !seen[k] {
				seen[k] = true
				out = append(out, k)
			}
		}
	}
	add(a)
	add(b)
	return out
}

func cLookupAttr(o *cAttrObj, k string) (string, bool) {
	if o == nil {
		return "", false
	}
	v, ok := o.val[k]
	return v, ok
}

func cShowMaybe(present bool, v string) string {
	if !present {
		return "undefined"
	}
	return jsonString(v)
}

// ---------- scenarios --------------------------------------------------------

func cOverlayPoint(page *bpage, overlaySlot, contentSlot string) (float64, float64, error) {
	ov, err := page.locBox("", "[data-slot="+overlaySlot+"]", 0)
	if err != nil {
		return 0, 0, err
	}
	ct, err := page.locBox("", "[data-slot="+contentSlot+"]", 0)
	if err != nil {
		return 0, 0, err
	}
	if ov == nil || ct == nil {
		return 0, 0, fmt.Errorf("overlay-point: element not visible (overlay=%v content=%v)", ov != nil, ct != nil)
	}
	x, y := ov.X+15, ov.Y+15
	inCt := func(px, py float64) bool {
		return px >= ct.X && px <= ct.X+ct.Width && py >= ct.Y && py <= ct.Y+ct.Height
	}
	if inCt(x, y) {
		x, y = ov.X+ov.Width-15, ov.Y+15
	}
	if inCt(x, y) {
		return 0, 0, fmt.Errorf("no overlay-only point")
	}
	return x, y, nil
}

func cOverlaySlotOf(def cdef) string {
	if def.OverlaySlot != "" {
		return def.OverlaySlot
	}
	return "dialog-overlay"
}

func cContentSlotOf(def cdef) string {
	if def.ContentSlot != "" {
		return def.ContentSlot
	}
	return "dialog-content"
}

func cCloseSelectorOf(def cdef) string {
	if def.CloseSelector != "" {
		return def.CloseSelector
	}
	return "[data-slot=dialog-close]"
}

// stepIt: ops chain ("focus:#t1+key:ArrowRight"); legacy single-op names
// still work.
func cStepIt(page *bpage, def cdef, step string) (string, error) {
	for _, op := range strings.Split(step, "+") {
		switch {
		case op == "overlay-mouse-click":
			x, y, err := cOverlayPoint(page, cOverlaySlotOf(def), cContentSlotOf(def))
			if err != nil {
				return "", err
			}
			if err := page.mouseClick(x, y); err != nil {
				return "", err
			}
		case op == "escape":
			if err := page.keyPress("Escape"); err != nil {
				return "", err
			}
		case op == "close-button":
			if err := page.locClickTimeout("", cCloseSelectorOf(def), 0, "left", 30000); err != nil {
				return "", err
			}
		case op == "outside-click":
			if err := page.mouseClick(5, 5); err != nil {
				return "", err
			}
		// steps: real pointers move continuously; a single-jump move races
		// radix's async grace-tracker attach (stays open forever) — artifact,
		// not semantics
		case op == "pointer-away":
			if err := page.mouseMove(5, 5, 10); err != nil {
				return "", err
			}
			if err := page.waitForTimeout(400); err != nil {
				return "", err
			}
		case op == "trigger-toggle":
			if err := page.locClickTimeout("", "[data-slot="+def.TriggerSlot+"]", 0, "left", 30000); err != nil {
				return "", err
			}
		// radix modal dropdown: body pointer-events:none while open — a real
		// playwright click on the trigger never lands; dispatch a DOM click
		case strings.HasPrefix(op, "js-click:"):
			sel := op[len("js-click:"):]
			if _, err := page.locEval("", sel, "(el) => el.click()", 0); err != nil {
				return "", err
			}
		// mouse click at element center — bypasses playwright actionability
		case strings.HasPrefix(op, "mouse-click:"):
			css := op[len("mouse-click:"):]
			box, err := page.locBox("", css, 0)
			if err != nil {
				return "", err
			}
			if box == nil {
				return "", fmt.Errorf("mouse-click: %s not visible", css)
			}
			if err := page.mouseClick(box.X+box.Width/2, box.Y+box.Height/2); err != nil {
				return "", err
			}
		// click at x%,y% inside element box (e.g. track clicks on a slider)
		case strings.HasPrefix(op, "clickAt:"):
			rest := op[len("clickAt:"):]
			parts := strings.SplitN(rest, "@", 2)
			sel := parts[0]
			xy := strings.SplitN(parts[1], ",", 2)
			fx, _ := strconv.ParseFloat(xy[0], 64)
			fy, _ := strconv.ParseFloat(xy[1], 64)
			box, err := page.locBox("", sel, 0)
			if err != nil {
				return "", err
			}
			if box == nil {
				return "", fmt.Errorf("clickAt: %s not visible", sel)
			}
			if err := page.mouseClick(box.X+box.Width*fx/100, box.Y+box.Height*fy/100); err != nil {
				return "", err
			}
		// move pointer to element center (hover state), then optional wheel
		case strings.HasPrefix(op, "move:"):
			css := op[len("move:"):]
			box, err := page.locBox("", css, 0)
			if err != nil {
				return "", err
			}
			if box == nil {
				return "", fmt.Errorf("move: %s not visible", css)
			}
			if err := page.mouseMove(box.X+box.Width/2, box.Y+box.Height/2, 5); err != nil {
				return "", err
			}
		case strings.HasPrefix(op, "wheel:"):
			xy := strings.SplitN(op[len("wheel:"):], ",", 2)
			dx, _ := strconv.ParseFloat(xy[0], 64)
			dy, _ := strconv.ParseFloat(xy[1], 64)
			if err := page.wheel(dx, dy); err != nil {
				return "", err
			}
		// generic steps: click:<css> / focus:<css> / key:<Key>
		case strings.HasPrefix(op, "click:"):
			if err := page.locClickTimeout("", op[len("click:"):], 0, "left", 30000); err != nil {
				return "", err
			}
		case strings.HasPrefix(op, "focus:"):
			if err := page.focus(op[len("focus:"):], 30000); err != nil {
				return "", err
			}
			// radix roving-focus moves via rAF — settle
			if err := page.waitForTimeout(120); err != nil {
				return "", err
			}
		case strings.HasPrefix(op, "key:"):
			if err := page.keyPress(op[len("key:"):]); err != nil {
				return "", err
			}
			if err := page.waitForTimeout(120); err != nil {
				return "", err
			}
		}
	}
	if err := page.waitForTimeout(350); err != nil {
		return "", err
	}
	if def.StateProbe != "" {
		v, err := page.evaluate(def.StateProbe)
		if err != nil {
			return "", err
		}
		s, _ := v.(string)
		return s, nil
	}
	// presence probe: both sides remove the content (incl. portal wrapper)
	// on close — verified per-kernel
	v, err := page.evaluate(fmt.Sprintf(`!document.querySelector("[data-slot=%s]") ? "closes" : "open"`, cContentSlotOf(def)))
	if err != nil {
		return "", err
	}
	s, _ := v.(string)
	return s, nil
}

// ---------- run both sides ---------------------------------------------------

func cMountedBagSrc(withClasses bool) string {
	wc := "false"
	if withClasses {
		wc = "true"
	}
	return `(() => {
  const bag = []
  for (const el of document.body.querySelectorAll("*")) {
    if (el.tagName === "SCRIPT" || el.tagName === "TEMPLATE") continue
    const cls = ` + wc + ` ? (el.getAttribute("class") || "").split(/\s+/).filter(Boolean).sort().join(" ") : ""
    bag.push(el.tagName + "|" + (el.getAttribute("data-slot") || "") + "|" + cls)
  }
  bag.sort()
  return bag
})()`
}

func cToStringSlice(v any) []string {
	arr, _ := v.([]any)
	out := make([]string, 0, len(arr))
	for _, e := range arr {
		if s, ok := e.(string); ok {
			out = append(out, s)
		}
	}
	return out
}

func cBagDiff(before, after []string) []string {
	remaining := append([]string{}, before...)
	var added []string
	for _, item := range after {
		idx := -1
		for i, r := range remaining {
			if r == item {
				idx = i
				break
			}
		}
		if idx >= 0 {
			remaining = append(remaining[:idx], remaining[idx+1:]...)
		} else {
			added = append(added, item)
		}
	}
	return added
}

func cMountedDiff(page *bpage, withClasses bool, driverCode string) ([]string, error) {
	beforeV, err := page.evaluate(cMountedBagSrc(withClasses))
	if err != nil {
		return nil, err
	}
	before := cToStringSlice(beforeV)
	if err := page.driver(driverCode); err != nil {
		return nil, err
	}
	if err := page.waitForTimeout(400); err != nil {
		return nil, err
	}
	afterV, err := page.evaluate(cMountedBagSrc(withClasses))
	if err != nil {
		return nil, err
	}
	after := cToStringSlice(afterV)
	return cBagDiff(before, after), nil
}

type cRunResult struct {
	result    string
	hasResult bool
	factRaw   json.RawMessage
	mounted   []string
	mountedOK bool
}

func cMountedClassesOf(def cdef) bool {
	return def.MountedClasses == nil || *def.MountedClasses
}

func cMountedCheckOf(def cdef) bool {
	return def.MountedCheck == nil || *def.MountedCheck
}

func cOracleRun(shell *browserShell, def cdef, out, step string) (cRunResult, error) {
	page, err := shell.newPage(false)
	if err != nil {
		return cRunResult{}, err
	}
	defer page.close()
	if err := page.gotoURL("file://" + absOrDie(filepath.Join(out, "oracle.html"))); err != nil {
		return cRunResult{}, err
	}
	if err := page.waitForTimeout(500); err != nil {
		return cRunResult{}, err
	}
	var mounted []string
	mountedOK := false
	if def.Open != "" {
		if step == "" {
			m, err := cMountedDiff(page, cMountedClassesOf(def), def.Open)
			if err != nil {
				return cRunResult{}, err
			}
			mounted, mountedOK = m, true
		} else {
			if err := page.driver(def.Open); err != nil {
				return cRunResult{}, err
			}
			if err := page.waitForTimeout(400); err != nil {
				return cRunResult{}, err
			}
		}
	}
	var result string
	hasResult := step != ""
	if hasResult {
		result, err = cStepIt(page, def, step)
		if err != nil {
			return cRunResult{}, err
		}
	}
	factRaw, err := page.evaluateOrdered(`window.__facts("oracle")`)
	if err != nil {
		return cRunResult{}, err
	}
	return cRunResult{result: result, hasResult: hasResult, factRaw: factRaw, mounted: mounted, mountedOK: mountedOK}, nil
}

func cShadlessRun(shell *browserShell, def cdef, out, step, recorder string) (cRunResult, []string, error) {
	page, err := shell.newPageErrorsOnly()
	if err != nil {
		return cRunResult{}, nil, err
	}
	defer page.close()
	if err := page.gotoURL("file://" + absOrDie(filepath.Join(out, "shadless.html"))); err != nil {
		return cRunResult{}, nil, err
	}
	if err := page.addScriptTag(recorder); err != nil {
		return cRunResult{}, nil, err
	}
	if err := page.waitForTimeout(400); err != nil {
		return cRunResult{}, nil, err
	}
	var mounted []string
	mountedOK := false
	if def.OpenShadless != "" {
		if step == "" {
			m, err := cMountedDiff(page, cMountedClassesOf(def), def.OpenShadless)
			if err != nil {
				return cRunResult{}, nil, err
			}
			mounted, mountedOK = m, true
		} else if err := page.driver(def.OpenShadless); err != nil { // def may use await
			return cRunResult{}, nil, err
		}
	}
	if err := page.waitForTimeout(300); err != nil {
		return cRunResult{}, nil, err
	}
	var result string
	hasResult := step != ""
	if hasResult {
		result, err = cStepIt(page, def, step)
		if err != nil {
			return cRunResult{}, nil, err
		}
	}
	factRaw, err := page.evaluateOrdered(`window.__facts("shadless")`)
	if err != nil {
		return cRunResult{}, nil, err
	}
	errs, _ := page.events()
	return cRunResult{result: result, hasResult: hasResult, factRaw: factRaw, mounted: mounted, mountedOK: mountedOK}, errs, nil
}

// ---------- entry point ------------------------------------------------------

func runContract(name string) int {
	OUT := filepath.Join("tools/contracts/out", name)

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	defer shell.close()

	def, err := cLoadDef(shell, name)
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	recorder := cRecorderSrc(def.Slots)

	if err := buildContractOracleGo(efDef{Imports: def.Imports, Usage: def.Usage, OracleCss: def.OracleCss}, OUT, recorder); err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}

	shadlessHTML, err := os.ReadFile(def.ShadlessPage)
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	dir := absOrDie(filepath.Dir(def.ShadlessPage))
	if err := os.WriteFile(filepath.Join(OUT, "shadless.html"), []byte(cRewriteRelativePaths(string(shadlessHTML), dir)), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}

	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}

	var oracleOpenRaw, shadlessOpenRaw json.RawMessage
	var oracleOpen, shadlessOpen *cFact
	var oracleMounted, shadlessMounted []string
	var oracleMountedOK, shadlessMountedOK bool
	oracleS := map[string]string{}
	shadlessS := map[string]string{}
	scenarioRan := map[string]bool{}
	var flaky []string

	steps := append([]string{""}, def.Scenarios...)
	for _, step := range steps {
		o, err := cOracleRun(shell, def, OUT, step)
		if err != nil {
			fmt.Fprintln(os.Stderr, "contracts:", err)
			return 1
		}
		c, errs, err := cShadlessRun(shell, def, OUT, step, recorder)
		if err != nil {
			fmt.Fprintln(os.Stderr, "contracts:", err)
			return 1
		}
		for _, e := range errs {
			fmt.Println("  [shadless pageerror]", e)
		}
		if step == "" {
			oracleOpenRaw, shadlessOpenRaw = o.factRaw, c.factRaw
			oracleMounted, oracleMountedOK = o.mounted, o.mountedOK
			shadlessMounted, shadlessMountedOK = c.mounted, c.mountedOK
			continue
		}
		if o.result != c.result {
			// A real behavioral difference reproduces; a timing race in
			// either browser page does not. Re-run BOTH sides once from a
			// fresh page. If they now agree, record the agreed value and
			// say so.
			o2, err := cOracleRun(shell, def, OUT, step)
			if err != nil {
				fmt.Fprintln(os.Stderr, "contracts:", err)
				return 1
			}
			c2, errs2, err := cShadlessRun(shell, def, OUT, step, recorder)
			if err != nil {
				fmt.Fprintln(os.Stderr, "contracts:", err)
				return 1
			}
			for _, e := range errs2 {
				fmt.Println("  [shadless pageerror]", e)
			}
			if o2.result == c2.result {
				flaky = append(flaky, fmt.Sprintf("%s: first run oracle=%s shadless=%s", step, o.result, c.result))
			}
			o, c = o2, c2
		}
		oracleS[step] = o.result
		shadlessS[step] = c.result
		scenarioRan[step] = true
	}
	if len(flaky) > 0 {
		fmt.Printf("contracts[%s]: %d scenario(s) agreed only on re-run (timing flake, not a diff)\n    %s\n",
			name, len(flaky), strings.Join(flaky, "\n    "))
	}

	oracleOpen, err = cBuildFact(oracleOpenRaw, def.Slots)
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	shadlessOpen, err = cBuildFact(shadlessOpenRaw, def.Slots)
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}

	// ---------- diff ----------------------------------------------------------
	pass := true
	fmt.Printf("contracts[%s]: open-state facts\n", name)
	for _, k := range def.Slots {
		aEl, bEl := oracleOpen.slots[k], shadlessOpen.slots[k]
		keys := cUnionKeysOrdered(aEl, bEl)
		ign := map[string]bool{"tag": true}
		for _, x := range def.IgnoreAttrs[k] {
			ign[x] = true
		}
		var diffs []string
		for _, kk := range keys {
			if strings.HasPrefix(kk, "data-radixuigo-") || strings.HasPrefix(kk, "data-radix-popper-") {
				continue // kernel glue protocol internals (positioning feedback on the anchor)
			}
			if kk == "data-radix-menu-content" { // radix internals marker
				continue
			}
			if ign[kk] {
				continue
			}
			av, aOk := cLookupAttr(aEl, kk)
			bv, bOk := cLookupAttr(bEl, kk)
			aCmp, bCmp := "<absent>", "<absent>"
			if aOk {
				aCmp = av
			}
			if bOk {
				bCmp = bv
			}
			if aCmp != bCmp {
				diffs = append(diffs, fmt.Sprintf("    %s: oracle=%s shadless=%s", kk, cShowMaybe(aOk, av), cShowMaybe(bOk, bv)))
			}
		}
		status := "match"
		if len(diffs) > 0 {
			status = "DIFF"
			pass = false
		}
		fmt.Printf("  %s: %s\n", k, status)
		for _, d := range diffs {
			fmt.Println(d)
		}
	}
	aeShow := func(f *cFact) string {
		if f.activeElement == nil {
			return "null"
		}
		return jsonString(*f.activeElement)
	}
	slShow := func(f *cFact) string {
		attr := "null"
		if f.scrollLockAttr != nil {
			attr = jsonString(*f.scrollLockAttr)
		}
		return `{"attr":` + attr + `,"pointerEvents":` + jsonString(f.scrollLockPE) + `}`
	}
	for _, k := range []string{"activeElement", "scrollLock"} {
		var aStr, bStr string
		if k == "activeElement" {
			aStr, bStr = aeShow(oracleOpen), aeShow(shadlessOpen)
		} else {
			aStr, bStr = slShow(oracleOpen), slShow(shadlessOpen)
		}
		if aStr == bStr {
			fmt.Printf("  %s: match\n", k)
		} else {
			fmt.Printf("  %s: DIFF oracle=%s shadless=%s\n", k, aStr, bStr)
			pass = false
		}
	}

	fmt.Printf("contracts[%s]: scenarios\n", name)
	for _, s := range def.Scenarios {
		label, oR, sR := s, oracleS[s], shadlessS[s]
		if !scenarioRan[s] {
			// A genuine array hole (an accidental extra comma in a def's
			// scenarios list, e.g. toggle-group.mjs): JS's spread/for-of
			// visits it as literal `undefined` — falsy, so the run loop
			// skips it like the null step, and the label and both values
			// print JS's stringification of undefined rather than being
			// omitted.
			label, oR, sR = "undefined", "undefined", "undefined"
		}
		same := oR == sR
		stat := ""
		if !same {
			stat = "DIFF"
			pass = false
		}
		fmt.Printf("  %s: oracle=%s shadless=%s %s\n", label, oR, sR, stat)
	}

	// mounted-diff structural check: the JS-created DOM must match too —
	// class drift inside portaled/mounted content had NO guard before
	if (oracleMountedOK || shadlessMountedOK) && cMountedCheckOf(def) {
		fmt.Printf("contracts[%s]: mounted DOM\n", name)
		om := append([]string{}, oracleMounted...)
		var onlyShadless []string
		for _, x := range shadlessMounted {
			idx := -1
			for i, r := range om {
				if r == x {
					idx = i
					break
				}
			}
			if idx >= 0 {
				om = append(om[:idx], om[idx+1:]...)
			} else {
				onlyShadless = append(onlyShadless, x)
			}
		}
		if len(om) == 0 && len(onlyShadless) == 0 {
			fmt.Printf("  %d mounted elements match\n", len(oracleMounted))
		} else {
			pass = false
			for i, x := range om {
				if i >= 4 {
					break
				}
				fmt.Printf("  only-oracle:   %s\n", cTruncate(x, 160))
			}
			for i, x := range onlyShadless {
				if i >= 4 {
					break
				}
				fmt.Printf("  only-shadless: %s\n", cTruncate(x, 160))
			}
		}
	}

	oracleOpenJSON, err := cRawToJsonable(oracleOpenRaw)
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	shadlessOpenJSON, err := cRawToJsonable(shadlessOpenRaw)
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	oracleObj, shadlessObj := jsonObj{}, jsonObj{}
	for _, s := range def.Scenarios {
		if !scenarioRan[s] { // array hole — JS's oracleS/shadlessS never gained this key
			continue
		}
		oracleObj = oracleObj.add(s, oracleS[s])
		shadlessObj = shadlessObj.add(s, shadlessS[s])
	}
	result := jsonObj{}.
		add("oracleOpen", oracleOpenJSON).
		add("shadlessOpen", shadlessOpenJSON).
		add("oracle", oracleObj).
		add("shadless", shadlessObj).
		add("pass", pass)
	if err := os.WriteFile(filepath.Join(OUT, "result.json"), []byte(marshalJS(result, "")), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}

	if pass {
		fmt.Printf("\nPASS  contracts %s\n", name)
		return 0
	}
	fmt.Printf("\nFAIL  contracts %s\n", name)
	return 1
}

func cTruncate(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n]
}

// runContractsAll: no arg — run every component def in a child process each;
// exit 1 if any FAILs.
func runContractsAll() int {
	ents, err := os.ReadDir("tools/contracts/components")
	if err != nil {
		fmt.Fprintln(os.Stderr, "contracts:", err)
		return 1
	}
	var names []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".mjs") {
			names = append(names, strings.TrimSuffix(e.Name(), ".mjs"))
		}
	}
	sort.Strings(names)

	self, err := os.Executable()
	if err != nil {
		self = os.Args[0]
	}
	failed := 0
	for _, c := range names {
		fmt.Printf("\n=== %s ===\n", c)
		cmd := exec.Command(self, "contract", c)
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		if err := cmd.Run(); err != nil {
			failed++
		}
	}
	if failed > 0 {
		fmt.Printf("\nFAIL  contracts full-run (%d/%d failed)\n", failed, len(names))
		return 1
	}
	fmt.Printf("\nPASS  contracts full-run (%d components)\n", len(names))
	return 0
}
