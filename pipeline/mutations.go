package main

// The mutation set — ported from gates/mutations/*.mjs.
//
// Every mutation is a deliberate, minimal break in a real file that its gate
// MUST notice. They exist because the most expensive recurring bug class in
// this repo's history is not a broken component, it is a gate that passes
// when it should fail; see meta.go for the incident list.
//
// Two invariants hold for the whole set and both are enforced, not trusted:
//
//   - every helper ASSERTS it changed something. A mutation that silently
//     no-ops would make its gate look vacuously green in the meta report —
//     the exact failure mode the meta-gate exists to catch, reproduced one
//     level up.
//   - a mutation touches only what it claims to. The JSON-shaped mutations
//     are textual splices rather than parse/re-serialize round-trips: a
//     round-trip through a decoder would reorder keys and reindent the whole
//     file, and the gate would then go red over the reformat instead of over
//     the mutation — "caught" for the wrong reason, which proves nothing.

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// Mutation is one break, its target gate, and the bug class it proves.
//
// Files is the snapshot set: the harness saves these before Apply and
// restores them afterwards, including on crash. Resolve replaces Files for a
// mutation whose target can only be located on a built tree — it runs first
// and its result is both the snapshot set and Apply's input, so the target
// cannot drift between the two.
type Mutation struct {
	ID      string
	Gate    NodeID
	Why     string
	Files   []string
	Resolve func(root string) ([]string, error)
	Apply   func(root string, files []string) error
}

// targets resolves the snapshot set: Resolve if present, else the static list.
func (m Mutation) targets(root string) ([]string, error) {
	if m.Resolve != nil {
		return m.Resolve(root)
	}
	return m.Files, nil
}

// ---------------------------------------------------------------- helpers

// mutEdit reads path, applies fn, and asserts the result differs. A missing
// target and a no-op edit are both errors: the first means the tree was not
// built, the second means the anchor text has moved and the mutation has
// quietly stopped proving anything.
func mutEdit(root, path string, fn func(string) (string, error)) error {
	full := filepath.Join(root, path)
	b, err := os.ReadFile(full)
	if err != nil {
		return fmt.Errorf("mutation target missing: %s (build first)", path)
	}
	before := string(b)
	after, err := fn(before)
	if err != nil {
		return err
	}
	if after == before {
		return fmt.Errorf("mutation no-op on %s — the anchor text is gone; fix the mutation", path)
	}
	return os.WriteFile(full, []byte(after), 0o644)
}

// mutReplaceOnce replaces the first occurrence of an exact anchor, asserting
// it was present.
func mutReplaceOnce(root, path, find, repl string) error {
	return mutEdit(root, path, func(s string) (string, error) {
		if !strings.Contains(s, find) {
			return "", fmt.Errorf("anchor not found in %s: %s", path, truncate(find, 60))
		}
		return strings.Replace(s, find, repl, 1), nil
	})
}

// mutReplaceAll replaces every occurrence of an exact anchor.
func mutReplaceAll(root, path, find, repl string) error {
	return mutEdit(root, path, func(s string) (string, error) {
		if !strings.Contains(s, find) {
			return "", fmt.Errorf("anchor not found in %s: %s", path, truncate(find, 60))
		}
		return strings.ReplaceAll(s, find, repl), nil
	})
}

// mutReplaceRe replaces the FIRST regexp match, matching JS `String.replace`
// with a non-global regex. repl may use $1 etc.
func mutReplaceRe(root, path string, re *regexp.Regexp, repl string) error {
	return mutEdit(root, path, func(s string) (string, error) {
		loc := re.FindStringSubmatchIndex(s)
		if loc == nil {
			return "", fmt.Errorf("pattern not found in %s: %s", path, re.String())
		}
		var out []byte
		out = append(out, s[:loc[0]]...)
		out = re.ExpandString(out, repl, s, loc)
		out = append(out, s[loc[1]:]...)
		return string(out), nil
	})
}

