package main

// convert.go — Go port of src/converter/index.mjs, the last @babel/parser
// consumer. Registry .tsx -> versioned IR (src/registry/ir/*.json), with the
// drift gates that kept the JS honest (string-count reconciliation, class
// verbatim checks, cn-completeness, tier agreement).
//
// Route (decided 2026-09-02): no importable pure-Go TSX AST exists (esbuild's
// js_ast is internal; typescript-go exposes only cmd/tsgo; goja parses no
// TS/JSX), so the TSX is DOWNGRADED first — esbuild Transform with the
// classic runtime turns JSX into `/* @__PURE__ */ React.createElement(tag,
// props, children…)` — and the plain-JS output is scanned. Everything the
// converter needs survives the downgrade: import/export statements, top-level
// cva() tables, fn signatures with literal defaults, fn-local tag vars, and
// the full call tree in document (pre-)order.
//
// The ONE thing the downgrade loses is child provenance: a JSXText child and
// a `{" "}` expression container both become a string argument, and a
// `{/* comment */}` container vanishes entirely — while babel's sketch
// distinguishes "text" / "expr" / "{ident}" / "OPT?". So the ORIGINAL source
// is scanned by a small JSX-children classifier (scanJsxKinds) producing one
// kind list per element in document order, and the two walks are zipped by
// position. Every structure/attribute/class decision comes from the
// downgraded text; only the child kind strings come from the original.
//
// Order is the interface (see HANDOFF): every object serialized into the IR
// — cva axes and values, defaults, attrs, tagHints — is an insertion-ordered
// jsonObj, and the element/component/cvaRef arrays follow document order.

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
	cvUI      = "build/resolved-ui/ui"
	cvOut     = "src/registry/ir"
	cvPinFile = "src/registry/pin.json"
	cvTiers   = "src/registry/tiers.json"
)

// tier decision tables [measured] from probes/h3,h5 (src/registry/tiers.json).
// The ORDER of each list is load-bearing: overlay's runtime:core hash pins
// the literal order these names were written in the JS source.
var cvTierSets = []struct {
	tier  string
	names []string
}{
	{"kernel", []string{"alert-dialog", "context-menu", "dialog", "dropdown-menu",
		"hover-card", "popover", "select", "slider", "scroll-area", "sheet", "tabs", "tooltip"}},
	{"trivial-js", []string{"accordion", "aspect-ratio", "avatar", "checkbox",
		"collapsible", "label", "progress", "radio-group", "separator", "switch",
		"toggle", "toggle-group"}},
	{"medium", []string{"menubar", "navigation-menu"}},
	{"logic", []string{"combobox", "field", "sidebar"}},
	// bases/radix addition: questionnaire is a foreign-runtime wrapper
	// (@shadcn/react/questionnaire) — external like react-day-picker, but the
	// @shadcn/ prefix is also used for build plumbing so the filter can't key
	// on it alone.
	{"external", []string{"questionnaire"}},
}

// lucide icon component names render as <svg> in the no-React emit;
// names ending in Icon resolve via the /Icon$/ fallback regardless.
var cvKnownIcons = []string{"ChevronRight", "ChevronDown",
	"MoreHorizontal", "Check", "X", "Plus", "Minus", "Search"}

var cvTierIndex = func() map[string]string {
	m := map[string]string{}
	for _, s := range cvTierSets {
		for _, n := range s.names {
			m[n] = s.tier
		}
	}
	return m
}()

func tierOf(name string, imports []string) string {
	if t, ok := cvTierIndex[name]; ok {
		return t
	}
	for _, i := range imports {
		if cvForeignImport(i) {
			return "external"
		}
	}
	return "static"
}

func cvForeignImport(i string) bool {
	for _, p := range []string{"radix-ui", "@radix-ui", "@shadcn/", "@/", "next/"} {
		if strings.HasPrefix(i, p) {
			return false
		}
	}
	switch i {
	case "react", "react-dom", "lucide-react", "class-variance-authority", "clsx", "tailwind-merge":
		return false
	}
	if strings.HasPrefix(i, ".") {
		return false
	}
	return true
}

// ---------------------------------------------------------------------------
// text scanning utilities (plain JS text — the downgraded output, and the
// masked code regions of the original)

// cvMaskEnd: if a masked token (string, template, comment, regex) starts at
// s[i], return the index just past it; else 0.
func cvMaskEnd(s string, i int) int {
	switch s[i] {
	case '"', '\'':
		return cvSkipString(s, i)
	case '`':
		return cvSkipTemplate(s, i)
	case '/':
		if i+1 < len(s) {
			switch s[i+1] {
			case '/':
				return cvSkipLineComment(s, i)
			case '*':
				return cvSkipBlockComment(s, i)
			}
			if cvRegexAllowed(s, i) {
				return cvSkipRegex(s, i)
			}
		}
	}
	return 0
}

func cvSkipString(s string, i int) int {
	q := s[i]
	i++
	for i < len(s) {
		if s[i] == '\\' {
			i += 2
			continue
		}
		if s[i] == q || s[i] == '\n' {
			return i + 1
		}
		i++
	}
	return i
}

func cvSkipTemplate(s string, i int) int {
	i++
	for i < len(s) {
		c := s[i]
		if c == '\\' {
			i += 2
			continue
		}
		if c == '`' {
			return i + 1
		}
		if c == '$' && i+1 < len(s) && s[i+1] == '{' {
			i = cvSkipBraces(s, i+2, 1)
			continue
		}
		i++
	}
	return i
}

// cvSkipBraces consumes until `depth` open braces are closed, mask-aware.
func cvSkipBraces(s string, i, depth int) int {
	for i < len(s) && depth > 0 {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		switch s[i] {
		case '{':
			depth++
		case '}':
			depth--
		}
		i++
	}
	return i
}

func cvSkipLineComment(s string, i int) int {
	for i < len(s) && s[i] != '\n' {
		i++
	}
	return i
}

func cvSkipBlockComment(s string, i int) int {
	i += 2
	for i+1 < len(s) && !(s[i] == '*' && s[i+1] == '/') {
		i++
	}
	return i + 2
}

func cvRegexAllowed(s string, i int) bool {
	for j := i - 1; j >= 0; j-- {
		c := s[j]
		if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
			continue
		}
		switch c {
		case ',', ';', ':', '(', '[', '{', '=', '<', '>', '&', '|', '^', '!', '~', '?', '-':
			return true
		}
		return false
	}
	return true
}

func cvSkipRegex(s string, i int) int {
	i++
	inClass := false
	for i < len(s) {
		c := s[i]
		if c == '\\' {
			i += 2
			continue
		}
		switch {
		case c == '[':
			inClass = true
		case c == ']':
			inClass = false
		case c == '/' && !inClass:
			i++
			for i < len(s) && (s[i] >= 'a' && s[i] <= 'z' || s[i] >= 'A' && s[i] <= 'Z') {
				i++
			}
			return i
		case c == '\n':
			return i
		}
		i++
	}
	return i
}

// cvSplitTop splits s on top-level commas (mask-aware; () [] {} all nest).
func cvSplitTop(s string) []string {
	var out []string
	depth := 0
	start := 0
	for i := 0; i < len(s); {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		switch s[i] {
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case ',':
			if depth == 0 {
				out = append(out, s[start:i])
				start = i + 1
			}
		}
		i++
	}
	return append(out, s[start:])
}

// cvTopOps: at a code position, return the candidate token's length for a
// hit, else 0.
type cvTopOps func(s string, i int) int

func cvOpsQ(s string, i int) int { // a real conditional '?': not ?. ?? ??=
	if s[i] != '?' || i+1 >= len(s) {
		return 0
	}
	switch s[i+1] {
	case '.', '?':
		return 0
	}
	return 1
}

func cvOpsQorLogical(s string, i int) int { // '?' (incl ??) / && / || — the OPT? family
	if s[i] == '?' {
		if i+1 >= len(s) {
			return 1
		}
		if s[i+1] == '.' {
			return 0
		}
		if s[i+1] == '?' && i+2 < len(s) && s[i+2] == '=' {
			return 0
		}
		return 1
	}
	if i+1 < len(s) && (s[i:i+2] == "&&" || s[i:i+2] == "||") {
		if i+2 < len(s) && s[i+2] == '=' {
			return 0
		}
		return 2
	}
	return 0
}

func cvOpsOr(s string, i int) int { // || / ?? (not ??=) — the LOWEST logical tier
	if i+1 < len(s) && (s[i:i+2] == "||" || s[i:i+2] == "??") {
		if i+2 < len(s) && s[i+2] == '=' {
			return 0
		}
		return 2
	}
	return 0
}

func cvOpsAnd(s string, i int) int { // && (not &&=)
	if i+1 < len(s) && s[i:i+2] == "&&" {
		if i+2 < len(s) && s[i+2] == '=' {
			return 0
		}
		return 2
	}
	return 0
}

// cvFirstTop finds the first top-level position matching ops. Masking as
// everywhere; {} [] nest; parens nest ONLY when they are call invocations
// (identifier or ]/)/ before '(') — grouping parens stay transparent,
// mirroring babel's AST where parens are not nodes. An => arrow with an
// expression body masks its body (a conditional inside an arrow belongs to
// the arrow, not the container): it ends at a ',' or a closer dropping below
// the arrow's depth.
func cvFirstTop(s string, ops cvTopOps) int {
	depth := 0
	arrowDepth := -1
	var parenStack []bool
	for i := 0; i < len(s); {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		c := s[i]
		switch {
		case c == '(':
			call := false
			for j := i - 1; j >= 0; j-- {
				ch := s[j]
				if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
					continue
				}
				call = ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' ||
					ch >= '0' && ch <= '9' || ch == '_' || ch == '$' || ch == ')' || ch == ']'
				break
			}
			parenStack = append(parenStack, call)
			if call {
				depth++
			}
		case c == ')':
			if len(parenStack) > 0 {
				if parenStack[len(parenStack)-1] {
					depth--
				}
				parenStack = parenStack[:len(parenStack)-1]
			}
			if arrowDepth >= 0 && depth < arrowDepth {
				arrowDepth = -1
			}
		case c == '[' || c == '{':
			depth++
		case c == ']' || c == '}':
			depth--
			if arrowDepth >= 0 && depth < arrowDepth {
				arrowDepth = -1
			}
		case c == ',' && arrowDepth >= 0 && depth == arrowDepth:
			arrowDepth = -1
		case c == '=' && i+1 < len(s) && s[i+1] == '>':
			k := i + 2
			for k < len(s) && (s[k] == ' ' || s[k] == '\t' || s[k] == '\n' || s[k] == '\r') {
				k++
			}
			if k < len(s) && s[k] != '{' {
				arrowDepth = depth
			}
			i += 2
			continue
		}
		if depth == 0 && arrowDepth < 0 {
			if n := ops(s, i); n > 0 {
				return i
			}
		}
		i++
	}
	return -1
}

// cvFindTernaryColon: the ':' matching a '?' at q (nested ternaries and
// masked tokens respected).
func cvFindTernaryColon(s string, q int) int {
	nest := 0
	depth := 0
	for i := q + 1; i < len(s); {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		switch s[i] {
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case '?':
			nest++
		case ':':
			if depth == 0 && nest == 0 {
				return i
			}
			nest--
		}
		i++
	}
	return -1
}

// ---------------------------------------------------------------------------
// expression model — just enough of babel's node types for the value shapes
// the converter resolves: "lit" | x ?? "lit" | x || "lit" | ident |
// ident ? "a" : "b" | cn(...) | Table({...}) | object literals.

type exprNode struct {
	kind  string // str tpl tplMulti ident bool nul num obj call cond logical other
	str   string // str: value; tpl: cooked; num: raw literal text
	ident string // ident: name; call: callee name
	args  []string
	obj   []cvProp
	test  string // cond
	cons  string
	alt   string
	op    string // logical: && || ??
	left  string
	right string
}

type cvProp struct {
	key       string
	val       string
	quoted    bool // "key": — babel prop.key.name is undefined for these
	shorthand bool
	spread    bool
}

var cvIdentRe = regexp.MustCompile(`^[A-Za-z_$][A-Za-z0-9_$]*$`)
var cvMemberRe = regexp.MustCompile(`^[A-Za-z_$][A-Za-z0-9_$-]*(?:\.[A-Za-z0-9_$-]+)+$`)
var cvNumRe = regexp.MustCompile(`^(?:0[xXbBoO][0-9a-fA-F_]+|[0-9][0-9_]*(?:\.[0-9_]+)?(?:[eE][+-]?[0-9]+)?|\.[0-9_]+)`)
var cvCalleeRe = regexp.MustCompile(`^[A-Za-z_$][A-Za-z0-9_$]*`)

