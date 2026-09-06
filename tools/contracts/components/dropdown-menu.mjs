// Per-component contract: dropdown-menu (Wave B, kernel wireMenu)
export default {

  usage: `
React.createElement(DropdownMenu, null,
  React.createElement(DropdownMenuTrigger, { id: "d1-trigger" }, "Open menu"),
  React.createElement(DropdownMenuContent, null,
    React.createElement(DropdownMenuItem, { id: "d1-item-1" }, "Item 1"),
    React.createElement(DropdownMenuItem, null, "Item 2"),
  ),
)`,

  imports: `
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem } from "@/registry/bases/radix/ui/dropdown-menu";
`,

  slots: ["dropdown-menu-trigger", "dropdown-menu-content", "&[role=menuitem]"],

  open: `await page.click("#d1-trigger")`,
  openShadless: `await page.click("#d1-trigger")`,
  triggerSlot: "dropdown-menu-trigger",
  contentSlot: "dropdown-menu-content",

  shadlessPage: "src/kernel/dropdown-menu.html",

  ignoreAttrs: {
    "dropdown-menu-trigger": ["data-radix-popper-align"],
    "dropdown-menu-content": ["data-align"],
  },

  stateProbe: `(function () {
    var c = document.querySelector("[data-slot=dropdown-menu-content]");
    var h = document.querySelector("[data-highlighted]");
    return (c ? "open" : "closed") + "|" + (h ? h.textContent.trim() : "-");
  })()`,

  scenarios: ["escape", "outside-click", "mouse-click:[data-slot=dropdown-menu-trigger]", "click:[role=menuitem]",
    "key:ArrowDown", "key:ArrowDown+key:ArrowDown", "key:ArrowDown+key:Enter", "key:ArrowDown+key:ArrowUp"],
}
