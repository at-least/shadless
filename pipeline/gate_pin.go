package main

// pin — record and verify the upstream pins (the shadcn-ui clone, the vendored
// kernel IIFE).
//
// The shadcn pin must sit on a stable release tag (shadcn@*), not an arbitrary
// main commit: recording the tag name makes that explicit and lets the drift
// check catch a pin that has slipped onto an unreleased commit. Upgrade tools
// write pin.json directly, so nothing else verifies the result.
//
//	pipeline pin              record, or verify an existing pin.json
//	pipeline pin --check-only verify only, record nothing (the gate)
//	pipeline pin --force      re-record at the checkout's HEAD despite drift
//	                          (the re-pin drill; without it drift is an error,
//	                          so a stray checkout cannot move the pin silently)

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
	"time"
)

const (
	shadcnDir   = ".upstream/shadcn-ui"
	kernelIife  = "vendor/radix-kernel.iife.js"
	pinFilePath = "src/registry/pin.json"
)

var releaseTagRe = regexp.MustCompile(`^shadcn@\d`)

type pinFile struct {
	ShadcnUI struct {
		Repo     string `json:"repo"`
		Tag      string `json:"tag"`
		Commit   string `json:"commit"`
		Registry string `json:"registry"`
	} `json:"shadcn_ui"`
	Kernel struct {
		File   string `json:"file"`
		Sha256 string `json:"sha256"`
	} `json:"kernel"`
	Recorded string `json:"recorded"`
}

func git(root string, args ...string) (string, error) {
	c := exec.Command("git", args...)
	c.Dir = root
	out, err := c.Output()
	return strings.TrimSpace(string(out)), err
}

func sha256File(path string) (string, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	sum := sha256.Sum256(b)
	return hex.EncodeToString(sum[:]), nil
}

func readPin(root string) (*pinFile, error) {
	b, err := os.ReadFile(filepath.Join(root, pinFilePath))
	if err != nil {
		return nil, err
	}
	var p pinFile
	if err := json.Unmarshal(b, &p); err != nil {
		return nil, err
	}
	return &p, nil
}