func cvParseExpr(text string) *exprNode {
	t := strings.TrimSpace(text)
	n := &exprNode{kind: "other"}
	// babel has no paren nodes — a fully-wrapping paren group is transparent
	for strings.HasPrefix(t, "(") {
		if e := cvMatchBracket(t, 0, '(', ')'); e == len(t)-1 {
			t = strings.TrimSpace(t[1:e])
			continue
		}
		break
	}
	if t == "" {
		return n
	}
	switch {
	case t[0] == '"' || t[0] == '\'':
		n.kind, n.str = "str", cvUnquote(t)
		return n
	case t[0] == '`':
		if cvTemplateHasSubst(t) {
			n.kind = "tplMulti"
		} else {
			n.kind, n.str = "tpl", cvTemplateCook(t)
		}
		return n
	case t == "true" || t == "false":
		n.kind = "bool"
		return n
	case t == "null":
		n.kind = "nul"
		return n
	case cvIdentRe.MatchString(t):
		n.kind, n.ident = "ident", t
		return n
	}
	if m := cvNumRe.FindString(t); m == t {
		n.kind, n.str = "num", m
		return n
	}
	// conditional before call/logical: `f(x) ? a : b` is a cond (call parens
	// are masked, so its '?' is top-level); `f(a ? b : c)` is a call (its '?'
	// hides inside the masked invocation).
	if q := cvFirstTop(t, cvOpsQ); q >= 0 {
		if colon := cvFindTernaryColon(t, q); colon >= 0 {
			n.kind = "cond"
			n.test = t[:q]
			n.cons = t[q+1 : colon]
			n.alt = t[colon+1:]
			return n
		}
	}
	// logical root: the first operator of the LOWEST precedence tier — || / ??
	// before && — so `x && y || "fb"` splits at || with right "fb".
	if pos := cvFirstTop(t, cvOpsOr); pos >= 0 {
		n.kind, n.op = "logical", t[pos:pos+2]
		n.left, n.right = t[:pos], t[pos+2:]
		return n
	}
	if pos := cvFirstTop(t, cvOpsAnd); pos >= 0 {
		n.kind, n.op = "logical", "&&"
		n.left, n.right = t[:pos], t[pos+2:]
		return n
	}
	if name := cvCalleeName(t); name != "" {
		e := cvMatchBracket(t, len(name), '(', ')')
		if e > 0 && strings.TrimSpace(t[e+1:]) == "" {
			n.kind = "call"
			n.ident = name
			n.args = cvSplitTop(t[len(name)+1 : e])
			return n
		}
	}
	if t[0] == '{' {
		if e := cvMatchBracket(t, 0, '{', '}'); e == len(t)-1 {
			n.kind = "obj"
			n.obj = cvParseObjProps(t[1:e])
			return n
		}
	}
	return n
}

// cvCalleeName: `name(` at the root of t → name, else "".
func cvCalleeName(t string) string {
	m := cvCalleeRe.FindString(t)
	if m == "" || len(t) <= len(m) || t[len(m)] != '(' {
		return ""
	}
	return m
}

// cvMatchBracket: s[open] is the opener; return the index of its MATCHING
// closer (mask-aware), or -1.
func cvMatchBracket(s string, open int, oc, cc byte) int {
	depth := 0
	for i := open; i < len(s); {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		switch s[i] {
		case oc:
			depth++
		case cc:
			depth--
			if depth == 0 {
				return i
			}
		}
		i++
	}
	return -1
}

func cvParseObjProps(inner string) []cvProp {
	var out []cvProp
	for _, part := range cvSplitTop(inner) {
		p := strings.TrimSpace(part)
		if p == "" {
			continue
		}
		if strings.HasPrefix(p, "...") {
			out = append(out, cvProp{spread: true, key: strings.TrimSpace(p[3:])})
			continue
		}
		if p[0] == '"' || p[0] == '\'' {
			end := cvSkipString(p, 0)
			key := cvUnquote(p[:end])
			rest := strings.TrimSpace(p[end:])
			if strings.HasPrefix(rest, ":") {
				out = append(out, cvProp{key: key, val: strings.TrimSpace(rest[1:]), quoted: true})
			} else {
				out = append(out, cvProp{key: key, quoted: true})
			}
			continue
		}
		colon := cvPropColon(p)
		if colon < 0 {
			out = append(out, cvProp{key: p, val: p, shorthand: true})
			continue
		}
		out = append(out, cvProp{key: strings.TrimSpace(p[:colon]), val: strings.TrimSpace(p[colon+1:])})
	}
	return out
}

func cvPropColon(p string) int {
	depth := 0
	for i := 0; i < len(p); {
		if e := cvMaskEnd(p, i); e > 0 {
			i = e
			continue
		}
		switch p[i] {
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case ':':
			if depth == 0 {
				return i
			}
		}
		i++
	}
	return -1
}

// cvUnquote parses a JS string literal's value (esbuild emits double-quoted,
// utf8; babel's .value is the cooked text).
func cvUnquote(lit string) string {
	if len(lit) < 2 {
		return ""
	}
	q := lit[0]
	var b strings.Builder
	var c byte
	for i := 1; i < len(lit) && lit[i] != q; i++ {
		if lit[i] != '\\' {
			b.WriteByte(lit[i])
			continue
		}
		i++
		if i >= len(lit) {
			break
		}
		switch lit[i] {
		case 'n':
			b.WriteByte('\n')
		case 't':
			b.WriteByte('\t')
		case 'r':
			b.WriteByte('\r')
		case 'b':
			b.WriteByte('\b')
		case 'f':
			b.WriteByte('\f')
		case 'v':
			b.WriteByte('\v')
		case 'x':
			if i+2 < len(lit) {
				fmt.Sscanf(lit[i+1:i+3], "%2x", &c)
				b.WriteByte(c)
				i += 2
			}
		case 'u':
			if i+1 < len(lit) && lit[i+1] == '{' {
				if j := strings.IndexByte(lit[i+2:], '}'); j >= 0 {
					var r rune
					fmt.Sscanf(lit[i+2:i+2+j], "%x", &r)
					b.WriteRune(r)
					i += 2 + j
				}
			} else if i+4 < len(lit) {
				var r rune
				fmt.Sscanf(lit[i+1:i+5], "%4x", &r)
				b.WriteRune(r)
				i += 4
			}
		default:
			b.WriteByte(lit[i])
		}
	}
	return b.String()
}

func cvTemplateHasSubst(t string) bool {
	i := 1
	for i < len(t) {
		if t[i] == '\\' {
			i += 2
			continue
		}
		if t[i] == '`' {
			return false
		}
		if t[i] == '$' && i+1 < len(t) && t[i+1] == '{' {
			return true
		}
		i++
	}
	return false
}

func cvTemplateCook(t string) string {
	return cvUnquote("`" + t[1:len(t)-1] + "`")
}

// ---------------------------------------------------------------------------
// word helpers

// cvNextWord: the next word at or after i, skipping whitespace and comments.
func cvNextWord(s string, i int) (string, int, int) {
	for i < len(s) {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		if s[i] == ' ' || s[i] == '\t' || s[i] == '\n' || s[i] == '\r' {
			i++
			continue
		}
		break
	}
	w, l := cvWordAt(s, i)
	if l == 0 {
		return "", i, i
	}
	return w, i, i + l
}

func cvWordAt(s string, i int) (string, int) {
	if i >= len(s) {
		return "", 0
	}
	c := s[i]
	if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_' || c == '$') {
		return "", 0
	}
	j := i
	for j < len(s) && (s[j] >= 'a' && s[j] <= 'z' || s[j] >= 'A' && s[j] <= 'Z' ||
		s[j] >= '0' && s[j] <= '9' || s[j] == '_' || s[j] == '$') {
		j++
	}
	return s[i:j], j - i
}

// cvSkipStmt: past the next ';' at depth 0 — or the next NEWLINE at depth 0,
// because the upstream sources carry no semicolons (ASI) and an import like
// `import { cva } from "…"` otherwise swallows the whole file. esbuild's own
// output always has the ';', and it never breaks a statement at a depth-0
// newline, so the newline rule is inert there.
func cvSkipStmt(s string, i int) int {
	depth := 0
	for i < len(s) {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		switch s[i] {
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case ';', '\n':
			if depth <= 0 {
				return i + 1
			}
		}
		i++
	}
	return i
}

func cvIsArrowAt(s string, i int) bool {
	j := i
	for j < len(s) && (s[j] == ' ' || s[j] == '\t' || s[j] == '\n' || s[j] == '\r') {
		j++
	}
	return j+1 < len(s) && s[j] == '=' && s[j+1] == '>'
}

func cvParenSpan(s string, i int) (int, int, bool) {
	if i >= len(s) || s[i] != '(' {
		return 0, 0, false
	}
	e := cvMatchBracket(s, i, '(', ')')
	if e < 0 {
		return 0, 0, false
	}
	return i, e, true
}

func cvBraceSpan(s string, i int) (int, int, bool) {
	for i < len(s) {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		if s[i] == '{' {
			e := cvMatchBracket(s, i, '{', '}')
			if e < 0 {
				return 0, 0, false
			}
			return i, e, true
		}
		if s[i] != ' ' && s[i] != '\t' && s[i] != '\n' && s[i] != '\r' {
			return 0, 0, false
		}
		i++
	}
	return 0, 0, false
}

// ---------------------------------------------------------------------------
// the original-source JSX children classifier.
//
// One record per JSX element, in document (pre-)order — matching both
// babel's walk and the downgraded createElement call order. Fragment
// elements (<>/React.Fragment) produce no record, exactly as babel's
// JSXFragment never entered sketchChildren.

type jsxKind struct {
	elem bool   // a nested element child (sketch built from the downgraded side)
	text string // "text" | "expr" | "{name}" | "OPT?"
}

type cvJsxRec struct{ kinds []jsxKind }

var cvJsxNameRe = regexp.MustCompile(`^[A-Za-z_$][A-Za-z0-9_$-]*(?:\.[A-Za-z0-9_$-]+)*`)
var cvJsxKeywords = map[string]bool{
	"return": true, "typeof": true, "case": true, "do": true, "else": true,
	"in": true, "of": true, "new": true, "delete": true, "void": true,
	"yield": true, "await": true, "instanceof": true, "throw": true,
}

type jsxScanner struct {
	s    string
	recs []cvJsxRec
}

func scanJsxKinds(src string) ([]cvJsxRec, error) {
	st := &jsxScanner{s: src}
	for i := 0; i < len(src); {
		if e := cvMaskEnd(src, i); e > 0 {
			i = e
			continue
		}
		if src[i] == '<' && st.openAt(i) {
			n, err := st.parseElement(i)
			if err != nil {
				return nil, err
			}
			i = n
			continue
		}
		i++
	}
	return st.recs, nil
}

// openAt: does the '<' at i open a JSX element? A `<` preceded by an
// identifier is a comparison or generic (Array<string>); preceded by a
// keyword (return <div>) it is JSX.
func (st *jsxScanner) openAt(i int) bool {
	if i+1 >= len(st.s) {
		return false
	}
	c := st.s[i+1]
	if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_' || c == '$' || c == '>') {
		return false
	}
	j := i - 1
	for j >= 0 && (st.s[j] == ' ' || st.s[j] == '\t' || st.s[j] == '\n' || st.s[j] == '\r') {
		j--
	}
	if j < 0 {
		return true
	}
	p := st.s[j]
	if p >= 'a' && p <= 'z' || p >= 'A' && p <= 'Z' || p >= '0' && p <= '9' || p == '_' || p == '$' {
		k := j
		for k >= 0 && (st.s[k] >= 'a' && st.s[k] <= 'z' || st.s[k] >= 'A' && st.s[k] <= 'Z' ||
			st.s[k] >= '0' && st.s[k] <= '9' || st.s[k] == '_' || st.s[k] == '$') {
			k--
		}
		return cvJsxKeywords[st.s[k+1:j+1]]
	}
	return p != ')' && p != ']'
}

