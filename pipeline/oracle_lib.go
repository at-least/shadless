package main

// oracle-lib, ported from tools/oracle-lib.mjs — the React oracle render
// environment shared by example-golden (and, once ported, example-oracle /
// example-fixture / contracts). Render env = upstream's provider stack
// (TooltipProvider + DirectionProvider for -rtl demos); the entry template
// closes the crash holes: flushSync propagates render exceptions into
// __err, and an empty #root is itself an error.
//
// Bundle cache: same directory as the JS tools (node_modules/.cache/
// shadless/oracle). The KEY differs by design — it hashes this Go file,
// not oracle-lib.mjs — so a mixed tree (JS example-oracle + Go golden)
// rebuilds the entry when alternating rather than reusing a bundle the
// other side compiled. Cache THRASH, never cache SILENCE: correctness over
// the 1-2s per-demo esbuild pass.

import (
	"crypto/sha256"
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"github.com/evanw/esbuild/pkg/api"
)

func oracleCacheDir() string {
	if d := os.Getenv("SHADLESS_CACHE"); d != "" {
		return d
	}
	return "node_modules/.cache/shadless/oracle"
}

// oracleBundleCacheKey: pin commit + lockfile + example + stubs +
// resolve-skins + skin tables + this file.
func oracleBundleCacheKey(name string) (string, error) {
	h := sha256.New()
	pinB, err := os.ReadFile("src/registry/pin.json")
	if err != nil {
		return "", err
	}
	var pin struct {
		ShadcnUI struct {
			Commit string `json:"commit"`
		} `json:"shadcn_ui"`
	}
	if err := json.Unmarshal(pinB, &pin); err != nil {
		return "", err
	}
	h.Write([]byte(pin.ShadcnUI.Commit + "\n"))
	for _, f := range []string{"package-lock.json",
		filepath.Join(".upstream/shadcn-ui/apps/v4/examples/radix", name+".tsx"),
		"pipeline/resolve_skins.go", "src/emitter/skin.mjs", "pipeline/oracle_lib.go"} {
		b, err := os.ReadFile(f)
		if err != nil {
			return "", err
		}
		h.Write(b)
	}
	stubs, err := os.ReadDir("tools/contracts/stubs")
	if err != nil {
		return "", err
	}
	var names []string
	for _, e := range stubs {
		names = append(names, e.Name())
	}
	sort.Strings(names)
	for _, n := range names {
		b, err := os.ReadFile(filepath.Join("tools/contracts/stubs", n))
		if err != nil {
			return "", err
		}
		h.Write([]byte(n))
		h.Write(b)
	}
	return hex.EncodeToString(h.Sum(nil)), nil
}

// oracleAliases: examples import GENERATED style dirs @/styles/<flavor>-
// <skin>/ui[-rtl]/*; the resolved tree is their tracked equivalent.
func oracleAliases() (map[string]string, error) {
	skins := []string{"nova", "vega", "lyra", "maia", "mira", "luma", "sera", "rhea"}
	resolved, err := filepath.Abs("build/resolved-ui")
	if err != nil {
		return nil, err
	}
	up, err := filepath.Abs(".upstream/shadcn-ui/apps/v4")
	if err != nil {
		return nil, err
	}
	abs := func(p string) string {
		a, _ := filepath.Abs(p)
		return a
	}
	a := map[string]string{
		"@":                                          up,
		"@/registry/bases/radix/ui":                  resolved + "/ui",
		"@/registry/bases/radix/lib":                 resolved + "/lib",
		"@/registry/bases/radix/hooks":               resolved + "/hooks",
		"@/components/language-selector":             abs("tools/contracts/stubs/app-components.jsx"),
		"@/components/markdown":                      abs("tools/contracts/stubs/app-components.jsx"),
		"@/components/message-animated":              abs("tools/contracts/stubs/message-animated.jsx"),
		"@/app/(create)/components/icon-placeholder": abs("tools/contracts/stubs/icon-placeholder.jsx"),
		"next/image":                                 abs("tools/contracts/stubs/next-image.jsx"),
		"next/link":                                  abs("tools/contracts/stubs/next-link.jsx"),
		"date-fns":                                   abs("tools/contracts/stubs/date-fns.mjs"),
		"sonner":                                     abs("tools/contracts/stubs/sonner.jsx"),
		"embla-carousel-autoplay":                    abs("tools/contracts/stubs/embla-autoplay.mjs"),
		"react-textarea-autosize":                    abs("tools/contracts/stubs/textarea-autosize.jsx"),
	}
	for _, f := range []string{"radix", "base", "aria"} {
		for _, s := range skins {
			a["@/styles/"+f+"-"+s+"/ui"] = resolved + "/ui"
			a["@/styles/"+f+"-"+s+"/ui-rtl"] = resolved + "/ui-rtl"
		}
	}
	return a, nil
}

