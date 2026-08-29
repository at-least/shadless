"use strict";
var __rkFeature = (() => {
  var __defProp = Object.defineProperty;
  var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __hasOwnProp = Object.prototype.hasOwnProperty;
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };
  var __copyProps = (to, from, except, desc) => {
    if (from && typeof from === "object" || typeof from === "function") {
      for (let key of __getOwnPropNames(from))
        if (!__hasOwnProp.call(to, key) && key !== except)
          __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
    }
    return to;
  };
  var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

  // src/entries/dialog.ts
  var dialog_exports = {};
  __export(dialog_exports, {
    wireDialog: () => wireDialog
  });

  // src/core/refcount.ts
  var hiddenCounts = /* @__PURE__ */ new Map();
  function refcountHiding(skipHideExtra) {
    const isSkipHide = (el, portal) => {
      if (el === portal) return true;
      if (el.hasAttribute("data-radix-popper-content-wrapper")) return true;
      if (el.hasAttribute("data-radix-focus-guard")) return true;
      return skipHideExtra?.(el) ?? false;
    };
    return {
      hideBackground(portal) {
        const touched = [];
        for (const el of Array.from(portal.ownerDocument.body.children)) {
          if (isSkipHide(el, portal)) continue;
          let rec = hiddenCounts.get(el);
          if (!rec) {
            rec = { count: 0, prev: el.getAttribute("aria-hidden") };
            hiddenCounts.set(el, rec);
          }
          rec.count++;
          el.setAttribute("aria-hidden", "true");
          touched.push(el);
        }
        return touched;
      },
      restoreBackground(touched) {
        for (const el of touched) {
          const rec = hiddenCounts.get(el);
          if (!rec) continue;
          if (--rec.count > 0) continue;
          hiddenCounts.delete(el);
          if (rec.prev == null) el.removeAttribute("aria-hidden");
          else el.setAttribute("aria-hidden", rec.prev);
        }
      },
      reset() {
        for (const [el, rec] of hiddenCounts) {
          if (rec.prev == null) el.removeAttribute("aria-hidden");
          else el.setAttribute("aria-hidden", rec.prev);
        }
        hiddenCounts.clear();
      }
    };
  }

  // src/core/presence.ts
  var FRAME_MS = 16;
  function presenceExit(el, durationMs, remove, writeFillMode = false) {
    let done = false;
    let timer = 0;
    const finish = () => {
      if (done) return;
      done = true;
      clearTimeout(timer);
      el.removeEventListener("animationend", onEnd);
      if (writeFillMode) {
        el.style.setProperty("animation-fill-mode", "forwards");
      }
      remove();
    };
    const onEnd = (event) => {
      if (event.target === el) finish();
    };
    el.addEventListener("animationend", onEnd);
    timer = setTimeout(finish, durationMs + FRAME_MS);
    return {
      cancel() {
        if (done) return;
        done = true;
        clearTimeout(timer);
        el.removeEventListener("animationend", onEnd);
      }
    };
  }

  // src/core/dismiss.ts
  var layers = [];
  function subtreeContains(layer, target) {
    if (layer.nodes.some((n) => n.contains(target))) return true;
    for (const branch of layer.branches) if (subtreeContains(branch, target)) return true;
    return false;
  }
  function registerDismissLayer(nodes, dismissFromOutside) {
    const layer = {
      nodes,
      branches: /* @__PURE__ */ new Set(),
      isOutside(target) {
        return !subtreeContains(layer, target);
      },
      dismissFromOutside
    };
    let parent;
    for (const candidate of layers) {
      if (nodes.some((n) => subtreeContains(candidate, n))) parent = candidate;
    }
    if (parent) parent.branches.add(layer);
    layers.push(layer);
    return layer;
  }
  function unregisterDismissLayer(layer) {
    const i = layers.indexOf(layer);
    if (i !== -1) layers.splice(i, 1);
    for (const l of layers) l.branches.delete(layer);
  }
  function isHighestDismissLayer(layer) {
    return layers[layers.length - 1] === layer;
  }
  function insideHigherLayer(target, layer) {
    const i = layers.indexOf(layer);
    for (let j = i + 1; j < layers.length; j++) {
      if (subtreeContains(layers[j], target)) return true;
    }
    return false;
  }

  // src/core/dialog.ts
  function wireDialog(options) {
    const {
      content,
      portal,
      trigger,
      onClosed,
      backgroundMode = "simple",
      scrollLock = "data-attr",
      unmount = "immediate",
      exitDuration = 160,
      contentExitDuration = 100,
      closeFocus = "always",
      contentStateFlip = false,
      preventDefaultOnKeys = true
    } = options;
    const body = content.ownerDocument.body;
    const doc = content.ownerDocument;
    const lockScroll = () => {
      if (scrollLock === "overflow") {
        body.style.overflow = "hidden";
        return;
      }
      body.setAttribute("data-scroll-locked", "1");
      body.style.setProperty("pointer-events", "none");
    };
    const unlockScroll = () => {
      if (scrollLock === "overflow") {
        body.style.overflow = "";
        return;
      }
      body.removeAttribute("data-scroll-locked");
      body.style.removeProperty("pointer-events");
    };
    lockScroll();
    const guardStart = makeGuard(doc);
    const guardEnd = makeGuard(doc);
    body.insertBefore(guardStart, body.firstChild);
    const hiding = refcountHiding(options.isPortalMarker);
    const hide = backgroundMode === "refcount" ? hiding.hideBackground(portal) : null;
    const hiddenOutside = [];
    if (hide === null) {
      for (const el of Array.from(body.children)) {
        if (el === guardStart || el === guardEnd) continue;
        const tag = el.tagName;
        if (tag === "SCRIPT" || tag === "STYLE" || tag === "LINK" || tag === "TEMPLATE") continue;
        el.setAttribute("aria-hidden", "true");
        el.setAttribute("data-aria-hidden", "true");
        hiddenOutside.push(el);
      }
    }
    body.append(portal, guardEnd);
    if (trigger) {
      trigger.setAttribute("data-state", "open");
      trigger.setAttribute("aria-expanded", "true");
      trigger.setAttribute("aria-controls", content.id);
    }
    if (contentStateFlip) {
      setStateAttr(portal, "open");
      setStateAttr(content, "open");
    }
    const focusables = () => focusableIn(content);
    (focusables()[0] ?? content).focus();
    const layer = registerDismissLayer(trigger ? [portal, trigger] : [portal], () => close(true));
    const onKeyDown = (event) => {
      if (event.key === "Escape") {
        if (!isHighestDismissLayer(layer)) return;
        if (preventDefaultOnKeys) event.preventDefault();
        close(true);
        return;
      }
      if (event.key !== "Tab") return;
      const list = focusables();
      const active = doc.activeElement;
      if (list.length === 0) {
        event.preventDefault();
        content.focus();
        return;
      }
      if (closeFocus === "parametrized") {
        event.preventDefault();
        const i = list.indexOf(active);
        const next = event.shiftKey ? i <= 0 ? list.length - 1 : i - 1 : i === list.length - 1 ? 0 : i + 1;
        list[next].focus();
        return;
      }
      const first = list[0];
      const last = list[list.length - 1];
      if (active === first && event.shiftKey) {
        event.preventDefault();
        last.focus();
      } else if (active === last && !event.shiftKey) {
        event.preventDefault();
        first.focus();
      } else if (!(isNode(active) && content.contains(active))) {
        event.preventDefault();
        (event.shiftKey ? last : first).focus();
      }
    };
    const onPortalClick = (event) => {
      if (!isNode(event.target)) return;
      if (content.contains(event.target)) return;
      if (insideHigherLayer(event.target, layer)) return;
      close(true);
    };
    doc.addEventListener("keydown", onKeyDown);
    portal.addEventListener("click", onPortalClick);
    let closed = false;
    const close = (restoreFocusArg) => {
      if (closed) return;
      closed = true;
      unregisterDismissLayer(layer);
      const restoreFocus = closeFocus === "always" ? true : restoreFocusArg ?? true;
      doc.removeEventListener("keydown", onKeyDown);
      portal.removeEventListener("click", onPortalClick);
      guardStart.remove();
      guardEnd.remove();
      if (contentStateFlip) {
        setStateAttr(portal, "closed");
        setStateAttr(content, "closed");
      }
      unlockScroll();
      if (trigger) {
        trigger.setAttribute("data-state", "closed");
        trigger.setAttribute("aria-expanded", "false");
        trigger.removeAttribute("aria-controls");
      }
      const finish = () => {
        portal.remove();
        if (hide !== null) hiding.restoreBackground(hide);
        for (const el of hiddenOutside) {
          el.removeAttribute("aria-hidden");
          el.removeAttribute("data-aria-hidden");
        }
        if (restoreFocus && trigger) {
          if (closeFocus === "parametrized") {
            setTimeout(() => trigger.focus(), 0);
          } else {
            trigger.focus();
          }
        }
      };
      onClosed?.();
      if (unmount === "exit-window") {
        presenceExit(
          content,
          contentExitDuration,
          () => {
            content.remove();
          },
          true
        );
        presenceExit(portal, exitDuration, finish);
      } else {
        finish();
      }
    };
    return { close };
  }
  function setStateAttr(el, state) {
    el.setAttribute("data-state", state);
  }
  function isNode(v) {
    return typeof v?.nodeType === "number";
  }
  function makeGuard(doc) {
    const guard = doc.createElement("span");
    guard.setAttribute("data-radix-focus-guard", "");
    guard.setAttribute("tabindex", "0");
    guard.setAttribute("aria-hidden", "true");
    guard.setAttribute("data-aria-hidden", "true");
    guard.setAttribute("style", "outline: none; opacity: 0; position: fixed; pointer-events: none;");
    return guard;
  }
  var FOCUSABLE_SELECTOR = 'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
  function focusableIn(root) {
    return Array.from(root.querySelectorAll(FOCUSABLE_SELECTOR));
  }
  return __toCommonJS(dialog_exports);
})();
globalThis.RadixKernel = Object.assign(globalThis.RadixKernel ?? {}, __rkFeature);
