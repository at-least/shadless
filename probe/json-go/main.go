package main

// jsonorder fixture printer — compiles the REAL pipeline/jsonorder.go (copied
// verbatim by setup.sh) against a battery of nasty values, and writes the
// expected bytes to golden.txt. The Rust port's unit test asserts byte
// equality against this file.

import (
	"fmt"
	"os"
)

func fixtureValues() []struct {
	name string
	v    any
} {
	weird := "a\"b\\c\bd\fe\nf\rg\th" + "\x00\x01\x1f" + "<b>&</b>" + "\u2028\u2029" + "é中\U0001F600" + "\u007f" + "\u00a0"
	return []struct {
		name string
		v    any
	}{
		{"plain-string", jsonString("hello world")},
		{"escapes", jsonString(weird)},
		{"rtl", jsonString("زر عربي עברית")},
		{"raw-numbers", marshalJS(jsonObj{}.add("a", jsonRaw("1.5")).add("b", jsonRaw("2e3")).add("c", jsonRaw("-0")).add("d", jsonRaw("0.30000000000000004")).add("e", jsonRaw("1e-5")), "")},
		{"ints", marshalJS(jsonObj{}.add("zero", 0).add("neg", -7).add("big", int(9223372036854775807)), "")},
		{"bools-null", marshalJS(jsonObj{}.add("t", true).add("f", false).add("null", jsonNull{}), "")},
		{"empty", marshalJS(jsonObj{}, "") + "\n" + marshalJS([]any{}, "")},
		{"nested", marshalJS(jsonObj{}.add("obj", jsonObj{}.add("list", []any{"a", int(1), true, jsonNull{}}).add("e", jsonObj{})).add("arr", []any{jsonObj{}.add("k", "v")}), "")},
		{"arrays-of-strings", marshalJS(jsonObj{}.add("ss", []string{"x", "y\"z"}), "")},
		{"step-indent-1", marshalJSStep(jsonObj{}.add("k", jsonObj{}.add("in", "v")), "", " ")},
		{"top-indent", marshalJS(jsonObj{}.add("k", "v"), "  ")},
		{"colon-slash", jsonString("hover:before:bg-[url('a:b')][/&]")},
	}
}

func main() {
	f, err := os.Create("golden.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	for _, fx := range fixtureValues() {
		fmt.Fprintf(f, "==== %s ====\n", fx.name)
		fmt.Fprintf(f, "%s", fx.v)
		fmt.Fprintln(f)
	}
}
