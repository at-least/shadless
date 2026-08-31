package main

// resolve-skins — upstream generation parity at the SOURCE level. Ported
// from tools/resolve-skins.mjs; semantics are its header, repeated here so
// this file stands alone:
//
// Upstream's generator (packages/shadcn/src/styles/transform-style-map.ts)
// resolves cn-* classes in bases/*/ui/*.tsx into the skin's inline Tailwind
// utilities before anything renders. shadless replicates that resolve:
// cn-X in the skin's @apply body expands in place, allowlisted names stay,
// marker-only names styled by no skin are DROPPED, and every class-string
// literal then passes twMerge (internal/twmerge — cn() == clsx+twMerge, so
//
// The output tree build/resolved-ui/{ui,ui-rtl} mirrors bases/radix, with
// ui-rtl additionally RTL-transformed (logical-class mapping, translate-x
// mirrored, cn-rtl-flip → rtl:rotate-180). lib/ and hooks/ copy verbatim.

import (
	_ "embed"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"shadless/pipeline/internal/tsx"
	"shadless/pipeline/internal/twmerge"
)

const (
	resolveSrc = ".upstream/shadcn-ui/apps/v4/registry/bases/radix"
	resolveOut = "build/resolved-ui"
	skinPath   = ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css"
)

// skinData mirrors src/emitter/skin.mjs: cn-X → @apply body, plus the
// transform-style-map ALLOWLIST. Both sides are generated from the same
// source file; this is the Go copy.
var skinData struct {
	Map       map[string]string
	Allowlist map[string]bool
}

var cnTokenRe = regexp.MustCompile(`(^|\s)cn-[a-z0-9-]+`)

type resolveEdit struct {
	start, end int
	value      string
}

func expandClassString(str string) string {
	toks := strings.Fields(str)
	hasCN := false
	for _, t := range toks {
		if strings.HasPrefix(t, "cn-") {
			hasCN = true
			break
		}
	}
	if !hasCN {
		return str
	}
	var expanded []string
	for _, t := range toks {
		if !strings.HasPrefix(t, "cn-") {
			expanded = append(expanded, t)
			continue
		}
		if skinData.Allowlist[t] {
			expanded = append(expanded, t)
			continue
		}
		if body, ok := skinData.Map[t]; ok {
			expanded = append(expanded, strings.Fields(body)...)
		}
		// styled-by-no-skin markers drop out (no entries appended)
	}
	return twmerge.Merge(strings.Join(expanded, " "))
}

// --- RTL transform (upstream transform-rtl.ts parity) ----------------------

var rtlMappings = [][2]string{
	{"-ml-", "-ms-"}, {"-mr-", "-me-"}, {"ml-", "ms-"}, {"mr-", "me-"},
	{"pl-", "ps-"}, {"pr-", "pe-"}, {"-left-", "-start-"}, {"-right-", "-end-"},
	{"left-", "start-"}, {"right-", "end-"}, {"inset-l-", "inset-inline-start-"},
	{"inset-r-", "inset-inline-end-"}, {"rounded-tl-", "rounded-ss-"},
	{"rounded-tr-", "rounded-se-"}, {"rounded-bl-", "rounded-es-"},
	{"rounded-br-", "rounded-ee-"}, {"rounded-l-", "rounded-s-"},
	{"rounded-r-", "rounded-e-"}, {"border-l-", "border-s-"}, {"border-r-", "border-e-"},
	{"border-l", "border-s"}, {"border-r", "border-e"}, {"text-left", "text-start"},
	{"text-right", "text-end"}, {"scroll-ml-", "scroll-ms-"}, {"scroll-mr-", "scroll-me-"},
	{"scroll-pl-", "scroll-ps-"}, {"scroll-pr-", "scroll-pe-"},
	{"float-left", "float-start"}, {"float-right", "float-end"},
	{"clear-left", "clear-start"}, {"clear-right", "clear-end"},
	{"origin-top-left", "origin-top-start"}, {"origin-top-right", "origin-top-end"},
	{"origin-bottom-left", "origin-bottom-start"}, {"origin-bottom-right", "origin-bottom-end"},
	{"origin-left", "origin-start"}, {"origin-right", "origin-end"},
}

