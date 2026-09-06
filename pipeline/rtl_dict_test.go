package main

import (
	"os"
	"os/exec"
	"reflect"
	"strings"
	"testing"

	"shadless/pipeline/internal/tsx"
)

// The committed src/registry/rtl-translations.json was written by the JS
// rtl-dict; the Go one must produce byte-identical output or the whole RTL
// demo family shifts underneath gates that hash it.
func TestUnitRtlDictParity(t *testing.T) {
	root := ".."
	committed, err := os.ReadFile(root + "/src/registry/rtl-translations.json")
	if err != nil {
		t.Skip(err)
	}
	pipelineBin := t.TempDir() + "/pipeline"
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "rtl-dict")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("rtl-dict: %v\n%s", err, out)
	}
	got, err := os.ReadFile(root + "/src/registry/rtl-translations.json")
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != string(committed) {
		t.Fatalf("rtl-translations.json diverges from the JS-written bytes (first 400:\n%s)", got[:400])
	}
}

// decodeJSString: the success path plus every error branch the real
// .upstream/*-rtl.tsx corpus never exercises (it has no unterminated
// escapes, no \z-style unsupported escapes, no truncated \u).
func TestUnitDecodeJSString(t *testing.T) {
	cases := []struct {
		name    string
		in      string
		want    string
		wantErr bool
	}{
		{"no backslash: fast path returns raw unchanged", "plain text", "plain text", false},
		{"newline", `a\nb`, "a\nb", false},
		{"tab", `a\tb`, "a\tb", false},
		{"carriage return", `a\rb`, "a\rb", false},
		{"escaped double quote", "a\\\"b", `a"b`, false},
		{"escaped single quote", "a\\'b", "a'b", false},
		{"escaped backslash", `a\\b`, `a\b`, false},
		{"escaped backtick", "a\\`b", "a`b", false},
		{"\\u escape", `a\u00e9b`, "aéb", false},
		{"trailing backslash", `a\`, "", true},
		{"short \\u escape", `a\u12`, "", true},
		{"unsupported escape", `a\zb`, "", true},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			got, err := decodeJSString(c.in)
			if c.wantErr {
				if err == nil {
					t.Fatalf("decodeJSString(%q) = %q, want an error", c.in, got)
				}
				return
			}
			if err != nil {
				t.Fatalf("decodeJSString(%q): unexpected error: %v", c.in, err)
			}
			if got != c.want {
				t.Errorf("decodeJSString(%q) = %q, want %q", c.in, got, c.want)
			}
		})
	}
}

// parseLangObject: happy path (two languages, one carrying a discarded
// extra key) plus the malformed-input branches the real corpus never
// reaches. Every error case here terminates before the unguarded src[i]
// read in parseLangEntry's colon check — an actually-unterminated `{ dir`
// (no trailing char at all) panics there today; that is a real gap, not
// fixed here (out of scope for this test-coverage finding).
func TestUnitParseLangObject(t *testing.T) {
	src := `{ en: { dir: "ltr", locale: "en-US", values: { title: "Title", greeting: "Hi" } }, ar: { dir: "rtl", values: {} } }`
	langs, next, err := parseLangObject(tsx.StringLiterals(src), src, 0)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if next != len(src) {
		t.Errorf("next = %d, want %d (end of object)", next, len(src))
	}
	if want := []string{"en", "ar"}; !reflect.DeepEqual(langs.names, want) {
		t.Errorf("names = %v, want %v", langs.names, want)
	}
	if langs.dir["en"] != "ltr" || langs.dir["ar"] != "rtl" {
		t.Errorf("dir = %+v", langs.dir)
	}
	enVals := langs.values["en"]
	if want := []string{"title", "greeting"}; !reflect.DeepEqual(enVals.keys, want) {
		t.Errorf("en values.keys = %v, want %v", enVals.keys, want)
	}
	if enVals.vals["title"] != "Title" || enVals.vals["greeting"] != "Hi" {
		t.Errorf("en values.vals = %+v", enVals.vals)
	}
	if len(langs.values["ar"].keys) != 0 {
		t.Errorf("ar values should be empty, got %+v", langs.values["ar"])
	}

	errCases := []struct{ name, src, wantSubstr string }{
		{"unterminated object", "{", "unterminated translations object"},
		// padded well past the error's own (unguarded) src[i:i+20] slice —
		// a short comma-only source panics there today; that's a
		// pre-existing bug outside this finding's scope, not fixed here.
		{"missing key", "{," + strings.Repeat(" ", 25), "expected identifier key"},
		{"missing colon after key", "{ en }", `expected ':' after "en"`},
		{"value not an object literal", `{ en: "x" }`, `as value of "en"`},
	}
	for _, c := range errCases {
		t.Run(c.name, func(t *testing.T) {
			_, _, err := parseLangObject(tsx.StringLiterals(c.src), c.src, 0)
			if err == nil || !strings.Contains(err.Error(), c.wantSubstr) {
				t.Errorf("parseLangObject(%q) error = %v, want substring %q", c.src, err, c.wantSubstr)
			}
		})
	}
}

// parseValues: happy path (insertion order, quoted keys, duplicate key
// keeps its first position but takes the last value) plus its two error
// branches. Both are bounds-checked before indexing (unlike parseLangEntry's
// colon check), so no hand-written malformed input here can panic.
func TestUnitParseValues(t *testing.T) {
	src := `{ a: "x", b: "y", a: "z" }`
	vals, next, err := parseValues(tsx.StringLiterals(src), src, 0)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if next != len(src) {
		t.Errorf("next = %d, want %d", next, len(src))
	}
	if want := []string{"a", "b"}; !reflect.DeepEqual(vals.keys, want) {
		t.Errorf("keys = %v, want %v (duplicate key must not append twice)", vals.keys, want)
	}
	if vals.vals["a"] != "z" {
		t.Errorf(`vals["a"] = %q, want "z" (last write wins)`, vals.vals["a"])
	}
	if vals.vals["b"] != "y" {
		t.Errorf(`vals["b"] = %q, want "y"`, vals.vals["b"])
	}

	quotedSrc := `{ "weird key": "v" }`
	qVals, _, err := parseValues(tsx.StringLiterals(quotedSrc), quotedSrc, 0)
	if err != nil {
		t.Fatalf("unexpected error on quoted key: %v", err)
	}
	if qVals.vals["weird key"] != "v" {
		t.Errorf(`quoted key: vals["weird key"] = %q, want "v"`, qVals.vals["weird key"])
	}

	if _, _, err := parseValues(nil, "{", 0); err == nil || !strings.Contains(err.Error(), "unterminated values object") {
		t.Errorf("unterminated: error = %v", err)
	}
	if _, _, err := parseValues(nil, "{ a }", 0); err == nil || !strings.Contains(err.Error(), `expected ':' after value key "a"`) {
		t.Errorf("missing colon: error = %v", err)
	}
	// a quoted key with no matching span (readStringAt's own error, passed
	// through): spans intentionally empty even though the source has one.
	if _, _, err := parseValues(nil, `{ "a": "x" }`, 0); err == nil || !strings.Contains(err.Error(), "no string literal at") {
		t.Errorf("quoted key, no span: error = %v", err)
	}
}