// parseElement consumes a full element (or fragment), appending records in
// pre-order. The element's OWN record slot is reserved before its children
// are parsed, so parent precedes children.
func (st *jsxScanner) parseElement(i int) (int, error) {
	if st.s[i+1] == '>' { // fragment <>
		_, n, err := st.parseChildren(i+2, "")
		return n, err
	}
	m := cvJsxNameRe.FindString(st.s[i+1:])
	name := m
	i += 1 + len(m)
	// React.Fragment as an explicit tag is indistinguishable from <> in the
	// downgraded output either way; both take the fragment path.
	if name == "React.Fragment" {
		_, n, err := st.parseChildren(i, "")
		return n, err
	}
	// reserve the record slot now — pre-order
	slot := len(st.recs)
	st.recs = append(st.recs, cvJsxRec{})
	var kinds []jsxKind
	for {
		for i < len(st.s) {
			if e := cvMaskEnd(st.s, i); e > 0 {
				i = e
				continue
			}
			if st.s[i] == ' ' || st.s[i] == '\t' || st.s[i] == '\n' || st.s[i] == '\r' {
				i++
				continue
			}
			break
		}
		if i >= len(st.s) {
			return 0, fmt.Errorf("unterminated element <%s>", name)
		}
		if st.s[i] == '/' && i+1 < len(st.s) && st.s[i+1] == '>' {
			i += 2
			break
		}
		if st.s[i] == '>' {
			var err error
			kinds, i, err = st.parseChildren(i+1, name)
			if err != nil {
				return 0, err
			}
			break
		}
		if st.s[i] == '{' {
			i = st.containerEnd(i) + 1
			continue
		}
		i++
	}
	st.recs[slot].kinds = kinds
	return i, nil
}

// parseChildren consumes children until the closing tag ("" closeName = a
// fragment `</>`), returning the child kind list.
func (st *jsxScanner) parseChildren(i int, closeName string) ([]jsxKind, int, error) {
	var kinds []jsxKind
	for {
		if i >= len(st.s) {
			return nil, 0, fmt.Errorf("unterminated children of <%s>", closeName)
		}
		c := st.s[i]
		if c == '<' {
			if st.s[i+1] == '/' {
				j := i + 2
				if closeName == "" {
					for j < len(st.s) && st.s[j] != '>' {
						j++
					}
					return kinds, j + 1, nil
				}
				m := cvJsxNameRe.FindString(st.s[j:])
				if m != closeName {
					return nil, 0, fmt.Errorf("closing </%s> does not match <%s>", m, closeName)
				}
				j += len(m)
				for j < len(st.s) && st.s[j] != '>' {
					j++
				}
				return kinds, j + 1, nil
			}
			if st.s[i+1] == '>' { // nested fragment: no kind of its own
				_, n, err := st.parseChildren(i+2, "")
				if err != nil {
					return nil, 0, err
				}
				i = n
				continue
			}
			n, err := st.parseElement(i)
			if err != nil {
				return nil, 0, err
			}
			kinds = append(kinds, jsxKind{elem: true})
			i = n
			continue
		}
		if c == '{' {
			end := st.containerEnd(i)
			kinds = append(kinds, jsxKind{text: cvClassifyContainer(st.s[i+1 : end])})
			i = end + 1
			continue
		}
		// JSX text: runs until < or { — NOT JS-masked (quotes are literal)
		j := i
		for j < len(st.s) && st.s[j] != '<' && st.s[j] != '{' {
			j++
		}
		if strings.TrimSpace(st.s[i:j]) != "" {
			kinds = append(kinds, jsxKind{text: "text"})
		}
		i = j
	}
}

// containerEnd: the matching '}' of s[i] == '{', recursing into nested JSX
// elements so their records land in pre-order.
func (st *jsxScanner) containerEnd(i int) int {
	depth := 1
	i++
	for i < len(st.s) && depth > 0 {
		if e := cvMaskEnd(st.s, i); e > 0 {
			i = e
			continue
		}
		c := st.s[i]
		if c == '<' && st.openAt(i) {
			if n, err := st.parseElement(i); err == nil {
				i = n
				continue
			}
		}
		switch c {
		case '{':
			depth++
		case '}':
			depth--
		}
		i++
	}
	return i - 1
}

// cvClassifyContainer: babel's sketchChildren on the container's expression.
func cvClassifyContainer(expr string) string {
	t := strings.TrimSpace(expr)
	if t == "" || strings.HasPrefix(t, "/*") || strings.HasPrefix(t, "//") {
		return "expr" // JSXEmptyExpression (comment-only container)
	}
	if cvIdentRe.MatchString(t) && t != "true" && t != "false" && t != "null" {
		return "{" + t + "}"
	}
	if cvFirstTop(t, cvOpsQorLogical) >= 0 {
		return "OPT?"
	}
	return "expr"
}

// ---------------------------------------------------------------------------
// downgraded-output scan: top-level declarations

type cvDecl struct {
	name        string
	isFn        bool // function declaration (always a component)
	isArrow     bool // const/let/var with arrow/function init
	exported    bool // declared with `export`
	params      [2]int
	body        [2]int
	singleParam string // arrow with one bare ident param (`x => …`)
}

type cvTopScan struct {
	js            string
	decls         []cvDecl
	exportedNames map[string]bool
	declIndex     map[string]int // name → decls index (fn/arrow only)
	defaultArrow  int            // decls index pushed as the "default" component, -1
}

func scanTopJs(js string) (*cvTopScan, error) {
	t := &cvTopScan{js: js, exportedNames: map[string]bool{}, declIndex: map[string]int{}, defaultArrow: -1}
	depth := 0
	i := 0
	for i < len(js) {
		if e := cvMaskEnd(js, i); e > 0 {
			i = e
			continue
		}
		c := js[i]
		if c == '(' || c == '[' || c == '{' {
			depth++
			i++
			continue
		}
		if c == ')' || c == ']' || c == '}' {
			depth--
			i++
			continue
		}
		if depth > 0 {
			i++
			continue
		}
		w, wlen := cvWordAt(js, i)
		switch w {
		case "import":
			// imports come from the ORIGINAL source (type-only imports are
			// dropped by the downgrade but babel counted them); skip here
			i = cvSkipStmt(js, i)
		case "export":
			// the token after `export` may be `{`, which is no word
			j := i + wlen
			for j < len(js) {
				if e := cvMaskEnd(js, j); e > 0 {
					j = e
					continue
				}
				if js[j] == ' ' || js[j] == '\t' || js[j] == '\n' || js[j] == '\r' {
					j++
					continue
				}
				break
			}
			w2, w2len := cvWordAt(js, j)
			var n int
			var err error
			switch {
			case j < len(js) && js[j] == '{':
				n, err = t.parseExportSpecs(j)
			case w2 == "function":
				n, err = t.parseFnDecl(j, true)
			case w2 == "const" || w2 == "let" || w2 == "var":
				n, err = t.parseVarDecl(j, w2len, true)
			default:
				n, err = cvSkipStmt(js, i), nil
			}
			if err != nil {
				return nil, err
			}
			i = n
		case "function":
			n, err := t.parseFnDecl(i, false)
			if err != nil {
				return nil, err
			}
			i = n
		case "const", "let", "var":
			n, err := t.parseVarDecl(i, wlen, false)
			if err != nil {
				return nil, err
			}
			i = n
		case "async":
			i += wlen
		default:
			i++
		}
	}
	return t, nil
}

func (t *cvTopScan) parseFnDecl(i int, exported bool) (int, error) {
	_, _, kwEnd := cvNextWord(t.js, i) // past `function`
	name, _, nameEnd := cvNextWord(t.js, kwEnd)
	if name == "" {
		return cvSkipStmt(t.js, i), nil
	}
	po, pe, ok := cvParenSpan(t.js, nameEnd)
	if !ok {
		return cvSkipStmt(t.js, i), nil
	}
	bo, be, ok := cvBraceSpan(t.js, pe+1)
	if !ok {
		return cvSkipStmt(t.js, i), nil
	}
	t.decls = append(t.decls, cvDecl{name: name, isFn: true, exported: exported,
		params: [2]int{po, pe}, body: [2]int{bo, be}})
	t.declIndex[name] = len(t.decls) - 1
	return be, nil
}

// parseVarDecl walks the declarators of a const/let/var statement; a
// declarator whose init is an arrow/function records its absolute
// params/body spans.
func (t *cvTopScan) parseVarDecl(i, kwlen int, exported bool) (int, error) {
	js := t.js
	end := cvSkipStmt(js, i)
	segEnd := end
	if segEnd > i && js[segEnd-1] == ';' {
		segEnd--
	}
	for _, dcl := range cvSplitTopIndexed(js[i+kwlen : segEnd]) {
		text := strings.TrimSpace(dcl.text)
		if text == "" {
			continue
		}
		name, nlen := cvWordAt(text, 0)
		if name == "" {
			continue
		}
		eq := -1
		{
			depth := 0
			for k := nlen; k < len(text); k++ {
				if e := cvMaskEnd(text, k); e > 0 {
					k = e
					continue
				}
				switch text[k] {
				case '(', '[', '{':
					depth++
				case ')', ']', '}':
					depth--
				case '=':
					if depth == 0 && !(k+1 < len(text) && (text[k+1] == '=' || text[k+1] == '>')) {
						eq = k
					}
				}
				if eq >= 0 {
					break
				}
			}
		}
		if eq < 0 {
			continue
		}
		initText := text[eq+1:]
		initBase := dcl.off + eq + 1 // absolute position of initText[0]
		declIdx := len(t.decls)
		t.decls = append(t.decls, cvDecl{name: name, exported: exported})
		if ar, ok := cvArrowSpan(initText, initBase); ok {
			t.decls[declIdx].isArrow = true
			t.decls[declIdx].params = ar.params
			t.decls[declIdx].body = ar.body
			t.decls[declIdx].singleParam = ar.single
			t.declIndex[name] = declIdx
		}
	}
	return end, nil
}

type cvSplitPart struct {
	text string
	off  int // offset of text's start within the scanned string
}

func cvSplitTopIndexed(s string) []cvSplitPart {
	var out []cvSplitPart
	depth := 0
	start := 0
	for i := 0; i < len(s); {
		if e := cvMaskEnd(s, i); e > 0 {
			i = e
			continue
		}
		switch s[i] {
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case ',':
			if depth == 0 {
				out = append(out, cvSplitPart{s[start:i], start})
				start = i + 1
			}
		}
		i++
	}
	return append(out, cvSplitPart{s[start:], start})
}

type cvArrowInfo struct {
	params [2]int
	body   [2]int
	single string
}

// cvArrowSpan: is init (starting at absolute offset base) a function value?
// If so, return absolute params/body spans.
func cvArrowSpan(init string, base int) (cvArrowInfo, bool) {
	var out cvArrowInfo
	t := strings.TrimLeft(init, " \t\n\r")
	off := base + (len(init) - len(t))
	if w, l := cvWordAt(t, 0); w == "async" {
		t = strings.TrimLeft(t[l:], " \t\n\r")
		off += l
	}
	if w, l := cvWordAt(t, 0); w == "function" {
		_, _, nend := cvNextWord(t, l) // optional fn name
		po, pe, ok := cvParenSpan(t, nend)
		if !ok {
			return out, false
		}
		bo, be, ok := cvBraceSpan(t, pe+1)
		if !ok {
			return out, false
		}
		out.params = [2]int{off + po, off + pe}
		out.body = [2]int{off + bo, off + be}
		return out, true
	}
	if strings.HasPrefix(t, "(") {
		po, pe, ok := cvParenSpan(t, 0)
		if !ok || !cvIsArrowAt(t, pe) {
			return out, false
		}
		out.params = [2]int{off + po, off + pe}
		out.body = [2]int{off + pe + 2, off + len(t)}
		return out, true
	}
	// single bare ident param: `x => …`
	if name, nlen := cvWordAt(t, 0); nlen > 0 && cvIsArrowAt(t, nlen) {
		out.single = name
		out.body = [2]int{off + nlen + 2, off + len(t)}
		return out, true
	}
	return out, false
}

// parseExportSpecs: `export { A, B as C }` — babel collects the EXPORTED
// name (s.exported?.name ?? s.local.name). esbuild rewrites
// `export default function D(){}` to `function D` + `D as default`, and an
// anonymous default to a synthesized `<x>_default` var; babel recorded the
// fn's own name (or "default" when anonymous), so a declared fn/arrow name
// exported as default registers under its own name, while a synthesized
// _default var registers as "default" (and its arrow is pushed as the
// "default" component, matching babel's anonymous-arrow branch).
func (t *cvTopScan) parseExportSpecs(i int) (int, error) {
	e := cvMatchBracket(t.js, i, '{', '}')
	if e < 0 {
		return cvSkipStmt(t.js, i), nil
	}
	for _, sp := range cvSplitTop(t.js[i+1 : e]) {
		fields := strings.Fields(sp)
		var local, exported string
		if len(fields) >= 3 && fields[1] == "as" {
			local, exported = fields[0], fields[2]
		} else if len(fields) >= 1 {
			local, exported = fields[0], fields[0]
		} else {
			continue
		}
		if exported == "default" {
			di, ok := t.declIndex[local]
			isFnLike := ok && (t.decls[di].isFn || t.decls[di].isArrow)
			if isFnLike && !strings.HasSuffix(local, "_default") {
				t.exportedNames[local] = true
			} else {
				t.exportedNames["default"] = true
				if ok && t.decls[di].isArrow {
					t.defaultArrow = di
				}
			}
			continue
		}
		t.exportedNames[exported] = true
	}
	return e + 1, nil
}

