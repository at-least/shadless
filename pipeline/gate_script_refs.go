package main

// script-refs — every pipeline invocation named in Makefile and
// package.json's "scripts" must resolve to something real: a `node
// <path>.mjs` call to a file that exists, a `./build/pipeline <verb>` (or
// Makefile's `./$(PIPELINE) <verb>`) call to a subcommand main.go actually
// dispatches, and a `go test … -run '^TestX'` to a test function that
// exists under pipeline/.
//
// The incident this proves: the Go port deleted gates/overlay.mjs,
// gates/path-parity.mjs, tools/example-oracle.mjs, tools/example-fixture.mjs
// and tools/css-direction-gate.mjs, but three Makefile targets and four npm
// scripts kept calling them — silent until someone ran `make overlay` or
// `npm run demo` and hit MODULE_NOT_FOUND. Nothing had ever asserted the
// wiring agreed with the files on disk.

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
	reNodeCall    = regexp.MustCompile(`\bnode\s+([^\s&|;"']+\.mjs)`)
	rePipelineCmd = regexp.MustCompile(`\./(?:build/pipeline|\$\(PIPELINE\))\s+([a-zA-Z][a-zA-Z0-9_-]*)`)
	reGoTestRun   = regexp.MustCompile(`-run\s+'\^(Test[A-Za-z0-9_]*)\$*'`)
	reMainVerb    = regexp.MustCompile(`cmd == "([a-zA-Z0-9_-]+)"`)
	reFuncTest    = regexp.MustCompile(`(?m)^func\s+(Test[A-Za-z0-9_]*)\s*\(`)
)

// topLevelVerbs dispatch through the plan/list/adopt/status/run switch in
// main.go rather than the `cmd == "…"` chain reMainVerb scrapes — named here
// since grepping that switch out from every other one in the package isn't
// worth it for five stable names.
var topLevelVerbs = map[string]bool{
	"plan": true, "list": true, "adopt": true, "status": true, "run": true,
}

func gateScriptRefs(root string) error {
	var fail []string

	mainSrc, err := os.ReadFile(filepath.Join(root, "pipeline", "main.go"))
	if err != nil {
		return fmt.Errorf("FAIL  script-refs (pipeline/main.go unreadable: %v)", err)
	}
	verbs := map[string]bool{}
	for v := range topLevelVerbs {
		verbs[v] = true
	}
	for _, m := range reMainVerb.FindAllStringSubmatch(string(mainSrc), -1) {
		verbs[m[1]] = true
	}

	testNames := map[string]bool{}
	entries, err := os.ReadDir(filepath.Join(root, "pipeline"))
	if err != nil {
		return fmt.Errorf("FAIL  script-refs (pipeline/ unreadable: %v)", err)
	}
	for _, e := range entries {
		if e.IsDir() || !strings.HasSuffix(e.Name(), "_test.go") {
			continue
		}
		src, err := os.ReadFile(filepath.Join(root, "pipeline", e.Name()))
		if err != nil {
			continue
		}
		for _, m := range reFuncTest.FindAllStringSubmatch(string(src), -1) {
			testNames[m[1]] = true
		}
	}

	check := func(source, label string) {
		for _, m := range reNodeCall.FindAllStringSubmatch(source, -1) {
			p := m[1]
			if _, err := os.Stat(filepath.Join(root, p)); err != nil {
				fail = append(fail, fmt.Sprintf("%s: `node %s` — file does not exist", label, p))
			}
		}
		for _, m := range rePipelineCmd.FindAllStringSubmatch(source, -1) {
			v := m[1]
			if !verbs[v] {
				fail = append(fail, fmt.Sprintf("%s: `pipeline %s` — not a subcommand in pipeline/main.go", label, v))
			}
		}
		for _, m := range reGoTestRun.FindAllStringSubmatch(source, -1) {
			name := m[1]
			if testNames[name] {
				continue
			}
			// a prefix pattern (`-run '^TestUnit'`, no trailing `$`) is
			// satisfied by any test name that starts with it
			found := false
			for tn := range testNames {
				if strings.HasPrefix(tn, name) {
					found = true
					break
				}
			}
			if !found {
				fail = append(fail, fmt.Sprintf("%s: `-run '^%s'` — no func Test%s* under pipeline/*_test.go", label, name, strings.TrimPrefix(name, "Test")))
			}
		}
	}

	pkgB, err := os.ReadFile(filepath.Join(root, "package.json"))
	if err != nil {
		return fmt.Errorf("FAIL  script-refs (package.json unreadable: %v)", err)
	}
	var pkg struct {
		Scripts map[string]string `json:"scripts"`
	}
	if err := json.Unmarshal(pkgB, &pkg); err != nil {
		return fmt.Errorf("FAIL  script-refs (package.json is not valid JSON: %v)", err)
	}
	names := make([]string, 0, len(pkg.Scripts))
	for n := range pkg.Scripts {
		names = append(names, n)
	}
	sort.Strings(names)
	for _, n := range names {
		check(pkg.Scripts[n], fmt.Sprintf("package.json script %q", n))
	}

	makefile, err := os.ReadFile(filepath.Join(root, "Makefile"))
	if err != nil {
		return fmt.Errorf("FAIL  script-refs (Makefile unreadable: %v)", err)
	}
	check(string(makefile), "Makefile")

	if len(fail) > 0 {
		sort.Strings(fail)
		return fmt.Errorf("FAIL  script-refs (%d problems)\n  %s", len(fail), strings.Join(fail, "\n  "))
	}
	fmt.Printf("PASS  script-refs (%d package.json scripts + Makefile — every node/pipeline/go-test call resolves)\n", len(names))
	return nil
}