// buildOracleGo bundles the pinned example and writes the oracle page.
// Returns the htmlFile path — goto it, then awaitOracleGo.
func buildOracleGo(name, tmp string) (string, error) {
	dir := "ltr"
	if strings.HasSuffix(name, "-rtl") {
		dir = "rtl"
	}
	cache := oracleCacheDir()
	for _, d := range []string{tmp, cache} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			return "", err
		}
	}
	entry := filepath.Join(cache, ".entry-"+name+".mjs")
	entrySrc := `
import * as React from "react"
import { createRoot } from "react-dom/client"
import { flushSync } from "react-dom"
import { Direction } from "radix-ui"
import * as Mod from "@/examples/radix/` + name + `"
import { TooltipProvider } from "@/registry/bases/radix/ui/tooltip"
const Demo = Mod.default ?? Object.values(Mod).find((v) => typeof v === "function")
try {
  const root = createRoot(document.getElementById("root"))
  flushSync(() => root.render(
    React.createElement(TooltipProvider, { delayDuration: 0 },
      React.createElement(Direction.Provider, { dir: "` + dir + `" }, React.createElement(Demo)))
  ))
  if (!document.querySelector("#root").hasChildNodes()) throw new Error("empty #root after render")
  window.__done = true
} catch (e) { window.__err = String(e?.message ?? e) }
`
	if err := os.WriteFile(entry, []byte(entrySrc), 0o644); err != nil {
		return "", err
	}
	outfile := filepath.Join(cache, "bundle-"+name+".js")
	keyFile := filepath.Join(cache, ".key-"+name)
	key, err := oracleBundleCacheKey(name)
	if err != nil {
		return "", err
	}
	oldKey, _ := os.ReadFile(keyFile)
	if string(oldKey) != key {
		aliases, err := oracleAliases()
		if err != nil {
			return "", err
		}
		res := api.Build(api.BuildOptions{
			EntryPoints: []string{entry},
			Bundle:      true,
			Format:      api.FormatIIFE,
			Outfile:     outfile,
			// the Go API's Write zero value is FALSE — without this the bundle
			// exists only in memory and every render ENOENTs. Went unseen for
			// the whole life of the Go port because the pre-port JS tool had
			// left bundles in node_modules/.cache and the .key hit skipped
			// rebuilding them; any cold cache (fresh clone, CI, the dagger
			// cache volume) rendered nothing.
			Write:    true,
			LogLevel: api.LogLevelError,
			Alias:    aliases,
			Loader:   map[string]api.Loader{".tsx": api.LoaderTSX},
			// automatic runtime: classic JSX emits free React.createElement
			// for sources without an explicit React import (aspect-ratio, …)
			JSX: api.JSXAutomatic,
		})
		if len(res.Errors) > 0 {
			return "", fmt.Errorf("esbuild: %s", res.Errors[0].Text)
		}
		if err := os.WriteFile(keyFile, []byte(key), 0o644); err != nil {
			return "", err
		}
	}
	htmlFile := filepath.Join(tmp, "oracle-"+name+".html")
	rel, err := filepath.Rel(tmp, outfile)
	if err != nil {
		return "", err
	}
	dirAttr := ""
	if dir == "rtl" {
		dirAttr = ` dir="rtl"`
	}
	html := "<!doctype html><html" + dirAttr + "><head><meta charset=\"utf-8\"></head>\n<body><div id=\"root\"></div><script src=\"" + rel + "\"></script></body></html>"
	if err := os.WriteFile(htmlFile, []byte(html), 0o644); err != nil {
		return "", err
	}
	return htmlFile, nil
}

