package main

// browser-shell driver — the Go half of Wave 3's chromium thin shell
// (tools/browser-shell.mjs). One long-lived node process; JSON line
// requests in, one JSON line response out. Go owns every judgement.
//
// Errors from the shell surface as Go errors; a shell crash fails the node.

import (
	"bufio"
	"path/filepath"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"strings"
)

type browserShell struct {
	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stdout *bufio.Reader
	reqs   int
}

func startBrowserShell() (*browserShell, error) {
	cmd := exec.Command("node", "tools/browser-shell.mjs")
	stdin, err := cmd.StdinPipe()
	if err != nil {
		return nil, err
	}
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, err
	}
	cmd.Stderr = os.Stderr // playwright diagnostics flow to the node log
	if err := cmd.Start(); err != nil {
		return nil, err
	}
	return &browserShell{cmd: cmd, stdin: stdin, stdout: bufio.NewReader(stdout)}, nil
}

func (s *browserShell) call(req map[string]any) (map[string]any, error) {
	s.reqs++
	b, err := json.Marshal(req)
	if err != nil {
		return nil, err
	}
	if _, err := s.stdin.Write(append(b, '\n')); err != nil {
		return nil, fmt.Errorf("browser-shell: write: %w", err)
	}
	line, err := s.stdout.ReadString('\n')
	if err != nil {
		return nil, fmt.Errorf("browser-shell: read (after %d reqs): %w", s.reqs, err)
	}
	var res map[string]any
	if err := json.Unmarshal([]byte(strings.TrimSpace(line)), &res); err != nil {
		return nil, fmt.Errorf("browser-shell: bad response %q: %w", line, err)
	}
	if e, ok := res["error"].(string); ok {
		return nil, fmt.Errorf("browser-shell: %s", e)
	}
	return res, nil
}

func (s *browserShell) callErr(req map[string]any) error {
	_, err := s.call(req)
	return err
}

func (s *browserShell) close() {
	s.call(map[string]any{"op": "close"})
	s.stdin.Close()
	s.cmd.Wait()
}

// page is a typed handle over one shell page.
type bpage struct {
	s *browserShell
	id int
}

func (s *browserShell) launch() error {
	_, err := s.call(map[string]any{"op": "launch"})
	return err
}

func (s *browserShell) newPage(capture bool) (*bpage, error) {
	return s.newPageOrigin(capture, "")
}

func (s *browserShell) newPageOrigin(capture bool, origin string) (*bpage, error) {
	res, err := s.call(map[string]any{"op": "newPage", "capture": capture, "origin": origin})
	if err != nil {
		return nil, err
	}
	id, _ := res["pageId"].(float64)
	return &bpage{s, int(id)}, nil
}

func (p *bpage) gotoURL(url string) error {
	_, err := p.s.call(map[string]any{"op": "goto", "pageId": p.id, "url": url})
	return err
}

// evaluate runs expr; for expressions taking an argument, the shell passes
// it as playwright's `arg` (the expression sees it as its single param).
func (p *bpage) evaluate(expr string) (any, error) {
	res, err := p.s.call(map[string]any{"op": "evaluate", "pageId": p.id, "expr": expr})
	if err != nil {
		return nil, err
	}
	return res["value"], nil
}

func (p *bpage) evaluateArg(expr string, arg any) (any, error) {
	res, err := p.s.call(map[string]any{"op": "evaluate", "pageId": p.id, "expr": expr, "arg": arg})
	if err != nil {
		return nil, err
	}
	return res["value"], nil
}

// evaluateFn runs a function-shaped expression ("() => {…}") in the page;
// arg becomes the function's single parameter.
func (p *bpage) evaluateFn(expr string) (any, error) {
	return p.evaluateFnArg(expr, nil)
}

func (p *bpage) evaluateFnArg(expr string, arg any) (any, error) {
	res, err := p.s.call(map[string]any{"op": "evaluateFn", "pageId": p.id, "expr": expr, "arg": arg})
	if err != nil {
		return nil, err
	}
	return res["value"], nil
}

func (p *bpage) setContent(html string) error {
	_, err := p.s.call(map[string]any{"op": "setContent", "pageId": p.id, "html": html})
	return err
}

func (p *bpage) addStyleTagPath(path string) error {
	abs, _ := filepath.Abs(path)
	_, err := p.s.call(map[string]any{"op": "addStyleTag", "pageId": p.id, "path": abs})
	return err
}

func (p *bpage) addStyleTag(content string) error {
	_, err := p.s.call(map[string]any{"op": "addStyleTag", "pageId": p.id, "content": content})
	return err
}

