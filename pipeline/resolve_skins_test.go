package main

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

// TestUnitResolveSkinsParity is a determinism/idempotency check, not a
// parity check: tools/resolve-skins.mjs (the independent JS reference this
// test originally diffed against) was deleted in 3f14328, 25 seconds after
// the commit that added this test — so there has been no independent
// implementation to compare against for the test's entire history. What it
// still proves: rerunning `pipeline resolve-skins` against an unchanged
// source tree reproduces byte-identical output to whatever produced the
// existing build/resolved-ui (today, always a prior Go run). The pure
// transforms it exercises only as a subprocess black box (expandClassString,
// splitClassName, applyRtlMapping, resolveSource) get direct unit coverage
// in TestUnitApplyRtlMapping / TestUnitSplitClassName /
// TestUnitExpandClassString instead — this test can't see that coverage
// (subprocess, not `go test` instrumentation) and shouldn't be read as
// providing it.
func TestUnitResolveSkinsParity(t *testing.T) {
	root := ".."
	tmp := t.TempDir()
	// build/resolved-ui is .gitignore'd and produced by whichever run of
	// `pipeline resolve-skins` happened last. Save, rerun, diff byte-for-byte,
	// restore.
	existing := filepath.Join(root, "build", "resolved-ui")
	if _, err := os.Stat(filepath.Join(existing, "ui")); err != nil {
		t.Skip("no existing resolved-ui tree to compare against")
	}
	backup := filepath.Join(tmp, "resolved-ui")
	if err := exec.Command("cp", "-a", existing, backup).Run(); err != nil {
		t.Fatalf("backup: %v", err)
	}
	t.Cleanup(func() {
		os.RemoveAll(existing)
		if out, err := exec.Command("cp", "-a", backup, existing).CombinedOutput(); err != nil {
			t.Errorf("restore: %v\n%s", err, out)
		}
	})

	pipelineBin := filepath.Join(tmp, "pipeline")
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build pipeline: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "resolve-skins")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("resolve-skins: %v\n%s", err, out)
	}
	if out, err := exec.Command("diff", "-r", backup, existing).CombinedOutput(); err != nil {
		t.Fatalf("resolved-ui changed on a rerun with no source changes (non-determinism):\n%s", out)
	}
}

