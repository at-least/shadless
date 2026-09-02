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

// Upstream ships THREE parallel registries under apps/v4/registry/bases —
// radix (radix-ui), base (@base-ui/react) and aria (react-aria-components) —
// with the same components and the same examples on different primitive
// libraries. This repo targets exactly one, and which one is recorded in
// pin.json's `registry` path.
//
// That fact was written in three places with nothing comparing them:
// pin.json, and two constants in the upstream-snapshot step (a docs directory
// and the crawl URL) with "radix" baked into both. The crawler now derives
// them; what remains to check is that the GRAPH agrees, because the convert
// node names its base in a glob. If those two ever disagree, the golden hop
// compares pages generated from one base against another base's live pages,
// and every diff looks like a real regression instead of a mismatched
// comparison.
var registryBaseRe = regexp.MustCompile(`registry/bases/([^/]+)/`)

func pinnedBase(p *pinFile) (string, error) {
	m := registryBaseRe.FindStringSubmatch(p.ShadcnUI.Registry)
	if m == nil {
		return "", fmt.Errorf("pin.json `shadcn_ui.registry` is %q, not a path of the form apps/v4/registry/bases/<base>/ui",
			p.ShadcnUI.Registry)
	}
	return m[1], nil
}

// checkPinnedBase verifies that the base pin.json names exists upstream and is
// the one the graph actually converts from.
func checkPinnedBase(root string, p *pinFile) error {
	base, err := pinnedBase(p)
	if err != nil {
		return err
	}
	if _, err := os.Stat(filepath.Join(root, shadcnDir, "apps/v4/registry/bases", base)); err != nil {
		return fmt.Errorf("pin.json targets base %q, which the pinned checkout does not have under apps/v4/registry/bases", base)
	}
	g, err := AuthoredGraph()
	if err != nil {
		return err
	}
	n, ok := g.Node(NConvert)
	if !ok {
		return fmt.Errorf("no %s node to check the pinned base against", NConvert)
	}
	want := "registry/bases/" + base + "/"
	for _, in := range n.Inputs {
		if strings.Contains(in, want) {
			return nil
		}
	}
	return fmt.Errorf("pin.json targets base %q but the %s node declares no input under %s — the pin and the graph name different registries",
		base, NConvert, want)
}

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

// cloneUpstream bootstraps a missing .upstream/shadcn-ui checkout by cloning
// at the tag src/registry/pin.json already records — a fresh checkout always
// has a committed pin to clone to, so this never has to guess a version.
func cloneUpstream(root string) error {
	p, err := readPin(root)
	if err != nil {
		return fmt.Errorf("cannot read %s to learn which tag to clone: %w", pinFilePath, err)
	}
	if p.ShadcnUI.Tag == "" {
		return fmt.Errorf("%s has no shadcn_ui.tag recorded", pinFilePath)
	}
	fmt.Fprintf(os.Stderr, "PIN: %s not found — cloning shadcn-ui at %s\n", shadcnDir, p.ShadcnUI.Tag)
	if _, err := git(root, "clone", "--quiet", "https://github.com/shadcn-ui/ui", shadcnDir); err != nil {
		return fmt.Errorf("git clone: %w", err)
	}
	if _, err := git(root, "-C", shadcnDir, "checkout", "--quiet", p.ShadcnUI.Tag); err != nil {
		return fmt.Errorf("git checkout %s: %w", p.ShadcnUI.Tag, err)
	}
	return nil
}

func runPin(root string, checkOnly, force bool) int {
	if _, err := os.Stat(filepath.Join(root, shadcnDir)); err != nil {
		if err := cloneUpstream(root); err != nil {
			fmt.Fprintf(os.Stderr, "PIN FAIL: %s not found and auto-clone failed: %v\n", shadcnDir, err)
			return 1
		}
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
		if err := checkPinnedBase(root, recorded); err != nil {
			fmt.Fprintf(os.Stderr, "PIN FAIL: %v\n", err)
			fail = true
		}
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