var rtlTranslateX = [][2]string{{"-translate-x-", "translate-x-"}, {"translate-x-", "-translate-x-"}}
var rtlReverse = [][2]string{{"space-x-", "space-x-reverse"}, {"divide-x-", "divide-x-reverse"}}
var rtlSwap = [][2]string{{"cursor-w-resize", "cursor-e-resize"}, {"cursor-e-resize", "cursor-w-resize"}}

var rtlLogicalSlide = [][3]string{
	{"data-[side=inline-start]", "slide-in-from-right", "slide-in-from-end"},
	{"data-[side=inline-start]", "slide-out-to-right", "slide-out-to-end"},
	{"data-[side=inline-end]", "slide-in-from-left", "slide-in-from-start"},
	{"data-[side=inline-end]", "slide-out-to-left", "slide-out-to-start"},
}

var positioningPrefixes = []string{"-left-", "-right-", "left-", "right-"}

// splitClassName mirrors upstream splitClassName: bracket-aware last-colon
// split for the variant, then first '/' outside brackets separates alpha.
func splitClassName(cls string) (variant, value, alpha *string) {
	lastColon := -1
	depth := 0
	for i := len(cls) - 1; i >= 0; i-- {
		switch cls[i] {
		case ']':
			depth++
		case '[':
			depth--
		case ':':
			if depth == 0 {
				lastColon = i
				goto colon
			}
		}
	}
colon:
	rest := cls
	if lastColon != -1 {
		v := cls[:lastColon]
		variant = &v
		rest = cls[lastColon+1:]
	}
	slash := -1
	depth = 0
	for i := 0; i < len(rest); i++ {
		switch rest[i] {
		case '[':
			depth++
		case ']':
			depth--
		case '/':
			if depth == 0 {
				slash = i
				goto sl
			}
		}
	}
sl:
	if slash == -1 {
		value = &rest
		return
	}
	v := rest[:slash]
	a := rest[slash+1:]
	value, alpha = &v, &a
	return
}

// variantPrefix builds "variant:value", or "rtl:variant:value" when rtl ==
// true (the JS emitted `${variant ? `rtl:${variant}:${v}` : `rtl:${v}`}` —
// a class already under rtl: never gains a second one, because the caller
// guards cls.startsWith("rtl:")).
func variantPrefixed(variant *string, value string, rtl bool) string {
	if variant == nil {
		if rtl {
			return "rtl:" + value
		}
		return value
	}
	if rtl {
		return "rtl:" + *variant + ":" + value
	}
	return *variant + ":" + value
}

// applyRtlMapping mirrors applyRtlMapping in tools/resolve-skins.mjs branch
// for branch.
func applyRtlMapping(input string) string {
	var out []string
	for _, cls := range strings.Fields(input) {
		if strings.HasPrefix(cls, "rtl:") || strings.HasPrefix(cls, "ltr:") {
			out = append(out, cls)
			continue
		}
		if cls == "cn-rtl-flip" {
			out = append(out, "rtl:rotate-180")
			continue
		}
		variant, valueP, alpha := splitClassName(cls)
		if valueP == nil {
			out = append(out, cls)
			continue
		}
		value := *valueP
		m := func(v string) string {
			if alpha != nil {
				return v + "/" + *alpha
			}
			return v
		}
		done := false
		for _, p := range rtlTranslateX {
			if strings.HasPrefix(value, p[0]) {
				rv := strings.Replace(value, p[0], p[1], 1)
				out = append(out, cls, variantPrefixed(variant, m(rv), true))
				done = true
				break
			}
		}
		if !done {
			for _, p := range rtlReverse {
				if strings.HasPrefix(value, p[0]) {
					out = append(out, cls, variantPrefixed(variant, p[1], true))
					done = true
					break
				}
			}
		}
		if !done {
			for _, p := range rtlSwap {
				if value == p[0] {
					out = append(out, cls, variantPrefixed(variant, p[1], true))
					done = true
					break
				}
			}
		}
		if !done {
			for _, p := range rtlLogicalSlide {
				if variant != nil && strings.Contains(*variant, p[0]) && strings.HasPrefix(value, p[1]) {
					mapped := strings.Replace(value, p[1], p[2], 1)
					out = append(out, variantPrefixed(variant, m(mapped), false))
					done = true
					break
				}
			}
		}
		if done {
			continue
		}
		isPhysSide := variant != nil &&
			(strings.Contains(*variant, "data-[side=left]") || strings.Contains(*variant, "data-[side=right]"))
		mapped := value
		for _, p := range rtlMappings {
			if isPhysSide && hasAnyPrefix(p[0], positioningPrefixes) {
				continue
			}
			if strings.HasPrefix(value, p[0]) {
				if !strings.HasSuffix(p[0], "-") && value != p[0] {
					continue
				}
				mapped = strings.Replace(value, p[0], p[1], 1)
				break
			}
		}
		out = append(out, variantPrefixed(variant, m(mapped), false))
	}
	return strings.Join(out, " ")
}

