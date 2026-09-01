package main

// interactivity-sweep, ported from tools/interactivity-sweep.mjs — every
// demo page that OFFERS an interaction must RESPOND to one. Born from the
// dead-button bug: kernel examples shipped as static oracle snapshots and
// nobody's gate asked "does the page respond".

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

var (
	reSweepRtlPage  = regexp.MustCompile(`-rtl-(en|he|fa)\.html$`)
	reSweepCand     = regexp.MustCompile(`data-slot="[^"]*-trigger"|aria-expanded=|role="(switch|checkbox|tab)"|data-slot="(carousel-next|carousel-prev)"`)
	reSweepFam      = regexp.MustCompile(`^(alert-dialog|navigation-menu|context-menu|dropdown-menu|hover-card|button-group|message-scroller|input-group|native-select|radio-group|scroll-area|toggle-group|carousel|accordion|attachment|avatar|breadcrumb|bubble|collapsible|checkbox|combobox|dialog|drawer|field|input|item|kbd|label|marker|menubar|message|pagination|popover|progress|select|sheet|slider|switch|table|tabs|toggle|tooltip)-`)
	reSweepRtlFam   = regexp.MustCompile(`-rtl(-|$).*`)
	reSweepHoverTrg = regexp.MustCompile(`^(tooltip|hover-card)-trigger$`)
	reSweepTrgSuf   = regexp.MustCompile(`-(trigger|item|next|prev)$`)
)

// static oracle snapshots pending family migration — keyed by family
var sweepKnownDead = map[string]bool{
	"message-scroller": true, // hand-authored pages (oracle cannot bundle the example)
}

const sweepCandidatesSel = `[data-slot$="-trigger"], [aria-expanded], [role="switch"], [role="checkbox"], [role="tab"], [data-slot="carousel-next"], [data-slot="carousel-prev"]`

// per ELEMENT (index), not a sorted multiset: switching tabs keeps the set
// {active, inactive, inactive} identical and read as "nothing responded"
const sweepFingerprint = `JSON.stringify({
  states: [...document.querySelectorAll("[data-state]")].map((e, i) => i + ":" + e.getAttribute("data-slot") + ":" + e.getAttribute("data-state")),
  expanded: [...document.querySelectorAll("[aria-expanded]")].map(e => (e.id || e.getAttribute("data-slot")) + ":" + e.getAttribute("aria-expanded")).sort(),
  checked: [...document.querySelectorAll("[aria-checked]")].map(e => (e.id || e.getAttribute("data-slot")) + ":" + e.getAttribute("aria-checked")).sort(),
  disabled: [...document.querySelectorAll("button, input")].map((e, i) => i + ":" + e.disabled),
  kids: document.body.children.length,
})`

func sweepFamilyOf(name string) string {
	if m := reSweepFam.FindStringSubmatch(name); m != nil {
		return m[1]
	}
	return reSweepRtlFam.ReplaceAllString(name, "")
}

