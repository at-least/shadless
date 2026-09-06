package main

// consumer-sim — machine-proof of the PRIMARY distribution path. Ported from
// tools/consumer-sim.mjs.
//
// The story we document: a consumer with only tailwindcss installed @imports
// shadless tokens + the per-component css files they use, pastes markup
// carrying inline utilities, and their own build emits exactly that
// component's styles. This gate simulates that consumer in a scratch directory
// and fails when any part of the story breaks:
//
//   - shadless.css must be self-contained (no @import beyond "tailwindcss"
//     itself — the animate layer is inlined at product-css time);
//   - the scratch build must compile clean;
//   - slot rules for the IMPORTED components must be present;
//   - ZERO rules for components that were not imported (file-granularity tree
//     shaking);
//   - inline utilities from the consumer's page must be emitted (their content
//     scan);
//   - theme variables must survive (dark mode flips on them);
//   - size sanity bound — catches "everything leaked into the build".
//
// The scratch dir is an OS temp dir — a consumer's project, not a corner of
// this repo. It must NOT sit under a gitignored path: tailwind's automatic
// source detection stops honouring ignore rules when the cwd itself is ignored
// (verified 2026-08-30 with build/consumer-sim: it followed
// node_modules/shadless into the whole repo and "tree-shook" a 589 KB build).
// tailwindcss resolves through a second symlink so `@import "tailwindcss"`
// works from outside the repo.

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"sort"
	"strings"
	"sync"
)

var reCoreImport = regexp.MustCompile(`(?m)^@import\s+("[^"]+"|url\([^)]*\));?$`)

var (
	simImported    = []string{"button", "alert"}
	simNotImported = []string{"dialog", "accordion", "select", "tooltip", "carousel"}
)

const consumerPage = `<!doctype html>
<html><head><meta charset="utf-8"><title>consumer</title></head><body>
<button data-slot="button" data-variant="outline" class="font-medium">Continue</button>
<div data-slot="alert" data-variant="default">Account created — you are signed in.</div>
</body></html>
`

