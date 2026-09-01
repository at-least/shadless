package main

// overlay — every manual intervention on top of the mechanical conversion,
// audited against the upstream it was written for. Ported from
// gates/overlay.mjs.
//
// Three kinds of unit:
//
//	rule      a table entry in the pipeline (DEFAULT_CONTENT,
//	          TEXT_ADJUSTMENTS, DEAD_UTILITIES, SKIN_ALLOWLIST, KNOWN_ICONS,
//	          tier sets, the Persian dictionary, contract ignoreAttrs).
//	          Precondition is a structural predicate on the IR / upstream
//	          tree.
//	authored  a whole hand-written file (kernel behavior files, the runtime,
//	          hand-authored demos). Anchored to the sha256 of the upstream
//	          inputs it was written against, recorded in overlays/manifest
//	          .json. Input changed ⇒ stale.
//	source    a git patch on the upstream tree itself, under
//	          overlays/upstream/*.patch. Conflict ⇒ conflict bucket.
//
// Each unit must re-prove on every re-pin:
//
//	applies    the thing it attaches to still exists  (requires)
//	needed     removing it would change the output    (dissolve check)
//	effective  it actually did what it claims         (the other gates)
//
//   pipeline overlay --audit     the gate: exit 1 on any orphaned / stale /
//                                conflict / unclassified
//   pipeline overlay --record    re-anchor every authored unit (after review)
//   pipeline overlay --tasks     write one task packet per stale or orphaned
//                                unit to build/gates/tasks/
//   pipeline overlay --report    audit without exiting non-zero

import (
	"crypto/sha256"
	_ "embed"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const (
	ovUP         = ".upstream/shadcn-ui"
	ovREG        = "apps/v4/registry/bases/radix/ui"
	ovExamples   = "apps/v4/examples/radix"
	ovExamplesAr = "apps/v4/examples/aria"
	// The dictionary is read from the file this repo owns rather than from
	// upstream directly — rtl-dict is the one PRODUCER that reads
	// examples/aria, and what it writes is checked against upstream by its
	// own node plus reproducible.
	ovMDX      = "apps/v4/content/docs/components/radix"
	ovManifest = "overlays/manifest.json"
	ovPatches  = "overlays/upstream"
	ovTasksDir = "build/gates/tasks"
)

type ovUnit struct {
	id, kind, home, file string
	inputs, extra        []string
	requires             func() string
	dissolved            func() string
	reason               string
	hash                 string
	recorded             *ovUnitRec
	bucket               string
}

type ovPinFile struct {
	ShadcnUI struct {
		Tag    string `json:"tag"`
		Commit string `json:"commit"`
	} `json:"shadcn_ui"`
	Kernel struct {
		Sha256 string `json:"sha256"`
	} `json:"kernel"`
}

type ovIR struct {
	Components []struct {
		Export   bool   `json:"export"`
		Fn       string `json:"fn"`
		Elements []struct {
			Slot string `json:"slot"`
		} `json:"elements"`
	} `json:"components"`
}

type ovUnitRec struct {
	File   string   `json:"file"`
	Inputs []string `json:"inputs"`
	Extra  []string `json:"extra"`
	Hash   string   `json:"hash"`
	Pin    string   `json:"pin"`
}

type ovManifestFile struct {
	Pin    string               `json:"pin"`
	Commit string               `json:"commit"`
	Units  map[string]ovUnitRec `json:"units"`
}

func ovUp(rel string) string { return ovUP + "/" + rel }

func ovUpExists(rel string) bool {
	_, err := os.Stat(ovUp(rel))
	return err == nil
}

func ovUpRead(rel string) string {
	b, _ := os.ReadFile(ovUp(rel))
	return string(b)
}

func ovIr(name string) *ovIR {
	b, err := os.ReadFile(filepath.Join("src/registry/ir", name+".json"))
	if err != nil {
		return nil
	}
	var ir ovIR
	if json.Unmarshal(b, &ir) != nil {
		return nil
	}
	return &ir
}

func ovRegistryNames() []string {
	ents, err := os.ReadDir(ovUp(ovREG))
	if err != nil {
		return nil
	}
	var out []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".tsx") {
			out = append(out, strings.TrimSuffix(e.Name(), ".tsx"))
		}
	}
	sort.Strings(out)
	return out
}

