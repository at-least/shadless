// Package main is the shadless pipeline as Dagger functions.
//
// The orchestration this replaces (pipeline/{key,run,graph,fanout,verify}.go,
// ~1800 lines) existed to answer one question the source tree could not:
// "is this step's work still valid?" It answered it by hand-declaring each
// node's inputs as globs and hashing them. 70% of those declarations named a
// single file, and every one was a chance to miss something — five real
// stale-green bugs came out of checking them.
//
// Here a step's inputs are the directories mounted into its container. A file
// that is not mounted is not there: reading it fails with ENOENT instead of
// silently leaving the step falsely fresh. The declaration does not disappear,
// but it moves from a glob list to a mount, at FOLDER granularity, and it
// stops being able to be quietly wrong.
//
// What does not change: the tools themselves. The converter needs
// @babel/parser, the emitter needs jsdom and tailwind-merge, docs-build needs
// mdx and shiki, and the oracle renders real React. None of those has a Go
// equivalent that produces identical bytes, and `reproducible` compares the
// committed trees byte for byte. They run here as they always did — in a
// container instead of on the host.

package main

import (
	"context"
	"fmt"
	"regexp"
	"strings"

	"dagger/shadless/internal/dagger"
)

// This file names no tool versions, on purpose.
//
// Versioning the repo's tools is a human decision recorded in the repo, and a
// build should depend on CONTENT rather than on a version string. Two
// consequences:
//
//   - the node runtime comes from .nvmrc, which the human edits. It is read,
//     not hardcoded. That declaration used to live only in ci.yml's
//     node-version, and deleting the workflow took it with it.
//   - the browser is installed by the playwright that package-lock.json
//     resolves, so it matches the locked package by construction. Naming an
//     image tag like mcr.microsoft.com/playwright:v1.62.1 would be a SECOND
//     declaration of the same version, free to drift from the lockfile — the
//     exact failure this port exists to remove.
//
// Dagger caches on the content of the files these come from, so a bump is a
// lockfile or .nvmrc edit, and the rebuild happens when the human makes one.
const (
	// oracleCache is the esbuild bundle cache oracle-lib keys on (upstream +
	// lockfile). A ~7MB bundle per demo and 1-2s of esbuild per demo, so it is
	// worth persisting across runs.
	oracleCache = "/w/node_modules/.cache/shadless"
)

type Shadless struct{}

// goImage resolves the Go toolchain from pipeline/go.mod.
//
// go.mod is where Go itself says this belongs: `toolchain` names a toolchain,
// `go` names the language version. Preferring the first and falling back to
// the second means the repo keeps the decision either way, and this file
// keeps none of it.
func goImage(ctx context.Context, source *dagger.Directory) (string, error) {
	mod, err := source.File("pipeline/go.mod").Contents(ctx)
	if err != nil {
		return "", fmt.Errorf("reading pipeline/go.mod: %w", err)
	}
	if m := reToolchain.FindStringSubmatch(mod); m != nil {
		return "golang:" + m[1] + "-bookworm", nil
	}
	if m := reGoLang.FindStringSubmatch(mod); m != nil {
		return "golang:" + m[1] + "-bookworm", nil
	}
	return "", fmt.Errorf("pipeline/go.mod declares neither a toolchain nor a go version")
}

var (
	reToolchain = regexp.MustCompile(`(?m)^toolchain\s+go(\S+)\s*$`)
	reGoLang    = regexp.MustCompile(`(?m)^go\s+(\S+)\s*$`)
)

// nodeImage resolves the runtime from the repo's own declaration.
func nodeImage(ctx context.Context, source *dagger.Directory) (string, error) {
	v, err := source.File(".nvmrc").Contents(ctx)
	if err != nil {
		return "", fmt.Errorf("reading .nvmrc: %w (the node version is the repo's "+
			"to declare, not this file's)", err)
	}
	return "node:" + strings.TrimSpace(v) + "-bookworm-slim", nil
}

