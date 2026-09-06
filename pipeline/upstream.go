package main

// The re-pin drill — ported from gates/upstream.mjs. One command from "a new
// shadcn release exists" to "green, or a classified report with task packets".
//
//	pipeline upstream --to=shadcn@4.20.0            full drill
//	pipeline upstream --to=shadcn@4.20.0 --fetch    fetch tags first (network)
//	pipeline upstream --to=shadcn@4.19.0            same tag: must be green (self-test)
//	pipeline upstream --report-only                 re-classify the last run
//
// Steps, each recorded in build/gates/upstream-report.md:
//
//	1. checkout the tag in .upstream, re-record src/registry/pin.json
//	2. ledger --dissolve — every auto-dissolve exemption is deleted; the
//	   rebuild has to re-earn each one with evidence
//	3. overlays/upstream/*.patch applied with git apply --3way
//	4. run all --keep-going — the WHOLE picture, not the first red
//	5. IR semantic diff (ir_diff.go) old pin -> new pin
//	6. overlay --report + --tasks — stale/orphaned manual work, as packets
//	7. classify every failed gate: EXPECTED (its components changed upstream)
//	   or UNEXPECTED (nothing upstream moved — our pipeline regressed)
//
// Nothing here needs a human to remember a checklist. What it cannot do
// (re-author a stale glue file, decide whether a new upstream slot needs a
// runtime) it hands over as a task packet with the exact diff and the gates
// that must be green afterwards.

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// upstreamDir lives in oracle_css.go — one constant for the pinned checkout.
const gatesOut = "build/gates"

// pinFile and readPin live in gate_pin.go — the pin has one reader.

// upGit runs git inside the pinned checkout and returns trimmed stdout.
func upGit(root string, args ...string) (string, error) {
	cmd := exec.Command("git", append([]string{"-C", upstreamDir}, args...)...)
	cmd.Dir = root
	out, err := cmd.Output()
	return strings.TrimSpace(string(out)), err
}

// inherit runs a command with the drill's own stdio, reporting only whether
// it succeeded — the drill continues past a red step on purpose, because the
// point is the whole picture.
func inherit(root, name string, args ...string) bool {
	cmd := exec.Command(name, args...)
	cmd.Dir = root
	cmd.Stdout, cmd.Stderr, cmd.Stdin = os.Stdout, os.Stderr, os.Stdin
	return cmd.Run() == nil
}

func upstreamStep(title string) { fmt.Printf("\n##### upstream: %s\n", title) }

// drillReport accumulates the markdown as the drill goes, so a step that
// fails still leaves everything before it in the report.
type drillReport struct{ parts []string }

func (r *drillReport) H(s string) { r.parts = append(r.parts, "\n## "+s+"\n") }
func (r *drillReport) P(s string) { r.parts = append(r.parts, s) }
func (r *drillReport) String() string {
	return "# Upstream drill report\n" + strings.Join(r.parts, "\n") + "\n"
}

type patchConflict struct {
	F   string `json:"f"`
	Out string `json:"out"`
}