// ovTierSets — the converter's tables, read from the JS file that still
// owns them (until the converter itself ports); jsSetLiteral preserves the
// literal's order, which runtime:core's hash depends on.
type ovTierSet struct {
	tier  string
	names []string
}

func ovLoadTierSets() ([]ovTierSet, []string, error) {
	src, err := os.ReadFile("src/converter/index.mjs")
	if err != nil {
		return nil, nil, err
	}
	var sets []ovTierSet
	for _, s := range []struct{ tier, name string }{
		{"kernel", "KERNEL"}, {"trivial", "TRIVIAL"}, {"medium", "MEDIUM"},
		{"logic", "LOGIC"}, {"external", "EXPLICIT_EXTERNAL"},
	} {
		names, err := jsSetLiteral(string(src), s.name)
		if err != nil {
			return nil, nil, fmt.Errorf("converter tier set %s: %w", s.name, err)
		}
		sets = append(sets, ovTierSet{s.tier, names})
	}
	icons, err := jsSetLiteral(string(src), "KNOWN_ICONS")
	if err != nil {
		return nil, nil, fmt.Errorf("converter KNOWN_ICONS: %w", err)
	}
	return sets, icons, nil
}

func ovSkinAllowlist() ([]string, error) {
	src, err := os.ReadFile("src/emitter/skin.mjs")
	if err != nil {
		return nil, err
	}
	return jsSetLiteral(string(src), "SKIN_ALLOWLIST")
}

// ---- rule units ----

