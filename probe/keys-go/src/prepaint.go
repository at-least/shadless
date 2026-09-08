package main

// theme-prepaint: the two constants the port lifts out of
// src/docs/theme-prepaint.mjs. THEME_PREPAINT_SCRIPT is the pre-paint IIFE
// (FOUC + live storage sync); it used to ship embedded in every demo page.
// SHADLESS_CSS_FIXES is the gap-closure layer appended to globals.css —
// currently empty (the ghost/outline gap moved into the emitter's own twin
// blocks; see theme-prepaint.mjs's header), kept because product_css.go
// and the JS emitter both interpolate it by regex.
//
// This file is the Go-side single source of truth. A lint in the emitter's
// test asserts the JS file carries the same constants verbatim.
import (
	"regexp"
	"strings"
)

const (
	ThemePrePaintScript = `<script>(function(){try{var k="shadless-docs-theme";var apply=function(d){document.documentElement.classList.toggle("dark",!!d)};var s=localStorage.getItem(k);var d=s?s==="dark":matchMedia("(prefers-color-scheme: dark)").matches;apply(d);addEventListener("storage",function(e){if(e.key===k)apply(e.newValue==="dark")});}catch(e){}})();</script>`
	ThemePrePaintSig    = `<script>(function(){try{var k="shadless-docs-theme"`
	ShadlessCSSFixes    = ``
)

// injectPrePaint mirrors src/docs/theme-prepaint.mjs exactly: prepend to
// </head> when present, otherwise into the opening <head …>, otherwise at
// the very front. Idempotent on the SIG prefix.
var headRe = regexp.MustCompile(`(?i)<head[^>]*>`)

func injectPrePaint(html string) string {
	if strings.Contains(html, ThemePrePaintSig) {
		return html
	}
	if strings.Contains(html, "</head>") {
		return strings.Replace(html, "</head>", ThemePrePaintScript+"</head>", 1)
	}
	if headRe.MatchString(html) {
		return headRe.ReplaceAllStringFunc(html, func(m string) string {
			return m + ThemePrePaintScript
		})
	}
	return ThemePrePaintScript + html
}