func runPin(root string, checkOnly, force bool) int {
	if _, err := os.Stat(filepath.Join(root, shadcnDir)); err != nil {
		fmt.Fprintf(os.Stderr, "PIN FAIL: %s not found — clone the upstream first:\n", shadcnDir)
		fmt.Fprintf(os.Stderr, "  git clone https://github.com/shadcn-ui/ui %s\n", shadcnDir)
		fmt.Fprintf(os.Stderr, "  git -C %s checkout shadcn@4.19.0\n", shadcnDir)
		return 1
	}
	head, err := git(root, "-C", shadcnDir, "rev-parse", "HEAD")
	if err != nil {
		fmt.Fprintf(os.Stderr, "PIN FAIL: cannot read upstream HEAD: %v\n", err)
		return 1
	}
	kernelSha, err := sha256File(filepath.Join(root, kernelIife))
	if err != nil {
		fmt.Fprintf(os.Stderr, "PIN FAIL: cannot hash %s: %v\n", kernelIife, err)
		return 1
	}

	if checkOnly {
		recorded, err := readPin(root)
		if err != nil {
			fmt.Fprintln(os.Stderr, "PIN FAIL: src/registry/pin.json missing — run npm run pin")
			return 1
		}
		fail := false
		if recorded.ShadcnUI.Commit != head {
			fmt.Fprintf(os.Stderr, "PIN FAIL: pin.json commit %s != upstream HEAD %s\n",
				short(recorded.ShadcnUI.Commit, 10), short(head, 10))
			fail = true
		}
		if !releaseTagRe.MatchString(recorded.ShadcnUI.Tag) {
			fmt.Fprintf(os.Stderr, "PIN FAIL: pin.json tag %q is not a shadcn@* release tag\n", recorded.ShadcnUI.Tag)
			fail = true
		}
		if recorded.Kernel.Sha256 != kernelSha {
			fmt.Fprintf(os.Stderr, "PIN FAIL: kernel sha256 drift (pin.json %s… != vendor %s…)\n",
				short(recorded.Kernel.Sha256, 12), short(kernelSha, 12))
			fail = true
		}
		if fail {
			return 1
		}
		fmt.Printf("pin OK (check-only): shadcn=%s (%s) kernel=%s…\n",
			recorded.ShadcnUI.Tag, short(head, 10), short(kernelSha, 12))
		return 0
	}

	// tags whose target commit == HEAD (peels annotated tags to their commit)
	raw, _ := git(root, "-C", shadcnDir, "for-each-ref", "--format=%(refname:short)", "--points-at", "HEAD", "refs/tags")
	var tagsAtHead []string
	for _, t := range strings.Split(raw, "\n") {
		if t = strings.TrimSpace(t); t != "" {
			tagsAtHead = append(tagsAtHead, t)
		}
	}
	releaseTag := ""
	for _, t := range tagsAtHead {
		if releaseTagRe.MatchString(t) {
			releaseTag = t
			break
		}
	}
	if releaseTag == "" {
		at := "(none)"
		if len(tagsAtHead) > 0 {
			at = strings.Join(tagsAtHead, ", ")
		}
		fmt.Fprintf(os.Stderr, "PIN FAIL: upstream HEAD %s is not a shadcn@* release tag\n", short(head, 10))
		fmt.Fprintf(os.Stderr, "  tags at HEAD: %s\n", at)
		fmt.Fprintf(os.Stderr, "  checkout a shadcn@* tag before pinning (e.g. git -C %s checkout shadcn@4.19.0)\n", shadcnDir)
		return 1
	}

	var next pinFile
	next.ShadcnUI.Repo = "https://github.com/shadcn-ui/ui"
	next.ShadcnUI.Tag = releaseTag
	next.ShadcnUI.Commit = head
	next.ShadcnUI.Registry = "apps/v4/registry/bases/radix/ui"
	next.Kernel.File = kernelIife
	next.Kernel.Sha256 = kernelSha
	next.Recorded = time.Now().Format("2006-01-02")

	write := func() error {
		b, err := json.MarshalIndent(next, "", "  ")
		if err != nil {
			return err
		}
		if err := os.MkdirAll(filepath.Join(root, "src/registry"), 0o755); err != nil {
			return err
		}
		return os.WriteFile(filepath.Join(root, pinFilePath), append(b, '\n'), 0o644)
	}

	old, err := readPin(root)
	if err != nil {
		if err := write(); err != nil {
			fmt.Fprintln(os.Stderr, "PIN FAIL:", err)
			return 1
		}
		fmt.Printf("pin recorded: shadcn=%s (%s) kernel=%s…\n", releaseTag, short(head, 10), short(kernelSha, 12))
		return 0
	}

	drift := old.ShadcnUI.Commit != next.ShadcnUI.Commit || old.Kernel.Sha256 != next.Kernel.Sha256
	if drift && force {
		if err := write(); err != nil {
			fmt.Fprintln(os.Stderr, "PIN FAIL:", err)
			return 1
		}
		fmt.Printf("pin re-recorded: shadcn=%s -> %s (%s) kernel=%s…\n",
			old.ShadcnUI.Tag, releaseTag, short(head, 10), short(kernelSha, 12))
		return 0
	}
	if drift {
		fmt.Fprintln(os.Stderr, "PIN DRIFT detected:")
		if old.ShadcnUI.Commit != next.ShadcnUI.Commit {
			fmt.Fprintf(os.Stderr, "  shadcn: %s -> %s\n", old.ShadcnUI.Commit, next.ShadcnUI.Commit)
		}
		if old.Kernel.Sha256 != next.Kernel.Sha256 {
			fmt.Fprintf(os.Stderr, "  kernel sha256: %s -> %s\n", old.Kernel.Sha256, next.Kernel.Sha256)
		}
		return 1
	}
	fmt.Printf("pin OK: shadcn=%s (%s) kernel=%s…\n", releaseTag, short(head, 10), short(kernelSha, 12))
	return 0
}

func short(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n]
}
