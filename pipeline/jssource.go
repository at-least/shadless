package main

// Reading data out of the JS modules that are still JS.
//
// Some tooling cannot move to Go: src/emitter/css.mjs needs tailwind-merge,
// src/converter/index.mjs needs @babel/parser, tools/docs-build.mjs needs
// mdx/remark/shiki. None has a Go equivalent that produces identical output,
// and `reproducible` compares the committed trees byte for byte — so a
// rewrite would not be a port.
//
// The Go tools that must agree with those modules therefore read their
// declarations as DATA rather than keeping a second copy. That is the
// existing pattern here (product_css.go lifts SHADLESS_CSS_FIXES the same
// way); this file is the shared, tested implementation of it.
//
// Every extractor FAILS LOUDLY when its anchor is gone. Returning an empty
// set instead would let a gate pass over nothing — the vacuous-green failure
// mode the meta-gate exists to catch, and the one that is hardest to notice
// because the report still says PASS.

import (
	"fmt"
	"regexp"
	"strings"
)

// reJSLineComment strips `//` comments. Only used on regions known to hold
// simple literals and comments — never on a whole module, where a `//` inside
// a template literal or a string would be misread.
var reJSLineComment = regexp.MustCompile(`(?m)//[^\n]*$`)

// reJSString matches a double-quoted JS string literal, honouring escapes.
var reJSString = regexp.MustCompile(`"((?:[^"\\]|\\.)*)"`)

// jsStringsIn returns every double-quoted string in src, in order, with
// comments removed first so a commented-out entry is not counted.
func jsStringsIn(src string) []string {
	src = reJSLineComment.ReplaceAllString(src, "")
	var out []string
	for _, m := range reJSString.FindAllStringSubmatch(src, -1) {
		out = append(out, jsUnescape(m[1]))
	}
	return out
}

// jsUnescape resolves the escapes a JS string literal can carry in these
// sources. \u is handled because upstream reasons contain em dashes.
func jsUnescape(s string) string {
	if !strings.Contains(s, `\`) {
		return s
	}
	var b strings.Builder
	for i := 0; i < len(s); i++ {
		if s[i] != '\\' || i+1 >= len(s) {
			b.WriteByte(s[i])
			continue
		}
		i++
		switch s[i] {
		case 'n':
			b.WriteByte('\n')
		case 't':
			b.WriteByte('\t')
		case 'r':
			b.WriteByte('\r')
		case 'u':
			if i+4 < len(s) {
				var r rune
				if _, err := fmt.Sscanf(s[i+1:i+5], "%04x", &r); err == nil {
					b.WriteRune(r)
					i += 4
					continue
				}
			}
			b.WriteByte(s[i])
		default:
			b.WriteByte(s[i])
		}
	}
	return b.String()
}

// jsSetLiteral extracts `export const NAME = new Set([ "a", "b" ])` — the
// shape every allowlist in this repo uses. The name is part of the anchor, so
// renaming the export in JS fails here instead of silently emptying the set.
func jsSetLiteral(src, name string) ([]string, error) {
	re := regexp.MustCompile(`(?:export\s+)?const\s+` + regexp.QuoteMeta(name) + `\s*=\s*new\s+Set\(\s*\[`)
	loc := re.FindStringIndex(src)
	if loc == nil {
		return nil, fmt.Errorf("%s: `new Set([...])` declaration not found", name)
	}
	body, err := jsBalanced(src[loc[1]-1:], '[', ']')
	if err != nil {
		return nil, fmt.Errorf("%s: %w", name, err)
	}
	return jsStringsIn(body), nil
}

// jsBalanced returns the text between the opening delimiter at src[0] and its
// match, exclusive. It skips over strings and template literals so a bracket
// inside one does not close the region early.
func jsBalanced(src string, open, close byte) (string, error) {
	if len(src) == 0 || src[0] != open {
		return "", fmt.Errorf("expected %q at the start of the region", open)
	}
	depth := 0
	for i := 0; i < len(src); i++ {
		switch c := src[i]; c {
		case '"', '\'', '`':
			j := i + 1
			for j < len(src) {
				if src[j] == '\\' {
					j += 2
					continue
				}
				if src[j] == c {
					break
				}
				j++
			}
			if j >= len(src) {
				return "", fmt.Errorf("unterminated string literal")
			}
			i = j
		case open:
			depth++
		case close:
			depth--
			if depth == 0 {
				return src[1:i], nil
			}
		}
	}
	return "", fmt.Errorf("unbalanced %q ... %q", open, close)
}

// jsObjectField returns the text of `name: { ... }` within src, or ok=false
// when the field is absent. Absence is legitimate for optional fields, so it
// is not an error — but a field that IS present and malformed is.
func jsObjectField(src, name string) (string, bool, error) {
	re := regexp.MustCompile(`(?m)^\s*` + regexp.QuoteMeta(name) + `\s*:\s*\{`)
	loc := re.FindStringIndex(src)
	if loc == nil {
		return "", false, nil
	}
	body, err := jsBalanced(src[loc[1]-1:], '{', '}')
	if err != nil {
		return "", false, fmt.Errorf("%s: %w", name, err)
	}
	return body, true, nil
}

// jsFieldIsFalse reports whether `name: false` appears at field position.
// Anchored to the line start so a mention inside a comment or a longer
// identifier does not count.
func jsFieldIsFalse(src, name string) bool {
	re := regexp.MustCompile(`(?m)^\s*` + regexp.QuoteMeta(name) + `\s*:\s*false\s*,?\s*$`)
	return re.MatchString(src)
}

// reJSAttrList matches one `"slot": ["a", "b"]` entry of an ignoreAttrs map.
var reJSAttrList = regexp.MustCompile(`"((?:[^"\\]|\\.)*)"\s*:\s*\[([^\]]*)\]`)

// jsAttrMap parses an `ignoreAttrs`-shaped object body into ordered
// (key, values) pairs. Order is preserved because the ledger ids derived from
// it are written to a file whose diff should stay readable.
type jsAttrEntry struct {
	Key    string
	Values []string
}

func jsAttrMap(body string) []jsAttrEntry {
	body = reJSLineComment.ReplaceAllString(body, "")
	var out []jsAttrEntry
	for _, m := range reJSAttrList.FindAllStringSubmatch(body, -1) {
		out = append(out, jsAttrEntry{Key: jsUnescape(m[1]), Values: jsStringsIn(m[2])})
	}
	return out
}

// jsCountSetEntries counts the string entries of a `new Set([...])` without
// caring what they are — the interactivity budget is a count, not a list.
func jsCountSetEntries(src, name string) (int, error) {
	items, err := jsSetLiteral(src, name)
	if err != nil {
		return -1, err
	}
	return len(items), nil
}