func runUpstream(args []string) int {
	root, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	to := flagValue(args, "to")
	reportOnly := has(args, "--report-only")
	noBuild := has(args, "--no-build")
	if to == "" && !reportOnly {
		fmt.Fprintln(os.Stderr, "usage: pipeline upstream --to=shadcn@X.Y.Z [--fetch] [--no-build]")
		return 2
	}
	if err := os.MkdirAll(filepath.Join(root, gatesOut), 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	from, err := readPin(root)
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	registry := from.ShadcnUI.Registry
	rep := &drillReport{}

	if !reportOnly {
		if code := drillRepin(root, to, args, from, rep); code != 0 {
			return code
		}
		if !noBuild {
			upstreamStep("full tier, keep going")
			inherit(root, "make", "pipeline")
			inherit(root, "./build/pipeline", "run", "all", "--keep-going")
		}
	}

	// --------------------------------------------------------- 5. classify
	upstreamStep("classify")
	toPin, err := readPin(root)
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	changed := changedUpstreamFiles(root, from.ShadcnUI.Commit, toPin.ShadcnUI.Commit, registry)
	registryNames := listRegistryNames(root, registry)
	changedComponents := map[string]bool{}
	for _, f := range changed {
		changedComponents[componentOfExample(componentOf(f.Path), registryNames)] = true
	}

	// The IR diff lives in ir_diff.go. The drill consumes it as data — the
	// --json shape is the interface, so there is one implementation of the
	// diff and no chance of the routing signal disagreeing with the report.
	irBefore := filepath.Join(gatesOut, "ir-before")
	irText, _ := captureOutput(root, "./build/pipeline", "ir-diff", irBefore, "generated/ir")
	irJSON, _ := captureOutput(root, "./build/pipeline", "ir-diff", irBefore, "generated/ir", "--json")
	var diff struct {
		Components map[string]json.RawMessage `json:"components"`
	}
	if err := json.Unmarshal([]byte(irJSON), &diff); err == nil {
		for n := range diff.Components {
			changedComponents[n] = true
		}
	}

	rep.H("Upstream changes in scope")
	if len(changed) == 0 {
		rep.P("- none")
	} else {
		var lines []string
		for _, f := range changed {
			lines = append(lines, fmt.Sprintf("- %s `%s`", f.Status, f.Path))
		}
		rep.P(strings.Join(lines, "\n"))
	}
	rep.H("IR semantic diff")
	rep.P("```\n" + strings.TrimRight(irText, "\n") + "\n```")

	run := readRunReport(root)
	rep.H("Gates")
	rep.P(fmt.Sprintf("- passed: %d\n- failed: %d\n- blocked: %d",
		len(run.Passed), len(run.Failed), len(run.Blocked)))

	expected, unexpected := classifyFailures(run, registryNames, changedComponents)
	if len(unexpected) > 0 {
		rep.H(fmt.Sprintf("UNEXPECTED failures (%d) — our pipeline, not upstream", len(unexpected)))
		rep.P(strings.Join(unexpected, "\n\n"))
	}
	if len(expected) > 0 {
		rep.H(fmt.Sprintf("EXPECTED failures (%d) — consequences of upstream changes", len(expected)))
		rep.P(strings.Join(expected, "\n\n"))
	}
	if len(run.Blocked) > 0 {
		rep.P("\nblocked (a dependency failed): " + strings.Join(idStrings(run.Blocked), ", "))
	}

	// ----------------------------------------------------------- 6. overlay
	upstreamStep("overlay audit + task packets")
	inherit(root, "./build/pipeline", "overlay", "--tasks")
	tasks := listDir(filepath.Join(root, gatesOut, "tasks"))
	conflicts := readConflicts(root)
	rep.H("Manual work")
	if len(tasks) == 0 {
		rep.P("- none: every manual intervention still applies")
	} else {
		var lines []string
		for _, t := range tasks {
			lines = append(lines, fmt.Sprintf("- `%s/tasks/%s`", gatesOut, t))
		}
		rep.P(strings.Join(lines, "\n"))
	}
	if len(conflicts) > 0 {
		var lines []string
		for _, c := range conflicts {
			lines = append(lines, fmt.Sprintf("- CONFLICT `overlays/upstream/%s`\n```\n%s\n```", c.F, c.Out))
		}
		rep.P(strings.Join(lines, "\n"))
	}

	rep.H("Next")
	rep.P(strings.Join([]string{
		"- 1. read UNEXPECTED failures first — those are ours",
		"- 2. work the task packets (each names the gates that must be green)",
		"- 3. `./build/pipeline ledger --record` for exemptions that legitimately survive; `./build/pipeline overlay --record` after re-authoring",
		"- 4. `make upstream-snapshot` (network) to refresh the ui.shadcn.com golden snapshot for the new release",
		"- 5. `make` must be green; then commit source + regenerated output together (the dist/ diff IS the review)",
	}, "\n"))

	reportPath := filepath.Join(root, gatesOut, "upstream-report.md")
	if err := os.WriteFile(reportPath, []byte(rep.String()), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	green := len(run.Failed) == 0 && len(tasks) == 0 && len(conflicts) == 0
	verdict := "REPORT"
	if green {
		verdict = "PASS "
	}
	fmt.Printf("\n%s upstream %s → %s: %d passed, %d failed (%d unexpected), %d task packets, %d conflicts\n  %s/upstream-report.md\n",
		verdict, from.ShadcnUI.Tag, toPin.ShadcnUI.Tag,
		len(run.Passed), len(run.Failed), len(unexpected), len(tasks), len(conflicts), gatesOut)
	if green {
		return 0
	}
	return 1
}

// drillRepin is steps 1-3: checkout, re-pin, dissolve, apply the patch series.
func drillRepin(root, to string, args []string, from *pinFile, rep *drillReport) int {
	upstreamStep("checkout " + to)
	if has(args, "--fetch") {
		if _, err := upGit(root, "fetch", "--tags", "--quiet"); err != nil {
			fmt.Fprintf(os.Stderr, "fetch failed (offline?): %v\n", err)
		}
	}
	dirty, _ := upGit(root, "status", "--porcelain")
	if dirty != "" {
		// an applied overlay series leaves the tree dirty by design; anything
		// else is a hand-edit that would be lost
		if len(patchSeries(root)) == 0 {
			fmt.Fprintf(os.Stderr,
				".upstream has uncommitted changes and no overlay series explains them:\n%s\n"+
					"  reset it or turn the change into overlays/upstream/*.patch\n", dirty)
			return 1
		}
		if _, err := upGit(root, "checkout", "--", "."); err != nil {
			fmt.Fprintln(os.Stderr, "pipeline:", err)
			return 1
		}
	}
	if _, err := upGit(root, "checkout", "--quiet", to); err != nil {
		fmt.Fprintf(os.Stderr, "cannot checkout %s: %v  (try --fetch)\n", to, err)
		return 1
	}
	if err := copyTree(filepath.Join(root, "generated/ir"),
		filepath.Join(root, gatesOut, "ir-before")); err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	if !inherit(root, "./build/pipeline", "pin", "--force") {
		return 1
	}
	toPin, err := readPin(root)
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	rep.H(fmt.Sprintf("Re-pin %s → %s", from.ShadcnUI.Tag, toPin.ShadcnUI.Tag))
	rep.P(fmt.Sprintf("- from: `%s` (%s)\n- to:   `%s` (%s)",
		from.ShadcnUI.Tag, truncate(from.ShadcnUI.Commit, 10),
		toPin.ShadcnUI.Tag, truncate(toPin.ShadcnUI.Commit, 10)))
	log, _ := upGit(root, "log", "--oneline", from.ShadcnUI.Commit+".."+toPin.ShadcnUI.Commit)
	rep.P(fmt.Sprintf("- upstream commits in range: %d", len(nonEmptyLines(log))))

	upstreamStep("dissolve auto-dissolve exemptions")
	inherit(root, "./build/pipeline", "ledger", "--dissolve")

	upstreamStep("apply overlays/upstream")
	series := patchSeries(root)
	var conflicts []patchConflict
	for _, f := range series {
		cmd := exec.Command("git", "-C", upstreamDir, "apply", "--3way",
			filepath.Join(root, "overlays/upstream", f))
		cmd.Dir = root
		out, err := cmd.CombinedOutput()
		if err != nil {
			conflicts = append(conflicts, patchConflict{F: f, Out: strings.TrimSpace(string(out))})
		}
	}
	msg := ""
	if len(conflicts) > 0 {
		msg = fmt.Sprintf(", %d CONFLICT", len(conflicts))
	}
	fmt.Printf("  %d/%d patches applied%s\n", len(series)-len(conflicts), len(series), msg)
	if conflicts == nil {
		conflicts = []patchConflict{}
	}
	b, _ := json.MarshalIndent(conflicts, "", "  ")
	_ = os.WriteFile(filepath.Join(root, gatesOut, "upstream-conflicts.json"), b, 0o644)
	return 0
}

// ------------------------------------------------------------- classify

type changedFile struct{ Status, Path string }

func changedUpstreamFiles(root, fromCommit, toCommit, registry string) []changedFile {
	out, err := upGit(root, "diff", "--name-status", "--no-renames",
		fromCommit+".."+toCommit, "--",
		registry, "apps/v4/examples", "apps/v4/registry/styles/style-nova.css",
		"apps/v4/content/docs/components/radix", "apps/v4/app/globals.css")
	if err != nil {
		return nil
	}
	var files []changedFile
	for _, l := range nonEmptyLines(out) {
		parts := strings.SplitN(l, "\t", 2)
		if len(parts) == 2 {
			files = append(files, changedFile{parts[0], parts[1]})
		}
	}
	return files
}

var reSrcExt = regexp.MustCompile(`\.(tsx|mdx|css)$`)

// componentOf strips the directory and the source extension: git reports
// forward-slash paths regardless of platform, so this does not use
// filepath.Base.
func componentOf(p string) string {
	base := p
	if i := strings.LastIndex(p, "/"); i >= 0 {
		base = p[i+1:]
	}
	return reSrcExt.ReplaceAllString(base, "")
}

// componentOfExample maps an example filename back to its component by the
// longest registry-name prefix: "accordion-multiple" -> "accordion".
func componentOfExample(name string, registryNames []string) string {
	best := ""
	for _, r := range registryNames {
		if name == r || strings.HasPrefix(name, r+"-") {
			if len(r) > len(best) {
				best = r
			}
		}
	}
	if best == "" {
		return name
	}
	return best
}

func listRegistryNames(root, registry string) []string {
	entries, err := os.ReadDir(filepath.Join(root, upstreamDir, registry))
	if err != nil {
		return nil
	}
	var out []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".tsx") {
			out = append(out, strings.TrimSuffix(e.Name(), ".tsx"))
		}
	}
	sort.Strings(out)
	return out
}