// mutFindFile returns the first file under dir (recursively, sorted, so the
// choice is deterministic) whose content satisfies pred.
func mutFindFile(root, dir string, pred func(string) bool, exts ...string) (string, error) {
	if len(exts) == 0 {
		exts = []string{".html"}
	}
	var candidates []string
	base := filepath.Join(root, dir)
	err := filepath.WalkDir(base, func(p string, d os.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		for _, x := range exts {
			if strings.HasSuffix(d.Name(), x) {
				candidates = append(candidates, p)
				break
			}
		}
		return nil
	})
	if err != nil {
		return "", fmt.Errorf("walking %s: %w", dir, err)
	}
	sort.Strings(candidates)
	for _, p := range candidates {
		b, err := os.ReadFile(p)
		if err != nil {
			continue
		}
		if pred(string(b)) {
			rel, err := filepath.Rel(root, p)
			if err != nil {
				return "", err
			}
			return rel, nil
		}
	}
	return "", fmt.Errorf("no file under %s matched the mutation's predicate", dir)
}

func truncate(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n]
}

// insertAfter splices text in immediately after the first occurrence of
// anchor. This is how the JSON mutations add a key: everything else in the
// file keeps its exact bytes, so the gate reacts to the new entry and not to
// a reformat.
func insertAfter(s, anchor, text string) (string, bool) {
	i := strings.Index(s, anchor)
	if i < 0 {
		return s, false
	}
	j := i + len(anchor)
	return s[:j] + text + s[j:], true
}

// ------------------------------------------------------------- the set

var reCommit = regexp.MustCompile(`"commit": "[0-9a-f]{40}"`)
var rePrimaryToken = regexp.MustCompile(`--primary: oklch\([^)]*\);`)
var rePadding = regexp.MustCompile(` px-[\d.]+`)
var reDialogScript = regexp.MustCompile(`<script src="[^"]*/js/dialog\.js"></script>`)
var reExportDefaultFn = regexp.MustCompile(`(export default function \w+\([^)]*\)\s*\{)`)
var reFirstReason = regexp.MustCompile(`"reason": "((?:[^"\\]|\\.)*)"`)
var reSheetHash = regexp.MustCompile(`"behavior:sheet"[\s\S]{0,400}?"hash": "[0-9a-f]{64}"`)
var reFirstShadlessCell = regexp.MustCompile(`"shadless": "((?:[^"\\]|\\.)*)"`)