func ovRuleUnits(pin ovPinFile, tierSets []ovTierSet, icons []string) ([]ovUnit, error) {
	var units []ovUnit
	push := func(u ovUnit) { units = append(units, u) }

	// DEFAULT_CONTENT — (component, fn) keyed example content for static
	// pages; the live table is the Go one (the emit node runs Go)
	var comps []string
	for c := range DEFAULT_CONTENT {
		comps = append(comps, c)
	}
	sort.Strings(comps)
	for _, comp := range comps {
		var fns []string
		for f := range DEFAULT_CONTENT[comp] {
			fns = append(fns, f)
		}
		sort.Strings(fns)
		for _, fn := range fns {
			comp, fn := comp, fn
			push(ovUnit{
				id: "default-content:" + comp + "." + fn, kind: "rule",
				home: "pipeline/default_content.go DEFAULT_CONTENT",
				requires: func() string {
					i := ovIr(comp)
					if i == nil {
						return fmt.Sprintf("component %s has no IR (gone upstream?)", comp)
					}
					for _, c := range i.Components {
						if c.Export && c.Fn == fn {
							return ""
						}
					}
					return fmt.Sprintf("%s no longer exports %s", comp, fn)
				},
				dissolved: func() string { return "" },
			})
		}
	}

	// TEXT_ADJUSTMENTS — prose rewrites anchored to upstream mdx sentences
	for _, adj := range textAdjustments {
		for i, op := range adj.ops {
			adj, op, i := adj, op, i
			push(ovUnit{
				id: fmt.Sprintf("text-adjustment:%s#%d", adj.id, i), kind: "rule",
				home: "pipeline/docs_transforms.go TEXT_ADJUSTMENTS",
				requires: func() string {
					for _, f := range adj.files {
						if !ovUpExists(ovMDX + "/" + f) {
							return fmt.Sprintf("%s missing upstream", f)
						}
						if !strings.Contains(fenceShadow(ovUpRead(ovMDX+"/"+f)), op.find) {
							find := op.find
							if len(find) > 50 {
								find = find[:50]
							}
							return fmt.Sprintf("find string no longer in %s: %q", f, find+"…")
						}
					}
					return ""
				},
				dissolved: func() string { return "" },
			})
		}
	}

	// DEAD_UTILITIES — classes the registry uses but no stylesheet defines
	registryText := func() string {
		var parts []string
		for _, n := range ovRegistryNames() {
			parts = append(parts, ovUpRead(ovREG+"/"+n+".tsx"))
		}
		return strings.Join(parts, "\n")
	}
	// the converter also runs over the examples (oracle bundles)
	sourceText := func() string {
		parts := []string{registryText()}
		for _, d := range []string{ovExamples, ovExamplesAr} {
			ents, err := os.ReadDir(ovUp(d))
			if err != nil {
				continue
			}
			for _, e := range ents {
				if strings.HasSuffix(e.Name(), ".tsx") {
					parts = append(parts, ovUpRead(d+"/"+e.Name()))
				}
			}
		}
		return strings.Join(parts, "\n")
	}
	// the ONE skin this build ingests — other skins defining a class is
	// irrelevant
	upstreamStyles := func() string { return ovUpRead(strings.TrimPrefix(skinPath, ovUP+"/")) }

	var deadKeys []string
	for k := range deadUtilities {
		deadKeys = append(deadKeys, k)
	}
	sort.Strings(deadKeys)
	for _, tok := range deadKeys {
		tok := tok
		push(ovUnit{
			id: "dead-utility:" + tok, kind: "rule",
			home: "pipeline/emitter_css.go DEAD_UTILITIES",
			requires: func() string {
				if strings.Contains(registryText(), tok) {
					return ""
				}
				return fmt.Sprintf("%s no longer referenced by the registry", tok)
			},
			dissolved: func() string {
				re := regexp.MustCompile(`@utility\s+` + regexp.QuoteMeta(tok) + `\b|\.` + regexp.QuoteMeta(tok) + `\s*[{,]`)
				if re.MatchString(upstreamStyles()) {
					return fmt.Sprintf("upstream now defines %s — it is not dead any more", tok)
				}
				return ""
			},
		})
	}

	allow, err := ovSkinAllowlist()
	if err != nil {
		return nil, err
	}
	for _, tok := range allow {
		tok := tok
		push(ovUnit{
			id: "skin-allowlist:" + tok, kind: "rule",
			home: "src/emitter/skin.mjs SKIN_ALLOWLIST",
			requires: func() string {
				if strings.Contains(registryText(), tok) || strings.Contains(upstreamStyles(), tok) {
					return ""
				}
				return fmt.Sprintf("%s appears nowhere upstream", tok)
			},
			dissolved: func() string {
				re := regexp.MustCompile(`@utility\s+` + regexp.QuoteMeta(tok) + `\b|\.` + regexp.QuoteMeta(tok) + `\s*\{`)
				if re.MatchString(upstreamStyles()) {
					return fmt.Sprintf("upstream now emits rules for %s — the allowlist entry would hide them", tok)
				}
				return ""
			},
		})
	}

	// KNOWN_ICONS — icon component names the converter treats as <svg>
	for _, icon := range icons {
		icon := icon
		push(ovUnit{
			id: "known-icon:" + icon, kind: "rule",
			home:     "src/converter/index.mjs KNOWN_ICONS",
			requires: func() string { return "" },
			dissolved: func() string {
				re := regexp.MustCompile(`\b` + regexp.QuoteMeta(icon) + `\b`)
				if re.MatchString(sourceText()) {
					return ""
				}
				return fmt.Sprintf("%s is used by no registry or example file", icon)
			},
		})
	}

	// tier classification — every registry component must be classified,
	// and every classified name must exist. A NEW upstream component with
	// no tier is exactly the re-pin case that silently shipped nothing
	// before.
	classified := map[string]bool{}
	for _, s := range tierSets {
		for _, name := range s.names {
			classified[name] = true
			name := name
			push(ovUnit{
				id: fmt.Sprintf("tier:%s:%s", s.tier, name), kind: "rule",
				home: "src/converter/index.mjs tier sets",
				requires: func() string {
					if ovUpExists(ovREG + "/" + name + ".tsx") {
						return ""
					}
					return fmt.Sprintf("%s.tsx is gone from the registry", name)
				},
				dissolved: func() string { return "" },
			})
		}
	}
	push(ovUnit{
		id: "tier:coverage", kind: "rule",
		home: "src/converter/index.mjs tier sets",
		requires: func() string {
			// static tier is the default (no set); the converter decides
			// static vs needs-runtime from the source. What must not
			// happen: a component that imports a radix primitive and sits
			// in no set. Slot / Direction / VisuallyHidden / Primitive are
			// structural helpers, not behavior.
			inert := map[string]bool{"Slot": true, "Direction": true, "VisuallyHidden": true, "Primitive": true}
			re := regexp.MustCompile(`import\s*\{([^}]*)\}\s*from\s*"(?:radix-ui|@radix-ui/[\w-]+)"`)
			nameRe := regexp.MustCompile(`\s+as\s+`)
			behaviorImport := func(src string) bool {
				for _, m := range re.FindAllStringSubmatch(src, -1) {
					for _, x := range strings.Split(m[1], ",") {
						name := strings.TrimSpace(nameRe.Split(strings.TrimSpace(x), -1)[0])
						if name == "" || inert[name] {
							continue
						}
						return true
					}
				}
				return false
			}
			var unclassified []string
			for _, n := range ovRegistryNames() {
				if !classified[n] && behaviorImport(ovUpRead(ovREG+"/"+n+".tsx")) {
					unclassified = append(unclassified, n)
				}
			}
			if len(unclassified) > 0 {
				return "radix-backed components with no tier: " + strings.Join(unclassified, ", ")
			}
			return ""
		},
		dissolved: func() string { return "" },
	})

	// Persian dictionary — keys must exist in upstream's Arabic dictionary
	{
		src, err := os.ReadFile("pipeline/build_rtl.go")
		if err != nil {
			return nil, err
		}
		re := regexp.MustCompile(`(?m)^\s*"(\w+)":\s+"[^"]*",?$`)
		var keys []string
		for _, m := range re.FindAllStringSubmatch(string(src), -1) {
			if m[1] != "dir" {
				keys = append(keys, m[1])
			}
		}
		push(ovUnit{
			id: "rtl:persian-dictionary", kind: "rule",
			home: "pipeline/build_rtl.go persian",
			requires: func() string {
				// the vendored dictionary, not examples/aria
				var dict map[string]struct {
					Ar struct {
						Values map[string]string `json:"values"`
					} `json:"ar"`
					Fa *struct {
						Values map[string]string `json:"values"`
					} `json:"fa"`
				}
				b, err := os.ReadFile("src/registry/rtl-translations.json")
				if err != nil {
					return "alert-rtl gone from src/registry/rtl-translations.json"
				}
				if json.Unmarshal(b, &dict) != nil || dict["alert-rtl"].Ar.Values == nil {
					return "alert-rtl has no Arabic dictionary"
				}
				var missing []string
				for _, k := range keys {
					if _, ok := dict["alert-rtl"].Ar.Values[k]; !ok {
						missing = append(missing, k)
					}
				}
				if len(missing) > 0 {
					return "Persian keys with no Arabic counterpart upstream: " + strings.Join(missing, ", ")
				}
				return ""
			},
			dissolved: func() string {
				var dict map[string]json.RawMessage
				b, err := os.ReadFile("src/registry/rtl-translations.json")
				if err == nil && json.Unmarshal(b, &dict) == nil {
					var t struct {
						Fa *json.RawMessage `json:"fa"`
					}
					if json.Unmarshal(dict["alert-rtl"], &t) == nil && t.Fa != nil {
						return "upstream now ships a Persian dictionary — use it instead"
					}
				}
				return ""
			},
		})
	}
	return units, nil
}

