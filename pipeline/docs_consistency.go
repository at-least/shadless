package main

// docs-consistency, ported from tools/docs-consistency.mjs. Six checks
// that a builder cannot answer for itself:
//
//  1. skin residue: shipped HTML carries zero non-allowlist cn-* classes
//     (src/emitter/skin.mjs's ALLOWLIST, re-read by pipeline/resolve_skins.go's
//     skinData.Allowlist)
//  2. install-import reality: every @import "shadless…" the pages TEACH
//     must resolve through package.json's exports to a file on disk
//  3. no built page may teach @/components/ui (React import retirement)
//  4. every dist/js/<name>.js that has a component page is shown on it — read
//     off the artifacts, never re-deriving which scripts a demo carries
//  5. no built page carries JSX expression residue in prose ("}>" left by a
//     mis-scanned opening tag)
//  6. every dist/… path a table row hands the reader is on disk
import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

var reCN = regexp.MustCompile(`\bcn-[a-z0-9-]+`)
var reImportTeach = regexp.MustCompile(`@import\s+(?:"|&quot;)shadless[^"&]*`)
var reDistPath = regexp.MustCompile("`(dist/[A-Za-z0-9._/-]+)`")

func runDocsConsistency() int {
	loadSkin()

	for _, dir := range []string{"docs/components", "docs/guides"} {
		if _, err := os.Stat(dir); err != nil {
			fmt.Fprintln(os.Stderr, "FAIL  docs-consistency: the markdown pages are not built — run the docs chain first (make docs)")
			return 1
		}
	}

	pageFiles := []string{}
	for _, d := range []string{"docs/components", "docs/guides"} {
		ents, err := os.ReadDir(d)
		if err != nil {
			continue
		}
		for _, e := range ents {
			if strings.HasSuffix(e.Name(), ".md") {
				pageFiles = append(pageFiles, filepath.Join(d, e.Name()))
			}
		}
	}

	type problemT struct{ kind, file, detail string }
	var problems []problemT
	addProblem := func(kind, file, detail string) {
		problems = append(problems, problemT{kind, file, detail})
	}

	// 1. skin residue
	skinScanned := 0
	for _, tree := range []string{"dist/components", "docs/demos"} {
		ents, err := os.ReadDir(tree)
		if err != nil {
			continue
		}
		for _, e := range ents {
			if !strings.HasSuffix(e.Name(), ".html") {
				continue
			}
			skinScanned++
			b, _ := os.ReadFile(filepath.Join(tree, e.Name()))
			var bad []string
			seen := map[string]bool{}
			for _, m := range reCN.FindAllString(string(b), -1) {
				if seen[m] {
					continue
				}
				seen[m] = true
				if !skinData.Allowlist[m] {
					bad = append(bad, m)
				}
			}
			if len(bad) > 0 {
				addProblem("cn-residue", tree+"/"+e.Name(), strings.Join(bad, " "))
			}
		}
	}

	// 2. install-import reality
	pkgB, err := os.ReadFile("package.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-consistency:", err)
		return 1
	}
	var pkg struct {
		Exports map[string]any `json:"exports"`
	}
	json.Unmarshal(pkgB, &pkg)
	importsChecked := 0
	seenImport := map[string]bool{}
	for _, f := range pageFiles {
		b, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		for _, spec := range reImportTeach.FindAllString(string(b), -1) {
			key := regexp.MustCompile(`@import\s+|"|&quot;`).ReplaceAllString(spec, "")
			if seenImport[key] {
				continue
			}
			seenImport[key] = true
			importsChecked++
			target := resolveImport(pkg.Exports, key)
			if target == "" {
				addProblem("install-import", f, key+": no exports rule resolves it")
				continue
			}
			if _, err := os.Stat(target); err != nil {
				addProblem("install-import", f, key+" → "+target+" does not exist on disk")
			}
		}
	}

	// 3. React-import retirement
	for _, f := range pageFiles {
		b, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		if strings.Contains(string(b), "@/components/ui") {
			addProblem("react-import", f, "built page still teaches a React import")
		}
	}

	// 4. shipped JS is documented. Artifact-only on purpose: it asks
	// "dist/js/<name>.js exists, so does the page show it?" and never
	// re-derives which script tags a demo carries. The bug it exists for was
	// exactly a defect inside that shared derivation — reDemoSrcScript
	// demanded a bare `<script src=` while 90 demo pages use `<script defer
	// src=`, so six components shipped JS with no behavior tab and every
	// checker that reused the same pattern agreed there was nothing to show.
	behaviorChecked := 0
	jsEnts, err := os.ReadDir("dist/js")
	if err == nil {
		for _, e := range jsEnts {
			name := strings.TrimSuffix(e.Name(), ".js")
			if e.IsDir() || !strings.HasSuffix(e.Name(), ".js") {
				continue
			}
			page := filepath.Join("docs/components", name+".md")
			b, err := os.ReadFile(page)
			if err != nil {
				continue // not every dist/js entry has a component page
			}
			behaviorChecked++
			if !strings.Contains(string(b), "js:line-numbers [behavior]") {
				addProblem("behavior-tab-missing", page,
					"dist/js/"+e.Name()+" ships but the page shows no [behavior] tab")
			}
		}
	}

	// 5. no JSX expression residue in prose. Artifact-only: it reads the built
	// page and never re-runs convertCallouts. The case it exists for printed a
	// bare "}>" as the first line of native-select's tip, because the opening
	// tag was matched with `<Callout\b[^>]*>` and the attribute
	// `icon={<InfoIcon … />}` carries a ">" of its own. docs-fidelity compares
	// headings, previews and fences, so leftover prose was invisible to it.
	for _, f := range pageFiles {
		b, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		for i, line := range strings.Split(fenceShadow(string(b)), "\n") {
			if t := strings.TrimSpace(line); strings.HasPrefix(t, "}>") || strings.HasPrefix(t, "/>}") {
				addProblem("jsx-residue", f, fmt.Sprintf("line %d: %q — an unclosed JSX attribute expression leaked into prose", i+1, t))
			}
		}
	}

	// 6. every dist/… path a table row hands the reader exists. Scoped to
	// table rows on purpose: prose legitimately names a file to say it is NOT
	// there ("there is no `dist/js/direction.js`"), while a row in "Files this
	// component needs" is an instruction. aspect-ratio's API Reference told
	// readers to check dist/css/aspect-ratio.css two hundred lines after its
	// own Installation section said the component has no stylesheet.
	distRefs := 0
	for _, f := range pageFiles {
		b, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		for i, line := range strings.Split(string(b), "\n") {
			if !strings.HasPrefix(strings.TrimSpace(line), "|") {
				continue
			}
			for _, m := range reDistPath.FindAllStringSubmatch(line, -1) {
				distRefs++
				if _, err := os.Stat(m[1]); err != nil {
					addProblem("dist-ref", f, fmt.Sprintf("line %d: table row names %s, which is not on disk", i+1, m[1]))
				}
			}
		}
	}

	// report
	byKind := map[string][]problemT{}
	for _, p := range problems {
		byKind[p.kind] = append(byKind[p.kind], p)
	}
	ks := []string{}
	for k := range byKind {
		ks = append(ks, k)
	}
	sort.Strings(ks)
	for _, k := range ks {
		fmt.Fprintf(os.Stderr, "FAIL  %s (%d):\n", k, len(byKind[k]))
		list := byKind[k]
		n := 10
		if len(list) < n {
			n = len(list)
		}
		for _, p := range list[:n] {
			fmt.Fprintf(os.Stderr, "  - %s: %s\n", p.file, p.detail)
		}
		if len(list) > 10 {
			fmt.Fprintf(os.Stderr, "  … +%d more\n", len(list)-10)
		}
	}
	fmt.Printf("docs consistency: %d shipped pages scanned for skin residue, %d taught @imports resolved, %d built pages checked for React imports, %d shipped js files checked for a behavior tab, %d table dist refs resolved — problems: %d\n",
		skinScanned, importsChecked, len(pageFiles), behaviorChecked, distRefs, len(problems))
	if len(problems) > 0 {
		return 1
	}
	return 0
}

