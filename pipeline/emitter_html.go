package main

import "strings"

// escHtml mirrors src/emitter/index.mjs's escHtml — the order load-bears
// (& first, so &lt; never becomes &amp;lt;).
func escHtml(s string) string {
	repl := strings.NewReplacer(
		"&", "&amp;",
		"<", "&lt;",
		">", "&gt;",
		`"`, "&quot;",
	)
	return repl.Replace(s)
}
