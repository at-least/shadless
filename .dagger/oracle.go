package main

// The oracle slice: the pinned upstream examples rendered by real React and
// real chromium, and the two hops that use that render as the authority.
//
// This is the expensive half of the pipeline and the reason the port exists at
// all. `example-oracle` is 56s on the host, `golden-gate` 47s, `example-gate`
// 27s, and every one of them needs a browser, a bundler and the resolved
// registry — the combination that has no Go equivalent.
//
// One thing changes meaning here, and it is worth being explicit about.
//
// On the host, `example-gate` is tautological. Its closure contains
// `example-oracle`, which WRITES the pages the gate then re-renders and
// compares — the same tool, the same inputs, seconds apart, so the comparison
// cannot fail for any reason a reviewer cares about. Here the two sides come
// from different places by construction: the pages are mounted from `source`
// (what is committed) and the render happens in the container. Nothing in the
// function can make them agree.

import (
	"context"

	"dagger/shadless/internal/dagger"
)

// renderBase is the browser half of the pipeline: chromium plus everything the
// React oracle bundles, and nothing that is downstream of a render.
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
//
// oracleBase is this plus the SHIPPED tree (dist/, probes/). The contract
// replays need those — they compare the oracle against the page this repo
// ships — but a pure upstream render never reads them, and mounting them would
// put every rebuild of dist/ into the key of a step whose output cannot depend
// on it.
func (m *Shadless) renderBase(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.deps(ctx, source)
	if err != nil {
		return nil, err
	}
	resolved, err := m.ResolvedUI(ctx, source)
	if err != nil {
		return nil, err
	}
	return withBrowser(c).
		// The contract DEFS are filtered out and mounted per contract instead.
		//
		// Filter, not WithDirectory's Exclude: Exclude narrows what gets
		// WRITTEN but the cache key still comes from the source directory's
		// full digest, so editing an excluded file invalidated the layer
		// anyway. Filter produces a new snapshot whose digest reflects only
		// what it kept. Measured with Exclude: touching one def re-ran all 29
		// (107s against 1.5s fully cached) and individually re-ran dialog,
		// select and tabs at 16-20s each.
		// Folder granularity is right for declarations but wrong for cache
		// keys here: with the whole of tools/ in the shared base, editing one
		// def invalidated the layer every contract builds on and all 29
		// re-ran. Measured — 2min+ instead of the ~10s one contract costs.
		WithDirectory("/w/tools", source.Directory("tools").Filter(
			dagger.DirectoryFilterOpts{Exclude: []string{"contracts/components/**"}})).
		// gates/ledger.json — golden-gate reads the exemption budgets from it.
		WithDirectory("/w/gates", source.Directory("gates")).
		WithDirectory("/w/src", source.Directory("src")).
		// oracleBundleCacheKey re-reads these two files at RUNTIME as cache-key
		// material (everything else it hashes is compiled into the binary or
		// already mounted: package-lock.json via deps, skin.mjs via src/,
		// stubs/ via tools/). A missing one is an ENOENT at the point of use.
		WithFile("/w/pipeline/resolve_skins.go", source.File("pipeline/resolve_skins.go")).
		WithFile("/w/pipeline/oracle_lib.go", source.File("pipeline/oracle_lib.go")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4",
			source.Directory(".upstream/shadcn-ui/apps/v4")).
		WithDirectory("/w/build/resolved-ui", resolved).
		WithMountedCache(oracleCache, dag.CacheVolume("shadless-oracle-bundles")), nil
}