func runInteractivitySweep() int {
	site := "docs/public/demos"
	tiersB, err := os.ReadFile("src/registry/tiers.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "interactivity-sweep:", err)
		return 1
	}
	var tiers map[string]struct {
		Tier string `json:"tier"`
	}
	json.Unmarshal(tiersB, &tiers)
	var staticFamilies []string
	for c, t := range tiers {
		if t.Tier == "static" {
			staticFamilies = append(staticFamilies, c)
		}
	}
	sort.Strings(staticFamilies)
	hoverFamilies := map[string]bool{"tooltip": true, "hover-card": true}

	ents, err := os.ReadDir(site)
	if err != nil {
		fmt.Fprintln(os.Stderr, "interactivity-sweep:", err)
		return 1
	}
	var pages, candidates []string
	for _, e := range ents {
		if !strings.HasSuffix(e.Name(), ".html") || reSweepRtlPage.MatchString(e.Name()) {
			continue
		}
		pages = append(pages, e.Name())
		b, _ := os.ReadFile(filepath.Join(site, e.Name()))
		if reSweepCand.MatchString(string(b)) {
			candidates = append(candidates, e.Name())
		}
	}

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "interactivity-sweep:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "interactivity-sweep:", err)
		return 1
	}
	page, _ := shell.newPage(false)

	verified := 0
	staticPages := len(pages) - len(candidates)
	var failures []string
	deadCount := map[string]int{}

	// usable-candidate predicate (runs per element with the static families)
	usableExpr := `(e, statics) => !e.disabled && e.getAttribute("aria-disabled") !== "true" && !e.closest("[hidden]") && e.getClientRects().length > 0 &&
        !(e.getAttribute("role") === "tab" && e.getAttribute("data-state") === "active") &&
        !statics.some((s) => (e.getAttribute("data-slot") || "") === s + "-trigger")`
	ownFamExpr := `(e) => (e.getAttribute("data-slot") || "").replace(/-(trigger|item|next|prev)$/, "")`
	hoverElExpr := `(e) => /^(tooltip|hover-card)-trigger$/.test(e.getAttribute("data-slot") || "")`
	ctxTrgExpr := `(e) => e.getAttribute("data-slot") === "context-menu-trigger"`

	for _, f := range candidates {
		name := strings.TrimSuffix(f, ".html")
		fam := sweepFamilyOf(name)
		if sweepKnownDead[fam] {
			deadCount[fam]++
			continue
		}
		abs, _ := filepath.Abs(filepath.Join(site, f))
		if err := page.gotoURL("file://" + abs); err != nil {
			failures = append(failures, name+": "+firstLine(err.Error()))
			continue
		}
		page.waitForTimeout(350)
		// evaluate candidates in one pass: usable flags via per-element fn
		nCand := 0
		if v, err := page.evaluate(`document.querySelectorAll(` + "`" + sweepCandidatesSel + "`" + `).length`); err == nil {
			nCand = int(v.(float64))
		}
		// per-element predicates, one locEvalAll each (index-stable)
		usableV, err := page.locEvalAllArg("", sweepCandidatesSel, usableExpr, staticFamilies)
		if err != nil {
			failures = append(failures, name+": "+firstLine(err.Error()))
			continue
		}
		ownV, _ := page.locEvalAll("", sweepCandidatesSel, ownFamExpr)
		hoverV, _ := page.locEvalAll("", sweepCandidatesSel, hoverElExpr)
		ctxV, _ := page.locEvalAll("", sweepCandidatesSel, ctxTrgExpr)
		var usableIdx []int
		for i := 0; i < nCand; i++ {
			if usableV != nil {
				if arr, ok := usableV.([]any); ok && i < len(arr) && arr[i] == true {
					usableIdx = append(usableIdx, i)
				}
			}
			if len(usableIdx) == 3 {
				break
			}
		}
		if len(usableIdx) == 0 {
			staticPages++
			continue
		}
		// the element's OWN family decides deadness
		ownFam := ""
		if arr, ok := ownV.([]any); ok && usableIdx[0] < len(arr) {
			ownFam, _ = arr[usableIdx[0]].(string)
		}
		if ownFam != "" && sweepKnownDead[ownFam] && ownFam != fam {
			deadCount[ownFam]++
			continue
		}
		responded := false
		for _, idx := range usableIdx {
			beforeV, _ := page.evaluate(sweepFingerprint)
			before, _ := beforeV.(string)
			hoverEl := false
			if arr, ok := hoverV.([]any); ok && idx < len(arr) {
				hoverEl = arr[idx] == true
			}
			isCtx := false
			if arr, ok := ctxV.([]any); ok && idx < len(arr) {
				isCtx = arr[idx] == true
			}
			if hoverFamilies[fam] || hoverEl {
				if box, _ := page.locBox("", sweepCandidatesSel, idx); box != nil {
					page.mouseMove(box.X+box.Width/2, box.Y+box.Height/2, 5)
				}
			} else if fam == "context-menu" || isCtx {
				page.locClick("", sweepCandidatesSel, idx, "right")
			} else {
				page.locClick("", sweepCandidatesSel, idx, "left")
			}
			// hover families open after radix's delay (tooltip/hover-card
			// 700ms provider default) — wait past it
			if hoverFamilies[fam] || hoverEl {
				page.waitForTimeout(1100)
			} else {
				page.waitForTimeout(600)
			}
			afterV, _ := page.evaluate(sweepFingerprint)
			after, _ := afterV.(string)
			if before != after {
				responded = true
				break
			}
		}
		if !responded {
			failures = append(failures, fmt.Sprintf("%s: interaction offered but nothing responded (%s)", name, fam))
		} else {
			verified++
		}
	}
	deadTotal := 0
	for _, n := range deadCount {
		deadTotal += n
	}
	if len(failures) > 0 {
		n := 12
		if len(failures) < n {
			n = len(failures)
		}
		fmt.Fprintf(os.Stderr, "FAIL  interactivity-sweep\n  %s", strings.Join(failures[:n], "\n  "))
		if len(failures) > 12 {
			fmt.Fprintf(os.Stderr, "\n  … +%d more", len(failures)-12)
		}
		fmt.Fprintln(os.Stderr)
		return 1
	}
	fmt.Printf("PASS  interactivity-sweep (%d pages responded, %d static-by-design, %d known-dead across %d families pending migration — see EXEMPTIONS)\n",
		verified, staticPages, deadTotal, len(deadCount))
	return 0
}