// ovIgnoreAttrUnits — contract ignoreAttrs: the slot they exempt must
// still exist. The defs load through the browser shell (node only, no
// chromium — loadContractDef never launches).
func ovIgnoreAttrUnits(shell *browserShell) ([]ovUnit, error) {
	ents, err := os.ReadDir("tools/contracts/components")
	if err != nil {
		return nil, err
	}
	var files []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".mjs") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)
	var units []ovUnit
	for _, f := range files {
		name := strings.TrimSuffix(f, ".mjs")
		base := strings.TrimSuffix(name, "-multiple")
		res, err := shell.call(map[string]any{
			"op":   "loadContractDef",
			"file": "file://" + absOrDie(filepath.Join("tools/contracts/components", f)),
		})
		if err != nil {
			return nil, err
		}
		var def struct {
			IgnoreAttrs map[string]any `json:"ignoreAttrs"`
		}
		b, _ := json.Marshal(res["def"])
		if json.Unmarshal(b, &def) != nil {
			continue
		}
		var slots []string
		for s := range def.IgnoreAttrs {
			slots = append(slots, s)
		}
		sort.Strings(slots)
		for _, slot := range slots {
			slot := slot
			units = append(units, ovUnit{
				id: fmt.Sprintf("ignore-attrs:%s:%s", name, slot), kind: "rule",
				home: "tools/contracts/components/" + f,
				requires: func() string {
					i := ovIr(base)
					if i == nil {
						return fmt.Sprintf("%s has no IR", base)
					}
					slotsSet := map[string]bool{}
					for _, c := range i.Components {
						for _, e := range c.Elements {
							if e.Slot != "" {
								slotsSet[e.Slot] = true
							}
						}
					}
					if slotsSet[slot] {
						return ""
					}
					return fmt.Sprintf("slot %s no longer emitted by %s", slot, base)
				},
				dissolved: func() string { return "" },
			})
		}
	}
	return units, nil
}