// deps is the npm dependency set, installed once and cached on the lockfile.
//
// --ignore-scripts skips playwright's browser download here; the steps that
// need a browser ask for one explicitly through withBrowser.
func (m *Shadless) deps(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	img, err := nodeImage(ctx, source)
	if err != nil {
		return nil, err
	}
	return dag.Container().
		From(img).
		WithWorkdir("/w").
		WithFile("/w/package.json", source.File("package.json")).
		WithFile("/w/package-lock.json", source.File("package-lock.json")).
		WithExec([]string{"npm", "ci", "--ignore-scripts", "--no-audit", "--no-fund"}), nil
}

// withBrowser installs chromium using the playwright the lockfile resolved, so
// the browser and its driver match by construction rather than through an
// image tag somebody has to remember to bump.
func withBrowser(c *dagger.Container) *dagger.Container {
	return c.WithExec([]string{"npx", "playwright", "install", "--with-deps", "chromium"})
}

// converted is the container after the registry has been converted: it holds
// both of that step's outputs — src/registry/ir (the IR) and build/resolved-ui
// (the skin-flattened registry the oracle bundles from).
//
// The inputs are three directory mounts:
//
//	src/         the converter, the tag table, the skin allowlist, the tiers
//	             and pin registries, the kernel fixtures
//	tools/       resolve-skins, which flattens the upstream skin first
//	.upstream/…/registry/   the .tsx being converted, and style-nova.css
//
// src/registry/ir is deliberately NOT mounted. It is this step's OUTPUT, and
// starting from an empty directory is what makes a retired component's IR
// disappear instead of lingering: src/registry/ir/form.json survived in the
// committed tree since the initial commit, for a component upstream no longer
// ships, because the converter writes in place and nothing removed it. Every
// gate missed it — including `reproducible`, which rebuilds in place too, so
// the orphan sat in both trees and never differed.
func (m *Shadless) converted(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/src", source.Directory("src"),
			dagger.ContainerWithDirectoryOpts{Exclude: []string{"registry/ir/**"}}).
		WithDirectory("/w/tools", source.Directory("tools")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithExec([]string{"node", "tools/resolve-skins.mjs"}).
		WithExec([]string{"node", "src/converter/index.mjs"}), nil
}

