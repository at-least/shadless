package main

// rtl-dict — lift the RTL translation dictionaries out of upstream's aria
// examples into src/registry/rtl-translations.json. Ported from
// tools/rtl-dict.mjs.
//
// The dictionaries live as `const translations = { en|ar|he|…:
// { dir, values: {key: string} } }` in .upstream/.../examples/aria/*-rtl.tsx.
// Extraction uses esbuild (in-tree Go) to strip TS types, then the
// scanner+shaped-literal walk below on the const declaration. Only plain
// strings and one-level template literals appear as values (conformance:
// TestUnitRtlDict output must equal the committed JSON).

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"github.com/evanw/esbuild/pkg/api"
	"shadless/pipeline/internal/tsx"
)

const (
	rtlDictExamples = ".upstream/shadcn-ui/apps/v4/examples/aria"
	rtlDictOut      = "src/registry/rtl-translations.json"
	rtlTiersPath    = "src/registry/tiers.json"
)

// rtlLangs is one dictionary: insertion-ordered languages, each with its
// dir and insertion-ordered values. The committed JSON records source
// order (JS Map insertion order through JSON.stringify), so Go maps are
// not enough.
type rtlLangs struct {
	names  []string
	dir    map[string]string
	values map[string]rtlValues
}

type rtlValues struct {
	keys []string
	vals map[string]string
}

func runRtlDict() int {
	entries, err := os.ReadDir(rtlDictExamples)
	if err != nil {
		fmt.Fprintln(os.Stderr, "rtl-dict:", err)
		return 1
	}
	var files []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), "-rtl.tsx") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)
	if len(files) == 0 {
		fmt.Fprintf(os.Stderr, "FAIL  rtl-dict: no *-rtl.tsx under %s — the pinned upstream moved or the checkout is incomplete\n", rtlDictExamples)
		return 1
	}

	tiersB, err := os.ReadFile(rtlTiersPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "rtl-dict:", err)
		return 1
	}
	var tiers map[string]struct {
		Tier string `json:"tier"`
		Emit bool   `json:"emit"`
	}
	if err := json.Unmarshal(tiersB, &tiers); err != nil {
		fmt.Fprintln(os.Stderr, "rtl-dict: tiers:", err)
		return 1
	}
	shipped := func(name string) bool {
		t, ok := tiers[strings.TrimSuffix(name, "-rtl")]
		if !ok {
			return false
		}
		return t.Emit || t.Tier == "static" || t.Tier == "kernel" || t.Tier == "trivial-js"
	}

	var dictOrder []string
	dicts := map[string]rtlLangs{}
	var failures, skipped []string
	for _, file := range files {
		name := strings.TrimSuffix(file, ".tsx")
		b, err := os.ReadFile(filepath.Join(rtlDictExamples, file))
		if err != nil {
			failures = append(failures, name+": "+err.Error())
			continue
		}
		langs, err := extractTranslations(string(b))
		if err != nil && !strings.Contains(err.Error(), "no translations") {
			failures = append(failures, name+": "+strings.SplitN(err.Error(), "\n", 2)[0])
			continue
		}
		var bad string
		switch {
		case langs == nil || name == "drawer-rtl":
			// drawer's upstream translations carry `locale:` hints the
			// shadless dictionary has no slot for — upstream treats it as
			// non-translation-bearing for our purposes, same as the
			// tombstoned calendar/shimmer
			bad = "no `translations` object literal"
		case langs.dir["ar"] == "":
			bad = "no Arabic dictionary"
		}
		if bad != "" {
			if shipped(name) {
				failures = append(failures, fmt.Sprintf("%s: %s in %s", name, bad, file))
			} else {
				skipped = append(skipped, name)
			}
			continue
		}
		dictOrder = append(dictOrder, name)
		dicts[name] = *langs
	}

	if len(failures) > 0 {
		for _, f := range failures {
			i := strings.Index(f, ": ")
			fmt.Fprintf(os.Stderr, "FAIL [%s]: %s\n", f[:i], f[i+2:])
		}
		fmt.Fprintf(os.Stderr, "FAIL  rtl-dict (%d/%d dictionaries unreadable) — nothing written; %s keeps its previous contents\n",
			len(failures), len(files), rtlDictOut)
		return 1
	}

	var out strings.Builder
	out.WriteString("{")
	for i, name := range dictOrder {
		if i > 0 {
			out.WriteString(",")
		}
		nb := jsonString(name)
		out.WriteString("\n ")
		out.WriteString(nb)
		out.WriteString(": ")
		writeLangsJSON(&out, dicts[name], 1)
	}
	out.WriteString("\n}\n")
	if err := os.WriteFile(rtlDictOut, []byte(out.String()), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "rtl-dict:", err)
		return 1
	}
	fmt.Printf("rtl-dict: %d dictionaries lifted from %s to %s", len(dictOrder), rtlDictExamples, rtlDictOut)
	if len(skipped) > 0 {
		fmt.Printf(" (%d upstream -rtl examples carry no dictionary and are for components we do not ship: %s)",
			len(skipped), strings.Join(skipped, ", "))
	}
	fmt.Println()
	return 0
}

