// shadless runtime base — delegation engine + component registry + theme.
// dist/shadless.js = the vendored radix kernel + this file. Behaviors live in
// dist/js/<name>.js, one per component. Global `shadless` with:
//   init(root?, {force}) — delegation on a root (default body); force
//     destroys and re-initializes (framework cache-restored DOM)
//   initAll({force})     — init(document.body)
//   destroy(root)        — remove delegation listeners, close every open
//     instance in root, remove the glue's element listeners and drop
//     handles (kernel-held wiring with no unwire — tabs, slider — stays
//     and is reused by the next init; the page-level menu protocol —
//     dropdown/context-menu, menubar, navigation-menu — installs once
//     per document and is not per-root)
//   refresh(element)     — (re-)run init-time behaviors on a subtree
//     (dynamically injected content inside a live root)
//   start()/stop()       — opt-in MutationObserver: runs init-time
//     behaviors on added subtrees. Delegation needs no re-init (events
//     bubble to the live root), so the observer only covers init-time
//     work (e.g. avatar settle). Never auto-started: contracts and demos
//     depend on explicit init's deterministic timing.
//   get(el)              — the programmatic handle of the instance el is in
// Events (bubbling CustomEvents, detail always carries `component`; `api`
// where the element has a handle), dispatched after the state change,
// whichever path caused it (gesture, keyboard, or the handle):
//   shadless:open / shadless:close   on the trigger of every openable —
//     dialog family, popover, tooltip, hover-card, select, the menus,
//     navigation-menu, and collapsible / accordion triggers
//   shadless:change                  on the element whose value changed —
//     checkbox / switch { checked }, toggle { pressed }, radio-group and
//     toggle-group root { value }, tabs root { index }, slider root
//     { values }, select trigger { value, label, item }
//   shadless:commit                  slider root { values }, once per
//     gesture (radix onValueCommit) — change is the live stream
//   shadless:themechange             on document, { mode }
// Forms: a control carrying a `name` attribute (checkbox, switch,
// radio-group, select trigger, slider) gets hidden inputs mirroring its
// value beside it, like radix's BubbleInput — a plain <form> submits it and
// form reset restores the initial state.
// Behaviors mirror radix semantics as recorded from the shadcn oracle
// (probes/t7/probe-facts.mjs); zero classes are added or required.
(function (global) {
  "use strict"

  // ---- helpers -------------------------------------------------------------
  function handler(ctx, type) {
    return function (ev) {
      // dispatch along the slot-ancestor chain (inner → outer), so a group
      // keydown fires even when the event target is an inner item
      for (var n = ev.target; n && n !== document; n = n.parentElement) {
        var s = n.getAttribute && n.getAttribute("data-slot")
        if (!s || !BEHAVIORS[s]) continue
        var b = BEHAVIORS[s][type]
        if (b) b(n, ctx, ev)
      }
    }
  }
  // roving-focus index math shared by radio-group / accordion / toggle-group
  // (three hand-rolled copies drifted apart before this was extracted):
  //   Home → first; End or unknown-origin → last; arrows wrap around.
  // Disabled items are skipped (radix roving focus never lands on one) and
  // under dir="rtl" the horizontal arrows swap (radix reads the direction
  // provider); returns -1 when no item can take focus.
  function isDisabled(el) {
    return !!(el.disabled || el.getAttribute("aria-disabled") === "true" || el.hasAttribute("data-disabled"))
  }
  function isRtl(el) {
    var d = el && el.closest ? el.closest("[dir]") : null
    return (d ? d.getAttribute("dir") : document.documentElement.getAttribute("dir")) === "rtl"
  }
  function nextIndex(ev, items) {
    var enabled = items.filter(function (it) { return !isDisabled(it) })
    if (!enabled.length) return -1
    var idx = enabled.indexOf(document.activeElement)
    var pick
    if (ev.key === "Home") pick = 0
    else if (ev.key === "End" || idx < 0) pick = enabled.length - 1
    else {
      var fwd = ev.key === "ArrowRight" || ev.key === "ArrowDown"
      if (isRtl(ev.currentTarget || enabled[idx]) && (ev.key === "ArrowRight" || ev.key === "ArrowLeft")) fwd = !fwd
      pick = (idx + (fwd ? 1 : -1) + enabled.length) % enabled.length
    }
    return items.indexOf(enabled[pick])
  }
  var NAV_KEYS = ["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"]
  function setChecked(root, checked) {
    root.setAttribute("aria-checked", String(checked))
    root.setAttribute("data-state", checked ? "checked" : "unchecked")
  }
  function setGroupItem(item, on) {
    item.setAttribute("aria-checked", String(on))
    item.setAttribute("data-state", on ? "on" : "off")
    if (!on) item.setAttribute("tabindex", "-1")
  }
  // template lookup: prefer the init root (each live root carries its own
  // fixture), fall back to document — the dist/demo convention places
  // templates as SIBLINGS of the live root at body level, so a consumer
  // calling shadless.init(someSubtree) must still find the page-level
  // template or the indicator silently never mounts.
  function findTemplate(ctx, dataFor) {
    var sel = 'template[data-for="' + dataFor + '"]'
    var scope = (ctx && ctx.root) || document
    return scope.querySelector(sel) || document.querySelector(sel)
  }
  // clone a template's first element; empty templates yield null instead of
  // the old `firstElementChild.cloneNode` TypeError
  function cloneTemplate(tpl) {
    var first = tpl && tpl.content && tpl.content.firstElementChild
    return first ? first.cloneNode(true) : null
  }
  function setRadioItem(item, on, ctx) {
    item.setAttribute("aria-checked", String(on))
    item.setAttribute("data-state", on ? "checked" : "unchecked")
    if (!on) item.setAttribute("tabindex", "-1")
    var ind = item.querySelector("[data-slot=radio-group-indicator]")
    if (on && !ind) {
      var node = cloneTemplate(findTemplate(ctx, "radio-group-indicator"))
      if (node) item.appendChild(node)
    } else if (!on && ind) ind.remove()
  }
  // collapsible/accordion shared: trigger + content state sync. Malformed
  // markup (missing content) degrades to trigger-only state instead of a
  // mid-loop TypeError.
  function setDisclosed(trigger, content, open) {
    trigger.setAttribute("aria-expanded", String(open))
    trigger.setAttribute("data-state", open ? "open" : "closed")
    if (content) {
      if (open) content.removeAttribute("hidden")
      else content.setAttribute("hidden", "")
      content.setAttribute("data-state", open ? "open" : "closed")
      linkControls(trigger, content)
    }
  }
  // radix renders aria-controls on the trigger permanently (closed
  // included) — the pairing is contract, not open-state decoration
  function linkControls(trigger, content) {
    if (content && content.id) trigger.setAttribute("aria-controls", content.id)
  }

  // ---- form integration -----------------------------------------------------
  // radix renders a hidden native input beside a control that carries a
  // `name` (BubbleInput / BubbleSelect) so a plain <form> submits it and a
  // reset restores it; the conversion had dropped that. Same contract here,
  // gated on the name attribute: formMirror(root, def) is called once at
  // wiring, def.read() yields the current value (boolean for checkbox /
  // switch, string|null for radio-group / select, number[] for slider),
  // def.write(v) restores one on form reset. syncForm(root) after every
  // change keeps the hidden inputs (name=value, one per value, none when
  // unchecked / unselected) in step. Without a name nothing is added — the
  // contract fixtures and demos carry none, so their DOM is unchanged.
  var MIRRORS = new WeakMap()
  function formMirror(root, def) {
    if (MIRRORS.has(root)) return MIRRORS.get(root)
    if (!root.getAttribute("name")) return null
    var m = { def: def, initial: def.read(), inputs: [] }
    MIRRORS.set(root, m)
    var form = root.closest("form")
    // native controls reset after the event; restore in the same turn order
    if (form) form.addEventListener("reset", function () {
      setTimeout(function () { def.write(m.initial); syncForm(root) }, 0)
    })
    syncForm(root)
    return m
  }
  function syncForm(root) {
    var m = MIRRORS.get(root)
    if (!m) return
    var name = root.getAttribute("name"), v = m.def.read()
    var vals = typeof v === "boolean" ? (v ? [root.getAttribute("value") || "on"] : [])
      : v == null ? [] : [].concat(v).map(String)
    while (m.inputs.length > vals.length) m.inputs.pop().remove()
    while (m.inputs.length < vals.length) {
      var input = document.createElement("input")
      input.type = "hidden"
      input.setAttribute("data-shadless-form", "")
      ;(m.inputs.length ? m.inputs[m.inputs.length - 1] : root).insertAdjacentElement("afterend", input)
      m.inputs.push(input)
    }
    m.inputs.forEach(function (input, k) { input.name = name; input.value = vals[k] })
  }

  // ---- behaviors -----------------------------------------------------------
  // checkbox: indicator mounts/unmounts with data-state (radix Presence).
  // ---- registry -------------------------------------------------------------
  // Behaviors arrive from per-component files (dist/js/<name>.js), each
  // calling shadless.register(name, def):
  //   def.slots  { "<data-slot>": { init(el), onClick(el, ctx, ev), onKeydown(el, ctx, ev) } }
  //              — delegated on every live root, like the trivial runtime
  //   def.init   function (root) — component-level wiring (kernel families:
  //              scan root, wire every instance once)
  // Registering after a root went live still works: init(root) is re-run
  // for the new component on every live root.
  var BEHAVIORS = {}
  var COMPONENTS = {}
  // programmatic handles: a component wires an element (its trigger or
  // root) and files the instance API here; shadless.get(el) walks up from
  // any element inside it. Same shape across families where it applies:
  // open() / close() / toggle() / isOpen(); tabs: activate(i) / active();
  // slider: values() / setValue(v, i); carousel: the embla api.
  // Deliberately NO handle for the trivial tier (checkbox, switch, toggle,
  // radio-group, toggle-group, collapsible, accordion, avatar): their whole
  // state is the attribute radix renders (aria-checked / aria-pressed /
  // aria-expanded / data-state) and el.click() is the driver — get()
  // returns null for them rather than wrapping what the DOM already says.
  var INSTANCES = new WeakMap()
  // shadless:<type> on the element, after the state change (the handle
  // already reflects it when the listener runs); extra keys merge into detail
  function emit(el, type, component, extra) {
    var detail = { component: component, api: INSTANCES.get(el) || null }
    for (var k in extra) detail[k] = extra[k]
    el.dispatchEvent(new CustomEvent("shadless:" + type, { bubbles: true, detail: detail }))
  }
  // the value a radio / toggle-group item stands for: value attr, else id
  function itemValue(item) {
    return item.getAttribute("value") || item.getAttribute("data-value") || item.id || null
  }
  function get(target) {
    var el = typeof target === "string" ? document.querySelector(target) : target
    if (el && el.nodeType !== 1) el = el.parentElement // text node inside a trigger
    for (var n = el; n && n.nodeType === 1; n = n.parentElement) {
      var api = INSTANCES.get(n)
      if (api) return api
    }
    return null
  }
  // ---- instance wiring (kernel families) ----------------------------------
  // A component's init(live) wires each instance once. wire(el, live)
  // returns null when el is already wired, else a record { signal, teardown,
  // persistent }: `signal` is an AbortSignal the glue passes to every
  // addEventListener it adds, so destroy(live) removes them; `teardown` is a
  // hook destroy calls (kernel / embla destroy); `persistent` marks wiring
  // the vendored kernel holds with no unwire of its own (tabs, slider) —
  // destroy leaves it in place and a re-init REUSES it instead of doubling
  // the kernel's listeners. destroy also closes every open instance in the
  // root (portals mounted into <body> come down with it) and drops handles.
  var WIRED = new WeakMap()      // el -> record
  var ROOT_WIRED = new WeakMap() // live root -> [el]
  function wire(el, live) {
    if (WIRED.has(el)) return null
    var ctl = typeof AbortController === "function" ? new AbortController() : null
    var rec = { signal: ctl ? ctl.signal : undefined, ctl: ctl, teardown: null, persistent: false }
    WIRED.set(el, rec)
    live = live || document.body
    var list = ROOT_WIRED.get(live)
    if (!list) ROOT_WIRED.set(live, (list = []))
    list.push(el)
    return rec
  }
  function unwire(live) {
    var list = ROOT_WIRED.get(live) || []
    var kept = []
    list.forEach(function (el) {
      var rec = WIRED.get(el)
      if (!rec) return
      var api = INSTANCES.get(el)
      if (api && typeof api.isOpen === "function" && typeof api.close === "function" && api.isOpen()) api.close(false)
      if (rec.persistent) { kept.push(el); return }
      if (rec.ctl) rec.ctl.abort()
      if (rec.teardown) { try { rec.teardown() } catch (e) { console.error("shadless: teardown failed", el, e) } }
      WIRED.delete(el)
      INSTANCES.delete(el)
    })
    if (kept.length) ROOT_WIRED.set(live, kept)
    else ROOT_WIRED.delete(live)
  }

  var LIVE_ROOTS = []
  function register(name, def) {
    if (COMPONENTS[name]) return
    COMPONENTS[name] = def
    if (def.slots) for (var s in def.slots) BEHAVIORS[s] = def.slots[s]
    LIVE_ROOTS.forEach(function (root) {
      if (def.slots) runInitBehaviors(root, false)
      if (def.init) safeInit(name, def, root)
    })
  }
  function safeInit(name, def, root) {
    try { def.init(root) }
    catch (e) { console.error("shadless: " + name + " init failed", e) }
  }
  function runComponentInits(root) {
    for (var n in COMPONENTS) if (COMPONENTS[n].init) safeInit(n, COMPONENTS[n], root)
  }

  // ---- per-root delegation -------------------------------------------------
  var LIVE = new WeakMap() // root element -> { click, keydown }

  // Init-time work (behaviors with `init`) runs per element; one failing
  // element must not abort the rest of the subtree (same defensive posture
  // as the malformed-markup guards in the behaviors above).
  function initBehavior(el) {
    var b = BEHAVIORS[el.getAttribute("data-slot")]
    if (!b || !b.init) return
    try { b.init(el) }
    catch (e) { console.error("shadless: init behavior failed", el, e) }
  }

  function runInitBehaviors(scope, includeSelf) {
    if (!scope || scope.nodeType !== 1) return
    if (includeSelf) initBehavior(scope)
    scope.querySelectorAll("[data-slot]").forEach(initBehavior)
  }

  function init(root, opts) {
    root = root || document.body
    if (opts && opts.force) destroy(root)
    if (LIVE.has(root)) return
    // nested live roots would double-dispatch (ancestor+descendant listeners
    // both see the bubbled event) — skip when delegation already covers root
    for (var a = root.parentElement; a; a = a.parentElement)
      if (LIVE.has(a)) return
    for (const d of root.querySelectorAll("*"))
      if (LIVE.has(d)) return
    // behaviors with init-time work (e.g. avatar image settle)
    runInitBehaviors(root, false)
    var ctx = { root: root }
    var onClick = handler(ctx, "onClick")
    var onKeydown = handler(ctx, "onKeydown")
    root.addEventListener("click", onClick)
    root.addEventListener("keydown", onKeydown)
    LIVE.set(root, { click: onClick, keydown: onKeydown })
    LIVE_ROOTS.push(root)
    runComponentInits(root)
  }

  function initAll(opts) { init(document.body, opts) }

  function destroy(root) {
    root = root || document.body
    var l = LIVE.get(root)
    if (!l) return
    root.removeEventListener("click", l.click)
    root.removeEventListener("keydown", l.keydown)
    LIVE.delete(root)
    LIVE_ROOTS = LIVE_ROOTS.filter(function (r) { return r !== root })
    unwire(root)
  }

  function refresh(element) { runInitBehaviors(element, true) }

  // ---- theme (J5) — vanilla theme semantics, consumer-compatible ---------
  // Same contract shadcn consumers expect from React theme providers (the
  // pattern behind shadcn's official dark-mode docs), minus React: toggle .dark on
  // <html>, persist, notify. Deliberately a SEPARATE storage key from the
  // docs-site pre-paint (shadless-docs-theme, src/docs/theme-prepaint.mjs):
  // that one is site chrome; this one is the product API. Consumers doing
  // FOUC avoidance read the same key before paint.
  var THEME_KEY = "shadless-theme"
  function themeGet() {
    return document.documentElement.classList.contains("dark") ? "dark" : "light"
  }
  function themeSet(mode) {
    var dark = mode === "dark"
    document.documentElement.classList.toggle("dark", dark)
    try { localStorage.setItem(THEME_KEY, dark ? "dark" : "light") } catch (e) { /* private mode etc. */ }
    document.dispatchEvent(new CustomEvent("shadless:themechange",
      { detail: { mode: dark ? "dark" : "light" } }))
  }

  // ---- opt-in observer (J3) ------------------------------------------------
  // For framework-injected content inside an already-live root: delegation
  // already covers clicks/keys, only init-time behaviors need a rescan.
  // Deliberately NOT auto-started; removals need no cleanup (WeakMap +
  // element-local listeners are garbage-collected with the subtree).
  var observer = null
  function start() {
    if (observer || typeof MutationObserver !== "function" || !document.body) return
    observer = new MutationObserver(function (mutations) {
      mutations.forEach(function (m) {
        m.addedNodes.forEach(function (n) { runInitBehaviors(n, true) })
      })
    })
    observer.observe(document.body, { childList: true, subtree: true })
  }
  function stop() {
    if (!observer) return
    observer.disconnect()
    observer = null
  }

  global.shadless = {
    init: init, initAll: initAll, destroy: destroy,
    refresh: refresh, start: start, stop: stop,
    register: register,
    get: get,
    instances: INSTANCES,
    // helpers the component files share (radix semantics, measured once)
    h: { nextIndex: nextIndex, NAV_KEYS: NAV_KEYS, setChecked: setChecked, setGroupItem: setGroupItem,
      findTemplate: findTemplate, cloneTemplate: cloneTemplate, setRadioItem: setRadioItem,
      setDisclosed: setDisclosed, linkControls: linkControls, emit: emit, itemValue: itemValue, wire: wire,
      isDisabled: isDisabled, isRtl: isRtl, formMirror: formMirror, syncForm: syncForm },
    theme: {
      get: themeGet,
      set: themeSet,
      toggle: function () { themeSet(themeGet() === "dark" ? "light" : "dark") },
    },
  }

  // auto-init: the page's sync scripts (base + component files) have all
  // run by DOMContentLoaded. Idempotent — an explicit shadless.init() before
  // or after is fine. Set window.shadlessNoAutoInit = true to opt out.
  if (typeof document !== "undefined" && !global.shadlessNoAutoInit) {
    var auto = function () { init(document.body) }
    if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", auto)
    else auto()
  }
})(typeof window !== "undefined" ? window : globalThis)