// ---- authored units ----

func ovAuthoredUnits(pin ovPinFile, trivial []string) []ovUnit {
	var units []ovUnit
	regFile := func(name string) string { return ovREG + "/" + strings.TrimSuffix(name, "-multiple") + ".tsx" }

	// src/kernel/*.html are GENERATED by pipeline example-fixture
	// --contracts from the contract defs' React usage trees (the defs are
	// the authored units). Per-component behavior files: written against
	// the component's tsx (and the vendored kernel, whose sha is part of
	// the anchor).
	ents, _ := os.ReadDir("src/runtime/components")
	var behaviorFiles []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".js") {
			behaviorFiles = append(behaviorFiles, e.Name())
		}
	}
	sort.Strings(behaviorFiles)
	for _, f := range behaviorFiles {
		name := strings.TrimSuffix(f, ".js")
		units = append(units, ovUnit{
			id: "behavior:" + name, kind: "authored",
			file:   "src/runtime/components/" + f,
			inputs: []string{regFile(name)},
			extra:  []string{"kernel:" + pin.Kernel.Sha256},
		})
	}
	// the trivial-tier runtime is written against every trivial component
	var coreInputs []string
	for _, t := range trivial {
		coreInputs = append(coreInputs, regFile(t))
	}
	units = append(units, ovUnit{
		id: "runtime:core", kind: "authored", file: "src/runtime/core.js",
		inputs: coreInputs,
	})
	// hand-authored demos: docs/demos pages the oracle does not own
	{
		var owned, fixtureOwned map[string]bool
		owned = map[string]bool{}
		fixtureOwned = map[string]bool{}
		if b, err := os.ReadFile("docs/example-oracle.json"); err == nil {
			var arr []struct {
				Name string `json:"name"`
			}
			if json.Unmarshal(b, &arr) == nil {
				for _, t := range arr {
					owned[t.Name] = true
				}
			}
		}
		if b, err := os.ReadFile("docs/example-fixture-targets.json"); err == nil {
			var arr []struct {
				Name string `json:"name"`
			}
			if json.Unmarshal(b, &arr) == nil {
				for _, t := range arr {
					fixtureOwned[t.Name] = true
				}
			}
		}
		reRtl := regexp.MustCompile(`-rtl-(he|en|fa)\.html$`)
		dents, _ := os.ReadDir("docs/demos")
		var demos []string
		for _, e := range dents {
			if strings.HasSuffix(e.Name(), ".html") && !reRtl.MatchString(e.Name()) {
				demos = append(demos, e.Name())
			}
		}
		sort.Strings(demos)
		for _, f := range demos {
			name := strings.TrimSuffix(f, ".html")
			if owned[name] || fixtureOwned[name] {
				continue // the oracle / example-fixture owns these
			}
			ex := ovExamples + "/" + name + ".tsx"
			u := ovUnit{
				id: "demo:" + name, kind: "authored",
				file: "docs/demos/" + f,
			}
			if ovUpExists(ex) {
				u.inputs = []string{ex}
			} else {
				u.extra = []string{"no-upstream-input"}
			}
			units = append(units, u)
		}
	}
	return units
}

func ovHashUnit(u ovUnit) string {
	var parts []string
	for _, p := range u.inputs {
		b, err := os.ReadFile(ovUp(p))
		if err != nil {
			b = []byte("<missing>")
		}
		parts = append(parts, base64.StdEncoding.EncodeToString(b))
	}
	for _, e := range u.extra {
		parts = append(parts, base64.StdEncoding.EncodeToString([]byte(e)))
	}
	sum := sha256.Sum256([]byte(strings.Join(parts, "|")))
	return hex.EncodeToString(sum[:])
}

