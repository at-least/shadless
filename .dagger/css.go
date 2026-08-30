package main

// The CSS steps. All four are the Go binary plus @tailwindcss/cli, so they
// need node_modules but no browser.
//
// Root resolution is the one thing that needed saying out loud, and tw.go had
// already said it: findRepoRoot walks up looking for pipeline/nodes.go, which
// "is fine on a developer checkout and wrong the moment the binary is deployed
// without its own source — a container that carries build/pipeline but no
// pipeline/ directory finds nothing". SHADLESS_ROOT is the escape hatch that
// comment exists for, and this is the case it was written for.

import (
	"context"

	"dagger/shadless/internal/dagger"
)

// cssTools is the binary, the CLI, and a root the binary can find.
func (m *Shadless) cssTools(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	bin, err := m.pipelineBin(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithEnvVariable("SHADLESS_ROOT", "/w").
		WithFile("/w/build/pipeline", bin, dagger.ContainerWithFileOpts{Permissions: 0o755}), nil
}

// OracleCss builds the stylesheet the React oracle renders against, from
// upstream's own globals and skin plus the resolved registry — no bytes from
// src/, which is what stops style-parity being circular.
func (m *Shadless) OracleCss(ctx context.Context, source *dagger.Directory) (*dagger.File, error) {
	c, err := m.cssTools(ctx, source)
	if err != nil {
		return nil, err
	}
	resolved, err := m.ResolvedUI(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		// the whole checkout, not apps/v4: this step reads
		// packages/shadcn/src/tailwind.css as well as apps/v4's theme files
		WithDirectory("/w/.upstream/shadcn-ui", source.Directory(".upstream/shadcn-ui").Filter(
			dagger.DirectoryFilterOpts{Exclude: []string{".git/**"}})).
		WithDirectory("/w/build/resolved-ui", resolved).
		// the entry this step writes adds `@source tools/contracts/components`
		// — the contract usage trees carry example classes. Leaving it out
		// does not fail; it silently drops rules (measured: `.static` and the
		// field component's data-[error=true] rule went missing).
		WithDirectory("/w/tools/contracts/components",
			source.Directory("tools/contracts/components")).
		WithExec([]string{"build/pipeline", "oracle-css"}).
		File("/w/build/gates/oracle.css"), nil
}

// scanTree assembles the tree `tw --cwd .` actually scans for out.css.
//
// This is the step where under-mounting does NOT produce an ENOENT. Tailwind
// walks whatever is there and silently emits a smaller stylesheet, so the only
// way to know the mount set is right is to compare the bytes.
//
// dist/ and docs/demos are each written by TWO steps, and both halves have to
// be here: demo writes the component pages, demo-rtl adds the RTL variants;
// the fixture generator's docs/demos already carries the oracle's pages
// (it mounts them and overwrites the 105 it owns), and demo-rtl adds the
// variants on top.
//
// docs/demos ALSO comes from source, underneath those, and that is not
// laziness. The directory is mixed: 11 message-scroller pages cannot be
// bundled at all (their upstream imports are not in this repo) and stay
// hand-authored, plus the demos with no upstream example. Mounting only the
// generated halves dropped .outline-card from out.css — it lives in
// docs/demos/message-scroller-visibility.html, which no step produces.
// The generated pages are layered on top, so they win where both exist.
// audit-boundary is the gate that knows which file is which.
func (m *Shadless) scanTree(ctx context.Context, source *dagger.Directory, c *dagger.Container) (*dagger.Container, error) {
	demo, err := m.Demo(ctx, source)
	if err != nil {
		return nil, err
	}
	rtl, err := m.DemoRtl(ctx, source)
	if err != nil {
		return nil, err
	}
	fixture, err := m.ExampleFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	kernel, err := m.ContractFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	ir, err := m.Convert(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/dist", demo).
		WithDirectory("/w/dist/components", rtl.Directory("dist/components")).
		WithDirectory("/w/docs/demos", source.Directory("docs/demos")).
		WithDirectory("/w/docs/demos", fixture.Directory("docs/demos")).
		WithDirectory("/w/docs/demos", rtl.Directory("docs/demos")).
		WithDirectory("/w/docs/content", source.Directory("docs/content")).
		WithDirectory("/w/src/kernel", kernel).
		WithDirectory("/w/src/registry/ir", ir).
		WithDirectory("/w/probes/t7", source.Directory("probes/t7")).
		WithDirectory("/w/probes/t8", source.Directory("probes/t8")), nil
}

// DemoCss compiles the stylesheet every demo page and contract fixture loads.
//
// tools/contracts/out is in dist/globals.css's @source list and is NOT mounted
// here, deliberately. That directory is the contract replays' review surface;
// it is gitignored, so a fresh clone does not have it, and `demo-css` does not
// declare `needs: contracts` either. If its absence changes these bytes, the
// committed out.css depends on an artifact no clone can rebuild in the right
// order — which is a finding, not a mount to add. The byte comparison answers
// it either way.
func (m *Shadless) DemoCss(ctx context.Context, source *dagger.Directory) (*dagger.File, error) {
	c, err := m.cssTools(ctx, source)
	if err != nil {
		return nil, err
	}
	c, err = m.scanTree(ctx, source, c)
	if err != nil {
		return nil, err
	}
	return c.
		WithExec([]string{"build/pipeline", "tw", "dist/globals.css", "dist/out.css", "--cwd", "."}).
		File("/w/dist/out.css"), nil
}

// productCss extracts the tokens and composes the consumer-facing entry.
func (m *Shadless) productCss(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.cssTools(ctx, source)
	if err != nil {
		return nil, err
	}
	demo, err := m.Demo(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/dist", demo).
		WithDirectory("/w/src/docs", source.Directory("src/docs")).
		WithDirectory("/w/probes/h4", source.Directory("probes/h4")).
		WithExec([]string{"build/pipeline", "product-css"}), nil
}

// ProductCss returns the two files the npm surface is built from.
func (m *Shadless) ProductCss(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.productCss(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w/dist").Filter(dagger.DirectoryFilterOpts{Include: []string{
		"shadless-core.css",
		"shadless.product.css",
	}}), nil
}

// ProductBuild is the no-build distribution artifact: the product entry
// compiled with NO --cwd, so tailwind scans nothing and only @apply-driven
// rules survive.
func (m *Shadless) ProductBuild(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.productCss(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithExec([]string{"build/pipeline", "tw", "dist/shadless.product.css", "dist/shadless.full.css"}).
		WithExec([]string{"build/pipeline", "tw", "dist/shadless.product.css", "dist/shadless.full.min.css", "--minify"}).
		Directory("/w/dist").Filter(dagger.DirectoryFilterOpts{Include: []string{
		"shadless.full.css",
		"shadless.full.min.css",
	}}), nil
}
