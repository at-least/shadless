package main

// upstream-snapshot, ported from tools/upstream-snapshot.mjs — golden-master
// snapshot of ui.shadcn.com example DOM (hop 1 of the 1:1 gate). Crawls
// /docs/components/<base>/<page>, slices each preview's demo DOM out of the
// SSR payload, normalizes radix auto ids, stores src/registry/upstream-snapshot/
// as a COMMITTED artifact. Network tool: run on re-pin (make upstream-snapshot)
// or through .dagger/ against a locally built pinned checkout.

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"time"
)

var (
	reRadixCSR1 = regexp.MustCompile(`radix-:r[a-z0-9]*:?`)
	reRadixCSR2 = regexp.MustCompile(`radix-_r_[a-z0-9-]*`)
	reRadixSSR  = regexp.MustCompile(`(?i)radix-_R_[a-z0-9-]*`)
	rePreviewWr = regexp.MustCompile(`<div data-slot="preview"[^>]*><div data-align="[^"]*" data-chromeless="false" class="preview[^"]*">`)
	reCompPrev  = regexp.MustCompile(`<ComponentPreview\b([^>]*)>`)
	reNameAttr  = regexp.MustCompile(`name="([^"]*)"`)
)

func normSnapshot(html string) string {
	out := reRadixCSR1.ReplaceAllString(html, "radix-<auto>")
	out = reRadixCSR2.ReplaceAllString(out, "radix-<auto>")
	// SSR react-useId ids share the CSR bucket — runtime-generated, not
	// part of the contract
	return reRadixSSR.ReplaceAllString(out, "radix-<auto>")
}

// snapshotPreviewNames: preview names in mdx document order (fences shadowed
// so fenced ComponentPreview text cannot reorder the mapping).
func snapshotPreviewNames(mdx string) []string {
	var out []string
	for _, m := range reCompPrev.FindAllStringSubmatch(fenceShadow(mdx), -1) {
		if nm := reNameAttr.FindStringSubmatch(m[1]); nm != nil {
			out = append(out, nm[1])
		}
	}
	return out
}

// snapshotSlicePreviews slices each preview's demo container content out of
// SSR HTML by stack-balancing <div>/</div>.
func snapshotSlicePreviews(html string) []string {
	var out []string
	for _, loc := range rePreviewWr.FindAllStringIndex(html, -1) {
		start := loc[1]
		depth := 1
		i := start
		for depth > 0 && i < len(html) {
			nextOpen := strings.Index(html[i:], "<div")
			nextClose := strings.Index(html[i:], "</div>")
			if nextClose == -1 {
				break
			}
			if nextOpen != -1 && nextOpen < nextClose {
				depth++
				i += nextOpen + 4
			} else {
				depth--
				i += nextClose + 6
			}
		}
		end := strings.LastIndex(html[:i-6], "</div>")
		if end >= start {
			out = append(out, html[start:end])
		}
	}
	return out
}

func runUpstreamSnapshot(args []string) int {
	only := ""
	for i, a := range args {
		if a == "--page" && i+1 < len(args) {
			only = args[i+1]
		}
	}

	pinB, err := os.ReadFile("src/registry/pin.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "upstream-snapshot:", err)
		return 1
	}
	var pin struct {
		ShadcnUI struct {
			Registry string `json:"registry"`
		} `json:"shadcn_ui"`
	}
	json.Unmarshal(pinB, &pin)
	m := regexp.MustCompile(`registry/bases/([^/]+)/`).FindStringSubmatch(pin.ShadcnUI.Registry)
	if m == nil {
		fmt.Fprintln(os.Stderr, "FAIL  upstream-snapshot: src/registry/pin.json has no `shadcn_ui.registry` of the form apps/v4/registry/bases/<base>/ui — cannot tell which base to crawl")
		return 1
	}
	base := m[1]
	docsDir := fmt.Sprintf(".upstream/shadcn-ui/apps/v4/content/docs/components/%s", base)
	outDir := "src/registry/upstream-snapshot"
	origin := os.Getenv("SHADLESS_SNAPSHOT_ORIGIN")
	if origin == "" {
		origin = "https://ui.shadcn.com"
	}
	crawlBase := fmt.Sprintf("%s/docs/components/%s", origin, base)

	// pages with no component-preview on upstream (verified: sidebar and
	// typography render through different chrome — and both sit in our grey
	// list anyway)
	skip := map[string]bool{"sidebar": true, "typography": true}

	ents, err := os.ReadDir(docsDir)
	if err != nil {
		fmt.Fprintln(os.Stderr, "upstream-snapshot:", err)
		return 1
	}
	var pages []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".mdx") {
			p := strings.TrimSuffix(e.Name(), ".mdx")
			if !skip[p] && (only == "" || p == only) {
				pages = append(pages, p)
			}
		}
	}
	sort.Strings(pages)

	if err := os.MkdirAll(outDir, 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "upstream-snapshot:", err)
		return 1
	}
	client := &http.Client{Timeout: 30 * time.Second}
	total, failed := 0, 0
	for _, page := range pages {
		mdxB, _ := os.ReadFile(filepath.Join(docsDir, page+".mdx"))
		names := snapshotPreviewNames(string(mdxB))
		res, err := client.Get(crawlBase + "/" + page)
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL %s: %v\n", page, err)
			failed++
			continue
		}
		if res.StatusCode != 200 {
			fmt.Fprintf(os.Stderr, "FAIL %s: HTTP %d\n", page, res.StatusCode)
			failed++
			res.Body.Close()
			continue
		}
		htmlB, _ := io.ReadAll(res.Body)
		res.Body.Close()
		slices := snapshotSlicePreviews(string(htmlB))
		if len(slices) != len(names) {
			fmt.Fprintf(os.Stderr, "FAIL %s: %d mdx previews != %d SSR slices\n", page, len(names), len(slices))
			failed++
			continue
		}
		// byte-stable JSON: JSON.stringify(doc, null, 1) + "\n", key order =
		// page, previews, preview names in mdx order
		var b strings.Builder
		b.WriteString("{\n \"page\": ")
		b.WriteString(jsonString(page))
		b.WriteString(",\n \"previews\": {")
		for i, n := range names {
			if i > 0 {
				b.WriteString(",")
			}
			b.WriteString("\n  ")
			b.WriteString(jsonString(n))
			b.WriteString(": ")
			b.WriteString(jsonString(normSnapshot(strings.TrimSpace(slices[i]))))
		}
		b.WriteString("\n }\n}\n")
		if err := os.WriteFile(filepath.Join(outDir, page+".json"), []byte(b.String()), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "upstream-snapshot:", err)
			return 1
		}
		total += len(names)
		fmt.Printf("%s: %d previews\n", page, len(names))
	}
	if failed > 0 {
		fmt.Fprintf(os.Stderr, "FAIL  upstream-snapshot (%d pages)\n", failed)
		return 1
	}
	fmt.Printf("upstream-snapshot: %d previews across %d pages -> %s\n", total, len(pages), outDir)
	return 0
}