func hasAnyPrefix(s string, prefixes []string) bool {
	for _, p := range prefixes {
		if strings.HasPrefix(s, p) {
			return true
		}
	}
	return false
}

// resolveSource transforms one tsx file: every StringLiteral carrying a
// cn-* token (or, when rtl, an RTL-mappable class pattern) gets replaced,
// spliced by source offset in reverse order. The scanner's spans are byte
// offsets; the JS implementation spliced on parser start/end which — on
// this corpus — are the same bytes.
var rtlMappable = regexp.MustCompile(`(^|\s)(ml-|mr-|pl-|pr-|left-|right-|rounded-[tlbr]+-|border-[lr]\b|text-(left|right)|translate-x-|space-x-|divide-x-|float-|clear-|origin-|scroll-[mp][lr]-|inset-[lr]-|cursor-[we]-resize)`)

func resolveSource(src string, rtl bool) (string, int) {
	var edits []resolveEdit
	for _, sp := range tsx.StringLiterals(src) {
		if sp.Template {
			continue // only plain strings; templates with interpolation are code
		}
		content := sp.Content(src)
		var next string
		switch {
		case cnTokenRe.MatchString(content):
			next = expandClassString(content)
			if rtl {
				next = applyRtlMapping(next)
			}
		case rtl && rtlMappable.MatchString(content):
			next = applyRtlMapping(content)
		default:
			continue
		}
		if next != content {
			edits = append(edits, resolveEdit{sp.Start, sp.End, jsStringQuote(next)})
		}
	}
	out := src
	sort.Slice(edits, func(i, j int) bool { return edits[i].start > edits[j].start })
	for _, e := range edits {
		out = out[:e.start-1] + e.value + out[e.end+1:]
	}
	return out, len(edits)
}

// jsStringQuote emits JSON.stringify(s): double-quoted, \-escapes for
// control chars, `\"`, `\\`; unicode passed through raw (JSON.stringify
// escapes NOTHING non-ASCII, unlike encoding/json's HTML-safe default which
// turned `&` into — a byte-difference the parity test caught).
func jsStringQuote(s string) string {
	var b strings.Builder
	b.WriteByte('"')
	for _, r := range s {
		switch {
		case r == '"':
			b.WriteString(`\"`)
		case r == '\\':
			b.WriteString(`\\`)
		case r == '\n':
			b.WriteString(`\n`)
		case r == '\r':
			b.WriteString(`\r`)
		case r == '\t':
			b.WriteString(`\t`)
		case r < 0x20:
			fmt.Fprintf(&b, `\u%04x`, r)
		default:
			b.WriteRune(r)
		}
	}
	b.WriteByte('"')
	return b.String()
}

// resolveFixtureHtml expands cn-* in kernel fixtures' class attributes.
// Idempotent.
func resolveFixtureHtml(html string) string {
	return classAttrRe.ReplaceAllStringFunc(html, func(m string) string {
		inner := m[len(`class="`) : len(m)-1]
		if !regexp.MustCompile(`(^|\s)cn-`).MatchString(inner) {
			return m
		}
		next := expandClassString(inner)
		return `class="` + next + `"`
	})
}

var classAttrRe = regexp.MustCompile(`class="[^"]*"`)