// ---------------------------------------------------------------------------
// imports — from the ORIGINAL source. babel counted every ImportDeclaration's
// source — INCLUDING type-only imports, which the downgrade drops — and
// mapped every ImportSpecifier (type ones included) local→imported.

type cvImpSpec struct {
	local, imported string
	ns              bool
}

func importsOf(src string) (imports, icons []string, importMap, moduleOf map[string]string) {
	importMap = map[string]string{}
	moduleOf = map[string]string{}
	depth := 0
	for i := 0; i < len(src); {
		if e := cvMaskEnd(src, i); e > 0 {
			i = e
			continue
		}
		c := src[i]
		if c == '(' || c == '[' || c == '{' {
			depth++
			i++
			continue
		}
		if c == ')' || c == ']' || c == '}' {
			depth--
			i++
			continue
		}
		if depth > 0 {
			i++
			continue
		}
		if w, _ := cvWordAt(src, i); w == "import" {
			stmtEnd := cvSkipStmt(src, i)
			from, list, defaultLocal := cvParseImport(src[i:stmtEnd])
			if from != "" {
				imports = append(imports, from)
				for _, sp := range list {
					if sp.ns {
						// babel: s.imported is undefined → local name
						if from == "lucide-react" {
							icons = append(icons, sp.local)
						}
						continue
					}
					importMap[sp.local] = sp.imported
					moduleOf[sp.local] = from
					if from == "lucide-react" {
						icons = append(icons, sp.imported)
					}
				}
				if defaultLocal != "" && from == "lucide-react" {
					icons = append(icons, defaultLocal)
				}
			}
			i = stmtEnd
			continue
		}
		i++
	}
	return imports, icons, importMap, moduleOf
}

// cvParseImport extracts the module path and specifiers from an import
// statement's text (default/namespace specifiers reported separately).
func cvParseImport(stmt string) (from string, list []cvImpSpec, defaultLocal string) {
	var strs []string
	for i := 0; i < len(stmt); {
		if e := cvMaskEnd(stmt, i); e > 0 {
			if stmt[i] == '"' || stmt[i] == '\'' {
				strs = append(strs, cvUnquote(stmt[i:e]))
			}
			i = e
			continue
		}
		i++
	}
	if len(strs) == 0 {
		return "", nil, ""
	}
	ob := strings.IndexByte(stmt, '{')
	eb := -1
	if ob >= 0 {
		eb = cvMatchBracket(stmt, ob, '{', '}')
	}
	from = strs[len(strs)-1]
	head := stmt
	if eb > 0 {
		head = stmt[:ob]
		for _, f := range cvSplitTop(stmt[ob+1 : eb]) {
			fields := strings.Fields(f)
			k := 0
			if k < len(fields) && fields[k] == "type" {
				k++
			}
			if k >= len(fields) {
				continue
			}
			imported := fields[k]
			local := imported
			if k+2 < len(fields) && fields[k+1] == "as" {
				local = fields[k+2]
			}
			list = append(list, cvImpSpec{local: local, imported: imported})
		}
	}
	if m := regexp.MustCompile(`\bimport\s*\*\s*as\s+([A-Za-z_$][\w$]*)`).FindStringSubmatch(head); m != nil {
		list = append(list, cvImpSpec{local: m[1], ns: true})
	} else if m := regexp.MustCompile(`\bimport\s+([A-Za-z_$][\w$]*)\s*,?\s*(?:\{|from\b)`).FindStringSubmatch(head); m != nil {
		defaultLocal = m[1]
	}
	return from, list, defaultLocal
}

// ---------------------------------------------------------------------------
// cva tables (downgraded output — top-level `const x = cva(…)` declarators)

type cvTable struct {
	base     string
	axes     []cvaAxis
	defaults []cvaKV // ordered; value: string | bool | jsonRaw (undefined omitted)
}

type cvaAxis struct {
	axis   string
	values []cvaKV // ordered; value: string ("" = legitimate no-classes)
}

type cvaKV struct {
	k string
	v any
}

func (t *cvTable) json() jsonObj {
	variants := jsonObj{}
	for _, a := range t.axes {
		vo := jsonObj{}
		for _, kv := range a.values {
			vo = vo.add(kv.k, kv.v)
		}
		variants = variants.add(a.axis, vo)
	}
	defaults := jsonObj{}
	for _, kv := range t.defaults {
		defaults = defaults.add(kv.k, kv.v)
	}
	return jsonObj{}.add("base", t.base).add("variants", variants).add("defaults", defaults)
}

// setDefault mirrors `(table.defaults ??= {})[axis] = val` — append-only,
// because the mutation runs only when the axis is absent.
func (t *cvTable) setDefault(axis, val string) {
	t.defaults = append(t.defaults, cvaKV{axis, val})
}

func (t *cvTable) hasDefault(axis string) (string, bool) {
	for _, kv := range t.defaults {
		if kv.k == axis {
			s, _ := kv.v.(string)
			return s, true
		}
	}
	return "", false
}

func (t *cvTable) valueFor(axis, val string) (string, bool) {
	for _, a := range t.axes {
		if a.axis != axis {
			continue
		}
		for _, kv := range a.values {
			if kv.k == val {
				s, _ := kv.v.(string)
				return s, true
			}
		}
	}
	return "", false
}

func (t *cvTable) hasAxis(axis string) bool {
	for _, a := range t.axes {
		if a.axis == axis {
			return true
		}
	}
	return false
}

type cvTables struct {
	names  []string
	byName map[string]*cvTable
}

func newCvTables() *cvTables {
	return &cvTables{byName: map[string]*cvTable{}}
}

func (ts *cvTables) add(name string, tb *cvTable) {
	if _, ok := ts.byName[name]; !ok {
		ts.names = append(ts.names, name)
	}
	ts.byName[name] = tb
}

func (ts *cvTables) get(name string) *cvTable { return ts.byName[name] }

func (ts *cvTables) json() jsonObj {
	o := jsonObj{}
	for _, n := range ts.names {
		o = o.add(n, ts.byName[n].json())
	}
	return o
}

// cvTablesOf collects the top-level `const NAME = cva(base, cfg)` tables of
// a downgraded module — plain (non-exported) consts included, matching
// babel's take() over every top-level VariableDeclaration.
func cvTablesOf(js string) *cvTables {
	ts := newCvTables()
	depth := 0
	for i := 0; i < len(js); {
		if e := cvMaskEnd(js, i); e > 0 {
			i = e
			continue
		}
		c := js[i]
		if c == '(' || c == '[' || c == '{' {
			depth++
			i++
			continue
		}
		if c == ')' || c == ']' || c == '}' {
			depth--
			i++
			continue
		}
		if depth > 0 {
			i++
			continue
		}
		if w, l := cvWordAt(js, i); w == "const" || w == "let" || w == "var" {
			end := cvSkipStmt(js, i)
			segEnd := end
			if segEnd > i && js[segEnd-1] == ';' {
				segEnd--
			}
			for _, dcl := range cvSplitTopIndexed(js[i+l : segEnd]) {
				text := strings.TrimSpace(dcl.text)
				name, nlen := cvWordAt(text, 0)
				if name == "" {
					continue
				}
				rest := strings.TrimSpace(text[nlen:])
				if !strings.HasPrefix(rest, "=") {
					continue
				}
				init := cvParseExpr(rest[1:])
				if init.kind != "call" || init.ident != "cva" || len(init.args) < 1 {
					continue
				}
				tb := &cvTable{}
				if a0 := cvParseExpr(init.args[0]); a0.kind == "str" {
					tb.base = a0.str
				}
				if len(init.args) > 1 {
					if cfg := cvParseExpr(init.args[1]); cfg.kind == "obj" {
						for _, p := range cfg.obj {
							if p.quoted || p.spread {
								continue
							}
							pv := cvParseExpr(p.val)
							if p.key == "variants" && pv.kind == "obj" {
								for _, ax := range pv.obj {
									av := cvParseExpr(ax.val)
									if av.kind != "obj" {
										continue
									}
									entry := cvaAxis{axis: ax.key}
									for _, vv := range av.obj {
										entry.values = append(entry.values, cvaKV{vv.key, cvValOfString(vv.val)})
									}
									tb.axes = append(tb.axes, entry)
								}
							}
							if p.key == "defaultVariants" && pv.kind == "obj" {
								for _, dv := range pv.obj {
									if v, ok := cvDefaultValue(dv.val); ok {
										tb.defaults = append(tb.defaults, cvaKV{dv.key, v})
									}
								}
							}
						}
					}
				}
				ts.add(name, tb)
			}
			i = end
			continue
		}
		i++
	}
	return ts
}

// cvValOfString: a cva variant value node — "lit" | ["a","b"] (join) | ""
func cvValOfString(text string) string {
	if n := cvParseExpr(text); n.kind == "str" {
		return n.str
	}
	t := strings.TrimSpace(text)
	if strings.HasPrefix(t, "[") {
		if e := cvMatchBracket(t, 0, '[', ']'); e > 0 {
			var parts []string
			for _, el := range cvSplitTop(t[1:e]) {
				if n2 := cvParseExpr(el); n2.kind == "str" && n2.str != "" {
					parts = append(parts, n2.str)
				}
			}
			return strings.Join(parts, " ")
		}
	}
	return ""
}

// cvDefaultValue: str(dv.value) ?? dv.value?.value — non-string literals
// keep their value (number/bool); anything else (undefined, templates,
// identifiers) omits the key entirely.
func cvDefaultValue(text string) (any, bool) {
	n := cvParseExpr(text)
	switch n.kind {
	case "str":
		return n.str, true
	case "num":
		return jsonRaw(n.str), true
	case "bool":
		return strings.TrimSpace(text) == "true", true
	}
	return nil, false
}

// ---------------------------------------------------------------------------
// the per-file conversion context

type compCvaEntry struct {
	cvaName string
	table   *cvTable
	file    string
}

type cvaExportEntry struct {
	file  string
	table *cvTable
}

type cvReg struct {
	pinCommit   any
	cvaByExport map[string]*cvaExportEntry
	compCva     map[string]*compCvaEntry
}

func newCvReg() *cvReg {
	return &cvReg{cvaByExport: map[string]*cvaExportEntry{}, compCva: map[string]*compCvaEntry{}}
}

type cvFile struct {
	pinCommit    any
	name, tier   string
	imports      []string
	icons        []string
	cva          *cvTables
	components   []cvComponent
	conditionals []jsonObj
	cvaRefs      []jsonObj
	tagHints     jsonObj
	metaModuleOf map[string]string
	metaTagVars  map[string]string
	metaImport   map[string]string
}

type cvComponent struct {
	fn       string
	isExport bool
	elements []jsonObj
}

// resolve context (fileCtx + extractFn's ctx in one)
type cvCtx struct {
	file          *cvFile
	reg           *cvReg
	src           string
	js            string
	paramDefaults map[string]string
	fnName        string
	bodyStart     int
	bodyEnd       int
	refKeys       map[string]bool
}

func (c *cvCtx) lookupCva(local string) (*cvTable, bool, bool) {
	if imp, ok := c.file.metaImport[local]; ok {
		if e, ok := c.reg.cvaByExport[imp]; ok {
			return e.table, true, true
		}
	}
	if t := c.file.cva.get(local); t != nil {
		return t, false, true
	}
	return nil, false, false
}

// cvElCtx — the element being resolved, as resolveCvaArgs sees it.
type cvElCtx struct {
	props []cvProp
}

