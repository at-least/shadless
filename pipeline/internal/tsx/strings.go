// Package tsx finds the UTF-8 byte offsets of string literals in TSX/TS
// source, matching @babel/parser's start/end exactly. (Babel counts UTF-16
// units internally, but the snapshot dump maps through the same source text
// the scanner reads, so byte offsets of one text are the interchange format;
// on this corpus — Arabic/Hebrew BMP, no astral planes — byte/rune/UTF-16
// ambiguity never separates them.)
//
// The tools that consumed @babel/parser (resolve-skins, converter, overlay)
// only ever needed "where is the literal and what's in it". A hand-rolled
// scanner with escapes, comments, template literals and regex-literal
// disambiguation covers it; conformance to babel is pinned over 562 files
// in spans-snapshot.json (regenerate: tools/tsx-spans-dump.mjs).
package tsx

import "strings"

// Span is a string literal's byte range, EXCLUSIVE of the quotes:
// src[Start:End] is the raw (escaped) literal text.
type Span struct {
	Start, End int
	Quote      byte // '"' or '\'' (or '`' when Template)
	Template   bool // a `…` span; only ones without ${} are surfaced
}

// Content reads the literal's raw text (escapes intact).
func (s Span) Content(src string) string { return src[s.Start:s.End] }

// StringLiterals returns every string literal's content span in source
// order. Comments and regex literals are skipped; a template literal that
// carries an interpolation (${}) contributes no span for its own TEXT, but
// the strings inside the interpolation are real literals (babel walks them)
// and ARE reported, in source order.
//
// Strings in directive position (the "use client" prologue: line-open
// literals before any other statement) are DROPPED, matching babel's AST
// where they live in Program.directives and never enter a StringLiteral
// walk.
func StringLiterals(src string) []Span {
	sc := &scanner{s: src}
	return sc.scan()
}

// inJSXDepth tracks how deep into JSX ELEMENT content the scan has
// descended. jsxTag increments it when the element is not self-closing and
// the scan resumes into its children; jsxClosingTag decrements. Only a
// depth>0 makes '`' backticks JSX text (inline code in prose), because at
// depth 0 they are real template literals.
type scanner struct {
	s        string
	jsxDepth int
	out      []Span
}

func (sc *scanner) scan() []Span {
	for i := 0; i < len(sc.s); {
		c := sc.s[i]
		switch {
		case c == '{' && sc.jsxExpr(i):
			i = sc.skipCode(i + 1)
		case sc.isJSXTag(i):
			i = sc.jsxTag(i) // attribute strings inside a <Tag> are recorded by jsxTag; jsxDepth managed there
		case c == '<' && sc.isClosingJSXTag(i):
			if sc.jsxDepth > 0 {
				sc.jsxDepth--
			}
			i = sc.jsxClosingTag(i)
		case c == '"' || c == '\'':
			start := i
			i = sc.stringLit(i)
			if isDirectivePosition(sc.s, start, i) {
				sc.out = sc.out[:len(sc.out)-1]
			}
		case c == '`':
			i = sc.templateLit(i)
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '/':
			for i < len(sc.s) && sc.s[i] != '\n' {
				i++
			}
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '*':
			i = sc.blockComment(i)
		case c == '/' && sc.regexAllowedHere(i):
			i = sc.regex(i)
		default:
			i++
		}
	}
	return sc.out
}

// jsxExpr: '{' opens a JSX expression container only when the previous
// significant byte is '>' (just after a close tag or self-close). That is
// the ONLY form this scanner recognises without an element stack: in the
// corpus JSX text nodes, `{" "}` and `{'…'}` are the only container shapes
// that carry strings.
//
// Babel's JSXExpressionContainer>StringLiteral node starts at the '{' —
// off by one from the quote — so its span never matches a quote-anchored
// scanner's. skipCode therefore DISCARDS the container string (mirroring
// the babel offsets would be a lie for splicing); the conformance delta
// (one span per `{" "}`-style node, 5 files in the corpus) is documented in
// strings_test.go's tolerated-divergence list.
func (sc *scanner) jsxExpr(i int) bool {
	for j := i - 1; j >= 0; j-- {
		c := sc.s[j]
		if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
			continue
		}
		return c == '>'
	}
	return false
}

