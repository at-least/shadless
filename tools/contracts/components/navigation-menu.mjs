// Per-component contract: navigation-menu (medium tier — viewport menu)
export default {
  name: "navigation-menu",

  usage: `
React.createElement(NavigationMenu, { viewport: true },
  React.createElement(NavigationMenuList, null,
    React.createElement(NavigationMenuItem, null,
      React.createElement(NavigationMenuTrigger, { id: "d1-trigger" }, "Getting started"),
      React.createElement(NavigationMenuContent, null,
        React.createElement("ul", { className: "grid gap-2 p-4", style: { gridTemplateColumns: "200px" } },
          React.createElement("li", null,
            React.createElement(NavigationMenuLink, null, "Documentation"))),
      ),
    ),
    React.createElement(NavigationMenuItem, null,
      React.createElement(NavigationMenuTrigger, { id: "d2-trigger" }, "Components"),
      React.createElement(NavigationMenuContent, null,
        React.createElement("ul", { className: "grid gap-2 p-4", style: { gridTemplateColumns: "200px" } },
          React.createElement("li", null,
            React.createElement(NavigationMenuLink, null, "Dialog"))),
      ),
    ),
  ),
  React.createElement(NavigationMenuViewport, { id: "d1-viewport" }),
)`,

  imports: `
import { NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuViewport, NavigationMenuLink } from "@/registry/bases/radix/ui/navigation-menu";
`,

  slots: ["navigation-menu", "navigation-menu-list", "navigation-menu-trigger", "navigation-menu-viewport"],

  open: `await page.click("#d1-trigger")`,
  openShadless: `await page.click("#d1-trigger")`,
  triggerSlot: "navigation-menu-trigger",
  contentSlot: "navigation-menu-content",

  // mountedCheck OFF (recorded): radix mounts MORE than the opened
  // content into the viewport machinery (sibling items' links appear in
  // the mounted set) while the shadless glue mounts only the active one —
  // a real structural difference needing its own investigation, not a
  // fixture patch. Track before enabling.
  mountedCheck: false,
  shadlessPage: "src/kernel/navigation-menu.html",

  stateProbe: `(function () {
    var vp = document.querySelector("[data-slot=navigation-menu-viewport]");
    var c = document.querySelector("[data-slot=navigation-menu-content]");
    var t1 = document.getElementById("d1-trigger"), t2 = document.getElementById("d2-trigger");
    var f = function (t) { return t.getAttribute("data-state") + "/" + t.getAttribute("aria-expanded"); };
    return "vp=" + (vp ? (vp.getAttribute("data-state") + "/" + vp.textContent.trim().slice(0, 10)) : "-")
      + "|c=" + (c ? "open" : "closed")
      + "|" + f(t1) + "|" + f(t2)
      + "|ae=" + (document.activeElement.id || document.activeElement.tagName);
  })()`,

  scenarios: [
    "key:Escape",
    "key:Escape+key:ArrowRight",
    "key:ArrowRight",
    "click:[data-slot=navigation-menu-link]",
    "click:#d1-trigger",
  ],
}