// resolveCvaArgs — port of resolveCvaArgs. el == nil when the ctx only
// collects literals.
func (c *cvCtx) resolveCvaArgs(el *cvElCtx, table *cvTable, args map[string]string, acc *[]string, ref string, cross bool) error {
	if table.base != "" {
		*acc = append(*acc, table.base)
	}
	var dyn []jsonObj
	var dynAxes []string
	dynDefaults := jsonObj{}
	ownAttr := func(name string) bool {
		if el == nil {
			return false
		}
		for _, p := range el.props {
			if !p.spread && p.key == name {
				return true
			}
		}
		return false
	}
	for _, axis := range table.axes {
		raw, hasArg := args[axis.axis]
		var val string
		var v *exprNode
		if hasArg {
			v = cvParseExpr(raw)
			switch v.kind {
			case "str":
				val = v.str
			case "logical":
				r := cvParseExpr(v.right)
				if r.kind == "str" {
					val = r.str
				} else if r.kind == "ident" {
					val = c.paramDefaults[r.ident]
				}
				l := cvParseExpr(v.left)
				// `context.size || size` with data-size={…} on the element:
				// the value arrives through React CONTEXT at runtime and the
				// element exposes it as its own attribute. Statically this
				// collapsed to the param default, so toggle-group items had
				// no size rules at all on the css-import path. Treat it as a
				// dynamic axis keyed on data-<axis>, with the param default
				// as the axis default (twin blocks in css.mjs).
				if l.kind != "str" && ownAttr("data-"+axis.axis) && val != "" {
					dynAxes = append(dynAxes, axis.axis)
					dynDefaults = dynDefaults.add(axis.axis, val)
					continue
				}
			case "ident":
				val = c.paramDefaults[v.ident]
			}
		}
		def, defOk := table.hasDefault(axis.axis)
		if val != "" {
			if cls, ok := table.valueFor(axis.axis, val); ok {
				if cls != "" {
					*acc = append(*acc, cls)
				}
				// A literal PARAM default (`size = "default"`) that feeds the
				// axis is the axis's default in every React render — record
				// it on the table so the css emitter's twin
				// :not([data-<axis>]) blocks exist; without it slot-only
				// markup got no size/orientation classes at all
				// (gates/path-parity.mjs, 12 properties on the attachment
				// root). Only the element's own root call binds the default;
				// a cross-file reference resolved elsewhere must not
				// redefine the source table.
				fromParam := v.kind == "ident" ||
					(v.kind == "logical" && cvParseExpr(v.right).kind == "ident")
				if fromParam && !defOk && !cross {
					table.setDefault(axis.axis, val)
				}
				continue
			}
			return fmt.Errorf("cva unknown variant value: %s=%s (ref %s)", axis.axis, jsonString(val), ref)
		}
		if v != nil && v.kind == "cond" {
			cons, alt := cvParseExpr(v.cons), cvParseExpr(v.alt)
			if cons.kind == "str" && alt.kind == "str" {
				if cls, ok := table.valueFor(axis.axis, alt.str); ok && cls != "" {
					*acc = append(*acc, cls) // falsy state = base
				}
				test := ""
				if tv := cvParseExpr(v.test); tv.kind == "ident" {
					test = tv.ident
				}
				attr := ""
				// attr scan: the element itself, then (asChild pattern) the
				// whole fn — bases/radix pagination binds
				// data-active={isActive} on the <a> child while the cva call
				// sits on the wrapping <Button>
				if test != "" {
					attr = c.findDataAttr(el, test)
				}
				// attr-driven consequent: without the matching data-* attr
				// the "then" classes would be silently lost (old behavior) —
				// fail loudly instead
				if attr == "" {
					show := test
					if show == "" {
						show = "null"
					}
					return fmt.Errorf("cva ident-ternary without data-* attr binding: %s (ref %s)", show, ref)
				}
				if cls, ok := table.valueFor(axis.axis, cons.str); ok && cls != "" {
					dyn = append(dyn, jsonObj{}.add("attr", attr).add("when", "true").add("classes", cls))
				}
				continue
			}
		}
		// dynamic axis: merge default into base (same-rule ordering keeps any
		// override classes pushed later in el.classes winning), emit all values
		if defOk {
			if cls, ok := table.valueFor(axis.axis, def); ok && cls != "" {
				*acc = append(*acc, cls)
			}
		}
		dynAxes = append(dynAxes, axis.axis)
	}
	if el != nil && (len(dyn) > 0 || len(dynAxes) > 0) {
		c.recordCvaRef(el, ref, table, dyn, dynAxes, cross, dynDefaults)
	}
	return nil
}

func (c *cvCtx) findDataAttr(el *cvElCtx, test string) string {
	// the element's own props first, in order
	if el != nil {
		for _, p := range el.props {
			if p.spread || !strings.HasPrefix(p.key, "data-") {
				continue
			}
			if n := cvParseExpr(p.val); n.kind == "ident" && n.ident == test {
				return p.key
			}
		}
	}
	// then every JSX prop in the fn body, in document order
	return cvScanFnDataAttr(c.js, c.bodyStart, c.bodyEnd, test)
}

var cvDataAttrRe = regexp.MustCompile(`"(data-[\w-]+)"\s*:\s*([A-Za-z_$][\w$]*)`)

// cvScanFnDataAttr finds the first `data-x: <ident>` JSX prop in the body
// span whose identifier equals test. Only unmasked matches count
// (strings/comments can carry the same bytes).
func cvScanFnDataAttr(js string, start, end int, test string) string {
	seg := js[start:end]
	for _, loc := range cvDataAttrRe.FindAllStringSubmatchIndex(seg, -1) {
		if !cvPosUnmasked(js, start, start+loc[0]) {
			continue
		}
		if seg[loc[4]:loc[5]] == test {
			return seg[loc[2]:loc[3]]
		}
	}
	return ""
}

func cvPosUnmasked(js string, base, pos int) bool {
	for i := base; i < pos; {
		if e := cvMaskEnd(js, i); e > 0 {
			if e > pos {
				return false
			}
			i = e
			continue
		}
		i++
	}
	return true
}

func (c *cvCtx) recordCvaRef(el *cvElCtx, ref string, table *cvTable, dyn []jsonObj, dynAxes []string, cross bool, defaults jsonObj) {
	if !cross {
		return
	}
	var slot any
	for _, p := range el.props {
		if !p.spread && p.key == "data-slot" {
			if n := cvParseExpr(p.val); n.kind == "str" {
				slot = n.str
			}
			break
		}
	}
	s, _ := slot.(string)
	if s == "" {
		return
	}
	var dkeys []string
	for _, d := range dyn {
		dkeys = append(dkeys, d[0].V.(string)+d[1].V.(string))
	}
	key := s + "|" + ref + "|" + strings.Join(dkeys, ",") + "|" + strings.Join(dynAxes, ",")
	if c.refKeys[key] {
		return
	}
	c.refKeys[key] = true
	// Ref rules are recorded only for cross-file refs — a file's own tables
	// already emit attribute rules via css.mjs's cvaSlot path, so inlining
	// their call-site classes into el.classes is enough (CSS-inert: those
	// slots sit inside cvaSlots and the plain-class path skips them).
	c.file.cvaRefs = append(c.file.cvaRefs, jsonObj{}.
		add("slot", slot).add("ref", ref).add("table", table.json()).
		add("dyn", cvAnyJSON(dyn)).add("dynAxes", cvStrSlice(dynAxes)).add("defaults", defaults))
}

func cvAnyJSON(o []jsonObj) []any {
	out := make([]any, len(o))
	for i, e := range o {
		out[i] = e
	}
	return out
}

func cvStrSlice(s []string) []any {
	out := make([]any, len(s))
	for i, e := range s {
		out[i] = e
	}
	return out
}

// classStrings — collect class strings from a className value node.
func (c *cvCtx) classStrings(text string, acc *[]string, el *cvElCtx) {
	n := cvParseExpr(text)
	switch n.kind {
	case "str":
		*acc = append(*acc, n.str)
	case "tpl":
		*acc = append(*acc, n.str)
	case "tplMulti":
		// interpolated template skipped (documented limit)
	case "call":
		if n.ident == "cn" {
			for _, a := range n.args {
				c.classStrings(a, acc, el)
			}
			return
		}
		if table, cross, ok := c.lookupCva(n.ident); ok {
			args := map[string]string{}
			if len(n.args) > 0 {
				if a0 := cvParseExpr(n.args[0]); a0.kind == "obj" {
					for _, p := range a0.obj {
						// babel: p.key?.name — identifier keys only
						if !p.quoted && !p.spread {
							args[p.key] = p.val
						}
					}
				}
			}
			_ = c.resolveCvaArgs(el, table, args, acc, n.ident, cross)
			return
		}
	default:
		if n.kind == "cond" {
			// class-cond pattern (H1b): ternary inside cn() — both branches
			// are collected as unconditional classes
			c.classStrings(n.cons, acc, el)
			c.classStrings(n.alt, acc, el)
		}
		if n.kind == "logical" {
			c.classStrings(n.right, acc, el)
		}
	}
}

// cvClassStringsNil — the ctx-nil path: literal strings only (used for the
// child sketches, which carry no cva context).
func cvClassStringsNil(text string, acc *[]string) {
	n := cvParseExpr(text)
	switch n.kind {
	case "str":
		*acc = append(*acc, n.str)
	case "tpl":
		*acc = append(*acc, n.str)
	case "call":
		if n.ident == "cn" {
			for _, a := range n.args {
				cvClassStringsNil(a, acc)
			}
		}
	case "cond":
		cvClassStringsNil(n.cons, acc)
		cvClassStringsNil(n.alt, acc)
	case "logical":
		cvClassStringsNil(n.right, acc)
	}
}

// ---------------------------------------------------------------------------
// convertFile

var cvIdentityAttr = regexp.MustCompile(`(?i)^(role|type|aria-[\w-]+|dir|lang|scope|colspan|rowspan|target|rel|tabindex)$`)

func esbuildTsx(src string) (string, error) {
	res := api.Transform(src, api.TransformOptions{
		Loader:      api.LoaderTSX,
		JSX:         api.JSXTransform,
		JSXFactory:  "React.createElement",
		JSXFragment: "React.Fragment",
		Format:      api.FormatESModule,
		Charset:     api.CharsetUTF8,
	})
	if len(res.Errors) > 0 {
		return "", fmt.Errorf("esbuild: %s", res.Errors[0].Text)
	}
	return string(res.Code), nil
}

type cvCallOcc struct {
	pos      int
	args     []string
	argPos   []int // absolute position of each arg's first non-space byte
	childArg []int // arg indexes of direct element-call children
	childPos []int // absolute positions of those calls
	tag      string
	tagOK    bool
}

func cvFindCalls(js string) ([]cvCallOcc, error) {
	var out []cvCallOcc
	for i := 0; i < len(js); {
		if e := cvMaskEnd(js, i); e > 0 {
			i = e
			continue
		}
		if js[i] == 'R' && strings.HasPrefix(js[i:], "React.createElement(") {
			// callee must be exactly React.createElement (not a member tail)
			if i > 0 {
				p := js[i-1]
				if p == '.' || p >= 'a' && p <= 'z' || p >= 'A' && p <= 'Z' ||
					p >= '0' && p <= '9' || p == '_' || p == '$' {
					i++
					continue
				}
			}
			occ, err := cvParseCall(js, i)
			if err != nil {
				return nil, err
			}
			// a fragment is not an element (babel's JSXFragment never entered
			// the walk); skip the record but keep scanning its children
			if !occ.tagOK && occ.args[0] == "React.Fragment" {
				i = occ.pos + 1
				continue
			}
			out = append(out, occ)
			i = occ.pos + 1 // resume inside the args; nested calls come later
			continue
		}
		i++
	}
	return out, nil
}

func cvParseCall(js string, i int) (cvCallOcc, error) {
	var occ cvCallOcc
	occ.pos = i
	open := i + len("React.createElement")
	close := cvMatchBracket(js, open, '(', ')')
	if close < 0 {
		return occ, fmt.Errorf("createElement: unbalanced call at %d", open)
	}
	offset := open + 1
	for _, a := range cvSplitTop(js[open+1 : close]) {
		trimmed := strings.TrimLeft(a, " \t\n\r")
		occ.argPos = append(occ.argPos, offset+(len(a)-len(trimmed)))
		occ.args = append(occ.args, strings.TrimSpace(a))
		offset += len(a) + 1
	}
	if len(occ.args) < 2 {
		return occ, fmt.Errorf("createElement: %d args at %d", len(occ.args), open)
	}
	tag := cvParseExpr(occ.args[0])
	switch {
	case tag.kind == "str":
		occ.tag, occ.tagOK = tag.str, true
	case occ.args[0] == "React.Fragment":
		// a fragment is not an element (babel's JSXFragment never entered
		// the walk); its children are visited via the plain scan
	case tag.kind == "ident":
		occ.tag, occ.tagOK = tag.ident, true
	case cvMemberRe.MatchString(occ.args[0]):
		occ.tag, occ.tagOK = occ.args[0], true
	}
	for k := 2; k < len(occ.args); k++ {
		t := occ.args[k]
		if strings.HasPrefix(t, "/* @__PURE__ */") {
			t = strings.TrimLeft(strings.TrimPrefix(t, "/* @__PURE__ */"), " \t\n\r")
		}
		if strings.HasPrefix(t, "React.createElement(") && t != "React.createElement(React.Fragment" &&
			!strings.HasPrefix(t, "React.createElement(React.Fragment,") &&
			!strings.HasPrefix(t, "React.createElement(React.Fragment)") {
			occ.childArg = append(occ.childArg, k)
			occ.childPos = append(occ.childPos, occ.argPos[k]+len(occ.args[k])-len(t))
		}
	}
	return occ, nil
}