// runResolveSkins is the `pipeline resolve-skins [--fixtures]` entry.
func runResolveSkins(args []string) int {
	loadSkin()
	if err := os.RemoveAll(resolveOut); err != nil {
		fmt.Fprintln(os.Stderr, "resolve-skins:", err)
		return 1
	}
	if err := os.MkdirAll(resolveOut, 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "resolve-skins:", err)
		return 1
	}
	files, edits := 0, 0
	copyTree := func(dir, outDir string, transform func(string) (string, int)) {
		walkRoot := resolveSrc + "/" + dir
		err := filepath.WalkDir(walkRoot, func(p string, d fs.DirEntry, err error) error {
			if err != nil {
				return err
			}
			rel, _ := filepath.Rel(walkRoot, p)
			o := filepath.Join(resolveOut, outDir, rel)
			if d.IsDir() {
				return os.MkdirAll(o, 0o755)
			}
			src, err := os.ReadFile(p)
			if err != nil {
				return err
			}
			if transform != nil && strings.HasSuffix(p, ".tsx") {
				out, n := transform(string(src))
				files++
				edits += n
				return os.WriteFile(o, []byte(out), 0o644)
			}
			return os.WriteFile(o, src, 0o644)
		})
		if err != nil {
			fmt.Fprintln(os.Stderr, "resolve-skins:", err)
			os.Exit(1)
		}
	}
	copyTree("ui", "ui", func(s string) (string, int) { return resolveSource(s, false) })
	copyTree("ui", "ui-rtl", func(s string) (string, int) { return resolveSource(s, true) })
	copyTree("lib", "lib", nil)
	copyTree("hooks", "hooks", nil)

	fixtures := false
	for _, a := range args {
		if a == "--fixtures" {
			fixtures = true
		}
	}
	if fixtures {
		fx := 0
		ents, _ := os.ReadDir("src/kernel")
		for _, e := range ents {
			if !strings.HasSuffix(e.Name(), ".html") {
				continue
			}
			p := "src/kernel/" + e.Name()
			html, _ := os.ReadFile(p)
			next := resolveFixtureHtml(string(html))
			if next != string(html) {
				if err := os.WriteFile(p, []byte(next), 0o644); err != nil {
					fmt.Fprintln(os.Stderr, "resolve-skins --fixtures:", err)
					return 1
				}
				fx++
			}
		}
		fmt.Printf("resolve-skins: fixtures rewritten: %d\n", fx)
	}
	fmt.Printf("resolve-skins: %d ui files resolved (%d class strings), tree at %s\n", files, edits, resolveOut)
	return 0
}

// loadSkin parses style-nova.css into skinData. It is a plain flat-block
// scan identical to skin.mjs's parseSkinMap; anything other than flat
// pure-@apply blocks fails loudly.
func loadSkin() {
	if skinData.Map != nil {
		return
	}
	b, err := os.ReadFile(skinPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "resolve-skins: skin:", err)
		os.Exit(1)
	}
	skinData.Map = map[string]string{}
	skinData.Allowlist = map[string]bool{
		"cn-menu-target":      true,
		"cn-menu-translucent": true,
		"cn-rtl-flip":         true,
		"cn-font-heading":     true,
	}
	parseSkinMap(string(b))
}

var skinBlockRe = regexp.MustCompile(`(?s)\.([\w-]+)\s*\{[^{}]*\}`)
var skinBlockInner = regexp.MustCompile(`(?s)^\.[\w-]+\s*\{(.*)\}$`)
var applyStmt = regexp.MustCompile(`^\s*@apply\s+([^;]+);\s*$`)

func parseSkinMap(css string) {
	start := strings.Index(css, "{")
	end := strings.LastIndex(css, "}")
	if start == -1 || end == -1 || !regexp.MustCompile(`^\s*\.style-nova\s*\{`).MatchString(css) {
		fmt.Fprintln(os.Stderr, "skin: expected a single top-level .style-nova block")
		os.Exit(1)
	}
	body := css[start+1 : end]
	for _, m := range skinBlockRe.FindAllString(body, -1) {
		inner := skinBlockInner.FindStringSubmatch(m)
		if inner == nil {
			fmt.Fprintf(os.Stderr, "skin: unparsable block: %.60s\n", m)
			os.Exit(1)
		}
		name := regexp.MustCompile(`^\.([\w-]+)`).FindStringSubmatch(m)[1]
		decls := strings.TrimSpace(inner[1])
		sub := applyStmt.FindStringSubmatch(decls)
		if sub == nil {
			fmt.Fprintf(os.Stderr, "skin: cn-%s is not a flat pure-@apply block: %.60s\n", name, decls)
			os.Exit(1)
		}
		skinData.Map[name] = sub[1]
	}
	if len(skinData.Map) == 0 {
		fmt.Fprintln(os.Stderr, "skin: no cn-* blocks found")
		os.Exit(1)
	}
}