// classifyFailures splits red gates into EXPECTED (the failure text names a
// component that moved upstream) and UNEXPECTED (it does not — we regressed).
// That routing is the drill's whole value: it says which failures are yours.
func classifyFailures(run runReport, registryNames []string, changed map[string]bool) (expected, unexpected []string) {
	var ids []NodeID
	for id := range run.Failed {
		ids = append(ids, id)
	}
	sort.Slice(ids, func(i, j int) bool { return ids[i] < ids[j] })
	for _, id := range ids {
		f := run.Failed[id]
		var mentioned, hits []string
		for _, n := range registryNames {
			if regexp.MustCompile(`\b` + regexp.QuoteMeta(n) + `\b`).MatchString(f.Tail) {
				mentioned = append(mentioned, n)
				if changed[n] {
					hits = append(hits, n)
				}
			}
		}
		var verdict string
		switch {
		case len(hits) > 0:
			verdict = "EXPECTED — upstream changed: " + strings.Join(hits, ", ")
		case len(mentioned) > 0:
			head := mentioned
			if len(head) > 6 {
				head = head[:6]
			}
			verdict = "UNEXPECTED — mentions " + strings.Join(head, ", ") + ", none changed upstream"
		default:
			verdict = "UNEXPECTED — no component attribution; read the tail"
		}
		entry := fmt.Sprintf("### %s\n\n%s\n\n```\n%s\n```\nrepro: `./build/pipeline run %s`",
			id, verdict, f.Tail, id)
		if len(hits) > 0 {
			expected = append(expected, entry)
		} else {
			unexpected = append(unexpected, entry)
		}
	}
	return
}