func convertFile(name, src, js string, reg *cvReg) (*cvFile, error) {
	f := &cvFile{
		name:         name,
		cva:          newCvTables(),
		metaModuleOf: map[string]string{},
		metaTagVars:  map[string]string{},
		metaImport:   map[string]string{},
	}
	f.imports, f.icons, f.metaImport, f.metaModuleOf = importsOf(src)

	top, err := scanTopJs(js)
	if err != nil {
		return nil, err
	}

	// per-file cva tables
	f.cva = cvTablesOf(js)

	recs, err := scanJsxKinds(src)
	if err != nil {
		return nil, fmt.Errorf("jsx scan: %w", err)
	}
	calls, err := cvFindCalls(js)
	if err != nil {
		return nil, err
	}
	if len(calls) != len(recs) {
		return nil, fmt.Errorf("element scan misalignment: %d downgraded calls vs %d source elements", len(calls), len(recs))
	}
	// pair every call with its record; a pair belongs to the (unique)
	// top-level fn whose body contains it — babel walked only pushed fns, so
	// pairs outside any pushed fn (a non-exported const arrow) drop on both
	// sides together.
	pairs := make([]cvCallPair, len(calls))
	callPosToPair := map[int]int{}
	for k := range pairs {
		pairs[k] = cvCallPair{call: calls[k], rec: recs[k], fnIndex: -1}
		callPosToPair[calls[k].pos] = k
	}
	// pushed fns in declaration order: function decls always; var arrows
	// only when declared WITH `export` (babel pushed ExportNamedDeclaration
	// declarations only); an anonymous default arrow as "default".
	var pushed []int
	for di, d := range top.decls {
		if d.isFn || (d.isArrow && d.exported) || di == top.defaultArrow {
			pushed = append(pushed, di)
		}
	}
	for k := range pairs {
		for pi, di := range pushed {
			d := top.decls[di]
			if pairs[k].call.pos > d.body[0] && pairs[k].call.pos < d.body[1] {
				pairs[k].fnIndex = pi
				break
			}
		}
	}

	// exported names: export-spec names + declarations declared with `export`
	exported := map[string]bool{}
	for n := range top.exportedNames {
		exported[n] = true
	}
	for _, d := range top.decls {
		if d.exported && (d.isFn || d.isArrow) {
			exported[d.name] = true
		}
	}

	// component assembly
	fileRefKeys := map[string]bool{}
	for pi, di := range pushed {
		d := top.decls[di]
		fnName := d.name
		if di == top.defaultArrow {
			fnName = "default"
		}
		// merged per file (conflicts throw inside cvTagVarsOf)
		tagVars, err := cvTagVarsOf(js, d.body)
		if err != nil {
			return nil, err
		}
		for k, v := range tagVars {
			f.metaTagVars[k] = v
		}
		comp := cvComponent{fn: fnName, isExport: exported[fnName]}
		ctx := &cvCtx{file: f, reg: reg, src: src, js: js,
			paramDefaults: cvParamDefaults(js, d), fnName: fnName,
			bodyStart: d.body[0], bodyEnd: d.body[1], refKeys: fileRefKeys}
		for k := range pairs {
			if pairs[k].fnIndex != pi {
				continue
			}
			el, conds, err := cvProcessElement(ctx, pairs, k, callPosToPair)
			if err != nil {
				return nil, err
			}
			comp.elements = append(comp.elements, el)
			f.conditionals = append(f.conditionals, conds...)
		}
		f.components = append(f.components, comp)
	}

	f.tier = tierOf(name, f.imports)
	cvSameFileWrap(f)
	return f, nil
}

type cvCallPair struct {
	call    cvCallOcc
	rec     cvJsxRec
	fnIndex int // index into the pushed-fn list, -1 = dropped
}

// cvProcessElement builds one element record + its conditionals from a
// paired (downgraded call, source kind list).
func cvProcessElement(ctx *cvCtx, pairs []cvCallPair, k int, callPosToPair map[int]int) (jsonObj, []jsonObj, error) {
	pair := &pairs[k]
	occ := &pair.call
	el := &cvElCtx{}
	var props []cvProp
	if propsText := occ.args[1]; propsText != "null" {
		if n := cvParseExpr(propsText); n.kind == "obj" {
			props = n.obj
		}
	}
	el.props = props
	var slot any
	var classes []string
	spread := false
	attrs := jsonObj{}
	for _, p := range props {
		if p.spread {
			spread = true
			continue
		}
		if p.key == "data-slot" {
			if n := cvParseExpr(p.val); n.kind == "str" {
				slot = n.str
			}
		}
		if p.key == "className" {
			ctx.classStrings(p.val, &classes, el)
		}
		if cvIdentityAttr.MatchString(p.key) {
			if n := cvParseExpr(p.val); n.kind == "str" {
				attrs = attrs.add(p.key, n.str)
			}
		}
	}
	// component-wrap: an imported component (e.g. Button) renders with the
	// wrapped table's classes — resolve them onto this element (bounded: axis
	// props must be literal / ??lit / param-default). Works for slot
	// overrides (`<Button data-slot=…>`, ref rules recorded) AND slotless
	// wraps (`<PaginationLink size="default">`). ORDER is React's: the
	// wrapped component composes cn(itsVariants(...), className) — its own
	// classes FIRST, the wrapper's className after. Appending them reversed
	// the cascade once the emitter started twMerge-ing lists: rounded-lg
	// (button) beat the carousel arrow's rounded-full.
	if occ.tagOK && cvIdentRe.MatchString(occ.tag) {
		if imp, ok := ctx.file.metaImport[occ.tag]; ok {
			if comp, ok := ctx.reg.compCva[imp]; ok {
				args := map[string]string{}
				for _, p := range props {
					if p.spread {
						continue
					}
					if comp.table.hasAxis(p.key) {
						args[p.key] = p.val
					}
				}
				var wrapped []string
				if err := ctx.resolveCvaArgs(el, comp.table, args, &wrapped,
					comp.file+":"+comp.cvaName, true); err != nil {
					return nil, nil, err
				}
				classes = append(wrapped, classes...)
			}
		}
	}
	var classesClean []string
	for _, c := range classes {
		if c != "" {
			classesClean = append(classesClean, c)
		}
	}
	// children from the ORIGINAL side's kind list; element kinds consume the
	// direct element-call args of THIS call in order
	elemKindCount := 0
	for _, kd := range pair.rec.kinds {
		if kd.elem {
			elemKindCount++
		}
	}
	if elemKindCount != len(occ.childPos) {
		return nil, nil, fmt.Errorf("children misalignment at %s:%s element %s: %d source element children vs %d downgraded",
			ctx.file.name, ctx.fnName, occ.tag, elemKindCount, len(occ.childPos))
	}
	var children []any
	ei := 0
	for _, kd := range pair.rec.kinds {
		if !kd.elem {
			children = append(children, kd.text)
			continue
		}
		child := &pairs[callPosToPair[occ.childPos[ei]]]
		ei++
		sketch := "<" + child.call.tag
		if s := cvChildSlot(&child.call); s != "" {
			sketch += " slot=" + s
		}
		if n := len(cvChildClasses(&child.call)); n > 0 {
			sketch += fmt.Sprintf(" class=[%d]", n)
		}
		sketch += ">"
		children = append(children, sketch)
	}
	tagAny := any(nil)
	if occ.tagOK {
		tagAny = occ.tag
	}
	o := jsonObj{}.add("tag", tagAny).add("slot", slot).
		add("classes", cvStrSlice(classesClean)).add("spread", spread).
		add("children", children)
	if len(attrs) > 0 {
		// additive and omitted when empty, so 60 of the 61 IR files are
		// unchanged
		o = o.add("attrs", attrs)
	}
	// child-cond: an expression-container child that is a logical or
	// conditional; then class-cond inside className cn() args
	var conds []jsonObj
	for _, kd := range pair.rec.kinds {
		if !kd.elem && kd.text == "OPT?" {
			conds = append(conds, jsonObj{}.add("kind", "child-cond").
				add("fn", ctx.fnName).add("parent", tagAny))
		}
	}
	for _, p := range props {
		if p.spread || p.key != "className" {
			continue
		}
		conds = append(conds, cvDetectCond(ctx, p.val, slot)...)
	}
	return o, conds, nil
}

// cvChildSlot / cvChildClasses — extractAttrs(child, null) for the sketch:
// literal-only classes, no cva context.
func cvChildSlot(occ *cvCallOcc) string {
	if occ.args[1] == "null" {
		return ""
	}
	if n := cvParseExpr(occ.args[1]); n.kind == "obj" {
		for _, p := range n.obj {
			if !p.spread && p.key == "data-slot" {
				if v := cvParseExpr(p.val); v.kind == "str" {
					return v.str
				}
			}
		}
	}
	return ""
}

func cvChildClasses(occ *cvCallOcc) []string {
	var acc []string
	if occ.args[1] == "null" {
		return acc
	}
	if n := cvParseExpr(occ.args[1]); n.kind == "obj" {
		for _, p := range n.obj {
			if !p.spread && p.key == "className" {
				cvClassStringsNil(p.val, &acc)
			}
		}
	}
	var out []string
	for _, c := range acc {
		if c != "" {
			out = append(out, c)
		}
	}
	return out
}

// cvDetectCond — port of extractFn's class-cond detect() walk.
func cvDetectCond(ctx *cvCtx, text string, slot any) []jsonObj {
	var out []jsonObj
	var detect func(t string)
	detect = func(t string) {
		n := cvParseExpr(t)
		if n.kind == "cond" {
			// the predicate, when it is `ident === "literal"` / `!==`: the
			// css emitter turns such branches into attribute-keyed twin
			// rules. `default` is the literal default of that ident anywhere
			// in the file (carousel: orientation = "horizontal" lives on the
			// ROOT fn and reaches the item via context) — absent attribute
			// ⇒ default.
			var test jsonObj
			hasTest := false
			if m := cvCondTestRe.FindStringSubmatch(strings.TrimSpace(n.test)); m != nil {
				o := jsonObj{}.add("name", m[1]).add("op", m[2]).add("value", cvUnquote(m[3]))
				if mm := cvCondDefaultRe(m[1]).FindStringSubmatch(ctx.src); mm != nil {
					o = o.add("default", mm[1])
				}
				test, hasTest = o, true
			}
			thenStr, elseStr := "", ""
			if cn := cvParseExpr(n.cons); cn.kind == "str" {
				thenStr = cn.str
			}
			if an := cvParseExpr(n.alt); an.kind == "str" {
				elseStr = an.str
			}
			c := jsonObj{}.add("kind", "class-cond").add("fn", ctx.fnName).add("slot", slot).
				add("then", thenStr).add("else", elseStr)
			if hasTest {
				c = c.add("test", test)
			}
			out = append(out, c)
			return
		}
		if n.kind == "logical" {
			detect(n.right)
			return
		}
		if n.kind == "call" {
			for _, a := range n.args {
				detect(a)
			}
		}
	}
	detect(text)
	return out
}

var cvCondTestRe = regexp.MustCompile(`^([A-Za-z_$][\w$]*)\s*(===|!==)\s*("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')$`)

func cvCondDefaultRe(name string) *regexp.Regexp {
	return regexp.MustCompile(`\b` + regexp.QuoteMeta(name) + `\s*=\s*"([^"]+)"`)
}

// cvTagVarsOf — fn-local tag variables: `const Comp = asChild ? Slot.Root :
// "div"` and `const Tag = "span"` — the native string is what the no-React
// emitter renders (the Slot branch is React-only).
func cvTagVarsOf(js string, body [2]int) (map[string]string, error) {
	out := map[string]string{}
	seg := js[body[0]:body[1]]
	re := regexp.MustCompile(`(?:^|[^\w$.])(const|let|var)\s+([A-Za-z_$][\w$]*)\s*=`)
	for _, loc := range re.FindAllStringSubmatchIndex(seg, -1) {
		if !cvPosUnmasked(js, body[0], body[0]+loc[0]) {
			continue
		}
		name := seg[loc[4]:loc[5]]
		// init runs to the next top-level , or ; (multi-declarator support)
		initStart := loc[1]
		depth := 0
		initEnd := len(seg)
		for i := initStart; i < len(seg); {
			if e := cvMaskEnd(seg, i); e > 0 {
				i = e
				continue
			}
			switch seg[i] {
			case '(', '[', '{':
				depth++
			case ')', ']', '}':
				depth--
			case ',', ';':
				if depth == 0 {
					initEnd = i
					break
				}
			}
			if initEnd != len(seg) {
				break
			}
			i++
		}
		init := cvParseExpr(seg[initStart:initEnd])
		var t string
		switch init.kind {
		case "str":
			t = init.str
		case "cond":
			if a := cvParseExpr(init.alt); a.kind == "str" {
				t = a.str
			}
		}
		if t == "" {
			continue
		}
		if old, ok := out[name]; ok && old != t {
			return nil, fmt.Errorf("conflicting tag var %s: %s vs %s", name, old, t)
		}
		out[name] = t
	}
	return out, nil
}