// skipCodeDiscard is skipCode with literal recording OFF: used only for
// JSX expression containers.
func (sc *scanner) skipCodeDiscard(i int) int {
	depth := 1
	for i < len(sc.s) && depth > 0 {
		c := sc.s[i]
		switch {
		case c == '{':
			depth++
			i++
		case c == '}':
			depth--
			i++
		case c == '"' || c == '\'':
			i = sc.discardString(i)
		case c == '`':
			if sc.jsxDepth > 0 {
				i++ // JSX text: `autoScroll` in prose is JSXText, not a template literal
			} else {
				i = sc.templateLit(i)
			}
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '/':
			for i < len(sc.s) && sc.s[i] != '\n' {
				i++
			}
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '*':
			i = sc.blockComment(i)
		case c == '/' && sc.regexAllowedHere(i):
			i = sc.regex(i)
		default:
			i++
		}
	}
	return i
}

func (sc *scanner) stringLit(i int) int {
	q := sc.s[i]
	start := i + 1
	i++
	for i < len(sc.s) {
		c := sc.s[i]
		if c == '\\' {
			i += 2
			continue
		}
		if c == q {
			sc.out = append(sc.out, Span{Start: start, End: i, Quote: q})
			return i + 1
		}
		if c == '\n' || c == '\r' {
			// unterminated; record what's there so offsets stay stable
			sc.out = append(sc.out, Span{Start: start, End: i, Quote: q})
			return i
		}
		i++
	}
	sc.out = append(sc.out, Span{Start: start, End: len(sc.s), Quote: q})
	return len(sc.s)
}

func (sc *scanner) templateLit(i int) int {
	start := i + 1
	i++
	interpolated := false
	for i < len(sc.s) {
		c := sc.s[i]
		if c == '\\' {
			i += 2
			continue
		}
		if c == '`' {
			if !interpolated {
				sc.out = append(sc.out, Span{Start: start, End: i, Quote: '`', Template: true})
			}
			return i + 1
		}
		if c == '$' && i+1 < len(sc.s) && sc.s[i+1] == '{' {
			interpolated = true
			i = sc.skipCode(i + 2)
			continue
		}
		i++
	}
	return len(sc.s)
}

// skipCode starts just past ${ and returns the index past the matching }.
// Strings/templates inside interpolation are code-context literals: babel
// reports them, so stringLit/templateLit run with recording ON.
func (sc *scanner) skipCode(i int) int {
	depth := 1
	for i < len(sc.s) && depth > 0 {
		c := sc.s[i]
		switch {
		case c == '{':
			depth++
			i++
		case c == '}':
			depth--
			i++
		case c == '"' || c == '\'':
			i = sc.stringLit(i) // strings inside ${} / JSX containers are real literals
		case c == '`':
			i = sc.templateLit(i)
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '/':
			for i < len(sc.s) && sc.s[i] != '\n' {
				i++
			}
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '*':
			i = sc.blockComment(i)
		case c == '/' && sc.regexAllowedHere(i):
			i = sc.regex(i)
		default:
			i++
		}
	}
	return i
}

func (sc *scanner) discardString(i int) int {
	q := sc.s[i]
	i++
	for i < len(sc.s) {
		c := sc.s[i]
		if c == '\\' {
			i += 2
			continue
		}
		if c == q {
			return i + 1
		}
		i++
	}
	return i
}

func (sc *scanner) discardTemplate(i int) int {
	i++
	for i < len(sc.s) {
		c := sc.s[i]
		if c == '\\' {
			i += 2
			continue
		}
		if c == '`' {
			return i + 1
		}
		if c == '$' && i+1 < len(sc.s) && sc.s[i+1] == '{' {
			i = sc.skipCodeDiscard(i + 2)
			continue
		}
		i++
	}
	return i
}

// regexAllowedHere: '/' opens a regex only where a value cannot precede it.
// The previous significant byte decides; the operator/opener set permits a
// regex, operand terminators (identifier char, literal end, ')' ']' '}')
// do not. '"' and '\'' are operand terminators too: a quote always closes a
// preceding string literal, so what follows is at least an expression —
// never a regex. A heuristic; the corpus conformance test is the arbiter.
func (sc *scanner) regexAllowedHere(i int) bool {
	for j := i - 1; j >= 0; j-- {
		c := sc.s[j]
		if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
			continue
		}
		switch c {
		case ',', ';', ':', '(', '[', '{', '=', '<', '>',
			'&', '|', '^', '!', '~', '?', '-':
			return true
		}
		return false
	}
	return true
}