func gateConsumerSim(root string) error {
	sim, err := os.MkdirTemp("", "shadless-consumer-sim-")
	if err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	defer os.RemoveAll(sim)

	// 1. core self-containment
	core, err := os.ReadFile(filepath.Join(root, "dist/shadless-core.css"))
	if err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	var stray []string
	for _, m := range reCoreImport.FindAllStringSubmatch(string(core), -1) {
		if m[1] != `"tailwindcss"` {
			stray = append(stray, m[1])
		}
	}
	if len(stray) > 0 {
		return fmt.Errorf("FAIL  consumer-sim: shadless-core.css is not self-contained — @import %s "+
			"would need an extra package", strings.Join(stray, ", "))
	}

	// install the package the way a consumer would have it: node_modules/
	// shadless → this repo. The entry then imports through the REAL
	// package.json exports map — the specifiers the docs document are the ones
	// machine-checked here, not just file copies.
	if err := os.MkdirAll(filepath.Join(sim, "node_modules"), 0o755); err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	if err := os.Symlink(root, filepath.Join(sim, "node_modules/shadless")); err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	if err := os.Symlink(filepath.Join(root, "node_modules/tailwindcss"),
		filepath.Join(sim, "node_modules/tailwindcss")); err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}

	entry, outCSS := filepath.Join(sim, "entry.css"), filepath.Join(sim, "out.css")
	writeEntry := func(imports ...string) error {
		body := `@import "shadless";` + "\n"
		for _, n := range imports {
			body += fmt.Sprintf("@import %q;\n", "shadless/"+n+".css")
		}
		return os.WriteFile(entry, []byte(body), 0o644)
	}
	if err := writeEntry(simImported...); err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	// the consumer's page: shadless markup (data-slots + inline utilities)
	if err := os.WriteFile(filepath.Join(sim, "page.html"), []byte(consumerPage), 0o644); err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}

	// 2. the consumer's own build
	if err := twCompile(root, entry, outCSS, sim, false, false); err != nil {
		return fmt.Errorf("FAIL  consumer-sim: the consumer's build did not compile: %v", err)
	}
	outBytes, err := os.ReadFile(outCSS)
	if err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	out := string(outBytes)

	var problems []string
	// 3. imported slot rules present
	for _, n := range simImported {
		if !strings.Contains(out, fmt.Sprintf(`[data-slot="%s"]`, n)) {
			problems = append(problems, "imported component "+n+": no slot rule in the consumer build")
		}
	}
	// 4. nothing from non-imported components
	for _, n := range simNotImported {
		if strings.Contains(out, fmt.Sprintf(`[data-slot="%s"]`, n)) {
			problems = append(problems, "tree-shaking broken: "+n+
				" rules leaked into a build that never imported it")
		}
	}
	// 5. inline utilities from the consumer page
	if !regexp.MustCompile(`\.font-medium\b`).MatchString(out) {
		problems = append(problems, "inline utility from the consumer page (font-medium) not emitted")
	}
	// 6. theme variables
	if !strings.Contains(out, "--background:") || !strings.Contains(out, ".dark") {
		problems = append(problems, "theme variables / .dark override missing from the consumer build")
	}
	// 7. size sanity — a full leak lands in the hundreds of KB
	kb := (len(out) + 512) / 1024
	if len(out) > 80*1024 {
		problems = append(problems, fmt.Sprintf("consumer build is %dKB — the whole library leaked in "+
			"(expected a couple dozen KB)", kb))
	}

	// 8. EVERY per-component stylesheet compiles individually with the core
	// (2026-08-27 survey became a gate: a component that only compiles as part
	// of the full product entry — e.g. referencing something a sibling part
	// defines — would ship a file consumers cannot import alone)
	//
	// Each name below is fully independent (its own scratch write + its own
	// tailwindcss subprocess), so they run concurrently, bounded by the same
	// channel-semaphore pattern run.go's dispatcher uses — each goroutine gets
	// its own entry-<name>.css/out-<name>.css so parallel writers don't race on
	// the shared entry.css/out.css paths used above.
	names, err := os.ReadDir(filepath.Join(root, "dist/css"))
	if err != nil {
		return fmt.Errorf("FAIL  consumer-sim: %v", err)
	}
	var toCompile []string
	for _, e := range names {
		if strings.HasSuffix(e.Name(), ".css") {
			toCompile = append(toCompile, strings.TrimSuffix(e.Name(), ".css"))
		}
	}
	type indivResult struct {
		name string
		err  error
	}
	sem := make(chan struct{}, runtime.NumCPU())
	resultsCh := make(chan indivResult, len(toCompile))
	var wg sync.WaitGroup
	for _, n := range toCompile {
		wg.Add(1)
		go func(n string) {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()
			entryN := filepath.Join(sim, "entry-"+n+".css")
			outN := filepath.Join(sim, "out-"+n+".css")
			body := fmt.Sprintf("@import \"shadless\";\n@import %q;\n", "shadless/"+n+".css")
			if err := os.WriteFile(entryN, []byte(body), 0o644); err != nil {
				resultsCh <- indivResult{n, err}
				return
			}
			resultsCh <- indivResult{n, twCompile(root, entryN, outN, sim, false, true)}
		}(n)
	}
	wg.Wait()
	close(resultsCh)
	individualOK := 0
	var individualFail []string
	for r := range resultsCh {
		if r.err != nil {
			individualFail = append(individualFail, r.name)
		} else {
			individualOK++
		}
	}
	sort.Strings(individualFail) // deterministic report regardless of goroutine finish order
	if len(individualFail) > 0 {
		problems = append(problems, "components that do NOT compile individually with the core: "+
			strings.Join(individualFail, ", "))
	}

	if len(problems) > 0 {
		return fmt.Errorf("FAIL  consumer-sim\n  %s", strings.Join(problems, "\n  "))
	}
	fmt.Printf("PASS  consumer-sim (%d components imported, tree-shaking intact, %dKB build, "+
		"core self-contained, %d/%d components compile individually)\n",
		len(simImported), kb, individualOK, individualOK+len(individualFail))
	return nil
}
