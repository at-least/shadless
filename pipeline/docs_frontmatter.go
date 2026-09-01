package main

// frontmatter (yaml-lite) + import stripping, ported from
// src/docs/frontmatter.mjs.

import (
	"regexp"
	"strings"
)

type frontmatter map[string]any

// parseFrontmatter tolerates CRLF (a /^---\n/ anchor silently dropped the
// whole block on CRLF files) and coerces true/false/integers; quotes are
// stripped only as a matched pair (so `years'` keeps its apostrophe).
var reFrontmatter = regexp.MustCompile(`(?s)^---\r?\n(.*?)\r?\n---`)

func parseFrontmatter(src string) frontmatter {
	m := reFrontmatter.FindStringSubmatch(src)
	if m == nil {
		return frontmatter{}
	}
	out := frontmatter{}
	var cur map[string]any
	for _, line := range regexp.MustCompile(`\r?\n`).Split(m[1], -1) {
		if strings.TrimSpace(line) == "" || strings.HasPrefix(strings.TrimSpace(line), "#") {
			continue
		}
		if top := reTopKey.FindStringSubmatch(line); top != nil && !startsWithSpace(line) {
			if top[2] == "" {
				inner := map[string]any{}
				out[top[1]] = inner
				cur = inner
			} else {
				out[top[1]] = coerceScalar(top[2])
				cur = nil
			}
			continue
		}
		if sub := reSubKey.FindStringSubmatch(line); sub != nil && cur != nil {
			cur[sub[1]] = coerceScalar(sub[2])
		}
	}
	return out
}

var (
	reTopKey = regexp.MustCompile(`^([A-Za-z][\w-]*):\s*(.*)$`)
	reSubKey = regexp.MustCompile(`^\s+([A-Za-z][\w-]*):\s*(.*)$`)
)

func startsWithSpace(s string) bool { return len(s) > 0 && (s[0] == ' ' || s[0] == '\t') }

func coerceScalar(v string) any {
	switch {
	case v == "true":
		return true
	case v == "false":
		return false
	case reIntOnly.MatchString(v):
		n := 0
		for _, c := range v {
			n = n*10 + int(c-'0')
		}
		return n
	case len(v) >= 2 && v[0] == v[len(v)-1] && (v[0] == '"' || v[0] == '\''):
		return v[1 : len(v)-1]
	}
	return v
}

var reIntOnly = regexp.MustCompile(`^-?\d+$`)

// fmString reads a scalar as a string ("" when absent/not a string).
func fmString(fm frontmatter, key string) string {
	if v, ok := fm[key].(string); ok {
		return v
	}
	return ""
}

// fmLinksOrdered reads frontmatter.links sub-keys from the RAW source in
// textual order — the page-links <p> emits them in that order and a Go map
// would lose it.
var reLinksBlock = regexp.MustCompile(`(?m)^links:\s*$((?:\n\s+[A-Za-z][\w-]*: .*)*)`)

func fmLinksOrdered(raw string) []attrPair {
	m := reLinksBlock.FindStringSubmatch(raw)
	if m == nil {
		return nil
	}
	var out []attrPair
	for _, line := range strings.Split(strings.TrimPrefix(m[1], "\n"), "\n") {
		if sub := reSubKey.FindStringSubmatch(line); sub != nil {
			if s, ok := coerceScalar(sub[2]).(string); ok {
				out = append(out, attrPair{sub[1], s})
			}
		}
	}
	return out
}

// stripImports removes ESM import statements outside fences is NOT this
// function's job (see stripImportsOutsideFences in docs-build): this is the
// fence-agnostic line-anchored strip.
var (
	reImportNamed    = regexp.MustCompile(`(?m)^[ \t]*import[ \t]*(?:\{[\s\S]*?\}[ \t]*|\*(?:[ \t]+as[ \t]+[\w$]+)[ \t]*|[\w$]+[ \t]*)from[ \t]*["'][^"']+["'];?[ \t]*$`)
	reImportBareLine = regexp.MustCompile(`(?m)^[ \t]*import[ \t]*["'][^"']+["'];?[ \t]*$`)
)

func stripImports(src string) string {
	out := reImportNamed.ReplaceAllString(src, "")
	out = reImportBareLine.ReplaceAllString(out, "")
	return out
}
