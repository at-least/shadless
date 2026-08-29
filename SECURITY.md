# Security

shadless ships static CSS and a vanilla-JS runtime; it makes no network
requests and stores nothing but the theme preference (`localStorage`
key `shadless-theme`). The runtime clones `<template>` content that the
page author wrote into the same document — it never inserts strings from
untrusted input.

## Reporting a vulnerability

Please do not open a public issue for a security problem. Report it
privately through the repository's GitHub security advisory ("Report a
vulnerability") so a fix can ship before disclosure. Include the affected
export (`shadless/runtime`, `shadless/js/<name>`, a stylesheet), the pinned
upstream tag from `src/registry/pin.json`, and a minimal page that
reproduces it.

## Supply chain

- The package installs **no dependencies** (`gates/pack.mjs` fails the
  build if `dependencies` is ever non-empty); `tailwindcss` is an optional
  peer for the CSS-import path.
- The vendored radix kernel (`vendor/radix-kernel.iife.js`) is pinned by
  sha256 in `src/registry/pin.json`; the `pin` gate verifies it on every
  run. `vendor/embla-carousel.iife.js` is the upstream release artifact of
  the `embla-carousel` version in `package-lock.json`.
- The upstream registry is pinned to a tag + commit; the nightly drill
  re-pins only through a reviewed pull request.