// resolveImport mirrors the JS gate's check, which reads package.json's
// exports the way node resolution does: exact keys first, then the "./*"
// wildcard (./* → ./dist/css/*.css per the exports map).
func resolveImport(exports map[string]any, spec string) string {
	if spec == "shadless" || spec == "shadless/" {
		if s, ok := exports["."].(string); ok {
			return strings.TrimPrefix(s, "./")
		}
		return ""
	}
	// exact key: "shadless/accordion.css" → "./accordion.css"
	sub := "." + strings.TrimPrefix(spec, "shadless") // spec carries "/" already
	if v, ok := exports[sub]; ok {
		switch x := v.(type) {
		case string:
			return strings.TrimPrefix(x, "./")
		case map[string]any:
			if imp, ok := x["import"].(string); ok {
				return strings.TrimPrefix(imp, "./")
			}
			if d, ok := x["default"].(string); ok {
				return strings.TrimPrefix(d, "./")
			}
		}
	}
	// wildcard: "./*" → "./dist/css/*.css"; the wildcard already supplies the
	// ".css" suffix — do not add one.
	if w, ok := exports["./*"].(string); ok {
		return strings.TrimPrefix(strings.Replace(w, "*", strings.TrimPrefix(sub, "./"), 1), "./")
	}
	return ""
}
