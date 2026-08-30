package main

// hooks — install (or remove) the shadless git hooks into .git/hooks/.
// Ported from tools/git-hooks/install.mjs.
//
//	pipeline hooks               install pre-commit + pre-push
//	pipeline hooks --uninstall   remove them
//
// Refuses to overwrite a hook that is not ours (no marker) unless --force.

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

var gitHooks = []string{"pre-commit", "pre-push"}

const hookMarker = "shadless"

func runHooks(args []string) int {
	uninstall, force := has(args, "--uninstall"), has(args, "--force")
	wd, _ := os.Getwd()
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "hooks:", err)
		return 1
	}
	ours := func(path string) bool {
		b, err := os.ReadFile(path)
		return err == nil && strings.Contains(string(b), hookMarker)
	}

	for _, name := range gitHooks {
		src := filepath.Join(root, "tools", "git-hooks", name)
		dst := filepath.Join(root, ".git", "hooks", name)
		_, exists := os.Stat(dst)
		if uninstall {
			if exists != nil {
				continue
			}
			if !ours(dst) && !force {
				fmt.Fprintf(os.Stderr, "hooks: %s is not a shadless hook — pass --force\n", dst)
				return 1
			}
			if err := os.Remove(dst); err != nil {
				fmt.Fprintln(os.Stderr, "hooks:", err)
				return 1
			}
			fmt.Printf("hooks: removed %s\n", dst)
			continue
		}
		if exists == nil && !ours(dst) && !force {
			fmt.Fprintf(os.Stderr, "hooks: %s exists and is not a shadless hook — inspect it, then --force\n", dst)
			return 1
		}
		body, err := os.ReadFile(src)
		if err != nil {
			fmt.Fprintln(os.Stderr, "hooks:", err)
			return 1
		}
		if err := os.WriteFile(dst, body, 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "hooks:", err)
			return 1
		}
		// WriteFile honours the umask on create and leaves an existing file's
		// mode alone, so set it explicitly — a hook that is not executable is
		// silently not a hook.
		if err := os.Chmod(dst, 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "hooks:", err)
			return 1
		}
		fmt.Printf("hooks: installed %s\n", dst)
	}
	if !uninstall {
		fmt.Print("\n  pre-commit → gates fast tier (<1s)   pre-push → medium tier (~10s)\n" +
			"  hand-edits to generated files are caught by 'make reproducible' (CI), not here\n")
	}
	return 0
}
