package main

// coverage — the product surface as a matrix, and which gate covers each cell.
//
// Every bug in this repo's history sat in a cell no gate had been written for:
// the dead-button pages (state=open, gate=none), the invisible ghost buttons
// (path=css-import x theme=light, gate=none), the font parity, the opaque
// menus. Gates were added one bug at a time. This enumerates the cells up front
// so "what is still unverified" is a number that can only go down, instead of
// something the next downstream consumer discovers.
//
//	component x path x theme x dir x state
//	  path   demo-inline | css-import | full-css
//	  theme  light | dark
//	  dir    ltr | rtl
//	  state  closed | open       (open only for components with behavior)
//
// A cell is COVERED when some gate makes a computed-style or behavioral
// assertion about it; presence-only checks are recorded as "shallow", not
// covered. The UNCOVERED count is budgeted in gates/ledger.json.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

func init() { gates["coverage"] = func(root string) error { return gateCoverage(root, os.Args[3:]) } }

type covCell struct {
	Component string   `json:"component"`
	Path      string   `json:"path"`
	Theme     string   `json:"theme"`
	Dir       string   `json:"dir"`
	State     string   `json:"state"`
	CoveredBy []string `json:"covered_by"`
	Shallow   []string `json:"shallow"`
}

type irFile struct {
	Components []struct {
		Elements []struct {
			Classes []string `json:"classes"`
		} `json:"elements"`
	} `json:"components"`
	Cva map[string]struct {
		Base     string                       `json:"base"`
		Variants map[string]map[string]string `json:"variants"`
	} `json:"cva"`
}

// The state-token test, kept in the same shape as gates/path-parity.mjs's
// stateConfigs. RE2 has no negative lookahead, so the `data-[...]` arm drops
// the (?!slot=|variant=|size=) and the exclusion is applied to the capture:
// the lookahead fires exactly when the bracket body starts with one of those
// names FOLLOWED BY "=", which is the same as "name is slot/variant/size and a
// value is present".
var (
	reStateNamed  = regexp.MustCompile(`(^|\s|:)(data-(open|closed|checked|unchecked|active|selected|disabled|horizontal|vertical|inset|highlighted|empty|pressed)|aria-(expanded|invalid|checked|disabled|pressed|selected|current)|aria-\[[\w-]+=[\w-]+\]):`)
	reStateData   = regexp.MustCompile(`(^|\s|:)data-\[([\w-]+)(=[\w-]+)?\]:`)
	reKnownDead   = regexp.MustCompile(`(?s)const KNOWN_DEAD = new Set\(\[(.*?)\]\)`)
	reQuotedIdent = regexp.MustCompile(`^["'](.*)["']$`)
)

func hasStateToken(all string) bool {
	if reStateNamed.MatchString(all) {
		return true
	}
	for _, m := range reStateData.FindAllStringSubmatch(all, -1) {
		name, value := m[2], m[3]
		if value != "" && (name == "slot" || name == "variant" || name == "size") {
			continue
		}
		return true
	}
	return false
}

