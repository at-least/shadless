// Per-component contract: avatar (Wave C, trivial runtime)
export default {
  usage: `
React.createElement("div", null,
  React.createElement(Avatar, { id: "av1" },
    React.createElement(AvatarImage, { id: "avimg", src: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16'%3E%3Crect width='16' height='16' fill='red'/%3E%3C/svg%3E" }),
    React.createElement(AvatarFallback, { id: "avfb" }, "CN"),
  ),
  React.createElement(Avatar, { id: "av2" },
    React.createElement(AvatarImage, { id: "avimg2", src: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUg" }),
    React.createElement(AvatarFallback, { id: "avfb2" }, "XX"),
  ),
)`,
  imports: `
import { Avatar, AvatarImage, AvatarFallback } from "@/registry/bases/radix/ui/avatar";
`,
  slots: ["avatar", "avatar-image", "avatar-fallback"],
  scenarios: [],
  stateProbe: `
var q = function (s) { return document.querySelector(s) ? "present" : "absent" };
"img=" + q("[data-slot=avatar-image]") + "|fb=" + q("[data-slot=avatar-fallback]") + "|av=" + q("[data-slot=avatar]");
`,
  shadlessPage: "probes/t7/avatar.html",
}