// awaitOracleGo waits for the oracle page to settle; the render error
// surfaces as a Go error.
func awaitOracleGo(p *bpage, htmlFile string) error {
	abs, err := filepath.Abs(htmlFile)
	if err != nil {
		return err
	}
	if err := p.gotoURL("file://" + abs); err != nil {
		return err
	}
	if err := p.waitForFunction("window.__done === true || window.__err !== undefined", 5000); err != nil {
		return err
	}
	errV, _ := p.evaluate("window.__err")
	if s, ok := errV.(string); ok && s != "" {
		return fmt.Errorf("%s", s)
	}
	return nil
}

// reRadixAutoId: every spelling React's useId has had, as radix prefixes it —
// React 18 `:r1:`, 19.0 `«r1»`, 19.1+ CSR `_r_1_` and SSR `_R_1H2_`. The
// trailing `_` is part of the id: the old `radix-_r_[a-z0-9-]*` stopped in
// front of it and left `radix-<auto>_` behind in every page.
var reRadixAutoId = regexp.MustCompile(`radix-(?::r[a-z0-9]*:?|«r[a-z0-9]*»|_[rR]_[A-Za-z0-9-]*_?)`)

// oracleNorm makes radix auto-ids STABLE without making them EQUAL. Each
// distinct id becomes radix-a1, radix-a2, … in order of first appearance, so
// a page renders to the same bytes on every capture (the runtime counter is
// gone) while `aria-controls`/`aria-labelledby` still point at the element
// they pointed at. It used to collapse every id to one literal, radix-<auto>,
// and that literal was written into the shipped pages: accordion-* carried it
// six times per page as `id=`, and every aria reference on the page resolved
// to the first of the six. Comparison never needed the collapse — canon
// treats an auto-id as opaque — only the shipped markup paid for it.
func oracleNorm(html string) string {
	seen := map[string]string{}
	return reRadixAutoId.ReplaceAllStringFunc(html, func(id string) string {
		if t, ok := seen[id]; ok {
			return t
		}
		t := fmt.Sprintf("radix-a%d", len(seen)+1)
		seen[id] = t
		return t
	})
}

// canonExpr is oracle-lib's canonOf verbatim, as a function of ([h]).
// DOM-STRUCTURAL canonical form: parsed in the real browser, canonicalized
// to (tag, sorted attr map, text) trees — SSR/CSR attribute-order and
// entity-escaping differences cannot false-positive.
// canonExpr is oracle-lib's canonOf verbatim (a function of ([h])), kept in
// oracle_canon.js because the body carries JS template literals that cannot
// live inside a Go backtick string.
//
//go:embed oracle_canon.js
var canonExpr string

// canonOfGo: canon a DOM fragment through the real browser.
func canonOfGo(p *bpage, html string) (string, error) {
	v, err := p.evaluateFnArg(canonExpr, []any{html})
	if err != nil {
		return "", err
	}
	b, err := json.Marshal(v)
	if err != nil {
		return "", err
	}
	return string(b), nil
}

// oracleRootHtml: the rendered #root innerHTML, normalized.
func oracleRootHtml(p *bpage) (string, error) {
	v, err := p.evaluateFn(`() => document.querySelector("#root").innerHTML`)
	if err != nil {
		return "", err
	}
	s, _ := v.(string)
	return oracleNorm(s), nil
}
