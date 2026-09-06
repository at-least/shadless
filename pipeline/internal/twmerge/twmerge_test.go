package twmerge

import "testing"

// Hand-checked against `twMerge` (tailwind-merge v3.6.0) outputs.
func TestUnitMergeBasics(t *testing.T) {
	for in, want := range map[string]string{
		// exact duplicates collapse
		"absolute inset-0 z-10 outline-none absolute inset-0 z-10 outline-none": "absolute inset-0 z-10 outline-none",
		// same group, later wins
		"text-sm text-lg":                     "text-lg",
		"w-full w-1/2":                        "w-1/2",
		"max-h-(--available-height) max-h-72": "max-h-72",
		// different groups coexist
		"p-4 px-2":        "p-4 px-2",
		"flex flex-col":   "flex flex-col",
		"ml-auto ms-auto": "ml-auto ms-auto",
		// variant-scoped groups do not collide across variants
		"bg-red-500 hover:bg-red-600":                                        "bg-red-500 hover:bg-red-600",
		"data-horizontal:w-full data-vertical:h-full data-horizontal:w-full": "data-vertical:h-full data-horizontal:w-full",
		// font-size conflicts with leading ONLY when the size carries a
		// /alpha-ish postfix (conflictingClassGroupModifiers) — the bundle's
		// rule, not intuition
		"text-lg leading-snug":               "text-lg leading-snug",
		"text-sm/7 text-lg":                  "text-lg",
		"p-1 text-sm leading-none text-md/5": "p-1 text-md/5",
		// non-tailwind tokens pass through untouched, position kept
		"foo text-lg baz text-sm": "foo baz text-sm",
		// arbitrary values match their group's validator
		"h-[100px] h-20":  "h-20",
		"inset-0 inset-2": "inset-2",
		// arbitrary property classes are a per-property group
		"[margin:0] [margin:4px]":       "[margin:4px]",
		"[mask:luminance] [mask:alpha]": "[mask:alpha]",
		// important marker keeps conflict scope separate
		"!p-4 p-2 !p-1": "p-2 !p-1",
		// order-sensitive modifier weights: "before" (unlike "hover") keeps
		// its relative position instead of sorting alphabetically, so
		// "hover:before" and "before:hover" are distinct classIDs and both
		// survive; if modifierWeight were never populated both would sort to
		// "before:hover" and collide, leaving only the later one
		"hover:before:p-2 before:hover:p-4": "hover:before:p-2 before:hover:p-4",
	} {
		if got := Merge(in); got != want {
			t.Errorf("Merge(%q) = %q, want %q", in, got, want)
		}
	}
}

func TestUnitMergeOrderPreserved(t *testing.T) {
	// survivors keep INPUT order; note p-2→px-4→p-6: px-4 dies to p-6's
	// conflicting px, and flex dies to grid (display group)
	in := "flex a-token p-2 px-4 grid p-6 b-token"
	want := "a-token grid p-6 b-token"
	if got := Merge(in); got != want {
		t.Errorf("Merge(%q) = %q, want %q", in, got, want)
	}
}

func TestUnitParseAndSort(t *testing.T) {
	// variant modifier sorting: predefined sort alphabetically, order-
	// sensitive ones keep position
	p := parseClassName("focus:hover:bg-red-500/50")
	if p.base != "bg-red-500/50" || len(p.modifiers) != 2 || p.postfixPosition == -1 {
		t.Fatalf("parse: %+v", p)
	}
	if got := Merge("data-[active]:focus:p-2 focus:p-1 data-[active]:focus:p-4"); got != "focus:p-1 data-[active]:focus:p-4" {
		t.Errorf("variant-scoped merge: got %q", got)
	}
}