func gateCoverage(root string, argv []string) error {
	has := func(f string) bool { return hasArg(argv, f) }
	read := func(p string) ([]byte, error) { return os.ReadFile(filepath.Join(root, p)) }

	tiersRaw, err := read("src/registry/tiers.json")
	if err != nil {
		return fmt.Errorf("FAIL  coverage: %v", err)
	}
	var tiers map[string]struct {
		Tier string `json:"tier"`
	}
	if err := json.Unmarshal(tiersRaw, &tiers); err != nil {
		return fmt.Errorf("FAIL  coverage: tiers.json: %v", err)
	}

	var components []string
	for n, t := range tiers {
		if t.Tier == "external" || t.Tier == "logic" {
			continue
		}
		if _, err := os.Stat(filepath.Join(root, "src/registry/ir", n+".json")); err == nil {
			components = append(components, n)
		}
	}
	sort.Strings(components)

	irOf := func(n string) irFile {
		var j irFile
		b, _ := read("src/registry/ir/" + n + ".json")
		_ = json.Unmarshal(b, &j)
		return j
	}

	contractDefs := map[string]bool{}
	if entries, err := os.ReadDir(filepath.Join(root, "tools/contracts/components")); err == nil {
		for _, e := range entries {
			if strings.HasSuffix(e.Name(), ".mjs") {
				contractDefs[strings.TrimSuffix(strings.TrimSuffix(e.Name(), ".mjs"), "-multiple")] = true
			}
		}
	}

	var oracleTargets []struct {
		Name string `json:"name"`
	}
	ob, _ := read("docs/example-oracle.json")
	_ = json.Unmarshal(ob, &oracleTargets)
	oracleDemoOf := func(n string) bool {
		for _, t := range oracleTargets {
			if t.Name == n+"-demo" || strings.HasPrefix(t.Name, n+"-") {
				return true
			}
		}
		return false
	}
	rtlDemoOf := func(n string) bool {
		_, err := os.Stat(filepath.Join(root, "docs/demos", n+"-rtl.html"))
		return err == nil
	}
	hasBehavior := func(n string) bool { return tiers[n].Tier != "static" }

	stateTokens := map[string]bool{}
	for _, n := range components {
		j := irOf(n)
		var parts []string
		for _, c := range j.Components {
			for _, e := range c.Elements {
				parts = append(parts, e.Classes...)
			}
		}
		cvaKeys := make([]string, 0, len(j.Cva))
		for k := range j.Cva {
			cvaKeys = append(cvaKeys, k)
		}
		sort.Strings(cvaKeys)
		for _, k := range cvaKeys {
			t := j.Cva[k]
			parts = append(parts, t.Base)
			vKeys := make([]string, 0, len(t.Variants))
			for vk := range t.Variants {
				vKeys = append(vKeys, vk)
			}
			sort.Strings(vKeys)
			for _, vk := range vKeys {
				vv := t.Variants[vk]
				iKeys := make([]string, 0, len(vv))
				for ik := range vv {
					iKeys = append(iKeys, ik)
				}
				sort.Strings(iKeys)
				for _, ik := range iKeys {
					parts = append(parts, vv[ik])
				}
			}
		}
		if hasStateToken(strings.Join(parts, " ")) {
			stateTokens[n] = true
		}
	}

	knownDead := map[string]bool{}
	if sweep, err := read("tools/interactivity-sweep.mjs"); err == nil {
		if m := reKnownDead.FindSubmatch(sweep); m != nil {
			for _, s := range strings.Split(string(m[1]), ",") {
				s = strings.TrimSpace(s)
				if q := reQuotedIdent.FindStringSubmatch(s); q != nil {
					if q[1] != "" {
						knownDead[q[1]] = true
					}
				}
			}
		}
	}

	paths := []string{"demo-inline", "css-import", "full-css"}
	themes := []string{"light", "dark"}
	dirs := []string{"ltr", "rtl"}

	var cells []covCell
	for _, c := range components {
		for _, path := range paths {
			for _, theme := range themes {
				for _, dir := range dirs {
					states := []string{"closed"}
					if hasBehavior(c) {
						states = []string{"closed", "open"}
					}
					for _, state := range states {
						by, shallow := []string{}, []string{}
						if path == "demo-inline" {
							if theme == "light" && dir == "ltr" {
								if oracleDemoOf(c) {
									by = append(by, "example-gate", "golden-gate")
								}
								if contractDefs[c] {
									by = append(by, "contracts")
								}
								if state == "open" && hasBehavior(c) && !knownDead[c] && oracleDemoOf(c) {
									by = append(by, "interactivity-sweep")
								}
								if state == "closed" {
									shallow = append(shallow, "demo-smoke")
								}
							}
							if contractDefs[c] {
								by = append(by, "style-parity")
							}
							if state == "closed" && oracleDemoOf(c) {
								by = append(by, "demo-parity")
							}
						}
						if path == "demo-inline" && theme == "light" && dir == "rtl" && state == "closed" && rtlDemoOf(c) {
							shallow = append(shallow, "docs-smoke", "css-direction")
						}
						if path == "css-import" || path == "full-css" {
							if state == "closed" || stateTokens[c] {
								by = append(by, "path-parity")
							}
						}
						cells = append(cells, covCell{c, path, theme, dir, state, by, shallow})
					}
				}
			}
		}
	}

	var covered, shallowOnly, uncovered []covCell
	for _, x := range cells {
		switch {
		case len(x.CoveredBy) > 0:
			covered = append(covered, x)
		case len(x.Shallow) > 0:
			shallowOnly = append(shallowOnly, x)
		default:
			uncovered = append(uncovered, x)
		}
	}
	byDim := func(get func(covCell) string) string {
		m := map[string]int{}
		var order []string
		for _, x := range uncovered {
			k := get(x)
			if _, ok := m[k]; !ok {
				order = append(order, k)
			}
			m[k]++
		}
		sort.SliceStable(order, func(i, j int) bool { return m[order[i]] > m[order[j]] })
		out := make([]string, 0, len(order))
		for _, k := range order {
			out = append(out, fmt.Sprintf("%s=%d", k, m[k]))
		}
		return strings.Join(out, "  ")
	}

	if err := os.MkdirAll(filepath.Join(root, "build/gates"), 0o755); err != nil {
		return err
	}
	report := struct {
		Total     int       `json:"total"`
		Covered   int       `json:"covered"`
		Shallow   int       `json:"shallow"`
		Uncovered int       `json:"uncovered"`
		Cells     []covCell `json:"cells"`
	}{len(cells), len(covered), len(shallowOnly), len(uncovered), cells}
	rb, err := json.MarshalIndent(report, "", " ")
	if err != nil {
		return err
	}
	if err := os.WriteFile(filepath.Join(root, "build/gates/coverage.json"), rb, 0o644); err != nil {
		return err
	}

	if has("--cells") {
		for _, x := range uncovered {
			fmt.Printf("%s %s %s %s %s\n", x.Component, x.Path, x.Theme, x.Dir, x.State)
		}
	}
	fmt.Printf("coverage: %d cells over %d components — %d covered (computed/behavioral), %d shallow (presence only), %d UNCOVERED\n",
		len(cells), len(components), len(covered), len(shallowOnly), len(uncovered))
	fmt.Printf("  uncovered by path:  %s\n", byDim(func(x covCell) string { return x.Path }))
	fmt.Printf("  uncovered by theme: %s\n", byDim(func(x covCell) string { return x.Theme }))
	fmt.Printf("  uncovered by dir:   %s\n", byDim(func(x covCell) string { return x.Dir }))
	fmt.Printf("  uncovered by state: %s\n", byDim(func(x covCell) string { return x.State }))
	fmt.Printf("  detail: build/gates/coverage.json  (--cells lists them)\n")

	return coverageBudget(root, len(uncovered), len(covered), has("--record"), has("--check"))
}

func hasArg(args []string, f string) bool {
	for _, a := range args {
		if a == f {
			return true
		}
	}
	return false
}
