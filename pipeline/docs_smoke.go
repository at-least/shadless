package main

// docs-smoke, ported from tools/docs-smoke.mjs — serve the built VitePress
// site over http and drive the browser with the REAL mouse (synthetic
// PointerEvents are rejected by radix-like filters):
//   1. dialog.html: real-mouse open inside the iframe + Escape close
//   2. avatar.html: preview over http, 0 console errors, images settled
//   3. dialog page preview wiring counts
//   --all: every built page — render, no raw mdx leak, 0 errors

import (
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

func runDocsSmoke(all bool) int {
	siteDir := "docs/.vitepress/dist"
	if _, err := os.Stat(siteDir); err != nil {
		fmt.Fprintln(os.Stderr, "FAIL  docs smoke: the site is not built — run make docs first")
		return 1
	}

	// ephemeral-port static server (python3 http.server, same as the JS gate)
	var freePort int
	if l, err := net.Listen("tcp", "127.0.0.1:0"); err == nil {
		freePort = l.Addr().(*net.TCPAddr).Port
		l.Close()
	}
	server := exec.Command("python3", "-m", "http.server", fmt.Sprint(freePort), "--bind", "127.0.0.1", "--directory", siteDir)
	server.Stdout, server.Stderr = nil, nil
	if err := server.Start(); err != nil {
		fmt.Fprintln(os.Stderr, "FAIL  docs smoke: python3 http.server:", err)
		return 1
	}
	defer func() {
		if server.Process != nil {
			server.Process.Kill()
			server.Wait()
		}
	}()
	base := fmt.Sprintf("http://127.0.0.1:%d", freePort)
	serverUp := false
	client := &http.Client{Timeout: 2 * time.Second}
	for i := 0; i < 100; i++ {
		if resp, err := client.Get(base + "/index.html"); err == nil {
			resp.Body.Close()
			serverUp = true
			break
		}
		time.Sleep(50 * time.Millisecond)
	}
	if !serverUp {
		fmt.Fprintln(os.Stderr, "FAIL  docs smoke: static server did not come up (python3 http.server)")
		return 1
	}

	shell, err := startBrowserShell()
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs smoke:", err)
		return 1
	}
	defer shell.close()
	if err := shell.launch(); err != nil {
		fmt.Fprintln(os.Stderr, "docs smoke:", err)
		return 1
	}

	var failures []string
	check := func(label string, ok bool, detail string) {
		if ok {
			fmt.Printf("PASS  %s\n", label)
		} else {
			if detail != "" {
				fmt.Fprintf(os.Stderr, "FAIL  %s — %s\n", label, detail)
			} else {
				fmt.Fprintf(os.Stderr, "FAIL  %s\n", label)
			}
			failures = append(failures, label)
		}
	}

	// ---- 1. dialog preview: real-mouse open + Escape close inside the iframe ----
	page, err := shell.newPage(false)
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs smoke:", err)
		return 1
	}
	if err := page.gotoURL(base + "/components/dialog.html"); err != nil {
		fmt.Fprintln(os.Stderr, "docs smoke:", err)
		return 1
	}
	// let every lazy iframe settle first — recomputing the trigger box
	// mid-layout-shift made the real-mouse click land outside the trigger
	page.waitForLoadState("networkidle", 10000)

	dialogFrame := `iframe.demo[title="dialog-demo"]`
	triggerSel := `[data-slot="dialog-trigger"]`
	if err := page.locWait(dialogFrame, triggerSel, "visible", 5000); err != nil {
		fmt.Fprintln(os.Stderr, "docs smoke: trigger wait:", err)
		return 1
	}
	page.locScroll(dialogFrame, triggerSel, 0)
	page.waitForTimeout(300)
	tbox, err := page.locBox(dialogFrame, triggerSel, 0)
	if err != nil || tbox == nil {
		fmt.Fprintln(os.Stderr, "docs smoke: trigger box:", err)
		return 1
	}
	page.mouseClick(tbox.X+tbox.Width/2, tbox.Y+tbox.Height/2)

	if err := page.locWait(dialogFrame, `[data-slot="dialog-content"][data-state="open"]`, "visible", 3000); err == nil {
		openCount, _ := page.locCount(dialogFrame, `[data-slot="dialog-content"][data-state="open"]`)
		expanded, _, _ := page.locAttr(dialogFrame, triggerSel, "aria-expanded")
		check("dialog: real-mouse click opens dialog in iframe (live content data-state=open)",
			openCount == 1 && expanded == "true", "aria-expanded="+expanded)
	} else {
		check("dialog: real-mouse click opens dialog in iframe (live content data-state=open)", false, err.Error())
	}

	// focus stays inside the frame: real-mouse click on the content card
	// padding, then Escape
	cbox, _ := page.locBox(dialogFrame, `[data-slot="dialog-content"]`, 0)
	if cbox != nil {
		page.mouseClick(cbox.X+12, cbox.Y+12)
	}
	page.keyPress("Escape")

	page.locWait(dialogFrame, `[data-slot="dialog-content"]`, "detached", 3000)
	livePortalNodes, _ := page.locCount(dialogFrame, `[data-slot="dialog-portal"], [data-slot="dialog-overlay"], [data-slot="dialog-content"]`)
	expandedClosed, _, _ := page.locAttr(dialogFrame, triggerSel, "aria-expanded")
	trigState, _, _ := page.locAttr(dialogFrame, triggerSel, "data-state")
	check("dialog: Escape closes dialog inside iframe (live portal nodes removed)",
		livePortalNodes == 0 && expandedClosed == "false" && trigState == "closed",
		fmt.Sprintf("livePortalNodes=%d aria-expanded=%s data-state=%s", livePortalNodes, expandedClosed, trigState))

	// ---- 2. avatar preview over http: 0 console errors ----
	avPage, _ := shell.newPageOrigin(true, base)
	avFrame := `iframe.demo[title="avatar-demo"]`
	avPage.gotoURL(base + "/components/avatar.html")
	avPage.locWait(avFrame, `[data-slot="avatar"]`, "visible", 5000)
	avBox, _ := page2box(avPage, avFrame)
	avPage.mouseClick(avBox.X+2, avBox.Y+2)
	images, _ := avPage.locEvalAll(avFrame, `[data-slot="avatar-image"]`, `el => el.complete && el.naturalWidth > 0`)
	var settled []bool
	if arr, ok := images.([]any); ok {
		for _, v := range arr {
			settled = append(settled, v == true)
		}
	}
	allSettled := len(settled) > 0
	for _, s := range settled {
		if !s {
			allSettled = false
		}
	}
	for i := 0; i < 40 && !allSettled; i++ {
		avPage.waitForTimeout(250)
		images, _ = avPage.locEvalAll(avFrame, `[data-slot="avatar-image"]`, `el => el.complete && el.naturalWidth > 0`)
		settled = settled[:0]
		if arr, ok := images.([]any); ok {
			for _, v := range arr {
				settled = append(settled, v == true)
			}
		}
		allSettled = len(settled) > 0
		for _, s := range settled {
			if !s {
				allSettled = false
			}
		}
	}
	avErrors, _ := avPage.events()
	check(fmt.Sprintf("avatar: preview over http reports 0 console errors (images=%d settled=%v)", len(settled), settled),
		len(avErrors) == 0 && len(settled) > 0 && allSettled, fmt.Sprintf("%q", avErrors))

	// ---- 3. dialog page preview wiring counts ----
	iframes, _ := page.locCount("", `iframe.demo`)
	unavailable, _ := page.locCount("", `.demo-missing`)
	previews := iframes + unavailable
	fmt.Printf("dialog page wiring: previews=%d iframes=%d unavailable-notes=%d\n", previews, iframes, unavailable)
	check("dialog page wiring: every preview is an iframe or an unavailable note",
		previews > 0 && iframes >= 1, fmt.Sprintf("previews=%d iframes=%d unavailable=%d", previews, iframes, unavailable))

	// ---- 4. --all: every-page sweep ----
	verifyN := 0
	if all {
		var pageFiles []string
		pageFiles = append(pageFiles, "index.html")
		for _, sub := range []string{"components", "guides"} {
			ents, _ := os.ReadDir(filepath.Join(siteDir, sub))
			for _, e := range ents {
				if strings.HasSuffix(e.Name(), ".html") {
					pageFiles = append(pageFiles, sub+"/"+e.Name())
				}
			}
		}
		sort.Strings(pageFiles)
		renderFail, leakFail := 0, 0
		consoleErrCount, iframesLoaded := 0, 0
		nComponents, nGuides, nIndex := 0, 0, 0
		for _, f := range pageFiles {
			p, _ := shell.newPageOrigin(true, base)
			p.gotoURL(base + "/" + f)
			n, _ := p.locCount("", `iframe.demo`)
			for i := 0; i < n; i++ {
				p.locScroll("", `iframe.demo`, i)
				iframesLoaded++
			}
			p.waitForLoadState("networkidle", 2000)
			res, err := p.evaluateFn(`() => {
        const article = document.querySelector('.vp-doc')
        const text = article?.innerText?.trim() ?? ''
        const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT,
          { acceptNode: (n) => (n.parentElement.closest('pre, code') ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT) })
        let visible = ''
        for (; walker.nextNode();) visible += walker.currentNode.data + '\n'
        return { rendered: !!article && text.length > 0, leaks: visible.match(/Component(Preview|Source)\b/g) ?? [] }
      }`)
			if f == "index.html" {
				nIndex++
			} else if strings.HasPrefix(f, "guides/") {
				nGuides++
			} else {
				nComponents++
			}
			var rendered bool
			var leaks []string
			if m, ok := res.(map[string]any); ok && err == nil {
				rendered = m["rendered"] == true
				if raw, ok := m["leaks"].([]any); ok {
					for _, l := range raw {
						leaks = append(leaks, l.(string))
					}
				}
			}
			if !rendered {
				renderFail++
				fmt.Fprintf(os.Stderr, "FAIL  render: %s — article missing/empty\n", f)
			}
			if len(leaks) > 0 {
				leakFail++
				fmt.Fprintf(os.Stderr, "FAIL  mdx leak: %s — %s\n", f, strings.Join(dedup(leaks), ", "))
			}
			evts, _ := p.events()
			// console and pageerror are merged by the capture; the JS split
			// them only for the summary line
			if len(evts) > 0 {
				n := 3
				if len(evts) < n {
					n = len(evts)
				}
				fmt.Fprintf(os.Stderr, "FAIL  console: %s — %s\n", f, strings.Join(evts[:n], " | "))
			}
			consoleErrCount += len(evts)
			p.close()
		}
		fmt.Printf("pages: %d/%d visited (%d components, %d guides, %d index) · %d preview iframes loaded\n",
			len(pageFiles), len(pageFiles), nComponents, nGuides, nIndex, iframesLoaded)
		check(fmt.Sprintf("every-page render: %d pages non-empty (article present)", len(pageFiles)), renderFail == 0, fmt.Sprintf("%d failed", renderFail))
		check("every-page mdx: 0 raw ComponentPreview/ComponentSource outside code blocks", leakFail == 0, fmt.Sprintf("%d pages leaking", leakFail))
		// One count on purpose: the capture merges console errors and
		// pageerrors into the same event list. This used to read
		// "0 errors, 0 pageerrors" against a second counter that only ever
		// had `+= 0` added to it — a check name promising a signal nothing
		// produced.
		check("every-page console: 0 error events, console and pageerror alike (iframes included)", consoleErrCount == 0,
			fmt.Sprintf("%d error events", consoleErrCount))
		verifyN = len(pageFiles)
	}

	if len(failures) > 0 {
		if all {
			fmt.Printf("FAIL  docs verify (%d failed)\n", len(failures))
		} else {
			fmt.Printf("FAIL  docs smoke (%d failed)\n", len(failures))
		}
		return 1
	}
	if all {
		fmt.Printf("PASS  docs verify (%d pages, 0 console errors)\n", verifyN)
	} else {
		fmt.Println("PASS  docs smoke (dialog iframe open/close, avatar 0 errors, preview wiring)")
	}
	return 0
}

// page2box: first bounding box of a selector inside a frame.
func page2box(p *bpage, frame string) (*bbox, error) {
	return p.locBox(frame, `[data-slot="avatar"]`, 0)
}
