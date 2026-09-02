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
// What does not change: the tools themselves. The emitter needs jsdom and
// tailwind-merge, docs-build needs mdx and shiki, and the oracle renders real
// React. None of those has a Go equivalent that produces identical bytes, and
// `reproducible` compares the committed trees byte for byte. They run here as
// they always did — in a container instead of on the host. The converter was
// the last @babel/parser consumer; it ported to `pipeline convert` and now
// runs in the Go container.

package main

import (
	"context"
	"fmt"
	"regexp"
	"sort"
	"strings"
	"sync"

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
// The inputs are two directory mounts:
//
//	src/         the pin and tiers registries and the kernel fixtures the
//	             Go convert step reads
//	.upstream/…/registry/   the .tsx being converted, and style-nova.css
//
// src/registry/ir is deliberately NOT mounted. It is this step's OUTPUT, and
// starting from an empty directory is what makes a retired component's IR
// disappear instead of lingering: src/registry/ir/form.json survived in the
// committed tree since the initial commit, for a component upstream no longer
// ships, because the converter writes in place and nothing removed it. Every
// gate missed it — including `reproducible`, which rebuilds in place too, so
// the orphan sat in both trees and never differed.
//
// Both steps run the Go pipeline binary (resolve-skins, then convert); the
// node toolchain is gone from this step entirely.
func (m *Shadless) converted(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	bin, err := m.pipelineBin(ctx, source)
	if err != nil {
		return nil, err
	}
	img, err := goImage(ctx, source)
	if err != nil {
		return nil, err
	}
	return dag.Container().
		From(img).
		WithWorkdir("/w").
		WithFile("/w/build/pipeline", bin).
		WithDirectory("/w/src", source.Directory("src").Filter(
			dagger.DirectoryFilterOpts{Exclude: []string{"registry/ir/**"}})).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithExec([]string{"./build/pipeline", "resolve-skins"}).
		WithExec([]string{"./build/pipeline", "convert"}), nil
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

// pipelineBin builds the Go runner. `emit` no longer needs it — the tailwind
// wrapper it used is one WithWorkdir in a container — but `product-css` and
// `oracle-css` are Go subcommands with real logic, so the next slice does.
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
// dist/ is NOT mounted, and that is the point. This step used to also run
// `tw … --cwd dist`, which made Tailwind scan dist for utility classes, so
// emit's output depended on what other steps had left there — an input that
// no declaration mentioned, and an output written into one of its own input
// mounts. That compile existed only to feed emit-smoke, which is deleted, so
// the whole entanglement went with it. emit now reads its sources and writes
// dist/; nothing more.
func (m *Shadless) emitted(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	ir, err := m.Convert(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/src", source.Directory("src").Filter(
			dagger.DirectoryFilterOpts{Exclude: []string{"registry/ir/**"}})).
		WithDirectory("/w/src/registry/ir", ir).
		WithDirectory("/w/probes", source.Directory("probes")).
		// the emitter reads style-nova.css directly; it was itself an
		// undeclared input until today, and the sandbox caught it missing here
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithExec([]string{"node", "src/emitter/index.mjs"}), nil
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

// oracleBase is renderBase plus the SHIPPED tree.
//
// The contract replays compare the oracle against the page this repo ships, so
// they need dist/ and probes/. Nothing else does: a pure upstream render reads
// neither, which is why they sit here rather than one level down.
func (m *Shadless) oracleBase(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.renderBase(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/dist", source.Directory("dist")).
		// The contract defs point at the shipped fixture pages under probes/
		// (shadlessPage: "probes/t7/<name>.html"). The `contracts` node in
		// pipeline/nodes.go declares no probes/ input at all — the sandbox
		// found that on its first run, as an ENOENT at the point of use rather
		// than a silently stale-fresh node.
		WithDirectory("/w/probes", source.Directory("probes")), nil
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
	bin, err := goBinary(ctx, source)
	if err != nil {
		return "", err
	}
	return withContractDef(c, source, name).
		WithFile("/usr/local/bin/pipeline", bin).
		WithExec([]string{"pipeline", "contract", name}).
		Stdout(ctx)
}

// withContractDef adds exactly the one def this contract replays, so a change
// to any other def leaves this container's key untouched.
func withContractDef(c *dagger.Container, source *dagger.Directory, name string) *dagger.Container {
	rel := "tools/contracts/components/" + name + ".mjs"
	return c.WithFile("/w/"+rel, source.File(rel))
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

// contractNames lists the component contract defs, which are the fan-out.
// Reading the directory rather than carrying a list means adding a def adds a
// job, with nothing to keep in step — the same reason pipeline/fanout.go
// discovers them at graph-load time instead of baking them into nodes.go.
func contractNames(ctx context.Context, source *dagger.Directory) ([]string, error) {
	entries, err := source.Directory("tools/contracts/components").Entries(ctx)
	if err != nil {
		return nil, fmt.Errorf("listing contract defs: %w", err)
	}
	var out []string
	for _, e := range entries {
		if strings.HasSuffix(e, ".mjs") {
			out = append(out, strings.TrimSuffix(e, ".mjs"))
		}
	}
	sort.Strings(out)
	return out, nil
}

// contractParallel caps how many contracts run at once. Each owns a chromium,
// which is why the host runner defaults PIPELINE_PARALLEL to 4 rather than to
// NumCPU; the same reasoning applies here.
const contractParallel = 4

// Contracts replays every component contract against the React oracle.
//
// The 29 defs are genuinely independent — separate outputs, no shared mutable
// state, each failing on its own — which is what makes the fan-out legal. They
// share one oracleBase, so npm ci, the browser install and the conversion are
// paid once and every contract starts from that same cached container.
//
// A failure does not stop the others: the point of running all 29 is the whole
// picture, so every result is collected and the failures are reported together.
func (m *Shadless) Contracts(ctx context.Context, source *dagger.Directory) (string, error) {
	names, err := contractNames(ctx, source)
	if err != nil {
		return "", err
	}
	base, err := m.oracleBase(ctx, source)
	if err != nil {
		return "", err
	}
	bin, err := goBinary(ctx, source)
	if err != nil {
		return "", err
	}
	base = base.WithFile("/usr/local/bin/pipeline", bin)

	type result struct {
		name string
		err  error
	}
	results := make([]result, len(names))
	sem := make(chan struct{}, contractParallel)
	var wg sync.WaitGroup
	for i, name := range names {
		wg.Add(1)
		go func(i int, name string) {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()
			_, err := withContractDef(base, source, name).
				WithExec([]string{"pipeline", "contract", name}).
				Stdout(ctx)
			results[i] = result{name: name, err: err}
		}(i, name)
	}
	wg.Wait()

	var b strings.Builder
	var failed []string
	for _, r := range results {
		if r.err != nil {
			failed = append(failed, r.name)
			fmt.Fprintf(&b, "  FAIL  %s\n", r.name)
			continue
		}
		fmt.Fprintf(&b, "  PASS  %s\n", r.name)
	}
	if len(failed) > 0 {
		return b.String(), fmt.Errorf("FAIL  contracts (%d/%d): %s",
			len(failed), len(names), strings.Join(failed, ", "))
	}
	fmt.Fprintf(&b, "\nPASS  contracts (%d/%d)\n", len(names), len(names))
	return b.String(), nil
}