// routeAbortExternal blocks http(s) subresource loads (initial-render pin).
func (p *bpage) routeAbortExternal() error {
	_, err := p.s.call(map[string]any{"op": "routeAbortExternal", "pageId": p.id})
	return err
}

func (p *bpage) waitForFunction(expr string, timeoutMS int) error {
	_, err := p.s.call(map[string]any{"op": "waitForFunction", "pageId": p.id, "expr": expr, "timeout": timeoutMS})
	return err
}

func (p *bpage) events() ([]string, error) {
	res, err := p.s.call(map[string]any{"op": "events", "pageId": p.id})
	if err != nil {
		return nil, err
	}
	var out []string
	if raw, ok := res["errors"].([]any); ok {
		for _, e := range raw {
			if s, ok := e.(string); ok {
				out = append(out, s)
			}
		}
	}
	return out, nil
}

func (p *bpage) close() {
	p.s.call(map[string]any{"op": "close", "pageId": p.id})
}

// ---- locator primitives (frame = an iframe selector; "" = the page itself) ----

func (p *bpage) locCount(frame, selector string) (int, error) {
	res, err := p.s.call(map[string]any{"op": "locCount", "pageId": p.id, "frame": frame, "selector": selector})
	if err != nil {
		return 0, err
	}
	return int(res["value"].(float64)), nil
}

func (p *bpage) locWait(frame, selector, state string, timeoutMS int) error {
	_, err := p.s.call(map[string]any{"op": "locWait", "pageId": p.id, "frame": frame, "selector": selector, "state": state, "timeout": timeoutMS})
	return err
}

func (p *bpage) locScroll(frame, selector string, index int) error {
	_, err := p.s.call(map[string]any{"op": "locScroll", "pageId": p.id, "frame": frame, "selector": selector, "index": index})
	return err
}

func (p *bpage) locAttr(frame, selector, attr string) (string, bool, error) {
	res, err := p.s.call(map[string]any{"op": "locAttr", "pageId": p.id, "frame": frame, "selector": selector, "attr": attr})
	if err != nil {
		return "", false, err
	}
	v, ok := res["value"].(string)
	return v, ok, nil
}

type bbox struct {
	X, Y, Width, Height float64
}

func (p *bpage) locBox(frame, selector string, index int) (*bbox, error) {
	res, err := p.s.call(map[string]any{"op": "locBox", "pageId": p.id, "frame": frame, "selector": selector, "index": index})
	if err != nil {
		return nil, err
	}
	m, ok := res["value"].(map[string]any)
	if !ok {
		return nil, nil
	}
	return &bbox{m["x"].(float64), m["y"].(float64), m["width"].(float64), m["height"].(float64)}, nil
}

// locEvalAll evaluates expr over every match (playwright evaluateAll).
func (p *bpage) locEvalAll(frame, selector, expr string) (any, error) {
	return p.locEvalAllArg(frame, selector, expr, nil)
}

func (p *bpage) locEvalAllArg(frame, selector, expr string, arg any) (any, error) {
	res, err := p.s.call(map[string]any{"op": "locEvalAll", "pageId": p.id, "frame": frame, "selector": selector, "expr": expr, "arg": arg})
	if err != nil {
		return nil, err
	}
	return res["value"], nil
}

// locClick clicks the nth match; button "right" for context-menu triggers.
func (p *bpage) locClick(frame, selector string, index int, button string) error {
	_, err := p.s.call(map[string]any{"op": "locClick", "pageId": p.id, "frame": frame, "selector": selector, "index": index, "button": button})
	return err
}

func (p *bpage) mouseMove(x, y float64, steps int) error {
	_, err := p.s.call(map[string]any{"op": "mouseMove", "pageId": p.id, "x": x, "y": y, "steps": steps})
	return err
}

func (p *bpage) waitForLoadState(state string, timeoutMS int) error {
	_, err := p.s.call(map[string]any{"op": "waitForLoadState", "pageId": p.id, "state": state, "timeout": timeoutMS})
	return err
}

func (p *bpage) waitForTimeout(ms int) error {
	_, err := p.s.call(map[string]any{"op": "waitForTimeout", "pageId": p.id, "ms": ms})
	return err
}

func (p *bpage) mouseClick(x, y float64) error {
	_, err := p.s.call(map[string]any{"op": "mouseClick", "pageId": p.id, "x": x, "y": y})
	return err
}

func (p *bpage) keyPress(key string) error {
	_, err := p.s.call(map[string]any{"op": "keyPress", "pageId": p.id, "key": key})
	return err
}