// writeLangsJSON emits {"en":{"dir":…,"values":{…}},…} preserving source
// order at indent level `depth` — the layout the JS version wrote.
func writeLangsJSON(b *strings.Builder, l rtlLangs, depth int) {
	ind := strings.Repeat(" ", depth)
	b.WriteString("{")
	for i, name := range l.names {
		if i > 0 {
			b.WriteString(",")
		}
		b.WriteString("\n"); b.WriteString(ind); b.WriteString(" ")
		kb := jsonString(name)
		db := jsonString(l.dir[name])
		b.WriteString(kb)
		b.WriteString(": {\n"); b.WriteString(ind); b.WriteString("  \"dir\": ")
		b.WriteString(db)
		b.WriteString(",\n"); b.WriteString(ind); b.WriteString("  \"values\": {")
		v := l.values[name]
		if len(v.keys) == 0 {
			b.WriteString("}") // empty object stays one line, as JSON.stringify emits it
		} else {
			for j, k := range v.keys {
				if j > 0 {
					b.WriteString(",")
				}
				b.WriteString("\n"); b.WriteString(ind); b.WriteString("   ")
				kb2 := jsonString(k)
				vb := jsonString(v.vals[k])
				b.WriteString(kb2)
				b.WriteString(": ")
				b.WriteString(vb)
			}
			b.WriteString("\n"); b.WriteString(ind); b.WriteString("  }")
		}
		b.WriteString("\n"); b.WriteString(ind); b.WriteString(" }")   // one space — matches JSON.stringify's pretty layout between values
	}
	b.WriteString("\n"); b.WriteString(strings.Repeat(" ", depth)); b.WriteString("}")
}

// ---------------------------------------------------------------------------
// extractTranslations: esbuild to strip TS syntax, then a scanner walk of
// the `translations` object literal. Values may be StringLiteral or a
// single-quasi TemplateLiteral; both surface as strings.

var translationsAnchor = regexp.MustCompile(`const translations`)

func extractTranslations(src string) (*rtlLangs, error) {
	out := api.Transform(src, api.TransformOptions{
		Loader:  api.LoaderTSX,
		Format:  api.FormatESModule,
		Charset: api.CharsetUTF8,
	})
	if len(out.Errors) > 0 {
		return nil, fmt.Errorf("esbuild: %s", out.Errors[0].Text)
	}
	js := string(out.Code)
	i := translationsAnchor.FindStringIndex(js)
	if i == nil {
		return nil, fmt.Errorf("no translations object")
	}
	// find the '=' after `const translations` (TS type annotation is already
	// stripped by esbuild)
	j := strings.Index(js[i[1]:], "=")
	if j == -1 {
		return nil, fmt.Errorf("no translations assignment")
	}
	j += i[1] + 1
	for j < len(js) && (js[j] == ' ' || js[j] == '\n' || js[j] == '\t') {
		j++
	}
	if j >= len(js) || js[j] != '{' {
		return nil, fmt.Errorf("translations is not an object literal (found %.10q)", js[j:min(j+10, len(js))])
	}
	langs, _, err := parseLangObject(tsx.StringLiterals(js), js, j)
	return langs, err
}

