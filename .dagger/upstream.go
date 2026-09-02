package main

// Upstream's own docs site, built from the pinned checkout and served.
//
// The golden hop (hop 1) compares this repo's React render against the DOM
// ui.shadcn.com actually ships. That comparison had an unpinned side: the live
// site serves whatever is deployed today, while .upstream sits at the tag
// src/registry/pin.json names. The snapshot is committed, so the gate is
// stable — but nothing recorded which upstream version it was crawled at, so a
// re-pin without a re-crawl silently compares pages built from one release
// against DOM captured from another, and every cell looks like a regression.
//
// Building upstream's app here removes the mismatch instead of detecting it.
// The site comes from the same commit the pin names, by construction.
//
// It also makes the snapshot reproducible by anyone rather than by whoever last
// had network and remembered to run `make upstream-snapshot`, which is what
// lets it become a graph node — and that closes one of the four reasons the
// re-pin drill can never go green (the golden snapshot being refreshed AFTER
// the drill compares against it).
//
// Nothing here names a version. node comes from the image, pnpm from
// upstream's own `packageManager` field, every package from its lockfile, and
// NEXT_PUBLIC_APP_URL from its own .env.example.

import (
	"context"
	"fmt"
	"strings"

	"dagger/shadless/internal/dagger"
)

const upstreamPort = 4000

// The bun upstream's registry:build needs and upstream does not pin. See the
// note in upstreamBuilt for why this is named here rather than read.
const upstreamBunVersion = "1.4.0"

// packageManagerPin reads upstream's own declaration of which pnpm to use.
func packageManagerPin(ctx context.Context, source *dagger.Directory) (string, error) {
	pkg, err := source.File(".upstream/shadcn-ui/package.json").Contents(ctx)
	if err != nil {
		return "", fmt.Errorf("reading upstream package.json: %w", err)
	}
	for _, line := range strings.Split(pkg, "\n") {
		if !strings.Contains(line, `"packageManager"`) {
			continue
		}
		return strings.Trim(strings.TrimSpace(strings.SplitN(line, ":", 2)[1]), `",`), nil
	}
	return "", fmt.Errorf(".upstream/shadcn-ui/package.json has no packageManager field to read pnpm's version from")
}

// upstreamBuilt installs and builds upstream's docs app.
//
// .git is filtered out: it is a third of the checkout and nothing in the build
// reads it, so including it would put every upstream commit into this layer's
// cache key.
func (m *Shadless) upstreamBuilt(ctx context.Context, source *dagger.Directory) (*dagger.Container, error) {
	pm, err := packageManagerPin(ctx, source)
	if err != nil {
		return nil, err
	}
	return dag.Container().
		From(mirrored("node:22-bookworm")).
		WithExec([]string{"npm", "install", "-g", "corepack@latest"}).
		WithExec([]string{"corepack", "enable"}).
		// registry:build ends in `bun run ./scripts/build-registry.mts`, and
		// upstream declares bun NOWHERE — not in a package.json, not in the
		// lockfile. So unlike everything else here, this version has no source
		// to be read from and is named below.
		//
		// That is not the thing this module's header refuses. It refuses a
		// SECOND declaration of a version the repo already records, free to
		// drift from the first (an image tag beside a lockfile). bun has no
		// first declaration anywhere, so recording it once, here, IS the
		// human decision the header asks for.
		//
		// tsx was tried instead and does not work, twice. build-registry.mts
		// uses no bun API, and upstream's root devDependencies carry tsx
		// (4.20.3, lockfile-pinned) which four other .mts scripts use — but
		// the script imports registry/styles.tsx, whose `<svg>` JSX sits
		// outside tsconfig.scripts.json's `scripts/**` include. esbuild falls
		// back to the classic transform there, emits React.createElement and
		// dies with `ReferenceError: React is not defined`; bun defaults to
		// the automatic runtime. A tsconfig extending upstream's and forcing
		// their own react-jsx setting did not reach the imported file either.
		// Do not re-attempt without solving that specifically.
		WithExec([]string{"npm", "install", "-g", "bun@" + upstreamBunVersion}).
		WithDirectory("/u", source.Directory(".upstream/shadcn-ui").Filter(
			dagger.DirectoryFilterOpts{Exclude: []string{".git/**"}})).
		WithWorkdir("/u").
		WithExec([]string{"corepack", "use", pm}, dagger.ContainerWithExecOpts{Expect: dagger.ReturnTypeAny}).
		WithExec([]string{"pnpm", "install", "--frozen-lockfile"}).
		WithWorkdir("/u/apps/v4").
		WithEnvVariable("NEXT_TELEMETRY_DISABLED", "1").
		// the app reads NEXT_PUBLIC_APP_URL in twelve places and throws
		// `TypeError: Invalid URL` during page-data collection without it;
		// .env.example is upstream's own answer for what it should be
		WithExec([]string{"cp", ".env.example", ".env"}).
		WithExec([]string{"pnpm", "registry:build"}).
		WithExec([]string{"pnpm", "exec", "next", "build"}), nil
}

// UpstreamSite serves the built app. Bind it and crawl it.
func (m *Shadless) UpstreamSite(ctx context.Context, source *dagger.Directory) (*dagger.Service, error) {
	c, err := m.upstreamBuilt(ctx, source)
	if err != nil {
		return nil, err
	}
	return c.
		WithExposedPort(upstreamPort).
		AsService(dagger.ContainerAsServiceOpts{
			Args: []string{"pnpm", "exec", "next", "start", "--port", fmt.Sprint(upstreamPort)},
		}), nil
}

// UpstreamSnapshot crawls the pinned site and returns the golden snapshot.
//
// The crawler's ORIGIN is the only thing that changes; which base it reads and
// which pages it walks still come from src/registry/pin.json and the pinned
// checkout's own mdx, so this cannot drift from what the rest of the pipeline
// targets.
func (m *Shadless) UpstreamSnapshot(ctx context.Context, source *dagger.Directory) (*dagger.Directory, error) {
	site, err := m.UpstreamSite(ctx, source)
	if err != nil {
		return nil, err
	}
	img, err := goImage(ctx, source)
	if err != nil {
		return nil, err
	}
	c := dag.Container().
		From(img).
		WithWorkdir("/w").
		WithDirectory("/w/pipeline", source.Directory("pipeline")).
		WithExec([]string{"go", "build", "-o", "/usr/local/bin/pipeline", "./pipeline"}).
		WithServiceBinding("site", site).
		WithEnvVariable("SHADLESS_SNAPSHOT_ORIGIN", fmt.Sprintf("http://site:%d", upstreamPort)).
		WithFile("/w/src/registry/pin.json", source.File("src/registry/pin.json")).
		WithDirectory("/w/.upstream/shadcn-ui/apps/v4/content/docs",
			source.Directory(".upstream/shadcn-ui/apps/v4/content/docs")).
		WithWorkdir("/w").
		WithExec([]string{"pipeline", "upstream-snapshot"})
	return c.Directory("/w/src/registry/upstream-snapshot"), nil
}