// cvParamDefaults — literal-default params feed cva-call resolution (e.g.
// `size = "icon-xs"`). Defaults live inside destructuring ObjectPatterns
// (`{ size = "icon" }`), not only as top-level AssignmentPattern params.
// babel only accepted an AssignmentPattern whose left is an Identifier, so
// depth-≥2 nested defaults are (still) not extracted.
func cvParamDefaults(js string, d cvDecl) map[string]string {
	out := map[string]string{}
	if d.singleParam != "" {
		return out
	}
	paramsText := js[d.params[0] : d.params[1]+1]
	inner := paramsText[1 : len(paramsText)-1]
	for _, pm := range cvSplitTop(inner) {
		p := strings.TrimSpace(pm)
		if p == "" {
			continue
		}
		if strings.HasPrefix(p, "{") {
			if e := cvMatchBracket(p, 0, '{', '}'); e > 0 {
				for _, q := range cvPatternDefaults(p[1:e]) {
					out[q[0]] = q[1]
				}
			}
			continue
		}
		if strings.HasPrefix(p, "[") {
			if e := cvMatchBracket(p, 0, '[', ']'); e > 0 {
				for _, q := range cvPatternDefaults(p[1:e]) {
					out[q[0]] = q[1]
				}
			}
			continue
		}
		if name, lit, ok := cvAssignPatternDefault(p); ok {
			out[name] = lit
		}
	}
	return out
}

// cvPatternDefaults: top-level elements of a destructuring pattern body —
// `ident`, `key: ident`, `ident = "lit"`, `key: ident = "lit"`.
func cvPatternDefaults(inner string) [][2]string {
	var out [][2]string
	for _, part := range cvSplitTop(inner) {
		p := strings.TrimSpace(part)
		if p == "" || strings.HasPrefix(p, "...") || strings.HasPrefix(p, "{") || strings.HasPrefix(p, "[") {
			continue
		}
		// a `key:` renames — the BINDING is after the colon
		if colon := cvPropColon(p); colon >= 0 {
			p = strings.TrimSpace(p[colon+1:])
		}
		if name, lit, ok := cvAssignPatternDefault(p); ok {
			out = append(out, [2]string{name, lit})
		}
	}
	return out
}

// cvAssignPatternDefault: `name = "lit"` → (name, lit); `name` alone → no.
func cvAssignPatternDefault(p string) (string, string, bool) {
	eq := -1
	depth := 0
	for i := 0; i < len(p); {
		if e := cvMaskEnd(p, i); e > 0 {
			i = e
			continue
		}
		switch p[i] {
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case '=':
			if depth == 0 {
				if i+1 < len(p) && (p[i+1] == '=' || p[i+1] == '>') {
					i++
					continue
				}
				eq = i
			}
		}
		if eq >= 0 {
			break
		}
		i++
	}
	if eq < 0 {
		return "", "", false
	}
	name := strings.TrimSpace(p[:eq])
	if !cvIdentRe.MatchString(name) {
		return "", "", false
	}
	lit := cvParseExpr(p[eq+1:])
	if lit.kind != "str" {
		return "", "", false
	}
	return name, lit.str, true
}

// cvSameFileWrap — same-file slotless wrap: a fn root that renders another
// same-file fn (e.g. PaginationPrevious → PaginationLink) and sets no
// data-slot rides the wrapped fn's root classes (cva-resolved), or the
// emitted anchor rule ships without the base styles. Cross-file wraps were
// already resolved via compCva; slotted roots keep that path.
func cvSameFileWrap(f *cvFile) {
	byFn := map[string]*cvComponent{}
	for i := range f.components {
		byFn[f.components[i].fn] = &f.components[i]
	}
	var effRoot func(fnName string, stack map[string]bool) []string
	effRoot = func(fnName string, stack map[string]bool) []string {
		c := byFn[fnName]
		if c == nil || len(c.elements) == 0 || stack[fnName] {
			return nil
		}
		root := c.elements[0]
		slot, _ := root[1].V.(string)
		if slot != "" {
			// slotted base case: classes live here
			return cvJSONStrings(root[2].V)
		}
		stack[fnName] = true
		tag, _ := root[0].V.(string)
		var wrapped []string
		if tag != "" && byFn[tag] != nil {
			wrapped = effRoot(tag, stack)
		}
		return append(wrapped, cvJSONStrings(root[2].V)...)
	}
	for i := range f.components {
		c := &f.components[i]
		if len(c.elements) == 0 {
			continue
		}
		root := c.elements[0]
		slot, _ := root[1].V.(string)
		tag, _ := root[0].V.(string)
		if slot == "" && tag != "" && byFn[tag] != nil && tag != c.fn {
			// [...new Set(effRoot(c.fn))].filter(Boolean) — insertion-ordered
			// dedupe, falsy dropped
			seen := map[string]bool{}
			var out []any
			for _, s := range effRoot(c.fn, map[string]bool{}) {
				if s == "" || seen[s] {
					continue
				}
				seen[s] = true
				out = append(out, s)
			}
			root[2].V = out
		}
	}
}

func cvJSONStrings(v any) []string {
	if arr, ok := v.([]any); ok {
		var out []string
		for _, e := range arr {
			if s, ok := e.(string); ok {
				out = append(out, s)
			}
		}
		return out
	}
	return nil
}

// ---------------------------------------------------------------------------
// tagHints

func isIconName(t string, icons []string) bool {
	for _, i := range icons {
		if i == t {
			return true
		}
	}
	for _, i := range cvKnownIcons {
		if i == t {
			return true
		}
	}
	return strings.HasSuffix(t, "Icon")
}

// buildTagHints — cross-file tag resolution (needs every IR); returns an
// error on unresolved → loud fail.
func buildTagHints(irs []*cvFile) error {
	byName := map[string]*cvFile{}
	for _, ir := range irs {
		byName[ir.name] = ir
	}
	fnRoot := map[string]string{}
	for _, ir := range irs {
		for _, c := range ir.components {
			if len(c.elements) > 0 {
				if t, ok := c.elements[0][0].V.(string); ok {
					fnRoot[ir.name+":"+c.fn] = t
				}
			}
		}
	}
	var resolveRoot func(file, fn string, seen map[string]bool) string
	hintFor := func(ir *cvFile, tag string, seen map[string]bool) string {
		if nativeTags[tag] {
			return tag
		}
		if m := ternaryRe.FindStringSubmatch(tag); m != nil {
			if nativeTags[m[2]] {
				return m[2]
			}
			return ""
		}
		if isIconName(tag, ir.icons) {
			return "svg"
		}
		// app-side icon helper (@/app/... import, renders an svg placeholder)
		if mod, ok := ir.metaModuleOf[tag]; ok && strings.HasPrefix(mod, "@/app/") {
			return "svg"
		}
		if tv, ok := ir.metaTagVars[tag]; ok {
			return tv
		}
		for _, c := range ir.components {
			if c.fn == tag {
				return resolveRoot(ir.name, tag, seen)
			}
		}
		mod, hasMod := ir.metaModuleOf[tag]
		if hasMod && mod != "" {
			stem := regexp.MustCompile(`\.[tj]sx?$`).ReplaceAllString(
				mod[strings.LastIndex(mod, "/")+1:], "")
			dep := byName[stem]
			if dep != nil {
				// registry files export the component under the imported name
				importedName := tag
				if im, ok := ir.metaImport[tag]; ok {
					importedName = im
				}
				found := false
				for _, c := range dep.components {
					if c.fn == importedName {
						found = true
						break
					}
				}
				if !found {
					return ""
				}
				return resolveRoot(stem, importedName, seen)
			}
			if strings.Contains(tag, ".") {
				return externalMemberTag(tag)
			}
			return ""
		}
		if strings.Contains(tag, ".") {
			return externalMemberTag(tag)
		}
		return ""
	}
	resolveRoot = func(file, fn string, seen map[string]bool) string {
		key := file + ":" + fn
		if seen[key] {
			return "" // cycle guard
		}
		raw, ok := fnRoot[key]
		if !ok {
			return ""
		}
		if nativeTags[raw] {
			return raw
		}
		if m := ternaryRe.FindStringSubmatch(raw); m != nil {
			if nativeTags[m[2]] {
				return m[2]
			}
			return ""
		}
		// root references another component → same-file fn, else imported
		target := byName[file]
		if target == nil {
			return ""
		}
		next := func() map[string]bool {
			n := map[string]bool{}
			for k := range seen {
				n[k] = true
			}
			n[key] = true
			return n
		}
		for _, c := range target.components {
			if c.fn == raw {
				return resolveRoot(file, raw, next())
			}
		}
		return hintFor(target, raw, next())
	}
	var unresolved []string
	for _, ir := range irs {
		hints := jsonObj{}
		for _, c := range ir.components {
			for _, el := range c.elements {
				rawTag, _ := el[0].V.(string)
				if el[0].V == nil || nativeTags[rawTag] || strings.HasPrefix(rawTag, "<ternary:") {
					continue
				}
				if hintsIndexOf(hints, rawTag) < 0 {
					h := hintFor(ir, rawTag, map[string]bool{})
					if h == "" {
						// external-tier files are tombstones (react-day-picker
						// etc.) — their framework tags are unresolvable by
						// design; only flag tiers the emitter actually renders
						if ir.tier != "external" {
							unresolved = append(unresolved, fmt.Sprintf("%s:%s: %s", ir.name, c.fn, rawTag))
						}
					} else {
						hints = hints.add(rawTag, h)
					}
				}
			}
		}
		ir.tagHints = hints
	}
	if len(unresolved) > 0 {
		return fmt.Errorf("tagHints unresolved (%d):\n  %s", len(unresolved), strings.Join(unresolved, "\n  "))
	}
	return nil
}

func hintsIndexOf(o jsonObj, k string) int {
	for i, p := range o {
		if p.K == k {
			return i
		}
	}
	return -1
}

// ---------------------------------------------------------------------------
// json assembly

func (f *cvFile) json() jsonObj {
	var comps []any
	for _, c := range f.components {
		els := make([]any, len(c.elements))
		for i, e := range c.elements {
			els[i] = e
		}
		comps = append(comps, jsonObj{}.
			add("fn", c.fn).add("export", c.isExport).add("elements", els))
	}
	return jsonObj{}.
		add("schema", 2).
		add("source", jsonObj{}.add("commit", f.pinCommit)).
		add("name", f.name).
		add("tier", f.tier).
		add("imports", cvStrSlice(f.imports)).
		add("icons", cvStrSlice(f.icons)).
		add("cva", f.cva.json()).
		add("components", comps).
		add("conditionals", cvAnyJSON(f.conditionals)).
		add("cvaRefs", cvAnyJSON(f.cvaRefs)).
		add("tagHints", f.tagHints)
}

// ---------------------------------------------------------------------------
// the convert verb: pipeline + drift gates