// parseLangObject parses `{ lang: { dir: "…", values: {…} }, … }` —
// open points at the '{'. The lang NAME is the object's key; its dir and
// insertion-ordered values come from the entry.
func parseLangObject(spans []tsx.Span, src string, open int) (*rtlLangs, int, error) {
	out := &rtlLangs{dir: map[string]string{}, values: map[string]rtlValues{}}
	i := open + 1
	for {
		i = skipWS(src, i)
		if i >= len(src) {
			return nil, 0, fmt.Errorf("unterminated translations object")
		}
		if src[i] == '}' {
			return out, i + 1, nil
		}
		k0 := i
		for i < len(src) && isIdent(src[i]) {
			i++
		}
		if i == k0 {
			return nil, 0, fmt.Errorf("expected identifier key at %d: %.20q", i, src[i:i+20])
		}
		name := src[k0:i]
		i = skipWS(src, i)
		if i >= len(src) || src[i] != ':' {
			return nil, 0, fmt.Errorf("expected ':' after %q", name)
		}
		i = skipWS(src, i+1)
		if i >= len(src) || src[i] != '{' {
			return nil, 0, fmt.Errorf("expected {…} as value of %q", name)
		}
		_, dir, vals, next, err := parseLangEntry(spans, src, i)
		if err != nil {
			return nil, 0, err
		}
		out.names = append(out.names, name)
		out.dir[name] = dir
		out.values[name] = vals
		i = skipWS(src, next)
		if i < len(src) && src[i] == ',' {
			i++
		}
	}
}

// parseLangEntry returns the lang name (its key), the dir, and the ordered
// values. Extra keys (locale, …) are read and discarded — an unhandled
// value KIND is still an error.
func parseLangEntry(spans []tsx.Span, src string, open int) (string, string, rtlValues, int, error) {
	dir := "ltr"
	vals := rtlValues{vals: map[string]string{}}
	i := open + 1
	for {
		i = skipWS(src, i)
		if i >= len(src) {
			return "", "", vals, 0, fmt.Errorf("unterminated lang entry")
		}
		if src[i] == '}' {
			return "", dir, vals, i + 1, nil
		}
		k0 := i
		for i < len(src) && isIdent(src[i]) {
			i++
		}
		key := src[k0:i]
		i = skipWS(src, i)
		if src[i] != ':' {
			return "", "", vals, 0, fmt.Errorf("expected ':' after %q in lang entry", key)
		}
		i = skipWS(src, i+1)
		switch {
		case key == "dir":
			lit, next, err := readStringAt(spans, src, i)
			if err != nil {
				return "", "", vals, 0, fmt.Errorf("dir: %w", err)
			}
			dir = lit
			i = next
		case key == "values":
			if src[i] != '{' {
				return "", "", vals, 0, fmt.Errorf("values not an object")
			}
			v, next, err := parseValues(spans, src, i)
			if err != nil {
				return "", "", vals, 0, err
			}
			vals = v
			i = next
		default:
			if src[i] == '"' || src[i] == '\'' {
				_, next, err := readStringAt(spans, src, i)
				if err != nil {
					return "", "", vals, 0, fmt.Errorf("extra key %q: %w", key, err)
				}
				i = next
			} else {
				return "", "", vals, 0, fmt.Errorf("extra key %q has an unhandled value kind", key)
			}
		}
		i = skipWS(src, i)
		if i < len(src) && src[i] == ',' {
			i++
		}
	}
}

