package main

// example-golden, ported from tools/example-golden.mjs — hop 1 of the 1:1
// gate: local oracle render == upstream live-site snapshot. Both sides are
// canonicalized in a real browser (canonOf); the snapshot is the committed
// SSR artifact. Exemptions are explicit and their staleness fails the gate.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// reGoldenAutoId: both spellings a normalised auto-id can have in a diff
// context — the snapshot corpus still holds the old collapsed form.
var reGoldenAutoId = regexp.MustCompile(`radix-(?:<auto>_?|a\\d+)`)

// firstDiffWindow finds the first index where a and b differ (the length of
// the shorter string if one is a prefix of the other) and returns a
// [i-before, i+after) slice of each, clamped to its own bounds.
func firstDiffWindow(a, b string, before, after int) (string, string) {
	i := 0
	for i < len(a) && i < len(b) && a[i] == b[i] {
		i++
	}
	window := func(s string) string {
		lo, hi := i-before, i+after
		if lo < 0 {
			lo = 0
		}
		if hi > len(s) {
			hi = len(s)
		}
		return s[lo:hi]
	}
	return window(a), window(b)
}

func runExampleGolden(args []string) int {
	const examplesDir = ".upstream/shadcn-ui/apps/v4/examples/radix"
	const snapshotDir = "src/registry/upstream-snapshot"
	const tmp = "build/example-golden"
	const exemptionsPath = snapshotDir + "/exemptions.json"

	mode := "gate"
	var diffName, diffPage string
	for i, a := range args {
		switch a {
		case "--classify":
			mode = "classify"
		case "--diff":
			mode = "diff"
			if i+1 < len(args) {
				diffName = args[i+1]
			}
			if i+2 < len(args) {
				diffPage = args[i+2]
			}
		}
	}

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "example-golden:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "example-golden:", err)
		return 1
	}
	page, _ := shell.newPage(false)
	// keep avatar-style examples in their INITIAL render state: a loaded
	// image flips radix Avatar to the img branch and the trees diverge on
	// structure, not styling
	page.routeAbortExternal()
	page.gotoURL("about:blank")

	oracleCanon := func(name string) (string, error) {
		htmlFile, err := buildOracleGo(name, tmp)
		if err != nil {
			return "", err
		}
		if err := awaitOracleGo(page, htmlFile); err != nil {
			return "", err
		}
		rootHtml, err := oracleRootHtml(page)
		if err != nil {
			return "", err
		}
		return canonOfGo(page, rootHtml)
	}

	if mode == "diff" {
		if diffPage == "" {
			diffPage = strings.SplitN(diffName, "-", 2)[0]
		}
		snapB, err := os.ReadFile(filepath.Join(snapshotDir, diffPage+".json"))
		if err != nil {
			fmt.Fprintln(os.Stderr, "example-golden:", err)
			return 1
		}
		var snap map[string]any
		json.Unmarshal(snapB, &snap)
		previews, _ := snap["previews"].(map[string]any)
		upstreamHtml, _ := previews[diffName].(string)
		if upstreamHtml == "" {
			fmt.Fprintf(os.Stderr, "no snapshot preview %q in %s.json\n", diffName, diffPage)
			return 1
		}
		a, err := oracleCanon(diffName)
		if err != nil {
			fmt.Fprintln(os.Stderr, "example-golden:", err)
			return 1
		}
		b, err := canonOfGo(page, upstreamHtml)
		if err != nil {
			fmt.Fprintln(os.Stderr, "example-golden:", err)
			return 1
		}
		if a == b {
			fmt.Println("EQUAL")
			return 0
		}
		wa, wb := firstDiffWindow(a, b, 80, 120)
		fmt.Println("ORACLE  :", wa)
		fmt.Println("UPSTREAM:", wb)
		return 1
	}

	type exemption struct {
		Stale  bool   `json:"stale"`
		Reason string `json:"reason"`
	}
	var exemptions struct {
		Examples map[string]exemption `json:"examples"`
	}
	if eb, err := os.ReadFile(exemptionsPath); err == nil {
		json.Unmarshal(eb, &exemptions)
	}
	if exemptions.Examples == nil {
		exemptions.Examples = map[string]exemption{}
	}

	sig := func(a, b string) string {
		wa, wb := firstDiffWindow(a, b, 60, 60)
		ctx := wa + " ||| " + wb
		ctx = reGoldenAutoId.ReplaceAllString(ctx, "#")
		if len(ctx) > 200 {
			ctx = ctx[:200]
		}
		return ctx
	}

	pass, fail, exempt := 0, 0, 0
	var staleExemptions []string
	buckets := map[string][]string{}
	type failureRec struct {
		Name      string `json:"name"`
		Page      string `json:"page"`
		Kind      string `json:"kind"`
		Signature string `json:"signature,omitempty"`
		Error     string `json:"error,omitempty"`
	}
	var failures []failureRec
	bucketOf := func(key, name string) {
		k := regexp.MustCompile(`"[^"]{20,}"`).ReplaceAllString(key, `"…"`)
		if len(k) > 110 {
			k = k[:110]
		}
		buckets[k] = append(buckets[k], name)
	}

	ents, _ := os.ReadDir(snapshotDir)
	var pages []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".json") && e.Name() != "exemptions.json" {
			pages = append(pages, e.Name())
		}
	}
	sort.Strings(pages)
	for _, pf := range pages {
		snapB, _ := os.ReadFile(filepath.Join(snapshotDir, pf))
		var snap struct {
			Previews map[string]string `json:"previews"`
		}
		if json.Unmarshal(snapB, &snap) != nil {
			continue
		}
		// insertion order for stable output: read raw
		names := orderedJSONKeys(snapB, "previews")
		for _, name := range names {
			upstreamHtml := snap.Previews[name]
			ex := exemptions.Examples[name]
			if ex.Reason != "" && !ex.Stale {
				exempt++
				continue
			}
			tsx := filepath.Join(examplesDir, name+".tsx")
			if !fileExists(tsx) {
				if ex.Reason != "" {
					exempt++
					continue
				}
				fmt.Fprintf(os.Stderr, "FAIL [%s]: snapshot demo has no example tsx and no exemption\n", name)
				fail++
				continue
			}
			sa, err := oracleCanon(name)
			if err != nil {
				if ex.Reason != "" {
					exempt++
					continue
				}
				msg := strings.SplitN(err.Error(), "\n", 2)[0]
				if mode == "classify" {
					bucketOf("RENDER-FAIL "+msg, name)
					failures = append(failures, failureRec{name, strings.TrimSuffix(pf, ".json"), "render", "", msg})
				} else {
					fmt.Fprintf(os.Stderr, "FAIL [%s]: oracle build/render failed — %s (add an exemption with a reason if unfixable)\n", name, msg)
				}
				fail++
				continue
			}
			sb, err := canonOfGo(page, upstreamHtml)
			if err != nil {
				fmt.Fprintf(os.Stderr, "FAIL [%s]: canon: %v\n", name, err)
				fail++
				continue
			}
			if sa == sb {
				pass++
			} else {
				if mode == "classify" {
					s := sig(sa, sb)
					bucketOf(s, name)
					failures = append(failures, failureRec{name, strings.TrimSuffix(pf, ".json"), "diff", s, ""})
				} else {
					fmt.Fprintf(os.Stderr, "FAIL [%s]: oracle != upstream snapshot\n", name)
				}
				fail++
			}
			if ex.Reason != "" {
				staleExemptions = append(staleExemptions, name)
			}
		}
	}
	exit := 0
	if len(staleExemptions) > 0 {
		fmt.Fprintf(os.Stderr, "FAIL  example-golden: stale exemptions (rendered fine, remove them): %s\n",
			strings.Join(staleExemptions, ", "))
		exit = 1
	}
	if mode == "classify" {
		b, _ := json.MarshalIndent(failures, "", " ")
		os.MkdirAll(tmp, 0o755)
		os.WriteFile(filepath.Join(tmp, "failures.json"), append(b, '\n'), 0o644)
		fmt.Printf("classify: %d pass, %d fail, %d exempt (%d recorded to %s/failures.json)\n",
			pass, fail, exempt, len(failures), tmp)
		var keys []string
		for k := range buckets {
			keys = append(keys, k)
		}
		sort.Slice(keys, func(i, j int) bool { return len(buckets[keys[i]]) > len(buckets[keys[j]]) })
		for _, k := range keys {
			fmt.Printf("\n%4d×  %s\n     %s\n", len(buckets[k]), k, strings.Join(buckets[k], ", "))
		}
		if fail > 0 {
			exit = 1
		}
	} else if fail > 0 {
		fmt.Fprintf(os.Stderr, "FAIL  example-golden (%d failed, %d passed, %d exempt)\n", fail, pass, exempt)
		exit = 1
	} else if exit == 0 {
		fmt.Printf("PASS  example-golden (%d == upstream snapshot, %d exempt)\n", pass, exempt)
	}
	return exit
}

// orderedJSONKeys recovers an object's key order from raw JSON.
func orderedJSONKeys(raw []byte, field string) []string {
	obj, err := decodeOrderedObject(extractRawField(raw, field))
	if err != nil {
		return nil
	}
	return obj.keys
}