// regex consumes a regex literal from src[i]=='/'. Char classes are tracked
// so a '/' inside […] doesn't close it; trailing flags are consumed.
func (sc *scanner) regex(i int) int {
	i++
	inClass := false
	for i < len(sc.s) {
		c := sc.s[i]
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
			for i < len(sc.s) && (sc.s[i] >= 'a' && sc.s[i] <= 'z' || sc.s[i] >= 'A' && sc.s[i] <= 'Z') {
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

func (sc *scanner) blockComment(i int) int {
	i += 2
	for i+1 < len(sc.s) {
		if sc.s[i] == '*' && sc.s[i+1] == '/' {
			return i + 2
		}
		i++
	}
	return len(sc.s)
}

// ---------------------------------------------------------------------------
// JSX

// isJSXTag reports src[i] == '<' opening an element: an ASCII letter (or
// uppercase component) follows. '<' in comparisons ("a < b") is followed by
// whitespace/digit/quote, which this test rejects.
func (sc *scanner) isJSXTag(i int) bool {
	if sc.s[i] != '<' || i+1 >= len(sc.s) {
		return false
	}
	c := sc.s[i+1]
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '>' // fragment <>
}

func (sc *scanner) isClosingJSXTag(i int) bool {
	return sc.s[i] == '<' && i+1 < len(sc.s) && sc.s[i+1] == '/'
}

// jsxTag consumes `<Ident attr=…>` through its '>'. Attributes:
//	k=v        (string attribute → recorded via stringLit)
//	k={expr}   (recorded via skipCode for its interior strings)
//	k          (boolean attribute)
//	{...spread}
// Strings in attribute values ARE StringLiterals in babel, so recording is
// on. After '>': if the previous byte before '>' was '/', the element
// self-closes; otherwise we are INSIDE its children and the main loop
// continues with JSX-text rules (jsxExpr recognises `{` containers there).
func (sc *scanner) jsxTag(i int) int {
	i++ // past '<'
	for i < len(sc.s) && isJSXIdentStart(sc.s[i]) {
		i++
	}
	for i < len(sc.s) {
		c := sc.s[i]
		switch {
		case c == '"' || c == '\'':
			i = sc.stringLit(i)
		case c == '{':
			i = sc.skipCode(i + 1)
		case c == '/' && i+1 < len(sc.s) && sc.s[i+1] == '>':
			return i + 2 // self-close: no depth change
		case c == '>':
			sc.jsxDepth++
			return i + 1 // children follow; main loop resumes with JSX-text rules
		default:
			i++
		}
	}
	return i
}

func (sc *scanner) jsxClosingTag(i int) int {
	i += 2
	for i < len(sc.s) && sc.s[i] != '>' {
		i++
	}
	if i < len(sc.s) {
		i++
	}
	return i
}

func isJSXIdentStart(c byte) bool {
	return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_' || c == '$'
}

// isDirectivePosition approximates babel's Program.directives placement:
// the literal owns its line (whitespace and an optional ';' around it) and
// every earlier line is empty, a comment, or itself such a string line.
func isDirectivePosition(src string, start, end int) bool {
	// after the literal: ws, optional ';', ws, then newline or EOF
	j := end
	for j < len(src) && (src[j] == ' ' || src[j] == '\t') {
		j++
	}
	if j < len(src) && src[j] == ';' {
		j++
	}
	for j < len(src) && (src[j] == ' ' || src[j] == '\t') {
		j++
	}
	if j < len(src) && src[j] != '\n' {
		return false
	}
	// nothing but whitespace before the literal on its line
	i := start
	for i > 0 && src[i-1] != '\n' {
		if src[i-1] != ' ' && src[i-1] != '\t' {
			return false
		}
		i--
	}
	// every earlier line: empty, comment, or a full-line quoted string
	k := i - 1
	for k > 0 {
		lo := k - 1
		for lo >= 0 && src[lo] != '\n' {
			lo--
		}
		line := strings.TrimSpace(src[lo+1 : k])
		if line == "" || strings.HasPrefix(line, "//") || strings.HasPrefix(line, "/*") ||
			strings.HasPrefix(line, "*") || strings.HasSuffix(line, "*/") {
			k = lo
			continue
		}
		if len(line) > 2 && (line[0] == '"' || line[0] == '\'') {
			k = lo
			continue
		}
		return false
	}
	return true
}