// ---- source (patch) units ----

func ovSourceUnits() []ovUnit {
	if _, err := os.Stat(ovPatches); err != nil {
		return nil
	}
	ents, err := os.ReadDir(ovPatches)
	if err != nil {
		return nil
	}
	var files []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".patch") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)
	var units []ovUnit
	for _, f := range files {
		units = append(units, ovUnit{
			id: "source:" + f, kind: "source",
			file: ovPatches + "/" + f,
		})
	}
	return units
}

// A patch series is "applied" when the upstream working tree carries
// exactly it: every patch reverse-applies cleanly. It "conflicts" when it
// neither applies forward (tree clean, patch broken) nor reverse-applies.
func ovSourceState(u ovUnit) (string, string) {
	cwd, _ := os.Getwd()
	check := func(args ...string) bool {
		cmd := exec.Command("git", append([]string{"-C", ovUP, "apply", "--check"}, args...)...)
		cmd.Args = append(cmd.Args, filepath.Join(cwd, u.file))
		return cmd.Run() == nil
	}
	if check("-R") {
		return "applied", ""
	}
	if check() {
		return "not-applied", "applies cleanly but is not applied — run pipeline upstream --apply-patches"
	}
	return "conflict", "neither applied nor applicable to the pinned upstream — rebase the patch"
}

// ---- audit / record / tasks ----

type ovBuckets struct {
	applied, dissolved, orphaned, stale, conflict, unrecorded []ovUnit
}

func ovAudit(shell *browserShell, pin ovPinFile, strict bool) *ovBuckets {
	tierSets, icons, err := ovLoadTierSets()
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		os.Exit(1)
	}
	trivial := []string{}
	for _, s := range tierSets {
		if s.tier == "trivial" {
			trivial = s.names
		}
	}

	buckets := &ovBuckets{}
	rules, err := ovRuleUnits(pin, tierSets, icons)
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		os.Exit(1)
	}
	ignoreUnits, err := ovIgnoreAttrUnits(shell)
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		os.Exit(1)
	}
	rules = append(rules, ignoreUnits...)
	sort.Slice(rules, func(i, j int) bool { return rules[i].id < rules[j].id })
	for _, u := range rules {
		if r := u.requires(); r != "" {
			u.reason = r
			buckets.orphaned = append(buckets.orphaned, u)
			continue
		}
		if d := u.dissolved(); d != "" {
			u.reason = d
			buckets.dissolved = append(buckets.dissolved, u)
			continue
		}
		buckets.applied = append(buckets.applied, u)
	}

	manifest := ovManifestFile{Units: map[string]ovUnitRec{}}
	if b, err := os.ReadFile(ovManifest); err == nil {
		json.Unmarshal(b, &manifest)
	}
	for _, u := range ovAuthoredUnits(pin, trivial) {
		rec, ok := manifest.Units[u.id]
		h := ovHashUnit(u)
		u.hash = h
		if !ok {
			buckets.unrecorded = append(buckets.unrecorded, u)
			continue
		}
		u.recorded = &rec
		if rec.Hash != h {
			in := strings.Join(u.inputs, ", ")
			if in == "" {
				in = strings.Join(u.extra, ", ")
			}
			u.reason = fmt.Sprintf("upstream inputs changed since %s: %s", rec.Pin, in)
			buckets.stale = append(buckets.stale, u)
		} else {
			buckets.applied = append(buckets.applied, u)
		}
	}
	for _, u := range ovSourceUnits() {
		state, reason := ovSourceState(u)
		if state == "applied" {
			buckets.applied = append(buckets.applied, u)
		} else {
			u.reason = reason
			buckets.conflict = append(buckets.conflict, u)
		}
	}
	// recorded units whose file is gone
	seen := map[string]bool{}
	for _, u := range buckets.applied {
		seen[u.id] = true
	}
	for _, u := range buckets.stale {
		seen[u.id] = true
	}
	for id := range manifest.Units {
		if !seen[id] {
			buckets.orphaned = append(buckets.orphaned, ovUnit{
				id: id, kind: "authored",
				reason: "recorded in the manifest but the unit no longer exists — delete the entry (--record)",
			})
		}
	}

	counts := fmt.Sprintf("%d applied, %d dissolved, %d orphaned, %d stale, %d conflict, %d unrecorded",
		len(buckets.applied), len(buckets.dissolved), len(buckets.orphaned),
		len(buckets.stale), len(buckets.conflict), len(buckets.unrecorded))
	if len(buckets.dissolved) > 0 {
		fmt.Printf("overlay: %d rules can be DELETED (upstream no longer needs them):\n", len(buckets.dissolved))
		for _, u := range buckets.dissolved {
			fmt.Printf("  %s  —  %s\n    at %s\n", u.id, u.reason, u.home)
		}
	}
	var bad []ovUnit
	for _, b := range []struct {
		name  string
		units []ovUnit
	}{{"orphaned", buckets.orphaned}, {"stale", buckets.stale}, {"conflict", buckets.conflict}, {"unrecorded", buckets.unrecorded}} {
		for _, u := range b.units {
			u.bucket = b.name
			bad = append(bad, u)
		}
	}
	if len(bad) > 0 {
		var lines []string
		for _, u := range bad {
			reason := u.reason
			if reason == "" {
				reason = "(new authored unit — record it: ./build/pipeline overlay --record)"
			}
			l := fmt.Sprintf("%-10s %s\n             %s", u.bucket, u.id, reason)
			if u.file != "" {
				l += "\n             file: " + u.file
			}
			lines = append(lines, l)
		}
		label := "REPORT"
		if strict {
			label = "FAIL "
		}
		fmt.Fprintf(os.Stderr, "%s overlay (%s)\n  %s\n", label, counts, strings.Join(lines, "\n  "))
		fmt.Fprintf(os.Stderr, "\n  task packets: ./build/pipeline overlay --tasks\n")
		if strict {
			os.Exit(1)
		}
		return buckets
	}
	fmt.Printf("PASS  overlay (%s; every manual intervention still applies to %s)\n", counts, pin.ShadcnUI.Tag)
	return buckets
}

