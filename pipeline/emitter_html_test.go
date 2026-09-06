package main

import "testing"

// escHtml's doc comment claims a load-bearing order: & must be escaped first
// so a literal & in the input is never re-escaped by the later </>/" rules
// (which would turn a literal &lt; into &amp;lt;). Every existing call site
// (pipeline/default_content.go's escHtml("Badge"), escHtml("2.4 MB"), etc.)
// passes strings with none of &, <, >, " in them, so this exercises escHtml
// on inputs that actually need escaping.
func TestUnitEscHtml(t *testing.T) {
	got := escHtml(`<a>A&B</a>`)
	want := `&lt;a&gt;A&amp;B&lt;/a&gt;`
	if got != want {
		t.Fatalf("escHtml(%q) = %q, want %q", `<a>A&B</a>`, got, want)
	}

	// The hazard the doc comment calls out: a literal & already followed by
	// the entity text for one of the other three characters must come out
	// escaped exactly once, not double-escaped into &amp;lt; etc.
	in := `&lt; &gt; &quot; plain & amp`
	got = escHtml(in)
	want = `&amp;lt; &amp;gt; &amp;quot; plain &amp; amp`
	if got != want {
		t.Fatalf("escHtml(%q) = %q, want %q (a literal & must not be re-escaped by the </>/\" rules)", in, got, want)
	}

	// all four characters together, plus plain text around them
	in = `<div class="a" data-x="b">x & y</div>`
	got = escHtml(in)
	want = `&lt;div class=&quot;a&quot; data-x=&quot;b&quot;&gt;x &amp; y&lt;/div&gt;`
	if got != want {
		t.Fatalf("escHtml(%q) = %q, want %q", in, got, want)
	}
}
