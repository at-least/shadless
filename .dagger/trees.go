package main

// The docs/demos tree that no single step owns.
//
// It is assembled from several producers and needed by every consumer that
// scans it — css.go's scanTree included, which is why this lives apart from
// css.go rather than inside it. Getting it short does not fail loudly: it
// shows up as a stylesheet missing a rule (measured: .outline-card) or a page
// quietly absent.
//
// Written while porting the docs chain, which was then STOPPED on purpose.
// Most of that subsystem mirrors upstream's React documentation — 65 pages
// teaching `npx shadcn add` and `import { Accordion } from …` for a product
// that ships static HTML and a vanilla runtime — and the plan is to replace it
// rather than move it. See the memory note; this helper is what survived,
// because it has nothing to do with docs.

import (
	"context"

	"dagger/shadless/internal/dagger"
)

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
