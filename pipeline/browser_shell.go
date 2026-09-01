package main

// browser-shell driver — the Go half of Wave 3's chromium thin shell
// (tools/browser-shell.mjs). One long-lived node process; JSON line
// requests in, one JSON line response out. Go owns every judgement.
//
// Errors from the shell surface as Go errors; a shell crash fails the node.

import (
	"bufio"
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
	res, err := s.call(map[string]any{"op": "newPage", "capture": capture})
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
