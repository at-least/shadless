package main

// tw — hermetic @tailwindcss/cli wrapper. Ported from tools/tw.mjs.
//
// Why this exists (2026-08-26, found via the +88-line out.css diff): the CLI's
// automatic source detection scans the CWD's tracked repo files. Two
// consequences we control explicitly:
//
//  1. out.css (the demo+site stylesheet) NEEDS the repo-wide scan — authored
//     docs/demos/*.html pages load it in iframes and their utilities are only
//     picked up that way (load-bearing, previously accidental). It compiles
//     with `--cwd .` plus `@source not` exclusions (see tools/demo.mjs) so
//     tool/source fixtures don't leak dead classes.
//  2. the PRODUCT build must emit ONLY @apply-driven rules: compile with no
//     --cwd → fresh empty scratch dir → zero content scanning.
//
//     pipeline tw <in> <out> [--minify] [--cwd DIR]
//
// The compile cwd is the whole point of the wrapper, so paths are resolved
// against the REPO ROOT and never against that cwd — including the CLI
// binary, which must come from this repo's node_modules and not from whatever
// package happens to sit above the scratch directory.

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

// findRepoRoot walks up from dir to the tree holding pipeline/nodes.go. The
// pipeline's own commands all run with the root as their cwd, but tw is also
// spawned by JS gates from wherever they happen to be, and its whole job is
// to control the cwd of something else.
func findRepoRoot(dir string) (string, error) {
	for {
		if _, err := os.Stat(filepath.Join(dir, "pipeline", "nodes.go")); err == nil {
			return dir, nil
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return "", fmt.Errorf("repo root (the tree holding pipeline/nodes.go) not found above the working directory")
		}
		dir = parent
	}
}

// twCompile is the wrapper itself: `in` and `out` are resolved against the
// repo root, never against the compile cwd, and so is the CLI binary — it must
// come from this repo's node_modules and not from whatever package happens to
// sit above the scratch directory. An empty compileCwd means "fresh empty
// scratch dir", i.e. zero content scanning.
func twCompile(root, in, out, compileCwd string, minify, quiet bool) error {
	abs := func(p string) string {
		if filepath.IsAbs(p) {
			return p
		}
		return filepath.Join(root, p)
	}
	argv := []string{"-i", abs(in), "-o", abs(out)}
	if minify {
		argv = append(argv, "--minify")
	}

	dir := ""
	if compileCwd != "" {
		dir = abs(compileCwd)
	} else {
		scratch, err := os.MkdirTemp("", "shadless-tw-")
		if err != nil {
			return err
		}
		defer os.RemoveAll(scratch)
		dir = scratch
	}

	cmd := exec.Command(filepath.Join(root, "node_modules", ".bin", "tailwindcss"), argv...)
	cmd.Dir = dir
	if !quiet {
		cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	}
	return cmd.Run()
}

func runTw(args []string) int {
	minify := false
	compileCwd := ""
	var positional []string
	for i := 0; i < len(args); i++ {
		switch args[i] {
		case "--minify":
			minify = true
		case "--cwd":
			if i+1 >= len(args) {
				fmt.Fprintln(os.Stderr, "tw: --cwd needs a directory")
				return 1
			}
			compileCwd = args[i+1]
			i++
		default:
			positional = append(positional, args[i])
		}
	}
	if len(positional) != 2 {
		fmt.Fprintln(os.Stderr, "usage: pipeline tw <in> <out> [--minify] [--cwd DIR]")
		return 1
	}
	wd, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, "tw:", err)
		return 1
	}
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "tw:", err)
		return 1
	}
	if err := twCompile(root, positional[0], positional[1], compileCwd, minify, false); err != nil {
		if ee, ok := err.(*exec.ExitError); ok {
			return ee.ExitCode()
		}
		fmt.Fprintln(os.Stderr, "tw:", err)
		return 1
	}
	return 0
}