func ovRecord(shell *browserShell, pin ovPinFile) {
	tierSets, _, err := ovLoadTierSets()
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		os.Exit(1)
	}
	trivial := []string{}
	for _, s := range tierSets {
		if s.tier == "trivial" {
			trivial = s.names
		}
	}
	manifest := ovManifestFile{Units: map[string]ovUnitRec{}}
	if b, err := os.ReadFile(ovManifest); err == nil {
		json.Unmarshal(b, &manifest)
	}
	next := ovManifestFile{Pin: pin.ShadcnUI.Tag, Commit: pin.ShadcnUI.Commit, Units: map[string]ovUnitRec{}}
	changed := 0
	for _, u := range ovAuthoredUnits(pin, trivial) {
		h := ovHashUnit(u)
		if manifest.Units[u.id].Hash != h {
			changed++
		}
		var extra []string
		for _, e := range u.extra {
			if !strings.HasPrefix(e, "kernel:") {
				extra = append(extra, e)
			}
		}
		if extra == nil {
			extra = []string{}
		}
		inputs := u.inputs
		if inputs == nil {
			inputs = []string{}
		}
		next.Units[u.id] = ovUnitRec{File: u.file, Inputs: inputs, Extra: extra, Hash: h, Pin: pin.ShadcnUI.Tag}
	}
	f, err := os.Create(ovManifest)
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		os.Exit(1)
	}
	enc := json.NewEncoder(f)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(next); err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		os.Exit(1)
	}
	f.Close()
	fmt.Printf("overlay recorded: %d authored units anchored to %s (%d re-anchored)\n",
		len(next.Units), pin.ShadcnUI.Tag, changed)
}

