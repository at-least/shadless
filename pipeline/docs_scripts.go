package main

// extractDemoScripts, ported from src/docs/demo-scripts.mjs — shared demo-page
// script extraction. Two consumers used to run their own regexes and both
// accidentally captured the FT9 theme pre-paint boilerplate as "demo JS".

import (
	"os"
	"regexp"
	"strings"
)

const prepaintSig = "shadless-docs-theme"

var (
	reDemoSrcScript = regexp.MustCompile(`<script src="\.\./(js/[\w.-]+\.js|shadless\.js)"></script>`)
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
		out.inlineScripts = append(out.inlineScripts, body)
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
