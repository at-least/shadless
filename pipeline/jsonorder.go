package main

// A JSON writer with JS semantics, for the generated files this pipeline
// commits.
//
// encoding/json cannot produce them: Go maps have no key order, and
// `JSON.stringify` output — which is what is in git — is ordered by insertion.
// Two more differences matter and both are silent:
//
//   - Go escapes <, > and & by default; JS does not. An upstream description
//     containing "&" would rewrite the file on the first Go run.
//   - JS DROPS a key whose value is `undefined` but KEEPS one whose value is
//     `null`. docs/catalog.json relies on that distinction: the radix scan
//     writes `styleName: attr(…) || null` (key present, null) and the guide
//     scan writes `styleName: attr(…)` (key absent when the attribute is).
//
// So the object type here is an ordered list of pairs and absence is modelled
// by simply not appending one.

import (
	"fmt"
	"strconv"
	"strings"
	"unicode/utf8"
)

type jsonPair struct {
	K string
	V any
}

// jsonObj is an ordered object. Append with add(); a key that should be
// absent is one you never add.
type jsonObj []jsonPair

func (o jsonObj) add(k string, v any) jsonObj { return append(o, jsonPair{k, v}) }

// jsonNull marks an explicit null (a key that exists with a null value), as
// distinct from a key that is not there at all.
type jsonNull struct{}

// jsonString escapes exactly as JSON.stringify does: quote, backslash and the
// C0 controls, with \b \f \n \r \t spelled out and everything else — including
// <, > and & — passed through as UTF-8.
func jsonString(s string) string {
	var b strings.Builder
	b.WriteByte('"')
	for _, r := range s {
		switch r {
		case '"':
			b.WriteString(`\"`)
		case '\\':
			b.WriteString(`\\`)
		case '\b':
			b.WriteString(`\b`)
		case '\f':
			b.WriteString(`\f`)
		case '\n':
			b.WriteString(`\n`)
		case '\r':
			b.WriteString(`\r`)
		case '\t':
			b.WriteString(`\t`)
		default:
			if r < 0x20 {
				fmt.Fprintf(&b, `\u%04x`, r)
			} else if r == utf8.RuneError {
				b.WriteString(`�`)
			} else {
				b.WriteRune(r)
			}
		}
	}
	b.WriteByte('"')
	return b.String()
}

// marshalJS renders v the way JSON.stringify(v, null, 2) would.
func marshalJS(v any, indent string) string {
	inner := indent + "  "
	switch x := v.(type) {
	case jsonObj:
		if len(x) == 0 {
			return "{}"
		}
		parts := make([]string, len(x))
		for i, p := range x {
			parts[i] = inner + jsonString(p.K) + ": " + marshalJS(p.V, inner)
		}
		return "{\n" + strings.Join(parts, ",\n") + "\n" + indent + "}"
	case []any:
		if len(x) == 0 {
			return "[]"
		}
		parts := make([]string, len(x))
		for i, e := range x {
			parts[i] = inner + marshalJS(e, inner)
		}
		return "[\n" + strings.Join(parts, ",\n") + "\n" + indent + "]"
	case []string:
		as := make([]any, len(x))
		for i, s := range x {
			as[i] = s
		}
		return marshalJS(as, indent)
	case string:
		return jsonString(x)
	case int:
		return strconv.Itoa(x)
	case bool:
		return strconv.FormatBool(x)
	case jsonNull, nil:
		return "null"
	default:
		panic(fmt.Sprintf("marshalJS: unsupported %T", v))
	}
}
