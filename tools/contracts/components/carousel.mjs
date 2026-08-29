// Per-component contract: carousel (Wave E, external — embla vanilla port)
// oracle = shadcn carousel.tsx running React embla-carousel-react
// shadless = vanilla embla-carousel IIFE + glue JS
// Both sides expose window.__api (embla api) for behavioral state probing.
export default {
  name: "carousel",
  usage: `
React.createElement(Carousel, { setApi: function (api) { window.__api = api; } },
  React.createElement(CarouselContent, null,
    React.createElement(CarouselItem, null, "1"),
    React.createElement(CarouselItem, null, "2"),
    React.createElement(CarouselItem, null, "3"),
    React.createElement(CarouselItem, null, "4"),
    React.createElement(CarouselItem, null, "5"),
  ),
  React.createElement(CarouselPrevious, null),
  React.createElement(CarouselNext, null),
)`,
  imports: `
import { Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext } from "@/registry/bases/radix/ui/carousel";
`,
  slots: ["carousel", "carousel-content", "carousel-item", "carousel-previous", "carousel-next"],
  scenarios: [
    "click:[data-slot=carousel-next]",
    "click:[data-slot=carousel-next]+click:[data-slot=carousel-next]+click:[data-slot=carousel-next]+click:[data-slot=carousel-next]",
    "click:[data-slot=carousel-next]+click:[data-slot=carousel-previous]",
    "focus:[data-slot=carousel-previous]+key:ArrowRight",
    "focus:[data-slot=carousel-previous]+key:ArrowLeft",
  ],
  stateProbe: `
var api = window.__api;
"snap=" + api.selectedScrollSnap() + " prev=" + api.canScrollPrev() + " next=" + api.canScrollNext();
`,
  oracleCss: `
[data-slot="carousel-content"] { overflow: hidden; }
[data-slot="carousel-content"] > div { display: flex; }
[data-slot="carousel-item"] { flex: 0 0 100%; min-width: 0; }
`,
  shadlessPage: "probes/t8/carousel.html",
  // data-orientation: shadless fixture passes axis to embla; oracle keeps it
  //   in React context only (no DOM attr). style: shadless fixture uses inline
  //   styles for embla layout (oracle uses class=, which the recorder ignores).
  ignoreAttrs: {
    "carousel": ["data-orientation"],
    "carousel-content": ["style"],
    "carousel-item": ["style"],
  },
}