// rendered is the container after tools/example-oracle.mjs has run: the
// upstream examples turned into the demo pages this repo ships.
//
// docs/demos starts EMPTY rather than mounted from the host. The tool writes
// one file per target and removes nothing, so a page whose upstream example
// has been retired would survive in a mounted tree forever — the same way
// generated/ir/form.json did, for the same reason, until the conversion
// step here started from an empty directory. Starting empty makes the output
// exactly the set that rendered.
//
// Only docs/catalog.json is mounted from docs/: it is the target list, and it
// is the one file under docs/ this step reads.
//
// KNOWN DIVERGENCE: the committed pages embed MEASURED geometry, so they are
// reproducible only on a machine whose font rendering matches the one that
// produced them. Measured across this slice, 27 files of 635 differ, from six
// base pages:
//
//	docs/demos/accordion-rtl.html   --radix-collapsible-content-height
//	                                36px here, 19px committed — the same
//	                                Arabic string wrapping to two lines
//	tooltip-*, popover-rtl,         --radix-popper-transform-origin and the
//	hover-card-rtl, kbd-tooltip     arrow's left: floating-ui positions
//	                                computed from rendered TEXT WIDTH, e.g.
//	                                28.1485px here against
//	                                27.991999999999997px committed
//
// plus the 16 RTL variants tools/build-rtl.mjs derives from those bases.
//
// Note the second group is LATIN text ("Save Changes"). This is not an Arabic
// font gap: the default sans differs enough to move a sub-pixel position, and
// radix serializes that position into the DOM at full float precision. The
// oracle harness loads no CSS at all (oracle-lib.mjs:141), so the default font
// is whatever the system supplies. Installing fonts-noto-core does NOT
// reproduce the committed values (measured), so it is not a missing font.
//
// The fix is not a font package in this file: either these tools stop baking
// measured geometry into committed pages, or the repo declares its font set
// and the pages are regenerated under it. Until then `reproducible` cannot see
// the problem, because everyone runs it on the laptop that produced the bytes.
//
// dist/components exists but is empty for the same reason, and because of what
// the first run in this sandbox found: one target, alert-demo, is written to
// dist/components/alert-demo.html rather than docs/demos (it replaced the
// retired build-demo hand emitter and kept that path). The host graph declares
// that file as demo-build's output — a node whose EMITTERS table is empty and
// which therefore writes nothing at all. Neither host check could see it: the
// write check skips a file whose bytes did not change, and fs-record.mjs
// records reads only. A missing mount, on the other hand, is an ENOENT.
func (m *Shadless) rendered(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	c, err := m.renderBase(ctx, source)
	if err != nil {
		return nil, err
	}
	bin, err := goBinary(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithFile("/usr/local/bin/pipeline", bin).
		WithFile("/w/docs/catalog.json", source.File("docs/catalog.json")).
		WithDirectory("/w/docs/demos", dag.Directory()).
		WithDirectory("/w/dist/components", dag.Directory()).
		WithExec([]string{"pipeline", "example-oracle"}), nil
}

// ExampleOracle returns everything the step writes, laid out as the repo lays
// it out, so `export --path=.` puts each file back where it belongs: the
// rendered demo pages, the two manifests the rest of the chain reads
// (docs/example-oracle.json, which pages the oracle owns, and
// docs/example-fixture-targets.json, which of them go to the fixture
// generator), and the one dist page.
//
// An Include list rather than the whole of /w: catalog.json and every mounted
// input are still sitting in there, and handing an input back as an output is
// how an export overwrites a committed file with itself.
func (m *Shadless) ExampleOracle(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	c, err := m.rendered(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.Directory("/w").Filter(dagger.DirectoryFilterOpts{Include: []string{
		"docs/demos/**",
		"docs/example-oracle.json",
		"docs/example-fixture-targets.json",
		"dist/components/alert-demo.html",
	}}), nil
}

// ExampleOracleCheck reports the render's own verdict without exporting
// anything. A render failure is fatal in the tool: it writes nothing at all
// rather than shrinking its manifest to whatever did render.
func (m *Shadless) ExampleOracleCheck(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.rendered(ctx, source)
	if err != nil {
		return "", err
	}
	return c.Stdout(ctx)
}

// GoldenGate is hop 1: the local React render must equal the committed
// ui.shadcn.com snapshot. It reads only upstream and src/registry/
// upstream-snapshot, so it needs no page this pipeline produces — which is
// what makes it the anchor the other hop hangs off.
func (m *Shadless) GoldenGate(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.renderBase(ctx, source)
	if err != nil {
		return "", err
	}
	bin, err := goBinary(ctx, source)
	if err != nil {
		return "", err
	}
	return c.WithFile("/usr/local/bin/pipeline", bin).
		WithExec([]string{"pipeline", "example-golden"}).Stdout(ctx)
}

// goBinary builds the pipeline binary in the Go toolchain the repo declares.
// The node-image containers (renderBase etc.) run it — Go toolchain absent
// there by design; the binary is the interface.
func goBinary(ctx context.Context, source *dagger.Directory) (*dagger.File, error) {
	img, err := goImage(ctx, source)
	if err != nil {
		return nil, err
	}
	return dag.Container().
		From(img).
		WithDirectory("/src/pipeline", source.Directory("pipeline")).
		WithWorkdir("/src/pipeline").
		WithExec([]string{"go", "build", "-o", "/out/pipeline", "."}).
		File("/out/pipeline"), nil
}

// ExampleGate is hop 2: each shipped demo page must equal a fresh oracle
// render. Together with hop 1 that is the chain shipped == React == live.
//
// The pages and the ownership manifest are mounted from `source` — the
// COMMITTED tree — and the render happens here. On the host this gate cannot
// fail: `example-oracle` sits in its closure and rewrites those same pages
// immediately before it runs. That is not a fixable ordering; it is what
// "everything is one working tree" means. Here it costs one mount.
//
// It currently reports one drift, and that is the gate working. See the
// KNOWN DIVERGENCE note on rendered(): docs/demos/accordion-rtl.html carries a
// font-dependent measurement, so the committed bytes and a fresh render
// genuinely differ. The host version of this gate reports nothing because it
// re-renders its own output; this one has something to compare against.
func (m *Shadless) ExampleGate(ctx context.Context, source *dagger.Directory) (string, error) {
	c, err := m.renderBase(ctx, source)
	if err != nil {
		return "", err
	}
	bin, err := goBinary(ctx, source)
	if err != nil {
		return "", err
	}
	return c.
		WithFile("/usr/local/bin/pipeline", bin).
		WithFile("/w/docs/catalog.json", source.File("docs/catalog.json")).
		WithFile("/w/docs/example-oracle.json", source.File("docs/example-oracle.json")).
		WithDirectory("/w/docs/demos", source.Directory("docs/demos")).
		// the manifest's out paths are not all under docs/demos: alert-demo's
		// is the dist page. Mounting the manifest without it is an ENOENT, not
		// a page that quietly goes unchecked.
		WithFile("/w/dist/components/alert-demo.html",
			source.File("dist/components/alert-demo.html")).
		WithExec([]string{"pipeline", "example-oracle", "--check"}).
		Stdout(ctx)
}
