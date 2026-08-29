// Per-component contract: context-menu (Wave B, kernel wireMenu, point anchor)
export default {
  name: "context-menu",

  usage: `
React.createElement(ContextMenu, null,
  React.createElement(ContextMenuTrigger, { id: "d1-trigger" }, "Right click here"),
  React.createElement(ContextMenuContent, null,
    React.createElement(ContextMenuItem, { id: "d1-item-1" }, "Item 1"),
    React.createElement(ContextMenuItem, null, "Item 2"),
  ),
)`,

  imports: `
import { ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem } from "@/registry/bases/radix/ui/context-menu";
`,

  slots: ["context-menu-trigger", "context-menu-content", "&[role=menuitem]"],

  open: `await page.click("#d1-trigger", { button: "right" })`,
  openShadless: `await page.click("#d1-trigger", { button: "right" })`,
  triggerSlot: "context-menu-trigger",
  contentSlot: "context-menu-content",

  shadlessPage: "src/kernel/context-menu.html",

  stateProbe: `(function () {
    var c = document.querySelector("[data-slot=context-menu-content]");
    var h = document.querySelector("[data-highlighted]");
    return (c ? "open" : "closed") + "|" + (h ? h.textContent.trim() : "-");
  })()`,

  scenarios: ["escape", "outside-click", "click:[role=menuitem]",
    "key:ArrowDown", "key:ArrowDown+key:Enter"],
}