func runConvert() int {
	pinRaw, err := os.ReadFile(cvPinFile)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	pinCommit := any(nil)
	if cm := regexp.MustCompile(`"shadcn_ui"\s*:\s*\{[^{}]*"commit"\s*:\s*"([^"]+)"`).FindSubmatch(pinRaw); cm != nil {
		pinCommit = string(cm[1])
	}
	tiersRaw, err := os.ReadFile(cvTiers)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	// tiers.json key order is load-bearing for the wantDist insertion order
	tiersOrder, tierByName, err := cvParseTiers(tiersRaw)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	if err := os.MkdirAll(cvOut, 0o755); err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	ents, err := os.ReadDir(cvUI)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	var files []string
	for _, e := range ents {
		if strings.HasSuffix(e.Name(), ".tsx") {
			files = append(files, strings.TrimSuffix(e.Name(), ".tsx"))
		}
	}
	sort.Strings(files)
	if len(files) != 61 {
		fmt.Fprintf(os.Stderr, "FAIL expected 61 files, found %d\n", len(files))
		return 1
	}
	// global cva variant keys (cross-file: pagination uses button's variants)
	// + cross-file cva registry: exported cva tables and their
	// convention-named components (buttonVariants ↔ Button), so consumers in
	// other files (pagination.tsx, toggle-group.tsx, attachment.tsx) can be
	// resolved.
	srcOf := map[string]string{}
	jsOf := map[string]string{}
	for _, name := range files {
		b, err := os.ReadFile(filepath.Join(cvUI, name+".tsx"))
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			return 1
		}
		srcOf[name] = string(b)
		js, err := esbuildTsx(srcOf[name])
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL transform[%s]: %v\n", name, err)
			return 1
		}
		jsOf[name] = js
	}
	reg, globalKeys := cvBuildReg(files, jsOf)
	reg.pinCommit = pinCommit
	allSrcJoined := func() string {
		parts := make([]string, len(files))
		for i, n := range files {
			parts[i] = srcOf[n]
		}
		return strings.Join(parts, "\n")
	}()
	var distPairs []jsonPair
	distCount := map[string]int{}
	var summary []cvSummary
	fail := false
	var irs []*cvFile
	for _, name := range files {
		src, js := srcOf[name], jsOf[name]

		// gate 1: babel string count == raw-text double-quote regex count.
		// Template spans (String.raw`…`, `…`) are excluded: babel's gate
		// counted StringLiteral+DirectiveLiteral nodes only.
		babelCount := 0
		for _, sp := range tsx.StringLiterals(src) {
			if !sp.Template {
				babelCount++
			}
		}
		babelCount += len(tsx.DirectiveSpans(src))
		stripped := regexp.MustCompile(`(?s)/\*[\s\S]*?\*/`).ReplaceAllString(src, "")
		stripped = regexp.MustCompile(`(^|[^:])//[^\n]*`).ReplaceAllString(stripped, "$1")
		rawCount := len(regexp.MustCompile(`"(?:[^"\\]|\\.)*"`).FindAllString(stripped, -1))
		if babelCount != rawCount {
			fmt.Fprintf(os.Stderr, "FAIL drift[%s]: babel strings %d != raw grep %d\n", name, babelCount, rawCount)
			fail = true
		}

		ir, err := convertFile(name, src, js, reg)
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL convert[%s]: %v\n", name, err)
			fail = true
			continue
		}
		ir.pinCommit = pinCommit
		irs = append(irs, ir)

		// gate 2: every IR class string appears verbatim in source. cva
		// variant values may be arrays joined across source lines — for
		// those, fall back to verbatim-per-token (each atomic class token
		// must appear; an invented token still fails)
		verbatim := func(s string) bool {
			return strings.Contains(src, s) || strings.Contains(allSrcJoined, s)
		}
		var allClasses []string
		for _, c := range ir.components {
			for _, el := range c.elements {
				allClasses = append(allClasses, cvJSONStrings(el[2].V)...)
			}
		}
		for _, tn := range ir.cva.names {
			t := ir.cva.get(tn)
			if t.base != "" {
				allClasses = append(allClasses, t.base)
			}
			for _, ax := range t.axes {
				for _, kv := range ax.values {
					if s, _ := kv.v.(string); s != "" {
						allClasses = append(allClasses, s)
					}
				}
			}
		}
		for _, c := range allClasses {
			ok := verbatim(c)
			if !ok {
				all := len(c) > 0
				for _, tok := range strings.Fields(c) {
					if !verbatim(tok) {
						all = false
						break
					}
				}
				ok = all
			}
			if !ok {
				fmt.Fprintf(os.Stderr, "FAIL drift[%s]: class string not in source: %s\n", name, jsonString(c))
				fail = true
			}
		}

		// gate 2b (completeness, independent scanner): every quoted string
		// inside a cn(...) call in the raw source must be recorded in IR
		// classes (external-tier files are tombstones — their
		// framework-specific cn calls (e.g. react-day-picker classNames
		// maps) are out of scope)
		if ir.tier != "external" {
			irStrings := map[string]bool{}
			for _, c := range allClasses {
				irStrings[c] = true
			}
			cmpLits := map[string]bool{}
			for _, m := range regexp.MustCompile(`[=!]==?\s*"((?:[^"\\]|\\.)*)"`).FindAllStringSubmatch(stripped, -1) {
				cmpLits[m[1]] = true
			}
			for _, tn := range ir.cva.names {
				t := ir.cva.get(tn)
				for _, ax := range t.axes {
					for _, kv := range ax.values {
						cmpLits[kv.k] = true
					}
				}
				for _, dv := range t.defaults {
					if s, ok := dv.v.(string); ok {
						cmpLits[s] = true
					}
				}
			}
			for k := range globalKeys {
				cmpLits[k] = true
			}
			for _, loc := range regexp.MustCompile(`\bcn\s*\(`).FindAllStringIndex(stripped, -1) {
				i := loc[1]
				depth := 1
				for i < len(stripped) && depth > 0 {
					if stripped[i] == '(' {
						depth++
					} else if stripped[i] == ')' {
						depth--
					}
					i++
				}
				inner := stripped[loc[1] : i-1]
				for _, m := range regexp.MustCompile(`"((?:[^"\\]|\\.)*)"`).FindAllStringSubmatch(inner, -1) {
					val := m[1]
					if val != "" && !cmpLits[val] && !irStrings[val] {
						fmt.Fprintf(os.Stderr, "FAIL drift[%s]: cn string not in IR: %s\n", name, jsonString(val))
						fail = true
					}
				}
			}
		} // tier !== external

		// gate 3: tier matches tiers.json
		want, hasWant := tierByName[name]
		if !hasWant || want != ir.tier {
			w := "undefined"
			if hasWant {
				w = want
			}
			fmt.Fprintf(os.Stderr, "FAIL tier[%s]: ir=%s want=%s\n", name, ir.tier, w)
			fail = true
		}
		distCount[ir.tier]++
		distPairs = cvBumpPairs(distPairs, ir.tier, distCount)
		summary = append(summary, cvSummary{name: name, tier: ir.tier,
			classes: len(allClasses), cond: len(ir.conditionals)})
	}

	// cross-file tag hints (needs all IRs; throws on unresolved → loud fail)
	if err := buildTagHints(irs); err != nil {
		fmt.Fprintf(os.Stderr, "FAIL tagHints: %v\n", err)
		fail = true
	}

	if !fail {
		for _, ir := range irs {
			if err := os.WriteFile(filepath.Join(cvOut, ir.name+".json"),
				[]byte(marshalJSStep(ir.json(), "", " ")), 0o644); err != nil {
				fmt.Fprintln(os.Stderr, err)
				return 1
			}
		}
	}
	wantCount := map[string]int{}
	var wantPairs []jsonPair
	for _, n := range tiersOrder {
		wantCount[tierByName[n]]++
		wantPairs = cvBumpPairs(wantPairs, tierByName[n], wantCount)
	}
	if !cvSortedEqual(distPairs, wantPairs) {
		fmt.Fprintf(os.Stderr, "FAIL tier distribution: %s != %s\n",
			cvCompactObj(distPairs), cvCompactObj(wantPairs))
		fail = true
	}
	condTotal := 0
	for _, s := range summary {
		condTotal += s.cond
	}
	fmt.Printf("convert: %d IR files -> %s\n", len(files), cvOut)
	fmt.Printf("tier dist: %s\n", cvCompactObj(distPairs))
	fmt.Printf("conditionals total: %d\n", condTotal)
	if fail {
		fmt.Println("FAIL  convert drift gates")
		return 1
	}
	fmt.Println("PASS  convert (0 drift, tiers match, tagHints resolved)")
	return 0
}

type cvSummary struct {
	name, tier string
	classes    int
	cond       int
}

func cvBumpPairs(pairs []jsonPair, tier string, count map[string]int) []jsonPair {
	if count[tier] == 1 {
		return append(pairs, jsonPair{tier, 1})
	}
	for i := range pairs {
		if pairs[i].K == tier {
			pairs[i].V = count[tier]
		}
	}
	return pairs
}

// cvCompactObj renders flat string→count pairs the way JSON.stringify does
// (insertion order).
func cvCompactObj(pairs []jsonPair) string {
	parts := make([]string, len(pairs))
	for i, p := range pairs {
		parts[i] = jsonString(p.K) + ":" + fmt.Sprint(p.V)
	}
	return "{" + strings.Join(parts, ",") + "}"
}

// cvSortedEqual compares the two dists the way JS did:
// JSON.stringify(Object.entries(a).sort()) === …same for b.
func cvSortedEqual(a, b []jsonPair) bool {
	as := make([]string, len(a))
	for i, p := range a {
		as[i] = jsonString(p.K) + "," + fmt.Sprint(p.V)
	}
	bs := make([]string, len(b))
	for i, p := range b {
		bs[i] = jsonString(p.K) + "," + fmt.Sprint(p.V)
	}
	sort.Strings(as)
	sort.Strings(bs)
	if len(as) != len(bs) {
		return false
	}
	for i := range as {
		if as[i] != bs[i] {
			return false
		}
	}
	return true
}

// cvParseTiers: tiers.json keys in file order + name→tier.
func cvParseTiers(raw []byte) ([]string, map[string]string, error) {
	top, err := decodeOrderedObject(raw)
	if err != nil {
		return nil, nil, err
	}
	out := map[string]string{}
	for _, k := range top.keys {
		inner, err := decodeOrderedObject(top.raw[k])
		if err != nil {
			return nil, nil, err
		}
		rawTier, ok := inner.raw["tier"]
		if !ok {
			continue
		}
		var tier string
		if err := json.Unmarshal(rawTier, &tier); err != nil {
			return nil, nil, err
		}
		out[k] = tier
	}
	return top.keys, out, nil
}

// cvExportedNames — collectExportedNames on the downgraded module: export
// spec names + fn/arrow declarations carrying the export keyword.
func cvExportedNames(js string) (map[string]bool, error) {
	top, err := scanTopJs(js)
	if err != nil {
		return nil, err
	}
	out := map[string]bool{}
	for n := range top.exportedNames {
		out[n] = true
	}
	for _, d := range top.decls {
		if d.exported && (d.isFn || d.isArrow) {
			out[d.name] = true
		}
	}
	return out, nil
}

// cvCvaCallPositions: every cva( call in the module at any depth (babel's
// walk saw them all), as absolute callee positions.
func cvCvaCallPositions(js string) []int {
	var out []int
	for i := 0; i < len(js); {
		if e := cvMaskEnd(js, i); e > 0 {
			i = e
			continue
		}
		if js[i] == 'c' && strings.HasPrefix(js[i:], "cva(") {
			if i > 0 {
				p := js[i-1]
				if p == '.' || p >= 'a' && p <= 'z' || p >= 'A' && p <= 'Z' ||
					p >= '0' && p <= '9' || p == '_' || p == '$' {
					i++
					continue
				}
			}
			out = append(out, i)
			i += 4
			continue
		}
		i++
	}
	return out
}

// cvCvaCfgAt: the config argument text of the cva( call at pos (its second
// top-level argument), or "".
func cvCvaCfgAt(js string, pos int) string {
	open := pos + len("cva")
	close := cvMatchBracket(js, open, '(', ')')
	if close < 0 {
		return ""
	}
	args := cvSplitTop(js[open+1 : close])
	if len(args) < 2 {
		return ""
	}
	return strings.TrimSpace(args[1])
}

// cvBuildReg — the cross-file cva registry every convertFile shares: global
// cva variant keys (cross-file: pagination uses button's variants), exported
// cva tables (cvaByExport) and their convention-named components
// (buttonVariants ↔ Button), so consumers in other files (pagination.tsx,
// toggle-group.tsx, attachment.tsx) can be resolved.
func cvBuildReg(files []string, jsOf map[string]string) (*cvReg, map[string]bool) {
	globalKeys := map[string]bool{}
	reg := newCvReg()
	for _, name := range files {
		js := jsOf[name]
		for _, pos := range cvCvaCallPositions(js) {
			cfg := cvCvaCfgAt(js, pos)
			if cfg == "" {
				continue
			}
			if n := cvParseExpr(cfg); n.kind == "obj" {
				for _, p := range n.obj {
					if p.quoted || p.spread || p.key != "variants" {
						continue
					}
					if pv := cvParseExpr(p.val); pv.kind == "obj" {
						for _, ax := range pv.obj {
							if av := cvParseExpr(ax.val); av.kind == "obj" {
								for _, vv := range av.obj {
									globalKeys[vv.key] = true
								}
							}
						}
					}
				}
			}
		}
		exported, err := cvExportedNames(js)
		if err != nil {
			continue
		}
		tables := cvTablesOf(js)
		for _, cvaName := range tables.names {
			if !exported[cvaName] {
				continue
			}
			reg.cvaByExport[cvaName] = &cvaExportEntry{file: name, table: tables.get(cvaName)}
			stem := strings.TrimSuffix(cvaName, "Variants")
			compName := strings.ToUpper(stem[:1]) + stem[1:]
			if exported[compName] {
				reg.compCva[compName] = &compCvaEntry{cvaName: cvaName, table: tables.get(cvaName), file: name}
			}
		}
	}
	return reg, globalKeys
}