func ovRunTasks(shell *browserShell, pin ovPinFile) {
	buckets := ovAudit(shell, pin, false)
	os.RemoveAll(ovTasksDir)
	os.MkdirAll(ovTasksDir, 0o755)
	manifest := ovManifestFile{}
	if b, err := os.ReadFile(ovManifest); err == nil {
		json.Unmarshal(b, &manifest)
	}
	gateFor := func(u ovUnit) string {
		switch {
		case strings.HasPrefix(u.id, "behavior:"):
			return "contracts, style-parity, interactivity-sweep"
		case strings.HasPrefix(u.id, "demo:"):
			return "example-gate, docs-smoke, interactivity-sweep"
		case strings.HasPrefix(u.id, "runtime:"):
			return "contracts, demo-smoke"
		}
		return "the full tier"
	}
	n := 0
	for _, bucket := range []string{"stale", "orphaned", "conflict"} {
		var units []ovUnit
		switch bucket {
		case "stale":
			units = buckets.stale
		case "orphaned":
			units = buckets.orphaned
		case "conflict":
			units = buckets.conflict
		}
		for _, u := range units {
			diff := ""
			if len(u.inputs) > 0 && manifest.Commit != "" {
				for _, p := range u.inputs {
					out, err := exec.Command("git", "-C", ovUP, "diff",
						manifest.Commit+".."+pin.ShadcnUI.Commit, "--", p).Output()
					if err != nil {
						msg := "failed"
						if ee, ok := err.(*exec.ExitError); ok && len(ee.Stderr) > 0 {
							msg = firstLine(string(ee.Stderr))
						}
						diff += fmt.Sprintf("(could not diff %s: %s)\n", p, msg)
						continue
					}
					diff += string(out)
				}
			}
			what := fmt.Sprintf("Rebase the patch onto the pinned upstream (`git -C %s apply --3way`), resolve, regenerate with `git -C %s diff > %s`.", ovUP, ovUP, u.file)
			if bucket == "stale" {
				what = fmt.Sprintf("The upstream inputs this file was written against changed. Read the diff below, update `%s` so it reflects the new upstream, then run the gates listed and `./build/pipeline overlay --record`.", u.file)
			} else if bucket == "orphaned" {
				what = "The thing this rule attaches to no longer exists upstream. Either delete the rule at its home, or re-anchor it. Then run the full tier."
			}
			fileLine, homeLine := "", ""
			if u.file != "" {
				fileLine = "**file**: `" + u.file + "`"
			}
			if u.home != "" {
				homeLine = "**home**: " + u.home
			}
			diffLine := ""
			if diff != "" {
				from, to := "null", "null"
				if manifest.Commit != "" {
					from = manifest.Commit[:10]
				}
				if len(pin.ShadcnUI.Commit) >= 10 {
					to = pin.ShadcnUI.Commit[:10]
				}
				diffLine = fmt.Sprintf("## Upstream diff (%s → %s)\n\n```diff\n%s```", from, to, diff)
			}
			curLine := ""
			if u.file != "" {
				if b, err := os.ReadFile(u.file); err == nil {
					curLine = fmt.Sprintf("## Current file\n\n```\n%s```", string(b))
				}
			}
			md := strings.Join([]string{
				"# " + u.id,
				"",
				"**bucket**: " + bucket,
				"**reason**: " + u.reason,
				fileLine,
				homeLine,
				"",
				"## What to do",
				"",
				what,
				"",
				fmt.Sprintf("**gates to satisfy**: %s  —  `./build/pipeline run <gate>`", gateFor(u)),
				"",
				diffLine,
				curLine,
			}, "\n")
			safe := regexp.MustCompile(`[^\w.-]+`).ReplaceAllString(u.id, "_")
			os.WriteFile(filepath.Join(ovTasksDir, safe+".md"), []byte(md), 0o644)
			n++
		}
	}
	fmt.Printf("overlay tasks: %d packets under %s/\n", n, ovTasksDir)
}

func runOverlay(args []string) int {
	pinB, err := os.ReadFile("src/registry/pin.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		return 1
	}
	var pin ovPinFile
	if err := json.Unmarshal(pinB, &pin); err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		return 1
	}
	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "overlay:", err)
		return 1
	}
	defer shell.close()
	// gates/overlay.mjs built "--"+f itself; the Go verb takes the flags
	// verbatim
	if has(args, "--record") {
		ovRecord(shell, pin)
		return 0
	}
	if has(args, "--tasks") {
		ovRunTasks(shell, pin)
		return 0
	}
	if has(args, "--report") {
		ovAudit(shell, pin, false)
		return 0
	}
	ovAudit(shell, pin, true)
	return 0
}