// applyRtlMapping: cases lifted from upstream's own
// transform-rtl.test.ts (packages/shadcn/src/utils/transformers), the
// independent oracle for this port — one per mapping category, plus the
// precedence/exclusion rules between them (physical-side exclusion,
// already-rtl:/ltr:-prefixed passthrough).
func TestUnitApplyRtlMapping(t *testing.T) {
	cases := []struct{ name, in, want string }{
		{"margin", "ml-2 mr-4 -ml-2 -mr-4", "ms-2 me-4 -ms-2 -me-4"},
		{"padding", "pl-2 pr-4", "ps-2 pe-4"},
		{"positioning", "left-0 right-0 -left-2 -right-2", "start-0 end-0 -start-2 -end-2"},
		{"inset", "inset-l-0 inset-r-0", "inset-inline-start-0 inset-inline-end-0"},
		{"border", "border-l border-r border-l-2 border-r-2", "border-s border-e border-s-2 border-e-2"},
		{"rounded corners", "rounded-l-md rounded-tl-md rounded-br-md", "rounded-s-md rounded-ss-md rounded-ee-md"},
		{"text align", "text-left text-right", "text-start text-end"},
		{"scroll margin/padding", "scroll-ml-2 scroll-pr-2", "scroll-ms-2 scroll-pe-2"},
		{"float", "float-left float-right", "float-start float-end"},
		{"clear", "clear-left clear-right", "clear-start clear-end"},
		{"origin", "origin-left origin-top-right", "origin-start origin-top-end"},
		{"variant prefix preserved", "hover:ml-2 sm:md:ml-2", "hover:ms-2 sm:md:ms-2"},
		{"named group selector with data attr", "sm:group-data-[size=default]/alert-dialog-content:text-left",
			"sm:group-data-[size=default]/alert-dialog-content:text-start"},
		{"arbitrary values", "ml-[10px] left-[50%]", "ms-[10px] start-[50%]"},
		{"alpha modifier reattached", "ml-2/50", "ms-2/50"},
		{"unrelated classes untouched", "bg-red-500 flex mx-auto px-4", "bg-red-500 flex mx-auto px-4"},
		{"partial-match guard: suffix after the mapped prefix blocks it",
			"border-ring border-ring/50 border-lime-500 scroll-m-4",
			"border-ring border-ring/50 border-lime-500 scroll-m-4"},
		{"translate-x gains a mirrored rtl: variant", "-translate-x-1/2", "-translate-x-1/2 rtl:translate-x-1/2"},
		{"translate-x positive form", "translate-x-full", "translate-x-full rtl:-translate-x-full"},
		{"translate-x with a variant prefix", "after:-translate-x-1/2", "after:-translate-x-1/2 rtl:after:translate-x-1/2"},
		{"translate-y is not translate-x", "-translate-y-1/2 translate-y-full", "-translate-y-1/2 translate-y-full"},
		{"space-x/divide-x gain rtl:-reverse", "space-x-4 divide-x-2", "space-x-4 rtl:space-x-reverse divide-x-2 rtl:divide-x-reverse"},
		{"space-x with a variant prefix", "md:space-x-4", "md:space-x-4 rtl:md:space-x-reverse"},
		{"space-y/divide-y untouched", "space-y-4 divide-y-2", "space-y-4 divide-y-2"},
		{"cursor resize swaps direction", "cursor-w-resize cursor-e-resize", "cursor-w-resize rtl:cursor-e-resize cursor-e-resize rtl:cursor-w-resize"},
		{"cursor resize with a variant prefix", "hover:cursor-w-resize", "hover:cursor-w-resize rtl:hover:cursor-e-resize"},
		{"cn-rtl-flip marker, alone", "cn-rtl-flip", "rtl:rotate-180"},
		{"cn-rtl-flip marker, leading", "cn-rtl-flip size-4", "rtl:rotate-180 size-4"},
		{"cn-rtl-flip marker, trailing", "size-4 cn-rtl-flip", "size-4 rtl:rotate-180"},
		{"cn-rtl-flip combined with a real mapping", "cn-rtl-flip ml-2", "rtl:rotate-180 ms-2"},
		{"logical slide inside a logical side variant", "data-[side=inline-start]:slide-in-from-right-2",
			"data-[side=inline-start]:slide-in-from-end-2"},
		{"logical slide, the other side/direction", "data-[side=inline-end]:slide-out-to-left-2",
			"data-[side=inline-end]:slide-out-to-start-2"},
		{"slide inside a PHYSICAL side variant is untouched", "data-[side=left]:slide-in-from-right-2",
			"data-[side=left]:slide-in-from-right-2"},
		{"positioning inside a physical side variant is excluded", "data-[side=left]:-right-1 data-[side=right]:-left-1 data-[side=left]:right-0",
			"data-[side=left]:-right-1 data-[side=right]:-left-1 data-[side=left]:right-0"},
		{"non-positioning classes still map inside a physical side variant",
			"data-[side=left]:ml-2 data-[side=right]:pl-4 data-[side=left]:text-left",
			"data-[side=left]:ms-2 data-[side=right]:ps-4 data-[side=left]:text-start"},
		{"already rtl:-prefixed classes pass through untouched", "rtl:ml-2 rtl:text-right rtl:space-x-reverse",
			"rtl:ml-2 rtl:text-right rtl:space-x-reverse"},
		{"already ltr:-prefixed classes pass through untouched", "ltr:ml-2 ltr:text-left", "ltr:ml-2 ltr:text-left"},
		{"rtl:/ltr: classes skipped, others in the same string still map",
			"ml-2 rtl:mr-2", "ms-2 rtl:mr-2"},
		{"hand-written ltr:/rtl: translate pair left alone (both already prefixed)",
			"ltr:-translate-x-1/2 rtl:-translate-x-1/2", "ltr:-translate-x-1/2 rtl:-translate-x-1/2"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := applyRtlMapping(c.in); got != c.want {
				t.Errorf("applyRtlMapping(%q) = %q, want %q", c.in, got, c.want)
			}
		})
	}
}

