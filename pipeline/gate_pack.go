package main

// pack — the npm surface, machine-checked.
//
// Everything a consumer touches before any CSS or JS runs lives in
// package.json and README.md, and until this gate nothing asked whether those
// two agreed with each other or with the tarball:
//
//   - `./runtime.min` was a bare string export, so `import` resolved to the
//     IIFE (no export statement) and yielded undefined;
//   - README documented `@import "shadless (bare)"` — not a specifier;
//   - React and the whole conversion toolchain sat in `dependencies`, so the
//     "React-free" package installed React 19 transitively;
//   - the tarball shipped 3.9 MB of demo pages and dead glue.
//
// Checks (no network — `npm pack --dry-run --json` lists the tarball):
//  1. dependencies is empty (a React-free library must not install React);
//  2. every export target — each condition of each entry, patterns expanded
//     against dist/ — exists on disk AND is in the tarball;
//  3. every `shadless…` specifier the README's export table and code fences
//     document resolves through the exports map;
//  4. the tarball carries nothing outside the product surface.

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

var (
	reExportStmt   = regexp.MustCompile(`\bexport[\s{]`)
	reSpecBacktick = regexp.MustCompile("`(shadless(?:/[^`\\s]*)?)`")
	reSpecImport   = regexp.MustCompile(`(?:@import|from|import)\s+"(shadless(?:/[^"]*)?)"`)
	reAllowed      = regexp.MustCompile(`^(package\.json|README\.md|CHANGELOG\.md|LICENSE|dist/(css|js|esm)/[^/]+|dist/shadless(-core|\.full(\.min)?)?\.(css|js)|dist/shadless\.min\.js)$`)
)

