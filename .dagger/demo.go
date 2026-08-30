package main

// The pages chain: the JS surface, and the RTL variants derived from the
// oracle's Arabic render.
//
// Neither of these needs a browser, and that is the point of keeping them out
// of renderBase. build-js is the one step in the whole pipeline with no node
// in it at all, and build-rtl is text substitution over a page somebody else
// rendered. Putting them behind a chromium install would buy nothing and cost
// it on every cache miss.

import (
	"context"

	"dagger/shadless/internal/dagger"
)

// BuildJs is the JS surface: dist/shadless.js (radix kernel + the runtime
// engine), one dist/js/<name>.js per component, and the esm and minified
// mirrors.
//
// It runs the Go binary directly in the toolchain image rather than in the
// node one. pipeline/jsbuild.go bundles through esbuild's Go API — the npm
// package ships the same implementation — so there is no node in this step and
// nothing here needs the dependency install.
//
// The binary takes its root from the working directory (runBuildJs calls
// os.Getwd), so /w with the two input trees under it is the whole contract; no
// repo marker has to be present.
func (m *Shadless) BuildJs(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	img, err := goImage(ctx, source)
	if err != nil {
		return nil, err
	}
	bin, err := m.pipelineBin(ctx, source)
	if err != nil {
		return nil, err
	}
	return dag.Container().
		From(img).
		WithWorkdir("/w").
		WithFile("/usr/local/bin/pipeline", bin).
		WithDirectory("/w/src/runtime", source.Directory("src/runtime")).
		WithDirectory("/w/vendor", source.Directory("vendor")).
		WithExec([]string{"pipeline", "build-js"}).
		Directory("/w/dist"), nil
}

// exampleFixture regenerates the kernel-family demo pages as INTERACTIVE
// fixtures. The oracle's render of those is a static snapshot with dead
// buttons; this tool rebuilds them with the glue protocol and then PROVES each
// one interactive in a real browser before letting it land.
//
// docs/site is the awkward part, and it is awkward on the host too. The
// self-test navigates to docs/site/components/<name>.html, whose relative
// assets (../out.css, ../shadless.js, ../js/*) resolve in the SITE tree — so
// the tool mirrors the JS it just built into docs/site itself. out.css it does
// not mirror: that file is docs-build's output, and docs-build runs AFTER this
// step. On the host the self-test therefore renders against the PREVIOUS
// build's stylesheet, and mounting the committed copy here reproduces that
// exactly rather than papering over it.
func (m *Shadless) exampleFixture(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.renderBase(ctx, source)
	if err != nil {
		return nil, err
	}
	oracle, err := m.ExampleOracle(ctx, source)
	if err != nil {
		return nil, err
	}
	js, err := m.BuildJs(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/w/docs/example-fixture-targets.json",
			oracle.File("docs/example-fixture-targets.json")).
		WithDirectory("/w/docs/demos", oracle.Directory("docs/demos")).
		// the JS surface from the build step, not from the host tree
		WithDirectory("/w/dist/js", js.Directory("js")).
		WithFile("/w/dist/shadless.js", js.File("shadless.js")).
		WithDirectory("/w/docs/site/components", dag.Directory()).
		WithFile("/w/docs/site/out.css", source.File("docs/site/out.css")).
		WithExec([]string{"node", "tools/example-fixture.mjs"}), nil
}

// ExampleFixture returns the interactive pages and the site mirror the
// self-test built to prove them.
//
// docs/site/{js,shadless.js,components} are in this list because the tool
// WRITES them — the host graph declares only docs/demos/*.html for this node,
// so those are undeclared writes of exactly the alert-demo kind: stable bytes,
// so the write check skips them, and fs-record.mjs never sees a write at all.
func (m *Shadless) ExampleFixture(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.exampleFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w").Filter(dagger.DirectoryFilterOpts{Include: []string{
		"docs/demos/*.html",
		"docs/site/components/*.html",
		"docs/site/js/**",
		"docs/site/shadless.js",
	}}), nil
}

// ExampleFixtureCheck reports the step's own verdict without exporting.
func (m *Shadless) ExampleFixtureCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.exampleFixture(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}

// DemoRtl derives the AR/HE/EN/FA variants from the Arabic oracle page and
// upstream's own aria dictionaries.
//
// docs/demos comes from the two steps that write it rather than from the host:
// the tool reads docs/demos/<name>.html as the base it substitutes into, and
// those pages are the oracle's output as revised by the fixture generator. On
// the host that correspondence held by ordering over a shared working tree —
// which is to say, by nothing that would notice if it stopped holding, and it
// had already stopped: see the note on the mount below.
//
// dist/components starts empty for the reason the oracle's does: the tool
// writes one file per variant and removes nothing, so a mounted tree would
// keep the variants of a retired example forever.
func (m *Shadless) demoRtl(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	oracle, err := m.ExampleOracle(ctx, source)
	if err != nil {
		return nil, err
	}
	fixture, err := m.ExampleFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/w/tools/build-rtl.mjs", source.File("tools/build-rtl.mjs")).
		WithFile("/w/tools/rtl-lib.mjs", source.File("tools/rtl-lib.mjs")).
		WithDirectory("/w/src", source.Directory("src")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/examples/aria",
			source.Directory(".upstream/shadcn-ui/apps/v4/examples/aria")).
		WithDirectory("/w/docs/demos", oracle.Directory("docs/demos")).
		// The fixture pages OVERWRITE the oracle's for the 105 targets that
		// carry kernel families, and four of the -rtl base pages this step
		// substitutes into are among them. Mounting the oracle alone produced
		// a build/rtl-langs.json missing alert-dialog-rtl, breadcrumb-rtl,
		// button-group-rtl and carousel-rtl — measured, and the reason
		// pipeline/nodes.go now gives demo-rtl a `needs: example-fixture` it
		// never had.
		WithDirectory("/w/docs/demos", fixture.Directory("docs/demos")).
		WithDirectory("/w/dist/components", dag.Directory()).
		WithExec([]string{"node", "tools/build-rtl.mjs"}), nil
}

// DemoRtl returns the RTL pages and the language manifest, laid out as the
// repo lays them out.
//
// The oracle pages this step read are filtered back out: they were the input,
// and only the *-rtl-* files are this step's own output.
func (m *Shadless) DemoRtl(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.demoRtl(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w").Filter(dagger.DirectoryFilterOpts{Include: []string{
		"docs/demos/*-rtl-*.html",
		"dist/components/*-rtl-*.html",
		"build/rtl-langs.json",
	}}), nil
}

// DemoRtlCheck reports the step's own verdict without exporting anything.
func (m *Shadless) DemoRtlCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.demoRtl(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}
