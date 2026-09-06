package main

// The golden-master upstream-snapshot logic: id normalization and
// stack-balanced HTML slicing, tested apart from the network-crawling
// driver (runUpstreamSnapshot) this repo's convention leaves untested.

import "testing"

func TestUnitNormSnapshot(t *testing.T) {
	cases := []struct{ in, want string }{
		{"radix-:r1:", "radix-<auto>"},  // CSR useId, colon-delimited
		{"radix-_r_ab", "radix-<auto>"}, // CSR useId, underscore-delimited
		{"radix-_R_AB", "radix-<auto>"}, // SSR useId shares the CSR bucket
	}
	for _, c := range cases {
		if got := normSnapshot(c.in); got != c.want {
			t.Errorf("normSnapshot(%q) = %q, want %q", c.in, got, c.want)
		}
	}
	// all three shapes normalize inside a real attribute soup, not just bare
	mixed := `<div id="radix-:r1:" aria-labelledby="radix-_r_ab-trigger">radix-_R_AB</div>`
	want := `<div id="radix-<auto>" aria-labelledby="radix-<auto>">radix-<auto></div>`
	if got := normSnapshot(mixed); got != want {
		t.Errorf("normSnapshot(mixed) = %q, want %q", got, want)
	}
}

func TestUnitSnapshotPreviewNames(t *testing.T) {
	// A decoy <ComponentPreview> sits inside a fenced code block: fenceShadow
	// must blank it so it cannot reorder or duplicate the real mapping.
	mdx := "text\n\n" +
		"<ComponentPreview name=\"accordion-demo\" />\n\n" +
		"```tsx\n" +
		"<ComponentPreview name=\"decoy-demo\" />\n" +
		"```\n\n" +
		"<ComponentPreview name=\"accordion-demo-2\" />\n"
	got := snapshotPreviewNames(mdx)
	want := []string{"accordion-demo", "accordion-demo-2"}
	if len(got) != len(want) {
		t.Fatalf("got %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("got %v, want %v", got, want)
			break
		}
	}
}

func TestUnitSnapshotSlicePreviews(t *testing.T) {
	// One level of nesting inside the preview's demo container: the
	// depth-balancing loop must walk past the demo's own <div>...</div>
	// before it finds the wrapper's closing tags.
	simple := `<div data-slot="preview" foo="bar"><div data-align="center" data-chromeless="false" class="preview foo"><div class="c">x</div></div></div>`
	got := snapshotSlicePreviews(simple)
	want := `<div class="c">x`
	if len(got) != 1 || got[0] != want {
		t.Fatalf("simple: got %#v, want [%q]", got, want)
	}

	// Two levels of nesting: the loop must keep balancing through BOTH the
	// inner and outer content divs before landing on the wrapper's own close.
	nested := `<div data-slot="preview"><div data-align="center" data-chromeless="false" class="preview"><div class="outer"><div class="inner">y</div></div></div></div>`
	got2 := snapshotSlicePreviews(nested)
	want2 := `<div class="outer"><div class="inner">y</div>`
	if len(got2) != 1 || got2[0] != want2 {
		t.Fatalf("nested: got %#v, want [%q]", got2, want2)
	}

	// Two previews back to back must each get their own slice.
	two := simple + nested
	got3 := snapshotSlicePreviews(two)
	if len(got3) != 2 || got3[0] != want || got3[1] != want2 {
		t.Fatalf("two previews: got %#v", got3)
	}

	// No preview wrapper at all: no slices, no panic.
	if got4 := snapshotSlicePreviews("<div>nothing here</div>"); len(got4) != 0 {
		t.Errorf("no wrapper: got %#v, want none", got4)
	}
}
