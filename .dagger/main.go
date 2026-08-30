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

	"dagger/shadless/internal/dagger"
)

// nodeImage pins the runtime the tools already run under (CI uses node 22).
const nodeImage = "node:22-bookworm-slim"

type Shadless struct{}

// deps is the npm dependency set, installed once and cached on the lockfile.
//
// --ignore-scripts skips playwright's browser download, which nothing in the
// conversion path needs; the browser gates will bring their own image.
func (m *Shadless) deps(source *dagger.Directory) *dagger.Container {
	return dag.Container().
		From(nodeImage).
		WithWorkdir("/w").
		WithFile("/w/package.json", source.File("package.json")).
		WithFile("/w/package-lock.json", source.File("package-lock.json")).
		WithExec([]string{"npm", "ci", "--ignore-scripts", "--no-audit", "--no-fund"})
}

// Convert turns the pinned registry .tsx into the versioned IR.
//
// The inputs are three directories, declared as mounts:
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
func (m *Shadless) Convert(source *dagger.Directory) *dagger.Directory {
	return m.deps(source).
		WithDirectory("/w/src", source.Directory("src"),
			dagger.ContainerWithDirectoryOpts{Exclude: []string{"registry/ir/**"}}).
		WithDirectory("/w/tools", source.Directory("tools")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithExec([]string{"node", "tools/resolve-skins.mjs"}).
		WithExec([]string{"node", "src/converter/index.mjs"}).
		Directory("/w/src/registry/ir")
}

// ConvertCheck reports the conversion's own verdict, so a caller can see the
// drift gates pass without exporting anything.
func (m *Shadless) ConvertCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	return m.deps(source).
		WithDirectory("/w/src", source.Directory("src"),
			dagger.ContainerWithDirectoryOpts{Exclude: []string{"registry/ir/**"}}).
		WithDirectory("/w/tools", source.Directory("tools")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithExec([]string{"node", "tools/resolve-skins.mjs"}).
		WithExec([]string{"node", "src/converter/index.mjs"}).
		Stdout(ctx)
}
