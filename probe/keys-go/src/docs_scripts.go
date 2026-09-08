package main

// extractDemoScripts, ported from src/docs/demo-scripts.mjs — shared demo-page
// script extraction. Two consumers used to run their own regexes and both
// accidentally captured the FT9 theme pre-paint boilerplate as "demo JS".

import (
	"os"
	"regexp"
	"strings"
)

// prepaintSig anchors on the pre-paint snippet's own first statement, not on
// the storage key alone. The key by itself also appears in demo JS that
// legitimately writes it — the dark-mode guide's mode-toggle is exactly that,
// and its whole behavior tab was being filtered away, leaving a code tab whose
// only content was a "see Installation" comment. 470 demo pages carry the
// snippet; all 470 match this, and only mode-toggle stops matching.
const prepaintSig = `var k="shadless-docs-theme"`

var (
	// `defer` is optional: 90 of the demo pages carry it and 6 components
	// (accordion, checkbox, radio-group, switch, toggle-group, toggle) shipped
	// JS whose behavior tab silently vanished because this pattern demanded a
	// bare `<script src=`.
	reDemoSrcScript = regexp.MustCompile(`<script (?:defer )?src="\.\./(js/[\w.-]+\.js|shadless\.js)"></script>`)
	reInlineScript  = regexp.MustCompile(`(?s)<script>(.*?)</script>`)
)

type demoScripts struct {
	srcScripts    []string // local script srcs (js/<name>.js + shadless.js)
	inlineScripts []string
}

func extractDemoScripts(html string) demoScripts {
	var out demoScripts
	for _, m := range reDemoSrcScript.FindAllStringSubmatch(html, -1) {
		out.srcScripts = append(out.srcScripts, m[1]) // JS pushed every match, no dedupe
	}
	for _, m := range reInlineScript.FindAllStringSubmatch(html, -1) {
		body := strings.TrimSpace(m[1])
		if body == "" {
			continue
		}
		if strings.Contains(body, prepaintSig) {
			continue // theme persistence, not demo JS
		}
		out.inlineScripts = append(out.inlineScripts, dedentScript(m[1]))
	}
	return out
}

// readDemoScripts is the file-backed helper docs-build uses.
func readDemoScripts(path string) demoScripts {
	b, err := os.ReadFile(path)
	if err != nil {
		return demoScripts{}
	}
	return extractDemoScripts(string(b))
}

// dedentScript removes the indentation the demo page's HTML gives an inline
// <script>. TrimSpace alone only unindents the first line, which is what the
// docs showed: line 1 flush left, every following line four spaces in.
func dedentScript(body string) string {
	lines := strings.Split(strings.Trim(body, "\n"), "\n")
	indent := -1
	for _, l := range lines {
		if strings.TrimSpace(l) == "" {
			continue
		}
		n := len(l) - len(strings.TrimLeft(l, " \t"))
		if indent < 0 || n < indent {
			indent = n
		}
	}
	if indent > 0 {
		for i, l := range lines {
			if len(l) >= indent {
				lines[i] = l[indent:]
			} else {
				lines[i] = strings.TrimLeft(l, " \t")
			}
		}
	}
	return strings.TrimRight(strings.Join(lines, "\n"), " \t\n")
}
