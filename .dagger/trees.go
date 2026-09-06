package main

// The two trees that no single step owns.
//
// Both are assembled from several producers, and both are needed by every
// remaining consumer — dist-complete, pack, the parity gates, consumer-sim.
// Getting either one short does not fail loudly: it shows up as a stylesheet
// missing a rule (measured: .outline-card) or a page quietly absent.
//
// Written while porting the docs chain, which was then STOPPED on purpose.
// Most of that subsystem mirrors upstream's React documentation — 65 pages
// teaching `npx shadcn add` and `import { Accordion } from …` for a product
// that ships static HTML and a vanilla runtime — and the plan is to replace it
// rather than move it. See the memory note; these two helpers are what
// survived, because they have nothing to do with docs.

import (
	"context"

	"dagger/shadless/internal/dagger"
)

// distTree is every step's dist/ output, merged.
//
// Six steps write into dist/ and no single one of them holds it: emit the
// static pages, demo the rest plus globals and the per-component @apply
// sources, demo-rtl the language variants, build-js the JS surface (which
// Demo mounts into its dist), demo-css the stylesheet every page loads, and
// product-css/product-build the consumer-facing entry. The host graph splits
// ownership between them for freshness; what docs-build needs is the union.
func (m *Shadless) distTree(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	demo, err := m.Demo(ctx, source)
	if err != nil {
		return nil, err
	}
	rtl, err := m.DemoRtl(ctx, source)
	if err != nil {
		return nil, err
	}
	outCSS, err := m.DemoCss(ctx, source)
	if err != nil {
		return nil, err
	}
	product, err := m.ProductCss(ctx, source)
	if err != nil {
		return nil, err
	}
	full, err := m.ProductBuild(ctx, source)
	if err != nil {
		return nil, err
	}
	return demo.
		WithDirectory("components", rtl.Directory("dist/components")).
		WithFile("out.css", outCSS).
		WithDirectory(".", product).
		WithDirectory(".", full), nil
}

// demosTree is docs/demos as it ships: hand-authored underneath, generated on
// top.
//
// The directory is MIXED — the 11 message-scroller pages cannot be bundled at
// all and stay hand-authored, along with the demos that have no upstream
// example. Mounting only the generated halves is what dropped .outline-card
// from out.css, so source goes down first and the producers layer over it.
func (m *Shadless) demosTree(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	fixture, err := m.ExampleFixture(ctx, source)
	if err != nil {
		return nil, err
	}
	rtl, err := m.DemoRtl(ctx, source)
	if err != nil {
		return nil, err
	}
	return source.Directory("docs/demos").
		WithDirectory(".", fixture.Directory("docs/demos")).
		WithDirectory(".", rtl.Directory("docs/demos")), nil
}