// splitClassName: bracket-aware last-colon-outside-brackets for the
// variant, then first-slash-outside-brackets for the alpha modifier.
func TestUnitSplitClassName(t *testing.T) {
	str := func(s string) *string { return &s }
	cases := []struct {
		name                              string
		in                                string
		wantVariant, wantValue, wantAlpha *string
	}{
		{"bare utility, no variant, no alpha", "ml-2", nil, str("ml-2"), nil},
		{"one variant", "hover:ml-2", str("hover"), str("ml-2"), nil},
		{"stacked variants split at the LAST colon", "sm:md:ml-2", str("sm:md"), str("ml-2"), nil},
		{"alpha modifier", "bg-red-500/50", nil, str("bg-red-500"), str("50")},
		{"variant + alpha", "hover:bg-red-500/50", str("hover"), str("bg-red-500"), str("50")},
		{"bracketed variant with an internal ':' does not split there",
			"data-[state=open]:bg-red-500/50", str("data-[state=open]"), str("bg-red-500"), str("50")},
		{"a '/' inside brackets is not the alpha divider",
			"data-[state=open]:w-1/2", str("data-[state=open]"), str("w-1"), str("2")},
	}
	eq := func(got, want *string) bool {
		if (got == nil) != (want == nil) {
			return false
		}
		return got == nil || *got == *want
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			gotVariant, gotValue, gotAlpha := splitClassName(c.in)
			if !eq(gotVariant, c.wantVariant) {
				t.Errorf("variant = %v, want %v", derefOrNil(gotVariant), derefOrNil(c.wantVariant))
			}
			if !eq(gotValue, c.wantValue) {
				t.Errorf("value = %v, want %v", derefOrNil(gotValue), derefOrNil(c.wantValue))
			}
			if !eq(gotAlpha, c.wantAlpha) {
				t.Errorf("alpha = %v, want %v", derefOrNil(gotAlpha), derefOrNil(c.wantAlpha))
			}
		})
	}
}

func derefOrNil(s *string) string {
	if s == nil {
		return "<nil>"
	}
	return *s
}

// expandClassString: allowlisted cn-* names stay literal, mapped cn-* names
// expand to the skin's @apply body, a cn-* marker styled by no skin drops
// out entirely, and a string with no cn-* token at all is returned
// verbatim (the early-return path keeps whitespace intact, unlike the
// twmerge-normalized path every cn-*-bearing string takes).
func TestUnitExpandClassString(t *testing.T) {
	origMap, origAllow := skinData.Map, skinData.Allowlist
	t.Cleanup(func() { skinData.Map, skinData.Allowlist = origMap, origAllow })
	skinData.Map = map[string]string{"cn-btn": "bg-blue-500 text-white"}
	skinData.Allowlist = map[string]bool{"cn-keep": true}

	cases := []struct{ name, in, want string }{
		{"no cn- token: returned verbatim, whitespace untouched", "flex  items-center", "flex  items-center"},
		{"allowlisted cn- name stays literal", "flex cn-keep", "flex cn-keep"},
		{"mapped cn- name expands to its @apply body", "cn-btn", "bg-blue-500 text-white"},
		{"marker cn- name (styled by no skin) drops out", "flex cn-ghost", "flex"},
		{"allowlist + map + marker together", "cn-keep cn-btn cn-ghost flex", "cn-keep bg-blue-500 text-white flex"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := expandClassString(c.in); got != c.want {
				t.Errorf("expandClassString(%q) = %q, want %q", c.in, got, c.want)
			}
		})
	}
}
