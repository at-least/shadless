// embla-carousel-autoplay stub: plugin object whose no-op setup keeps the
// carousel demo's initial render identical (autoplay is runtime behavior).
// options must be an object — embla's arePluginsEqual Object.keys()s it.
export default function Autoplay() {
  return { name: "autoplay-stub", options: {}, init: () => () => {} }
}