// ---------------------------------------------------------------- helpers

func readRunReport(root string) runReport {
	r := runReport{Failed: map[NodeID]failedNode{}}
	b, err := os.ReadFile(filepath.Join(root, gatesOut, "run-report.json"))
	if err != nil {
		return r
	}
	_ = json.Unmarshal(b, &r)
	if r.Failed == nil {
		r.Failed = map[NodeID]failedNode{}
	}
	return r
}

func readConflicts(root string) []patchConflict {
	var c []patchConflict
	b, err := os.ReadFile(filepath.Join(root, gatesOut, "upstream-conflicts.json"))
	if err != nil {
		return nil
	}
	_ = json.Unmarshal(b, &c)
	return c
}

func patchSeries(root string) []string {
	names := listDir(filepath.Join(root, "overlays/upstream"))
	var out []string
	for _, n := range names {
		if strings.HasSuffix(n, ".patch") {
			out = append(out, n)
		}
	}
	return out
}

func listDir(dir string) []string {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil
	}
	var out []string
	for _, e := range entries {
		out = append(out, e.Name())
	}
	sort.Strings(out)
	return out
}

func nonEmptyLines(s string) []string {
	var out []string
	for _, l := range strings.Split(s, "\n") {
		if strings.TrimSpace(l) != "" {
			out = append(out, l)
		}
	}
	return out
}

func captureOutput(root, name string, args ...string) (string, error) {
	cmd := exec.Command(name, args...)
	cmd.Dir = root
	b, err := cmd.Output()
	return string(b), err
}

// flagValue reads --name=value from argv.
func flagValue(args []string, name string) string {
	prefix := "--" + name + "="
	for _, a := range args {
		if strings.HasPrefix(a, prefix) {
			return a[len(prefix):]
		}
	}
	return ""
}

// copyTree copies src over dst, replacing it. The drill snapshots the IR
// before a re-pin so ir-diff has a "before" to compare against.
func copyTree(src, dst string) error {
	if err := os.RemoveAll(dst); err != nil {
		return err
	}
	return filepath.WalkDir(src, func(p string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, err := filepath.Rel(src, p)
		if err != nil {
			return err
		}
		target := filepath.Join(dst, rel)
		if d.IsDir() {
			return os.MkdirAll(target, 0o755)
		}
		b, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return err
		}
		return os.WriteFile(target, b, 0o644)
	})
}
