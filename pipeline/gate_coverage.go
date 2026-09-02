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
//
// The leading anchor is `(^|\s|:|-)`, not `(^|\s|:)`: Tailwind's compound
// variants (`group-data-[...]:`, `peer-data-[...]:`) put a literal `-`
// directly in front of `data-`/`aria-`, and a component whose state is only
// visible through a sibling/group (label's `group-data-[disabled=true]:`)
// was being scored as stateless — undercounting real coverage.
var (
	reStateNamed  = regexp.MustCompile(`(^|\s|:|-)(data-(open|closed|checked|unchecked|active|selected|disabled|horizontal|vertical|inset|highlighted|empty|pressed)|aria-(expanded|invalid|checked|disabled|pressed|selected|current)|aria-\[[\w-]+=[\w-]+\]):`)
	reStateData   = regexp.MustCompile(`(^|\s|:|-)data-\[([\w-]+)(=[\w-]+)?\]:`)
	reKnownDead   = regexp.MustCompile(`(?s)const KNOWN_DEAD = new Set\(\[(.*?)\]\)`)
	reQuotedIdent = regexp.MustCompile(`^["'](.*)["']$`)
	reExtDepGate  = regexp.MustCompile(`^external dep `)
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

	// noCSS[n]: the component's IR carries zero classes and zero cva entries
	// at all (upstream ships it unstyled — aspect-ratio, avatar and
	// collapsible say so in their own runtime source comments: "zero classes
	// added"). css-import/full-css have no CSS file to diverge by state in
	// that case, so "open" is trivially indistinguishable from "closed" —
	// the same triviality that already lets state=="closed" through
	// unconditionally below, just made to apply to "open" too instead of
	// counting a cell nothing could ever assert as debt.
	stateTokens := map[string]bool{}
	noCSS := map[string]bool{}
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
		joined := strings.Join(parts, " ")
		if hasStateToken(joined) {
			stateTokens[n] = true
		}
		if strings.TrimSpace(joined) == "" {
			noCSS[n] = true
		}
	}

	// oracleExempt[n]: n's canonical example is recorded in exemptions.json
	// with an "external dep" reason (example-oracle.go applies the same
	// match to decide what stays hand-authored). Its demo-inline pages exist
	// but were never rendered by the oracle, so no theme/dir combo beyond
	// the ones a hand-authored page happens to cover can ever get a
	// computed/behavioral assertion — a documented, already-accepted gap,
	// not unexplained debt.
	oracleExempt := map[string]bool{}
	if eb, err := read("src/registry/upstream-snapshot/exemptions.json"); err == nil {
		var ex struct {
			Examples map[string]struct {
				Reason string `json:"reason"`
			} `json:"examples"`
		}
		if json.Unmarshal(eb, &ex) == nil {
			for name, e := range ex.Examples {
				if !reExtDepGate.MatchString(e.Reason) {
					continue
				}
				for _, n := range components {
					if name == n+"-demo" || strings.HasPrefix(name, n+"-") {
						oracleExempt[n] = true
					}
				}
			}
		}
	}

	// noStateAxis: components tiers.json calls non-static (so hasBehavior
	// gives them a state=open cell at all), but whose upstream source has
	// NO CSS keyed on any state attribute — verified by hand against
	// upstream, not merely undetected by hasStateToken, so unlike a
	// detection gap this is not something a future path-parity fix could
	// ever close:
	//   avatar   — upstream/.../ui/avatar.tsx has no data-state anywhere;
	//              its real state (image loaded vs AvatarFallback shown) is
	//              DOM presence, already tested by the avatar contract's
	//              presence probe — invisible to a CSS-only diff by
	//              construction, not by gap.
	//   progress — upstream/.../ui/progress.tsx sets data-state via Radix
	//              but styles ONLY through an inline value-driven
	//              `style={{transform}}`; zero Tailwind class in the
	//              component reads data-state.
	noStateAxis := map[string]bool{"avatar": true, "progress": true}

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
						if path == "demo-inline" && state == "closed" && len(by) == 0 && len(shallow) == 0 && oracleExempt[c] {
							shallow = append(shallow, "example-oracle")
						}
						// A noCSS component's isolated page carries no
						// theme/dir-conditional markup at all (no classes to
						// vary by), so it renders byte-identical across every
						// combo — the canonical light/ltr crawl demo-smoke
						// already did IS the check for the other three.
						if path == "demo-inline" && state == "closed" && len(by) == 0 && len(shallow) == 0 && noCSS[c] {
							shallow = append(shallow, "demo-smoke")
						}
						if path == "css-import" || path == "full-css" {
							if state == "closed" || stateTokens[c] || noCSS[c] || noStateAxis[c] {
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

// gateCoverageExit is the CLI entry for `pipeline coverage --record` — the
// test (TestCoverage) always runs the read-only `--check` path itself, so
// this is the only caller that can ever pass `--record`.
func gateCoverageExit(argv []string) int {
	wd, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, "coverage:", err)
		return 1
	}
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "coverage:", err)
		return 1
	}
	if err := gateCoverage(root, argv); err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	return 0
}

func hasArg(args []string, f string) bool {
	for _, a := range args {
		if a == f {
			return true
		}
	}
	return false
}