// Convert turns the pinned registry .tsx into the versioned IR.
func (m *Shadless) Convert(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.converted(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w/src/registry/ir"), nil
}

// ResolvedUI is the skin-flattened registry: upstream's cn-* utilities already
// expanded, which is what the oracle bundles React against.
func (m *Shadless) ResolvedUI(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.converted(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w/build/resolved-ui"), nil
}

// ConvertCheck reports the conversion's own verdict without exporting anything.
func (m *Shadless) ConvertCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.converted(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}

// pipelineBin builds the Go runner. `emit` shells out to `pipeline tw`, the
// hermetic @tailwindcss/cli wrapper whose whole job is controlling the compile
// CWD, so the step needs the real binary rather than a reimplementation.
func (m *Shadless) pipelineBin(ctx context.Context, source *dagger.Directory) (*dagger.File, error) {
	img, err := goImage(ctx, source)
	if err != nil {
		return nil, err
	}
	return dag.Container().
		From(img).
		WithDirectory("/s", source.Directory("pipeline")).
		WithWorkdir("/s").
		WithExec([]string{"go", "build", "-o", "/pipeline", "."}).
		File("/pipeline"), nil
}

// emitted is the container after the static tier has been emitted: IR becomes
// component HTML plus the per-slot stylesheet.
//
// src/registry/ir is mounted from the CONVERSION step, not from the host, so
// this emits what the pipeline just produced rather than whatever the working
// tree happens to hold. In the current graph that correspondence holds only by
// ordering luck — emit declares no IR input at all and relies on `needs`.
//
// dist/ IS mounted from the host, and that is a deliberate compromise worth
// naming: the second command is `tw … --cwd dist`, which makes Tailwind scan
// dist for utility classes, so the emitted out.css depends on what else is in
// there. Starting from an empty dist would change the result. That makes dist
// both an input and an output of this step — exactly the mixing this port is
// meant to remove — so it is the next thing to untangle, not the last word.
func (m *Shadless) emitted(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	ir, err := m.Convert(ctx, source)
	if err != nil {
		return nil, err
	}
	bin, err := m.pipelineBin(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/src", source.Directory("src"),
			dagger.ContainerWithDirectoryOpts{Exclude: []string{"registry/ir/**"}}).
		WithDirectory("/w/src/registry/ir", ir).
		WithDirectory("/w/probes", source.Directory("probes")).
		// the emitter reads style-nova.css directly; it was itself an
		// undeclared input until today, and the sandbox caught it missing here
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithDirectory("/w/dist", source.Directory("dist")).
		WithFile("/w/build/pipeline", bin,
			dagger.ContainerWithFileOpts{Permissions: 0o755}).
		// the binary ships without its own source, so it cannot find the repo
		// root by walking up to pipeline/nodes.go
		WithEnvVariable("SHADLESS_ROOT", "/w").
		WithExec([]string{"node", "src/emitter/index.mjs"}).
		WithExec([]string{"./build/pipeline", "tw",
			"build/emit/globals.css", "build/emit/out.css", "--cwd", "dist"}), nil
}

// Emit returns the static tier's shipped output: the component pages and the
// per-slot stylesheet.
func (m *Shadless) Emit(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.emitted(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w/dist"), nil
}

// EmitCheck reports the emitter's own verdict without exporting anything.
func (m *Shadless) EmitCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.emitted(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}

// oracleBase is the browser half of the pipeline: chromium plus everything the
// React oracle bundles.
//
// This is the part that decided Dagger over Bazel. The oracle renders the
// pinned registry with real React and radix and replays real input against the
// shipped page; there is no Go equivalent and never will be. Bazel would need
// a hermetic browser toolchain inside its sandbox — the hardest part of any JS
// Bazel migration — where here it is one exec against the locked playwright.
//
// build/resolved-ui comes from the conversion step rather than the host, so
// the oracle bundles what this pipeline just produced instead of whatever the
// working tree happens to hold.
func (m *Shadless) oracleBase(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	resolved, err := m.ResolvedUI(ctx, source)
	if err != nil {
		return nil, err
	}
	return withBrowser(c).
		WithDirectory("/w/tools", source.Directory("tools")).
		WithDirectory("/w/gates", source.Directory("gates")).
		WithDirectory("/w/src", source.Directory("src")).
		WithDirectory("/w/dist", source.Directory("dist")).
		// The contract defs point at the shipped fixture pages under probes/
		// (shadlessPage: "probes/t7/<name>.html"). The `contracts` node in
		// pipeline/nodes.go declares no probes/ input at all — the sandbox
		// found that on its first run, as an ENOENT at the point of use rather
		// than a silently stale-fresh node.
		WithDirectory("/w/probes", source.Directory("probes")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4",
			source.Directory(".upstream/shadcn-ui/apps/v4")).
		WithDirectory("/w/build/resolved-ui", resolved).
		WithMountedCache(oracleCache, dag.CacheVolume("shadless-oracle-bundles")), nil
}

// Contract replays one component's contract against the React oracle: the
// pinned registry bundled with real React+radix, driven with real mouse and
// keyboard, compared to the shipped page under the same input.
//
// `name` is a contract def under tools/contracts/components (label is the
// cheapest — native `for`, zero JS).
func (m *Shadless) Contract(ctx context.Context, source *dagger.Directory, name string) (string, error) {
	c, err := m.oracleBase(ctx, source)
	if err != nil {
		return "", err
	}
	return c.WithExec([]string{"node", "tools/contracts/run.mjs", name}).Stdout(ctx)
}

// Doctor reports the toolchain each half of the pipeline actually resolved to,
// so what the build used is visible rather than assumed. Nothing here is
// pinned by this file: node comes from .nvmrc and the browser from the locked
// playwright.
func (m *Shadless) Doctor(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.oracleBase(ctx, source)
	if err != nil {
		return "", err
	}
	return c.
		WithExec([]string{"sh", "-c",
			`echo "node:       $(node --version)"; ` +
				`echo "npm:        $(npm --version)"; ` +
				`echo "playwright: $(node -p "require('playwright/package.json').version")"; ` +
				`echo "tailwind:   $(node -p "require('@tailwindcss/cli/package.json').version")"; ` +
				`echo "esbuild:    $(node -p "require('esbuild/package.json').version")"; ` +
				`echo "chromium:   $(node -e "console.log(require('playwright').chromium.executablePath())")"`}).
		Stdout(ctx)
}
