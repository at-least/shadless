// Per-component contract: tabs (Wave B, kernel wireTabs; always rendered).
// stateProbe = index of the aria-selected trigger (tabs have no open/closed).
// "&<css>" slots = raw selectors (inactive panel is absent in oracle DOM —
// radix unmounts it; we record only the visible panel).
export default {

  usage: `
React.createElement(Tabs, { defaultValue: "account" },
  React.createElement(TabsList, null,
    React.createElement(TabsTrigger, { id: "t1", value: "account" }, "Account"),
    React.createElement(TabsTrigger, { id: "t2", value: "password" }, "Password"),
  ),
  React.createElement(TabsContent, { id: "d1", value: "account" }, "Make changes to your account here."),
  React.createElement(TabsContent, { id: "d2", value: "password" }, "Enter your password here."),
)`,

  imports: `
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/registry/bases/radix/ui/tabs";
`,

  slots: [
    "&[data-slot=tabs][data-orientation=horizontal]",
    "&[data-slot=tabs-list]",
    "&[data-slot=tabs-trigger]#t1",
    "&[data-slot=tabs-trigger]#t2",
    "&[data-slot=tabs-content]:not([hidden])",
  ],

  // no open step — tabs render at rest (defaultValue=account active)
  shadlessPage: "src/kernel/tabs.html",

  stateProbe: `String([].slice.call(document.querySelectorAll("[data-slot=tabs-trigger]")).findIndex(function (t) { return t.getAttribute("aria-selected") === "true" }))`,

  scenarios: [
    "click:#t2",
    "focus:#t1+key:ArrowRight",
    "focus:#t2+key:ArrowLeft",
    "focus:#t2+key:Home",
    "focus:#t1+key:End",
  ],
}