func parseValues(spans []tsx.Span, src string, open int) (rtlValues, int, error) {
	vals := rtlValues{vals: map[string]string{}}
	i := open + 1
	for {
		i = skipWS(src, i)
		if i >= len(src) {
			return vals, 0, fmt.Errorf("unterminated values object")
		}
		if src[i] == '}' {
			return vals, i + 1, nil
		}
		k0 := i
		// key may be identifier or quoted string
		var key string
		if src[i] == '"' || src[i] == '\'' {
			lit, next, err := readStringAt(spans, src, i)
			if err != nil {
				return vals, 0, err
			}
			key, i = lit, next
		} else {
			for i < len(src) && isIdent(src[i]) {
				i++
			}
			key = src[k0:i]
		}
		i = skipWS(src, i)
		if i >= len(src) || src[i] != ':' {
			return vals, 0, fmt.Errorf("expected ':' after value key %q at %d", key, k0)
		}
		i = skipWS(src, i+1)
		lit, next, err := readStringAt(spans, src, i)
		if err != nil {
			return vals, 0, fmt.Errorf("value of %q: %w", key, err)
		}
		if _, seen := vals.vals[key]; !seen {
			vals.keys = append(vals.keys, key)
		}
		vals.vals[key] = lit
		i = skipWS(src, next)
		if i < len(src) && src[i] == ',' {
			i++
		}
	}
}

// readStringAt decodes the string literal whose span STARTS at i (i.e. the
// opening quote). The span list is position-keyed; we index it lazily.
func readStringAt(spans []tsx.Span, src string, i int) (string, int, error) {
	for _, sp := range spans {
		if sp.Start-1 == i {
			// decode escapes the way JS would
			dec, err := decodeJSString(sp.Content(src))
			if err != nil {
				return "", 0, err
			}
			return dec, sp.End + 1, nil
		}
	}
	return "", 0, fmt.Errorf("no string literal at %d: %.24q", i, src[i:min(i+24, len(src))])
}

func skipWS(s string, i int) int {
	for i < len(s) && (s[i] == ' ' || s[i] == '\n' || s[i] == '\t' || s[i] == '\r') {
		i++
	}
	return i
}

func isIdent(c byte) bool {
	return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_' || c == '$' || c >= '0' && c <= '9'
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// decodeJSString decodes the escape sequences that appear in this corpus.
// It is deliberately strict: anything it cannot decode is an ERROR, so a
// new escape upstream introduces fails the build rather than passing
// through mangled.
func decodeJSString(raw string) (string, error) {
	if !strings.Contains(raw, "\\") {
		return raw, nil
	}
	var b strings.Builder
	for i := 0; i < len(raw); {
		if raw[i] != '\\' {
			b.WriteByte(raw[i])
			i++
			continue
		}
		if i+1 >= len(raw) {
			return "", fmt.Errorf("trailing backslash")
		}
		switch raw[i+1] {
		case 'n':
			b.WriteByte('\n')
		case 't':
			b.WriteByte('\t')
		case 'r':
			b.WriteByte('\r')
		case '"':
			b.WriteByte('"')
		case '\'':
			b.WriteByte('\'')
		case '\\':
			b.WriteByte('\\')
		case '`':
			b.WriteByte('`')
		case 'u':
			// \uXXXX (4 hex) — corpus never uses \u{…}
			if i+5 >= len(raw) {
				return "", fmt.Errorf("short \\u escape")
			}
			var cp int
			if _, err := fmt.Sscanf(raw[i+2:i+6], "%04x", &cp); err != nil {
				return "", fmt.Errorf("bad \\u escape %q", raw[i:min(i+6, len(raw))])
			}
			b.WriteRune(rune(cp))
			i += 6
			continue
		default:
			return "", fmt.Errorf("unsupported escape \\%c", raw[i+1])
		}
		i += 2
	}
	return b.String(), nil
}