func gatePack(root string) error {
	raw, err := os.ReadFile(filepath.Join(root, "package.json"))
	if err != nil {
		return fmt.Errorf("FAIL  pack (package.json unreadable: %v)", err)
	}
	var pkg struct {
		Dependencies map[string]any `json:"dependencies"`
		Exports      map[string]any `json:"exports"`
		Style        any            `json:"style"`
	}
	if err := json.Unmarshal(raw, &pkg); err != nil {
		return fmt.Errorf("FAIL  pack (package.json is not valid JSON: %v)", err)
	}
	var fail []string

	// 1. dependencies
	if len(pkg.Dependencies) > 0 {
		deps := make([]string, 0, len(pkg.Dependencies))
		for d := range pkg.Dependencies {
			deps = append(deps, d)
		}
		sort.Strings(deps)
		fail = append(fail, "dependencies must be empty (React-free means installing nothing): "+strings.Join(deps, ", "))
	}

	// 2. exports -> files
	targets := map[string][]string{}
	var addTarget func(key, cond string, value any)
	addTarget = func(key, cond string, value any) {
		switch v := value.(type) {
		case string:
			label := fmt.Sprintf("%s [%s]", key, cond)
			if strings.Contains(v, "*") {
				pre, post, _ := strings.Cut(v, "*")
				trimmed := strings.TrimPrefix(pre, "./")
				dir := trimmed
				if i := strings.LastIndex(trimmed, "/"); i >= 0 {
					dir = trimmed[:i]
				} else {
					dir = ""
				}
				stem := pre[strings.LastIndex(pre, "/")+1:]
				var files []string
				if entries, err := os.ReadDir(filepath.Join(root, dir)); err == nil {
					for _, e := range entries {
						if strings.HasPrefix(e.Name(), stem) && strings.HasSuffix(e.Name(), post) {
							files = append(files, dir+"/"+e.Name())
						}
					}
				}
				sort.Strings(files)
				if len(files) == 0 {
					fail = append(fail, fmt.Sprintf("export %s [%s] → %s: pattern matches nothing", key, cond, v))
				}
				targets[label] = files
			} else {
				targets[label] = []string{strings.TrimPrefix(v, "./")}
			}
		case map[string]any:
			keys := make([]string, 0, len(v))
			for c := range v {
				keys = append(keys, c)
			}
			sort.Strings(keys)
			for _, c := range keys {
				addTarget(key, c, v[c])
			}
		}
	}
	exportKeys := make([]string, 0, len(pkg.Exports))
	for k := range pkg.Exports {
		exportKeys = append(exportKeys, k)
	}
	sort.Strings(exportKeys)
	for _, k := range exportKeys {
		addTarget(k, "default", pkg.Exports[k])
	}
	if s, ok := pkg.Style.(string); ok {
		targets["style"] = []string{strings.TrimPrefix(s, "./")}
	}
	// the `import` condition must resolve to a real ES module
	for _, k := range exportKeys {
		obj, ok := pkg.Exports[k].(map[string]any)
		if !ok {
			continue
		}
		imp, ok := obj["import"].(string)
		if !ok || strings.Contains(imp, "*") {
			continue
		}
		src, _ := os.ReadFile(filepath.Join(root, imp))
		if !reExportStmt.Match(src) {
			fail = append(fail, fmt.Sprintf("export %s [import] → %s has no export statement (an IIFE under the import condition yields undefined)", k, imp))
		}
	}

	// ONE ES-module base. Every dist/esm/<name>.mjs opens with
	// `import "./shadless.mjs"` and then registers on the global, so a second
	// base module cannot be shared with them: a consumer importing it next to
	// any component holds an instance nothing ever registers on, and every
	// get() returns null. shadless/js.min shipped exactly that for one
	// release. Read off the artifacts — a base is any dist/esm/*.mjs that does
	// not import the base.
	esmEnts, _ := os.ReadDir(filepath.Join(root, "dist/esm"))
	var bases []string
	for _, e := range esmEnts {
		if e.IsDir() || !strings.HasSuffix(e.Name(), ".mjs") {
			continue
		}
		src, err := os.ReadFile(filepath.Join(root, "dist/esm", e.Name()))
		if err != nil {
			continue
		}
		if !bytes.Contains(src, []byte(`import "./shadless.mjs"`)) {
			bases = append(bases, "dist/esm/"+e.Name())
		}
	}
	if len(bases) > 1 {
		fail = append(fail, fmt.Sprintf("%d ES-module bases in dist/esm (%s) — component modules import ./shadless.mjs by path, so any other base is an instance they never register on",
			len(bases), strings.Join(bases, ", ")))
	}

	packed, err := npmPackFiles(root)
	if err != nil {
		return fmt.Errorf("FAIL  pack (npm pack failed: %v)", err)
	}
	labels := make([]string, 0, len(targets))
	for l := range targets {
		labels = append(labels, l)
	}
	sort.Strings(labels)
	for _, label := range labels {
		for _, f := range targets[label] {
			if _, err := os.Stat(filepath.Join(root, f)); err != nil {
				fail = append(fail, fmt.Sprintf("export %s → %s: file does not exist", label, f))
			} else if !packed[f] {
				fail = append(fail, fmt.Sprintf("export %s → %s: not in the tarball (package.json \"files\")", label, f))
			}
		}
	}

	// 3. README specifiers resolve
	readme, _ := os.ReadFile(filepath.Join(root, "README.md"))
	specSet := map[string]bool{}
	for _, m := range reSpecBacktick.FindAllStringSubmatch(string(readme), -1) {
		specSet[m[1]] = true
	}
	for _, m := range reSpecImport.FindAllStringSubmatch(string(readme), -1) {
		specSet[m[1]] = true
	}
	specs := make([]string, 0, len(specSet))
	for s := range specSet {
		specs = append(specs, s)
	}
	sort.Strings(specs)
	resolves := func(spec string) bool {
		sub := "."
		if spec != "shadless" {
			sub = "." + strings.TrimPrefix(spec, "shadless")
		}
		if _, ok := pkg.Exports[sub]; ok {
			return true
		}
		for _, k := range exportKeys {
			if !strings.Contains(k, "*") {
				continue
			}
			pre, post, _ := strings.Cut(k, "*")
			if strings.HasPrefix(sub, pre) && strings.HasSuffix(sub, post) && len(sub) > len(pre)+len(post) {
				return true
			}
		}
		return false
	}
	for _, s := range specs {
		if !resolves(strings.ReplaceAll(s, "<name>", "button")) { // the README's placeholder
			fail = append(fail, fmt.Sprintf("README documents %q but package.json exports do not resolve it", s))
		}
	}

	// 4. nothing outside the product surface
	packedNames := make([]string, 0, len(packed))
	for f := range packed {
		packedNames = append(packedNames, f)
	}
	sort.Strings(packedNames)
	for _, f := range packedNames {
		if !reAllowed.MatchString(f) {
			fail = append(fail, fmt.Sprintf("tarball carries %s — outside the product surface", f))
		}
	}

	if len(fail) > 0 {
		return fmt.Errorf("FAIL  pack (%d problems)\n  %s", len(fail), strings.Join(fail, "\n  "))
	}
	fmt.Printf("PASS  pack (%d export targets in a %d-file tarball; %d README specifiers resolve; dependencies empty)\n",
		len(targets), len(packed), len(specs))
	return nil
}

func npmPackFiles(root string) (map[string]bool, error) {
	c := exec.Command("npm", "pack", "--dry-run", "--json", "--ignore-scripts")
	c.Dir = root
	c.Stderr = nil
	out, err := c.Output()
	if err != nil {
		return nil, err
	}
	var res []struct {
		Files []struct {
			Path string `json:"path"`
		} `json:"files"`
	}
	if err := json.Unmarshal(out, &res); err != nil || len(res) == 0 {
		return nil, fmt.Errorf("unexpected npm pack output")
	}
	packed := map[string]bool{}
	for _, f := range res[0].Files {
		packed[f.Path] = true
	}
	return packed, nil
}
