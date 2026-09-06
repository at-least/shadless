package main

import (
	"strings"
	"testing"
)

// buildOracleEntryCSS's line-classification loop is the actual content
// transform in runOracleCSS (inline shadcn's tailwind.css, splice in
// legacy-themes.css only if present, drop the app's own @source lines and
// replace them with the resolved ones) — this exercises it directly on
// small fixture strings instead of through the full upstream tree + tailwind
// compile.
func TestUnitBuildOracleEntryCSS(t *testing.T) {
	app := strings.Join([]string{
		`@import "tailwindcss";`,
		`@import "shadcn/tailwind.css";`,
		`@import "./legacy-themes.css";`,
		`@source "./app/own-dir";`,
		`.custom { color: red; }`,
	}, "\n")
	shadcnTw := `.tw-base { all: unset; }`
	skin := `.style-nova .cn-button { color: blue; }`
	sourceDirs := []string{"/abs/resolved-ui", "/abs/tools/contracts/components"}

	t.Run("legacy present", func(t *testing.T) {
		got := buildOracleEntryCSS(app, shadcnTw, skin, "/abs/legacy-themes.css", true, sourceDirs)
		want := strings.Join([]string{
			`@import "tailwindcss";`,
			`/* shadcn/tailwind.css (inlined from packages/shadcn/src) */`,
			shadcnTw,
			`@import "/abs/legacy-themes.css";`,
			`.custom { color: red; }`,
			`@source "/abs/resolved-ui";`,
			`@source "/abs/tools/contracts/components";`,
			`/* === style-nova.css (the pinned skin, verbatim) === */`,
			skin,
		}, "\n")
		if got != want {
			t.Fatalf("got:\n%s\nwant:\n%s", got, want)
		}
	})

	t.Run("legacy absent", func(t *testing.T) {
		got := buildOracleEntryCSS(app, shadcnTw, skin, "/abs/legacy-themes.css", false, sourceDirs)
		if strings.Contains(got, "legacy-themes") {
			t.Fatalf("legacy import present despite hasLegacy=false: %s", got)
		}
		want := strings.Join([]string{
			`@import "tailwindcss";`,
			`/* shadcn/tailwind.css (inlined from packages/shadcn/src) */`,
			shadcnTw,
			`.custom { color: red; }`,
			`@source "/abs/resolved-ui";`,
			`@source "/abs/tools/contracts/components";`,
			`/* === style-nova.css (the pinned skin, verbatim) === */`,
			skin,
		}, "\n")
		if got != want {
			t.Fatalf("got:\n%s\nwant:\n%s", got, want)
		}
	})

	t.Run("app @source lines dropped", func(t *testing.T) {
		got := buildOracleEntryCSS(app, shadcnTw, skin, "/abs/legacy-themes.css", true, sourceDirs)
		if strings.Contains(got, "own-dir") {
			t.Fatalf("app's own @source line survived: %s", got)
		}
	})

	t.Run("passthrough line kept verbatim", func(t *testing.T) {
		got := buildOracleEntryCSS(app, shadcnTw, skin, "/abs/legacy-themes.css", true, sourceDirs)
		if !strings.Contains(got, `.custom { color: red; }`) {
			t.Fatalf("passthrough line dropped: %s", got)
		}
	})
}
