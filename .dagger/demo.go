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
// The self-test renders from build/fixture, a scratch tree the tool assembles
// from the JS it just built plus dist/out.css. out.css is the awkward part, on
// the host too: demo-css writes it LATER in the graph, so the self-test always
// renders against the PREVIOUS build's stylesheet. Mounting the committed copy
// here reproduces that exactly rather than papering over it.
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
	bin, err := goBinary(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/usr/local/bin/pipeline", bin).
		WithFile("/w/docs/example-fixture-targets.json",
			oracle.File("docs/example-fixture-targets.json")).
		WithDirectory("/w/docs/demos", oracle.Directory("docs/demos")).
		// the JS surface from the build step, not from the host tree
		WithDirectory("/w/dist/js", js.Directory("js")).
		WithFile("/w/dist/shadless.js", js.File("shadless.js")).
		WithFile("/w/dist/out.css", source.File("dist/out.css")).
		WithExec([]string{"pipeline", "example-fixture"}), nil
}

// ExampleFixture returns the interactive pages the self-test proved.
//
// Nothing else the step writes is exported: build/fixture is scratch the
// self-test renders from and dies with the container.
func (m *Shadless) ExampleFixture(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.exampleFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w").Filter(dagger.DirectoryFilterOpts{Include: []string{
		"docs/demos/*.html",
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

// RtlDict lifts the RTL translation dictionaries out of upstream's aria
// registry into a file this repo owns.
//
// It is the only step that touches examples/aria. This repo targets the radix
// registry, and the dictionaries are the one thing anything wanted from aria —
// plain {en,ar,he} -> {dir,values} strings, not a React Aria artifact.
func (m *Shadless) RtlDict(ctx context.Context, source *dagger.Directory) (*dagger.File, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/w/tools/rtl-dict.mjs", source.File("tools/rtl-dict.mjs")).
		WithFile("/w/tools/rtl-lib.mjs", source.File("tools/rtl-lib.mjs")).
		WithFile("/w/src/registry/tiers.json", source.File("src/registry/tiers.json")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/examples/aria",
			source.Directory(".upstream/shadcn-ui/apps/v4/examples/aria")).
		WithExec([]string{"node", "tools/rtl-dict.mjs"}).
		File("/w/src/registry/rtl-translations.json"), nil
}

// DemoRtl derives the AR/HE/EN/FA variants from the Arabic oracle page and
// upstream's own aria dictionaries.
//
// The dictionaries come from RtlDict; this step no longer reads examples/aria
// at all.
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
	dict, err := m.RtlDict(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/w/tools/build-rtl.mjs", source.File("tools/build-rtl.mjs")).
		WithFile("/w/tools/rtl-lib.mjs", source.File("tools/rtl-lib.mjs")).
		WithDirectory("/w/src", source.Directory("src")).
		// from the step that produces it, not the committed copy
		WithFile("/w/src/registry/rtl-translations.json", dict).
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

// contractFixture harvests the kernel contract fixtures (src/kernel/*.html)
// from the contract defs' own React render.
//
// This is the one step that legitimately depends on EVERY contract def, so it
// mounts the directory renderBase filters out. The filter exists so that
// editing one def does not invalidate the layer all 29 contract replays build
// on; this step is not one of those, and it genuinely reads them all.
//
// src/kernel starts empty for the reason the oracle's docs/demos does: the
// tool writes one file per def and removes nothing, so a retired def's fixture
// would live forever in a mounted tree.
func (m *Shadless) contractFixture(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.renderBase(ctx, source)
	if err != nil {
		return nil, err
	}
	js, err := m.BuildJs(ctx, source)
	if err != nil {
		return nil, err
	}
	bin, err := goBinary(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/usr/local/bin/pipeline", bin).
		WithDirectory("/w/tools/contracts/components",
			source.Directory("tools/contracts/components")).
		WithDirectory("/w/src/kernel", dag.Directory()).
		WithDirectory("/w/dist/js", js.Directory("js")).
		WithFile("/w/dist/shadless.js", js.File("shadless.js")).
		// the fixture pages link ../../dist/out.css, which is demo-css's
		// output and therefore not available until later in the graph — the
		// same previous-build dependency example-fixture has on docs/site
		WithFile("/w/dist/out.css", source.File("dist/out.css")).
		WithExec([]string{"pipeline", "example-fixture", "--contracts"}), nil
}

// ContractFixture returns the harvested kernel fixtures.
func (m *Shadless) ContractFixture(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.contractFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w/src/kernel"), nil
}

// ContractFixtureCheck reports the step's own verdict without exporting.
func (m *Shadless) ContractFixtureCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.contractFixture(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}

// demoTree assembles the browsable dist: the unified globals.css with every
// slot rule folded in, the demo index, the per-component @apply sources the npm
// surface exports, and a page for each non-static component.
//
// No browser: this step composes fixtures other steps rendered.
//
// It does need the Go binary, because tools/demo.mjs spawns `./build/pipeline
// build-js` itself (the builder stopped living in JS). That is a genuine
// redundancy in the host graph — demo both `needs` build-js and re-runs it —
// and mounting the binary reproduces the tool as written rather than quietly
// improving it.
//
// generated/ir and src/kernel come from the steps that produce them, not
// from the host tree: the IR from the conversion, the kernel fixtures from the
// contract harvest. dist/components comes from emit, which is a real
// dependency and not just ordering — demo throws if a static page is missing.
func (m *Shadless) demoTree(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	ir, err := m.Convert(ctx, source)
	if err != nil {
		return nil, err
	}
	kernel, err := m.ContractFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	emitted, err := m.Emit(ctx, source)
	if err != nil {
		return nil, err
	}
	bin, err := m.pipelineBin(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithDirectory("/w/src", source.Directory("src").Filter(
			dagger.DirectoryFilterOpts{Exclude: []string{"registry/ir/**", "kernel/**"}})).
		WithDirectory("/w/generated/ir", ir).
		WithDirectory("/w/src/kernel", kernel).
		WithFile("/w/tools/demo.mjs", source.File("tools/demo.mjs")).
		WithFile("/w/tools/demo-lib.mjs", source.File("tools/demo-lib.mjs")).
		WithDirectory("/w/probes", source.Directory("probes")).
		WithDirectory("/w/vendor", source.Directory("vendor")).
		// src/emitter/css.mjs reads upstream's style-nova.css, and demo
		// imports componentCss from it. The host node does not declare that
		// file and does not have to: `convert` does, and convert is in demo's
		// closure, so it already reaches demo's key. The access check is
		// therefore right to stay quiet. A container asks a different
		// question — is the file THERE — and the two are not the same
		// property. This mount answers the second one.
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/registry",
			source.Directory(".upstream/shadcn-ui/apps/v4/registry")).
		WithDirectory("/w/dist/components", emitted.Directory("components")).
		WithFile("/w/build/pipeline", bin, dagger.ContainerWithFileOpts{Permissions: 0o755}).
		WithExec([]string{"node", "tools/demo.mjs"}), nil
}

// Demo returns the whole dist tree at this point in the graph, which is what
// the directory means: emit's static pages, this step's own, and the JS surface
// the tool rebuilt on its way through. The per-node ownership split lives in
// pipeline/nodes.go, where the freshness check needs it; here the useful unit
// is the shipped surface.
//
// The RTL variants are NOT in it — DemoRtl writes those, from the oracle side
// of the graph.
func (m *Shadless) Demo(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.demoTree(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w/dist"), nil
}

// DemoCheck reports the step's own verdict without exporting.
func (m *Shadless) DemoCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.demoTree(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}
