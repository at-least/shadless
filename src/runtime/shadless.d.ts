// shadless runtime — types for the ES-module entry (dist/esm/shadless.mjs).
// The IIFE build sets the same object on window.shadless.

/** every handle carries the component it belongs to — narrow on it */
export type OpenComponent = "dialog" | "alert-dialog" | "sheet" | "popover" | "tooltip" | "hover-card" |
  "dropdown-menu" | "context-menu" | "menubar" | "navigation-menu"
export interface OpenHandle {
  component: OpenComponent
  open(): void
  close(restoreFocus?: boolean): void
  toggle(): void
  isOpen(): boolean
}
export interface SelectHandle extends Omit<OpenHandle, "component"> {
  component: "select"
  select(item: Element | string): void
  /** the selected option's value: its `value` / `data-value` attribute or id, else its label text */
  value(): string | null
  /** the text shown in the trigger's select-value slot */
  label(): string | null
  /** the selected option element */
  selected(): Element | null
}
export interface TabsHandle {
  component: "tabs"
  activate(index: number, focus?: boolean): void
  active(): number
}
export interface SliderHandle {
  component: "slider"
  values(): number[]
  setValue(value: number, index?: number, opts?: { commit?: boolean }): void
}
/** carousel: the embla-carousel api itself (scrollNext(), scrollTo(i), on("select", …), …) */
export type CarouselHandle = Record<string, any> & { component: "carousel" }
export type Handle = OpenHandle | SelectHandle | TabsHandle | SliderHandle | CarouselHandle

export interface SlotBehavior {
  init?(el: Element): void
  onClick?(el: Element, ctx: { root: Element }, ev: MouseEvent): void
  onKeydown?(el: Element, ctx: { root: Element }, ev: KeyboardEvent): void
}
export interface ComponentDef {
  slots?: Record<string, SlotBehavior>
  init?(root: Element): void
}

export interface EventDetail { component: string; api: Handle | null }
export interface ChangeDetail extends EventDetail {
  checked?: boolean          // checkbox, switch
  pressed?: boolean          // toggle
  value?: string | string[] | null  // radio-group, toggle-group (single | multiple), select — the item's value / data-value attr or id (select: else its label)
  label?: string | null      // select: the option's text
  item?: Element             // radio-group, toggle-group, select
  index?: number             // tabs
  trigger?: Element          // tabs
  values?: number[]          // slider
}
export interface ShadlessEventMap {
  "shadless:open": CustomEvent<EventDetail>
  "shadless:close": CustomEvent<EventDetail>
  "shadless:change": CustomEvent<ChangeDetail>
  /** slider only: once per gesture (radix onValueCommit) — the value to persist */
  "shadless:commit": CustomEvent<EventDetail & { values: number[] }>
  "shadless:themechange": CustomEvent<{ mode: "light" | "dark" }>
}
declare global {
  interface GlobalEventHandlersEventMap extends ShadlessEventMap {}
  interface Window {
    shadless: Shadless
    /** set before the base script runs to skip the DOMContentLoaded init(document.body) */
    shadlessNoAutoInit?: boolean
  }
}

