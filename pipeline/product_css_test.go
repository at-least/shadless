package main

// Pure-function tests for the tokens extractor and the product verify checks,
// ported from tools/unit/product-css.mjs. The extractor's job is a NARROW
// keep-list out of a globals.css that mixes library config with oracle-site
// chrome; these pin both directions (keep + exclude) on a synthetic replica of
// that shape.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// Fixture tokens are CONCATENATED at runtime on purpose: tracked files are
// scanned by the tailwind CLI's repo-wide auto-scan (see pipeline/tw.go) and
// literals here would surface as phantom utility rules in dist/out.css.
var syntheticGlobals = `@import "tailwindcss";
@import "tw-animate-css";
/* === begin inlined shadcn/tailwind.css === */
@custom-variant data-open {
  &:where([data-state="open"]) {
    @slot;
  }
}
@utility shimmer {
  color: red;
}
/* === end inlined shadcn/tailwind.css === */
@source "./demo.html";

@custom-variant style-vega (&:where(.style-vega *));
@custom-variant dark (&:is(.dark *));

@theme inline {
  --color-background: var(--background);
}

@theme inline {
  --font-ar: "Noto Naskh Arabic";
}

:root {
  --background: oklch(1 0 0);
}

.dark {
  --background: oklch(0.145 0 0);
}

@layer base {
  * {
    @apply ` + u("border", "-border") + ` ` + u("outline", "-ring") + `/50;
  }
  a:active,
  button:active {
    @apply opacity-60;
  }
}

@layer components {
  .dialog-ring {
    @apply rounded-xl;
  }
}
`

func TestUnitProductCssExtractTokens(t *testing.T) {
	tokens, err := extractTokens(syntheticGlobals)
	if err != nil {
		t.Fatal(err)
	}
	keeps := []struct {
		label string
		ok    bool
	}{
		{"keeps @import lines", len(reImportLine.FindAllString(tokens, -1)) == 2},
		{"keeps inlined library block", strings.Contains(tokens, "data-open") && strings.Contains(tokens, "@utility shimmer")},
		{"keeps dark variant", strings.Contains(tokens, "@custom-variant dark")},
		{"keeps token @theme (color-background)", strings.Contains(tokens, "--color-background:")},
		{"keeps :root/.dark vars", strings.Contains(tokens, "--background: oklch(1 0 0)") && strings.Contains(tokens, ".dark")},
		{"keeps base border reset", strings.Contains(tokens, u("border", "-border"))},
	}
	for _, k := range keeps {
		if !k.ok {
			t.Errorf("tokens: %s — failed", k.label)
		}
	}
	// excludes (docs-site chrome)
	excludes := []struct{ label, needle string }{
		{"excludes style packs", "style-vega"},
		{"excludes site-only @theme", "--font-ar"},
		{"excludes a:active dimming", "opacity-60"},
		{"excludes dialog-ring chrome", "dialog-ring"},
		{"excludes @source", "@source"},
	}
	for _, e := range excludes {
		if strings.Contains(tokens, e.needle) {
			t.Errorf("tokens: %s — %q leaked in", e.label, e.needle)
		}
	}
}

func TestUnitProductCssEntryOrder(t *testing.T) {
	entry := buildProductEntry("TOKENS", "FIXES", "PARTS")
	if !(strings.Index(entry, "TOKENS") < strings.Index(entry, "FIXES") &&
		strings.Index(entry, "FIXES") < strings.Index(entry, "PARTS")) {
		t.Errorf("entry assembly is not tokens, fixes, parts: %q", entry)
	}
}

func TestUnitProductCssVerify(t *testing.T) {
	parts := `[data-slot="alert"] { @apply grid; } [data-slot="alert-title"] { @apply font-medium; }`
	out := `:root { --background: 1; --radius: 1 } [data-slot="alert"] { display: grid } [data-slot="alert-title"] { font-weight: 500 }`

	if r := verifyProduct(out, out, parts, ""); len(r.Missing)+len(r.DemoDropped)+len(r.Chrome)+len(r.Tokens) != 0 {
		t.Errorf("healthy build reported problems: %+v", r)
	}
	broken := strings.Replace(out, `[data-slot="alert-title"] { font-weight: 500 }`, "", 1)
	if r := verifyProduct(broken, out, parts, ""); len(r.Missing) != 1 || r.Missing[0] != "alert-title" {
		t.Errorf("dropped slot rule not caught: %v", r.Missing)
	}
	leaky := out + ` [data-rehype-pretty-code-figure] { color: red } [data-slot="docs"] { margin: 0 }`
	if r := verifyProduct(leaky, out, parts, ""); len(r.Chrome) < 2 {
		t.Errorf("chrome leak not caught: %v", r.Chrome)
	}
	noTokens := strings.ReplaceAll(out, "--", "XX")
	if r := verifyProduct(noTokens, out, parts, ""); len(r.Tokens) != 2 {
		t.Errorf("missing token vars not caught: %v", r.Tokens)
	}

	// stray-class check: a standalone class with no origin in the product
	// source is a content-scan leak
	phantom := u("m", "s-auto")
	strayCSS := out + "\n  ." + phantom + " {\n    margin-inline-start: auto;\n  }\n"
	if r := verifyProduct(strayCSS, out, parts, "TOKENS shadless.css PARTS"); len(r.Stray) != 1 || r.Stray[0] != phantom {
		t.Errorf("stray class not caught: %v", r.Stray)
	}
	if r := verifyProduct(out, out, parts, "grid font-medium"); len(r.Stray) != 0 {
		t.Errorf("sourced classes reported stray: %v", r.Stray)
	}
}

func TestUnitProductCssRealGlobals(t *testing.T) {
	// the real globals must stay extractable (marker/format drift fails loud)
	b, err := os.ReadFile(filepath.Join(repoRoot(t), "probes/h4/globals.css"))
	if err != nil {
		t.Fatal(err)
	}
	tokens, err := extractTokens(string(b))
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(tokens, "--color-background:") || !strings.Contains(tokens, "scroll-fade") {
		t.Error("real globals did not extract the expected keeps")
	}
	if strings.Contains(tokens, "rehype") || strings.Contains(tokens, "style-vega") {
		t.Error("real globals extraction leaked docs chrome")
	}
}
