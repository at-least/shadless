package main

// Ported from tools/unit/prepaint.mjs (deleted: the JS injectPrePaint it
// tested has no production caller left — see src/docs/theme-prepaint.mjs's
// history). This is the implementation that actually ships today, called
// from runDemo/runBuildRtl/runExampleOracle; it had zero test coverage in
// either language before this file.
import (
	"strings"
	"testing"
)

func TestUnitInjectPrePaint(t *testing.T) {
	once := injectPrePaint("<html><head><title>t</title></head><body></body></html>")
	if !strings.Contains(once, ThemePrePaintSig) {
		t.Fatalf("prePaint: not injected into head: %.120s", once)
	}
	if !strings.HasSuffix(once, "</head><body></body></html>") {
		t.Fatalf("prePaint: not prepended to </head>: %s", once)
	}
	twice := injectPrePaint(once)
	if twice != once {
		t.Fatalf("prePaint: not idempotent:\n  once:  %s\n  twice: %s", once, twice)
	}
}

// Every real page the pipeline feeds injectPrePaint has a well-formed
// <head>...</head>, so the two remaining branches — an opening <head …> tag
// with no matching </head> anywhere in the doc, and no head tag at all — are
// otherwise never exercised.
func TestUnitInjectPrePaintNoCloseHead(t *testing.T) {
	in := `<html><head foo="bar">no closing head tag<body></body></html>`
	got := injectPrePaint(in)
	if !strings.Contains(got, ThemePrePaintSig) {
		t.Fatalf("prePaint: not injected: %.160s", got)
	}
	want := `<html><head foo="bar">` + ThemePrePaintScript + `no closing head tag<body></body></html>`
	if got != want {
		t.Fatalf("prePaint: not spliced right after the opening <head> tag:\n got:  %s\nwant: %s", got, want)
	}
	if twice := injectPrePaint(got); twice != got {
		t.Fatalf("prePaint: not idempotent:\n  once:  %s\n  twice: %s", got, twice)
	}
}

func TestUnitInjectPrePaintNoHeadAtAll(t *testing.T) {
	in := `<div id="root">fragment, no head tag</div>`
	got := injectPrePaint(in)
	want := ThemePrePaintScript + in
	if got != want {
		t.Fatalf("prePaint: not prepended to the front:\n got:  %s\nwant: %s", got, want)
	}
	if twice := injectPrePaint(got); twice != got {
		t.Fatalf("prePaint: not idempotent:\n  once:  %s\n  twice: %s", got, twice)
	}
}