// Mutations is the whole set, in id order. TestMeta cross-checks it against
// the graph in both directions: every gate must be named by at least one
// mutation, and every mutation must name a gate that lists it.
var Mutations = []Mutation{
	{
		ID: "consumer-sim-unknown-utility", Gate: "consumer-sim",
		Why:   "a component stylesheet references a utility that does not exist",
		Files: []string{"dist/css/switch.css"},
		// A per-component stylesheet that only compiles as part of the full
		// product entry is a file consumers cannot import alone (ed1bef4).
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "@apply ", "@apply mutation-not-a-real-utility ")
		},
	},
	{
		ID: "contracts-strip-glue", Gate: "contracts",
		Why:   "a kernel-tier page ships without its behavior file — the oracle opens, the page does not",
		Files: []string{"src/kernel/dialog.html"},
		// The contract runner replays real mouse/keyboard against the SHIPPED
		// page. Remove the behavior and the replay must stop matching React.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				`<script src="../../dist/js/dialog.js">`,
				`<script data-mutated src="../../dist/js/MISSING.js">`)
		},
	},
	{
		ID: "coverage-drop-contract", Gate: "coverage",
		Why:   "a contract def disappears and its component's cells go uncovered",
		Files: []string{"tools/contracts/components/dialog.mjs"},
		// Losing a contract def silently removes behavioral + computed
		// coverage for a component. The matrix must notice.
		Apply: func(root string, f []string) error {
			full := filepath.Join(root, f[0])
			if _, err := os.Stat(full); err != nil {
				return fmt.Errorf("mutation target missing: %s", f[0])
			}
			return os.Remove(full)
		},
	},
	{
		ID: "css-direction-new-physical", Gate: "css-direction",
		Why:   "a new physical (non-logical) direction utility enters the emitted css",
		Files: []string{"dist/shadless.css"},
		// Physical reading-direction utilities are inventoried against a
		// baseline: a new one means upstream moved the RTL story.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "@apply ", "@apply right-[13px] ")
		},
	},
	{
		ID: "demo-parity-token-drift", Gate: "demo-parity",
		Why:   "the --primary token in the shipped stylesheet no longer matches upstream's",
		Files: []string{"dist/out.css"},
		// A theme token that drifts from upstream: every element painted with
		// it differs on the shipped demo pages while the DOM stays
		// byte-identical. (A slot-rule mutation would NOT do here — the demo
		// DOM carries React's inline utilities, which shadow slot rules for
		// the properties they set.)
		Apply: func(root string, f []string) error {
			return mutReplaceRe(root, f[0], rePrimaryToken, "--primary: oklch(0.5 0.2 250);")
		},
	},
	{
		ID: "demo-smoke-console-error", Gate: "demo-smoke",
		Why:   "a shipped demo page throws on load",
		Files: []string{"dist/components/accordion.html"},
		// demo-smoke's whole job is "no page throws". A page that throws on
		// load looks fine in a screenshot and in any DOM comparison.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "</body>",
				`<script>throw new Error("mutation: page error")</script></body>`)
		},
	},
	{
		ID: "dist-complete-drop-component", Gate: "dist-complete",
		Why:   "dist/out.css loses a component's slot rules (a partial-build out.css got committed)",
		Files: []string{"dist/out.css"},
		// The incident: out.css committed without the interactive components'
		// rules. Reproduce the smallest version — one component's rules gone.
		Apply: func(root string, f []string) error {
			return mutReplaceAll(root, f[0], `[data-slot="dialog-content"]`, `[data-slot="dialog-MUTATED"]`)
		},
	},
	{
		ID: "docs-consistency-react-import", Gate: "docs-consistency",
		Why:   "a built page teaches `@/components/ui` again after an upstream mdx reshape",
		Files: []string{"docs/components/accordion.md"},
		// bc755b5: a no-React product must not teach React imports. The
		// detector exists so a NEW upstream mdx shape lands loudly.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "## Installation",
				"```tsx\nimport { Accordion } from \"@/components/ui/accordion\"\n```\n\n## Installation")
		},
	},
	{
		ID: "docs-fidelity-drop-heading", Gate: "docs-fidelity",
		Why:   "a built page silently loses a heading its mdx source has",
		Files: []string{"docs/components/accordion.md"},
		// One missing newline in a transform once glued a heading into the
		// previous paragraph and silently removed it from 51 pages. Render
		// and console checks cannot see content loss; only the mdx compare can.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "## Installation", "## Installation-mutated")
		},
	},
	{
		ID: "docs-smoke-broken-iframe", Gate: "docs-smoke",
		Why: "a preview iframe points at a page that does not exist",
		// the BUILT page: docs-smoke drives the rendered site, and rebuilding
		// it is not part of running the gate
		Files: []string{"docs/.vitepress/dist/components/accordion.html"},
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				`src="/demos/accordion-demo.html"`, `src="/demos/mutation-missing.html"`)
		},
	},
	{
		ID: "pin-base-drift", Gate: "pin",
		Why:   "pin.json names a different one of upstream's parallel registries than the graph converts from",
		Files: []string{"src/registry/pin.json"},
		// Upstream keeps radix, base and aria side by side with the same
		// component names. Repointing the pin without repointing convert (or
		// the reverse) makes the golden hop compare pages built from one
		// primitive library against another library's live pages — every cell
		// differs and none of it means what it looks like.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "registry/bases/radix/ui", "registry/bases/base/ui")
		},
	},
	{
		ID: "rtl-dict-missing-dictionary", Gate: "rtl-dict",
		Why:   "an upstream -rtl example loses its `translations` object — the extraction must fail instead of dropping that language set",
		Files: []string{".upstream/shadcn-ui/apps/v4/examples/aria/alert-rtl.tsx"},
		// alert-rtl because `alert` is static-tier and therefore SHIPPED: the
		// step skips a missing dictionary only for components this repo does
		// not build, so the mutation has to land on one it does. Before the
		// two-phase rewrite this whole class was a `continue` — the run stayed
		// green and rtl-langs.json quietly came back smaller, which is exactly
		// how demo-rtl's missing `needs: example-fixture` stayed hidden.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "const translations", "const translationsRenamedByMutation")
		},
	},
	{
		ID: "example-oracle-render-failure", Gate: "example-oracle",
		Why: "an upstream example stops rendering — the build must fail instead of dropping that page from the ownership manifest",
		// The oracle build used to SURVIVE a broken example: a render failure
		// printed `KEEP …` and the run then rewrote docs/example-oracle.json
		// to list only the pages that happened to render. One bad run shrank
		// the owned set to zero pages, and every downstream gate that counts
		// from the manifest went green over an empty surface.
		//
		// The target is whichever owned page still has an upstream example,
		// so a re-pin that retires one file does not silently disarm this.
		Resolve: func(root string) ([]string, error) {
			return resolveOwnedExample(root)
		},
		Apply: func(root string, f []string) error {
			return mutReplaceRe(root, f[0], reExportDefaultFn,
				"$1\n  throw new Error(\"example-oracle mutation: render failure\")")
		},
	},
	{
		ID: "example-perturb-shipped", Gate: "example-gate",
		Why: "a shipped demo page drifts from the React oracle render of its example",
		// Hop 2 of the dual hop: each shipped demo page must equal a fresh
		// oracle render of its upstream example.
		Resolve: func(root string) ([]string, error) {
			p, err := mutFindFile(root, "docs/demos", func(s string) bool {
				return strings.Contains(s, `data-slot="badge"`)
			})
			if err != nil {
				return nil, err
			}
			return []string{p}, nil
		},
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], `data-slot="badge"`, `data-slot="badge" data-mutation="1"`)
		},
	},
	{
		ID: "golden-perturb-oracle", Gate: "golden-gate",
		Why:   "the local oracle render stops matching the recorded live-site snapshot",
		Files: []string{"src/registry/upstream-snapshot/accordion.json"},
		// Hop 1 of the dual hop: the local React oracle render must equal what
		// ui.shadcn.com actually serves.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], `data-slot=\"accordion\"`, `data-slot=\"accordion-mutated\"`)
		},
	},
	{
		ID: "interactivity-strip-script", Gate: "interactivity-sweep",
		Why:   "an interactive example ships without its behavior — a dead button",
		Files: []string{"docs/public/demos/dialog.html"},
		// f4759ef: contracts click FIXTURES, golden compares SNAPSHOTS, smoke
		// listens to the CONSOLE — the dead-button bug lived in the gap where
		// nobody asked whether the shipped page RESPONDS.
		Apply: func(root string, f []string) error {
			return mutReplaceRe(root, f[0], reDialogScript, "")
		},
	},
	{
		ID: "ledger-budget-exceeded", Gate: "ledger",
		Why:   "the count of golden-exempt demos grows past its recorded budget",
		Files: []string{"src/registry/upstream-snapshot/exemptions.json"},
		// Budgets are the ratchet. style-parity recorded counts but compared
		// only presence, so tracked numbers could grow forever while the gate
		// stayed green. Reuse an EXISTING reason so only the BUDGET moves,
		// not the id set — otherwise this would be the undocumented-exemption
		// mutation wearing a different name.
		Apply: func(root string, f []string) error {
			return mutEdit(root, f[0], func(s string) (string, error) {
				m := reFirstReason.FindStringSubmatch(s)
				if m == nil {
					return "", fmt.Errorf("%s has no exemption to copy a reason from", f[0])
				}
				out, ok := insertAfter(s, `"examples": {`,
					"\n  \"__mutation-extra-demo\": {\n   \"reason\": \""+m[1]+"\"\n  },")
				if !ok {
					return "", fmt.Errorf(`%s has no "examples" object`, f[0])
				}
				return out, nil
			})
		},
	},
	{
		ID: "ledger-undocumented-exemption", Gate: "ledger",
		Why:   "a golden exemption exists in the source with no ledger entry",
		Files: []string{"src/registry/upstream-snapshot/exemptions.json"},
		// A new exemption appearing in a source with nobody writing down why.
		Apply: func(root string, f []string) error {
			return mutEdit(root, f[0], func(s string) (string, error) {
				out, ok := insertAfter(s, `"examples": {`,
					"\n  \"__mutation-demo\": {\n   \"reason\": \"mutation: an undocumented brand-new reason\"\n  },")
				if !ok {
					return "", fmt.Errorf(`%s has no "examples" object`, f[0])
				}
				return out, nil
			})
		},
	},
	{
		ID: "overlay-orphaned-rule", Gate: "overlay",
		Why:   "a conversion rule's anchor no longer exists (contract ignoreAttrs exempts a slot nothing emits)",
		Files: []string{"tools/contracts/components/accordion.mjs"},
		// A rule whose anchor is gone: the ignore-attrs rule audits that
		// every exempted slot is still emitted by the IR. The mutation
		// targets a def because the gate reads defs at RUNTIME — mutating
		// the pipeline's own Go tables cannot move a gate that runs the
		// prebuilt binary, and a compile step inside the gate would leave
		// the mutated binary behind after the harness restores the source.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				"  ignoreAttrs: {\n    \"accordion\": [\"text\"],",
				"  ignoreAttrs: {\n    \"accordion\": [\"text\"],\n    \"accordion-phantom\": [\"text\"],")
		},
	},
	{
		ID: "overlay-stale-authored", Gate: "overlay",
		Why:   "the upstream input a kernel fixture was written against has changed",
		Files: []string{"overlays/manifest.json"},
		// A hand-written file whose upstream input changed since it was
		// written. Simulated from the other side: the manifest records a
		// different hash than the pinned upstream produces — exactly what a
		// re-pin does.
		Apply: func(root string, f []string) error {
			return mutEdit(root, f[0], func(s string) (string, error) {
				loc := reSheetHash.FindStringIndex(s)
				if loc == nil {
					return "", fmt.Errorf(`%s has no "behavior:sheet" unit with a hash`, f[0])
				}
				seg := s[loc[0]:loc[1]]
				zeroed := regexp.MustCompile(`"hash": "[0-9a-f]{64}"`).
					ReplaceAllString(seg, `"hash": "`+strings.Repeat("0", 64)+`"`)
				return s[:loc[0]] + zeroed + s[loc[1]:], nil
			})
		},
	},
	{
		ID: "script-refs-dead-node-call", Gate: "script-refs",
		Why:   "a package.json script calls `node` on a file that does not exist — the exact shape of the bug this gate exists to catch (a JS tool deleted during the Go port, an npm script left pointing at it)",
		Files: []string{"package.json"},
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				`"unit": "node tools/unit-check.mjs"`, `"unit": "node tools/unit-check-MUTATED.mjs"`)
		},
	},
	{
		ID: "pack-broken-export", Gate: "pack",
		Why:   "package.json exports point at a file the tarball does not carry (a README-documented specifier that cannot resolve)",
		Files: []string{"package.json"},
		// The incident: `./runtime.min` was a bare string export, so the
		// `import` condition resolved to an IIFE with no export statement.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				`"./dist/shadless.full.min.css"`, `"./dist/shadless.full.MUTATED.css"`)
		},
	},
	{
		ID: "path-parity-drop-utility", Gate: "path-parity",
		Why:   "badge's slot rule drops its padding — css-import consumers get an unpadded badge",
		Files: []string{"dist/css/badge.css"},
		// A per-component stylesheet that silently loses a utility: the
		// slot-only path diverges from React while every DOM-structural gate
		// stays green.
		Apply: func(root string, f []string) error {
			return mutReplaceRe(root, f[0], rePadding, "")
		},
	},
	{
		ID: "pin-commit-drift", Gate: "pin",
		Why:   "pin.json records a commit the .upstream checkout is not sitting at",
		Files: []string{"src/registry/pin.json"},
		// The pin must describe the checkout it was taken from. Upgrade tools
		// write pin.json directly, so a tag(old)/commit(new) record could ship.
		Apply: func(root string, f []string) error {
			return mutReplaceRe(root, f[0], reCommit,
				`"commit": "`+strings.Repeat("0", 40)+`"`)
		},
	},
	{
		ID: "product-drop-slot-rule", Gate: "product-verify",
		Why:   "the product build loses a component's slot rules",
		Files: []string{"dist/shadless.full.css"},
		// The no-build artifact must actually carry the component rules; a
		// compile that silently drops them still produces a plausible file.
		Apply: func(root string, f []string) error {
			return mutReplaceAll(root, f[0], `[data-slot="badge"]`, `[data-slot="badge-MUTATED"]`)
		},
	},
	{
		ID: "reproducible-hand-edit", Gate: "reproducible",
		Why:   "a shipped file differs from what the pipeline produces",
		Files: []string{"dist/components/badge.html"},
		// A hand-edit to a generated file. The old hook guessed at this from
		// which paths were staged together; the gate rebuilds and compares.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0], "</body>", "<!-- hand edit --></body>")
		},
	},
	{
		ID: "style-parity-perturb-padding", Gate: "style-parity",
		Why:   "a shipped fixture's computed padding diverges from the oracle on one slot",
		Files: []string{"tools/contracts/out/dialog/shadless.html"},
		// "same DOM + same css => same styles" was an inference. A fixture
		// whose computed style silently differs from the React oracle must
		// fail — and with the per-cell ratchet, it must fail even for a
		// component that already carries recorded drift elsewhere.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				`data-slot="dialog-content"`, `data-slot="dialog-content" style="padding-top:37px"`)
		},
	},
	{
		ID: "style-parity-recorded-value-drift", Gate: "style-parity",
		Why:   "a cell that is already on the baseline starts differing by a different amount",
		Files: []string{"gates/style-parity-baseline.json"},
		// The baseline used to record cell IDS only. Once a cell was on the
		// list it could compute anything at all against the oracle — 4px
		// today, 400px tomorrow — and the gate stayed green over 210 of them.
		//
		// Perturbing the RECORDED value rather than the fixture is the point:
		// it is the only mutation that can distinguish "the gate reads the
		// values" from "the gate reads the ids", which is what was broken.
		Apply: func(root string, f []string) error {
			return mutEdit(root, f[0], func(s string) (string, error) {
				if !strings.Contains(s, `"cells"`) {
					return "", fmt.Errorf("%s has no cells array", f[0])
				}
				loc := reFirstShadlessCell.FindStringIndex(s)
				if loc == nil {
					// either there are no cells, or they are still bare ids —
					// both mean this mutation would prove nothing
					return "", fmt.Errorf("%s records no cell values to perturb "+
						"(no cells, or still bare ids — re-record it with values)", f[0])
				}
				return s[:loc[0]] + `"shadless": "999px /* mutation */"` + s[loc[1]:], nil
			})
		},
	},
	{
		ID: "unit-break-pure-fn", Gate: "unit",
		Why:   "a pure helper (splitMarkers) stops separating marker classes from utilities",
		Files: []string{"src/emitter/css.mjs"},
		// The regression that created the unit gate: a "dead code" delete in a
		// pure helper passed `node --check` and only surfaced minutes later.
		// The unit gate tests the JS surface (tools/unit-check.mjs imports
		// src/emitter/css.mjs), so the mutation lands there — breaking the
		// Go twin cannot move a gate that never reads it.
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				"apply: toks.filter((t) => !MARKER.test(t) &&",
				"apply: toks.filter((t) => (true) &&")
		},
	},
	{
		ID: "variant-merge-defaults", Gate: "path-parity",
		Why:   "default-variant utilities cascade into non-default variants",
		Files: []string{"dist/css/button.css"},
		// The FINDING bug, reproduced exactly: merge the default variant's
		// utilities into the bare slot rule. CSS has no un-apply, so every
		// variant that does not restate the same group inherits the default
		// look — ghost/outline buttons rendered invisible for a downstream
		// consumer (0dd7391, 30 broken cells).
		Apply: func(root string, f []string) error {
			return mutReplaceOnce(root, f[0],
				`[data-slot="button"] { @apply `,
				`[data-slot="button"] { @apply bg-primary text-primary-foreground `)
		},
	},
}