/** instance wiring record — see Helpers.wire */
export interface WireRecord {
  /** pass to every addEventListener the glue adds; destroy(root) aborts it */
  signal: AbortSignal | undefined
  /** runs at destroy (kernel / embla destroy hooks) */
  teardown: (() => void) | null
  /** wiring a vendored engine holds with no unwire: survives destroy, reused by the next init */
  persistent: boolean
}
export interface FormMirrorDef<V> {
  read(): V
  write(v: V): void
}
/** the helpers component files share (radix semantics, measured once) */
export interface Helpers {
  NAV_KEYS: string[]
  /** roving-focus index math: Home → first, End / unknown origin → last, arrows wrap; skips disabled items, swaps horizontal arrows under dir=rtl; -1 when nothing can take focus */
  nextIndex(ev: KeyboardEvent, items: Element[]): number
  isDisabled(el: Element): boolean
  isRtl(el: Element | null): boolean
  setChecked(root: Element, checked: boolean): void
  setGroupItem(item: Element, on: boolean): void
  setRadioItem(item: Element, on: boolean, ctx: { root: Element } | null): void
  setDisclosed(trigger: Element, content: Element | null, open: boolean): void
  linkControls(trigger: Element, content: Element | null): void
  findTemplate(ctx: { root: Element } | null, dataFor: string): HTMLTemplateElement | null
  cloneTemplate(tpl: HTMLTemplateElement | null): Element | null
  /** dispatch shadless:<type> on el (bubbling; detail = { component, api, ...extra }) */
  emit(el: Element, type: "open" | "close" | "change" | "commit", component: string, extra?: Record<string, unknown>): void
  /** the value an item stands for: value attr, data-value, id, else null */
  itemValue(item: Element): string | null
  /** wire el once under the live root: null when already wired */
  wire(el: Element, live?: Element): WireRecord | null
  /** hidden-input mirror for a control carrying a name (null without one); call once at wiring */
  formMirror(root: Element, def: FormMirrorDef<boolean | string | null | number[]>): unknown
  /** refresh the mirror after a state change */
  syncForm(root: Element): void
  /** body-scroll-lock idiom shared by the menu family and select: lock the body and (while locking) restore hit-testing on contentEl; unlock reverts the body only */
  lockBody(on: boolean, contentEl?: Element | null): void
  /** clone tpl's content into a scratch host and return the one slot element found by `slot` (a CSS selector) */
  mountFromTemplate(tpl: HTMLTemplateElement, slot: string): Element | null
  /** wire a kernel's { on<Event>: handler } map onto trigger, aborting every listener with signal */
  bindHandlers(trigger: Element, handlers: Record<string, EventListener>, signal: AbortSignal | undefined): void
  /** shared dropdown-menu/context-menu init body — both files register with this same function */
  installMenuFamily(root: Element): void
  /** dialog/alert-dialog/sheet portal-glue factory — pass the slot-name prefix and each family's 1-2 config values to get the component's init(live) */
  wireDialogFamily(component: string, opts: { dismissSelector: string; swallowOverlayClick?: boolean }): (live: Element) => void
}

/**
 * Forms: a control carrying a `name` attribute — checkbox, switch,
 * radio-group root, select trigger, slider root — gets hidden inputs
 * (`<input type="hidden" name=… value=…>`, one per value, none while
 * unchecked / unselected) inserted after it, like radix's BubbleInput: a
 * plain <form> submits it and `form.reset()` restores the initial state.
 * checkbox / switch submit their `value` attribute (default "on").
 */
export interface Shadless {
  init(root?: Element, opts?: { force?: boolean }): void
  initAll(opts?: { force?: boolean }): void
  /**
   * remove delegation on root, close every open instance inside it (portals
   * mounted into <body> come down), remove the glue's element listeners and
   * drop handles. Kernel-held wiring with no unwire (tabs, slider) stays and
   * is reused by the next init; the page-level menu protocol (dropdown /
   * context-menu, menubar, navigation-menu) installs once per document.
   */
  destroy(root?: Element): void
  refresh(element: Element): void
  start(): void
  stop(): void
  register(name: string, def: ComponentDef): void
  /** the handle of the instance `target` is inside (element, text node or selector); null outside any */
  get(target: Element | Node | string): Handle | null
  instances: WeakMap<Element, Handle>
  h: Helpers
  theme: { get(): "light" | "dark"; set(mode: "light" | "dark"): void; toggle(): void }
}

declare const shadless: Shadless
export default shadless
export declare const init: Shadless["init"]
export declare const initAll: Shadless["initAll"]
export declare const destroy: Shadless["destroy"]
export declare const refresh: Shadless["refresh"]
export declare const start: Shadless["start"]
export declare const stop: Shadless["stop"]
export declare const register: Shadless["register"]
export declare const get: Shadless["get"]
export declare const instances: Shadless["instances"]
export declare const h: Shadless["h"]
export declare const theme: Shadless["theme"]
