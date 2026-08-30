package main

// dist-complete — the tracked no-build stylesheet must carry every component's
// slot rules.
//
// Why (2026-08-29): dist/out.css had been committed from a state where only the
// emitter's 23 static pages existed, so 123 slot selectors from 25 components
// were gone and anyone on the no-build path had unstyled dialogs, menus and
// selects. Nothing caught it: product-verify checks shadless.full.css,
// docs-consistency compares copies of out.css with each other, neither asks
// whether out.css is whole.
//
// Oracle: dist/css/<name>.css is the per-component @apply source, one per
// component; every [data-slot="…"] selector it declares must appear in
// dist/out.css.

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

var slotSelector = regexp.MustCompile(`\[data-slot="[^"]+"\]`)

func init() { gates["dist-complete"] = gateDistComplete }

func gateDistComplete(root string) error {
	outPath := filepath.Join(root, "dist/out.css")
	outBytes, err := os.ReadFile(outPath)
	if err != nil {
		return fmt.Errorf("FAIL  dist-complete: dist/out.css missing")
	}
	out := string(outBytes)

	entries, err := os.ReadDir(filepath.Join(root, "dist/css"))
	if err != nil {
		return fmt.Errorf("FAIL  dist-complete: dist/css unreadable: %v", err)
	}
	var names []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".css") {
			names = append(names, e.Name())
		}
	}
	sort.Strings(names)

	var missing []string
	selectors, files := 0, 0
	for _, name := range names {
		files++
		src, err := os.ReadFile(filepath.Join(root, "dist/css", name))
		if err != nil {
			continue
		}
		seen := map[string]bool{}
		for _, sel := range slotSelector.FindAllString(string(src), -1) {
			if seen[sel] {
				continue
			}
			seen[sel] = true
			selectors++
			if !strings.Contains(out, sel) {
				missing = append(missing, strings.TrimSuffix(name, ".css")+": "+sel)
			}
		}
	}

	if len(missing) > 0 {
		comps := map[string]bool{}
		var order []string
		for _, m := range missing {
			c := strings.SplitN(m, ":", 2)[0]
			if !comps[c] {
				comps[c] = true
				order = append(order, c)
			}
		}
		head := order
		ell := ""
		if len(head) > 6 {
			head, ell = head[:6], ", …"
		}
		shown := missing
		more := ""
		if len(shown) > 8 {
			shown, more = shown[:8], "\n  …"
		}
		return fmt.Errorf("FAIL  dist-complete: dist/out.css lacks %d slot selectors from %d components (%s%s)\n  %s%s\n"+
			"  out.css was built from a partial dist/components — run the full `npm run demo` and commit its out.css",
			len(missing), len(comps), strings.Join(head, ", "), ell, strings.Join(shown, "\n  "), more)
	}
	fmt.Printf("PASS  dist-complete (%d slot selectors from %d component sources all present in dist/out.css)\n", selectors, files)
	return nil
}
