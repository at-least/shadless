"use strict";
var RadixKernel = (() => {
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

  // src/index.ts
  var index_exports = {};
  __export(index_exports, {
    configureTooltipProvider: () => configureTooltipProvider,
    positionItemAligned: () => positionItemAligned,
    positionPopper: () => positionPopper,
    wireDialog: () => wireDialog,
    wireHoverCard: () => wireHoverCard,
    wireMenu: () => wireMenu,
    wirePopover: () => wirePopover,
    wireScrollArea: () => wireScrollArea,
    wireSelect: () => wireSelect,
    wireSlider: () => wireSlider,
    wireTabs: () => wireTabs,
    wireTooltip: () => wireTooltip
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
  function dismissLayersOutside(target) {
    for (let i = layers.length - 1; i >= 0; i--) {
      const l = layers[i];
      if (l.isOutside(target)) l.dismissFromOutside();
    }
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
    const hide3 = backgroundMode === "refcount" ? hiding.hideBackground(portal) : null;
    const hiddenOutside = [];
    if (hide3 === null) {
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
        if (hide3 !== null) hiding.restoreBackground(hide3);
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

  // node_modules/@floating-ui/utils/dist/floating-ui.utils.mjs
  var sides = ["top", "right", "bottom", "left"];
  var min = Math.min;
  var max = Math.max;
  var round = Math.round;
  var floor = Math.floor;
  var createCoords = (v) => ({
    x: v,
    y: v
  });
  var oppositeSideMap = {
    left: "right",
    right: "left",
    bottom: "top",
    top: "bottom"
  };
  function clamp(start, value, end) {
    return max(start, min(value, end));
  }
  function evaluate(value, param) {
    return typeof value === "function" ? value(param) : value;
  }
  function getSide(placement) {
    return placement.split("-")[0];
  }
  function getAlignment(placement) {
    return placement.split("-")[1];
  }
  function getOppositeAxis(axis) {
    return axis === "x" ? "y" : "x";
  }
  function getAxisLength(axis) {
    return axis === "y" ? "height" : "width";
  }
  function getSideAxis(placement) {
    const firstChar = placement[0];
    return firstChar === "t" || firstChar === "b" ? "y" : "x";
  }
  function getAlignmentAxis(placement) {
    return getOppositeAxis(getSideAxis(placement));
  }
  function getAlignmentSides(placement, rects, rtl) {
    if (rtl === void 0) {
      rtl = false;
    }
    const alignment = getAlignment(placement);
    const alignmentAxis = getAlignmentAxis(placement);
    const length = getAxisLength(alignmentAxis);
    let mainAlignmentSide = alignmentAxis === "x" ? alignment === (rtl ? "end" : "start") ? "right" : "left" : alignment === "start" ? "bottom" : "top";
    if (rects.reference[length] > rects.floating[length]) {
      mainAlignmentSide = getOppositePlacement(mainAlignmentSide);
    }
    return [mainAlignmentSide, getOppositePlacement(mainAlignmentSide)];
  }
  function getExpandedPlacements(placement) {
    const oppositePlacement = getOppositePlacement(placement);
    return [getOppositeAlignmentPlacement(placement), oppositePlacement, getOppositeAlignmentPlacement(oppositePlacement)];
  }
  function getOppositeAlignmentPlacement(placement) {
    return placement.includes("start") ? placement.replace("start", "end") : placement.replace("end", "start");
  }
  var lrPlacement = ["left", "right"];
  var rlPlacement = ["right", "left"];
  var tbPlacement = ["top", "bottom"];
  var btPlacement = ["bottom", "top"];
  function getSideList(side, isStart, rtl) {
    switch (side) {
      case "top":
      case "bottom":
        if (rtl) return isStart ? rlPlacement : lrPlacement;
        return isStart ? lrPlacement : rlPlacement;
      case "left":
      case "right":
        return isStart ? tbPlacement : btPlacement;
      default:
        return [];
    }
  }
  function getOppositeAxisPlacements(placement, flipAlignment, direction, rtl) {
    const alignment = getAlignment(placement);
    let list = getSideList(getSide(placement), direction === "start", rtl);
    if (alignment) {
      list = list.map((side) => side + "-" + alignment);
      if (flipAlignment) {
        list = list.concat(list.map(getOppositeAlignmentPlacement));
      }
    }
    return list;
  }
  function getOppositePlacement(placement) {
    const side = getSide(placement);
    return oppositeSideMap[side] + placement.slice(side.length);
  }
  function expandPaddingObject(padding) {
    var _padding$top, _padding$right, _padding$bottom, _padding$left;
    return {
      top: (_padding$top = padding.top) != null ? _padding$top : 0,
      right: (_padding$right = padding.right) != null ? _padding$right : 0,
      bottom: (_padding$bottom = padding.bottom) != null ? _padding$bottom : 0,
      left: (_padding$left = padding.left) != null ? _padding$left : 0
    };
  }
  function getPaddingObject(padding) {
    return typeof padding !== "number" ? expandPaddingObject(padding) : {
      top: padding,
      right: padding,
      bottom: padding,
      left: padding
    };
  }
  function rectToClientRect(rect) {
    const {
      x,
      y,
      width,
      height
    } = rect;
    return {
      width,
      height,
      top: y,
      left: x,
      right: x + width,
      bottom: y + height,
      x,
      y
    };
  }

  // node_modules/@floating-ui/core/dist/floating-ui.core.mjs
  function computeCoordsFromPlacement(_ref, placement, rtl) {
    let {
      reference,
      floating
    } = _ref;
    const sideAxis = getSideAxis(placement);
    const alignmentAxis = getAlignmentAxis(placement);
    const alignLength = getAxisLength(alignmentAxis);
    const side = getSide(placement);
    const isVertical = sideAxis === "y";
    const commonX = reference.x + reference.width / 2 - floating.width / 2;
    const commonY = reference.y + reference.height / 2 - floating.height / 2;
    const commonAlign = reference[alignLength] / 2 - floating[alignLength] / 2;
    let coords;
    switch (side) {
      case "top":
        coords = {
          x: commonX,
          y: reference.y - floating.height
        };
        break;
      case "bottom":
        coords = {
          x: commonX,
          y: reference.y + reference.height
        };
        break;
      case "right":
        coords = {
          x: reference.x + reference.width,
          y: commonY
        };
        break;
      case "left":
        coords = {
          x: reference.x - floating.width,
          y: commonY
        };
        break;
      default:
        coords = {
          x: reference.x,
          y: reference.y
        };
    }
    const alignment = getAlignment(placement);
    if (alignment) {
      coords[alignmentAxis] += commonAlign * (alignment === "end" ? 1 : -1) * (rtl && isVertical ? -1 : 1);
    }
    return coords;
  }
  async function detectOverflow(state, options) {
    var _await$platform$isEle;
    if (options === void 0) {
      options = {};
    }
    const {
      x,
      y,
      platform: platform2,
      rects,
      elements,
      strategy
    } = state;
    const {
      boundary = "clippingAncestors",
      rootBoundary = "viewport",
      elementContext = "floating",
      altBoundary = false,
      padding = 0
    } = evaluate(options, state);
    const paddingObject = getPaddingObject(padding);
    const altContext = elementContext === "floating" ? "reference" : "floating";
    const element = elements[altBoundary ? altContext : elementContext];
    const clippingClientRect = rectToClientRect(await platform2.getClippingRect({
      element: ((_await$platform$isEle = await (platform2.isElement == null ? void 0 : platform2.isElement(element))) != null ? _await$platform$isEle : true) ? element : element.contextElement || await (platform2.getDocumentElement == null ? void 0 : platform2.getDocumentElement(elements.floating)),
      boundary,
      rootBoundary,
      strategy
    }));
    const rect = elementContext === "floating" ? {
      x,
      y,
      width: rects.floating.width,
      height: rects.floating.height
    } : rects.reference;
    const offsetParent = await (platform2.getOffsetParent == null ? void 0 : platform2.getOffsetParent(elements.floating));
    const offsetScale = await (platform2.isElement == null ? void 0 : platform2.isElement(offsetParent)) && await (platform2.getScale == null ? void 0 : platform2.getScale(offsetParent)) || {
      x: 1,
      y: 1
    };
    const elementClientRect = rectToClientRect(platform2.convertOffsetParentRelativeRectToViewportRelativeRect ? await platform2.convertOffsetParentRelativeRectToViewportRelativeRect({
      elements,
      rect,
      offsetParent,
      strategy
    }) : rect);
    return {
      top: (clippingClientRect.top - elementClientRect.top + paddingObject.top) / offsetScale.y,
      bottom: (elementClientRect.bottom - clippingClientRect.bottom + paddingObject.bottom) / offsetScale.y,
      left: (clippingClientRect.left - elementClientRect.left + paddingObject.left) / offsetScale.x,
      right: (elementClientRect.right - clippingClientRect.right + paddingObject.right) / offsetScale.x
    };
  }
  var MAX_RESET_COUNT = 50;
  var computePosition = async (reference, floating, config) => {
    const {
      placement = "bottom",
      strategy = "absolute",
      middleware = [],
      platform: platform2
    } = config;
    const platformWithDetectOverflow = platform2.detectOverflow ? platform2 : {
      ...platform2,
      detectOverflow
    };
    const rtl = await (platform2.isRTL == null ? void 0 : platform2.isRTL(floating));
    let rects = await platform2.getElementRects({
      reference,
      floating,
      strategy
    });
    let {
      x,
      y
    } = computeCoordsFromPlacement(rects, placement, rtl);
    let statefulPlacement = placement;
    let resetCount = 0;
    const middlewareData = {};
    for (let i = 0; i < middleware.length; i++) {
      const currentMiddleware = middleware[i];
      if (!currentMiddleware) {
        continue;
      }
      const {
        name,
        fn
      } = currentMiddleware;
      const {
        x: nextX,
        y: nextY,
        data,
        reset
      } = await fn({
        x,
        y,
        initialPlacement: placement,
        placement: statefulPlacement,
        strategy,
        middlewareData,
        rects,
        platform: platformWithDetectOverflow,
        elements: {
          reference,
          floating
        }
      });
      x = nextX != null ? nextX : x;
      y = nextY != null ? nextY : y;
      middlewareData[name] = {
        ...middlewareData[name],
        ...data
      };
      if (reset && resetCount < MAX_RESET_COUNT) {
        resetCount++;
        if (typeof reset === "object") {
          if (reset.placement) {
            statefulPlacement = reset.placement;
          }
          if (reset.rects) {
            rects = reset.rects === true ? await platform2.getElementRects({
              reference,
              floating,
              strategy
            }) : reset.rects;
          }
          ({
            x,
            y
          } = computeCoordsFromPlacement(rects, statefulPlacement, rtl));
        }
        i = -1;
      }
    }
    return {
      x,
      y,
      placement: statefulPlacement,
      strategy,
      middlewareData
    };
  };
  var arrow = (options) => ({
    name: "arrow",
    options,
    async fn(state) {
      const {
        x,
        y,
        placement,
        rects,
        platform: platform2,
        elements,
        middlewareData
      } = state;
      const {
        element,
        padding = 0
      } = evaluate(options, state) || {};
      if (element == null) {
        return {};
      }
      const paddingObject = getPaddingObject(padding);
      const coords = {
        x,
        y
      };
      const axis = getAlignmentAxis(placement);
      const length = getAxisLength(axis);
      const arrowDimensions = await platform2.getDimensions(element);
      const isYAxis = axis === "y";
      const minProp = isYAxis ? "top" : "left";
      const maxProp = isYAxis ? "bottom" : "right";
      const clientProp = isYAxis ? "clientHeight" : "clientWidth";
      const endDiff = rects.reference[length] + rects.reference[axis] - coords[axis] - rects.floating[length];
      const startDiff = coords[axis] - rects.reference[axis];
      const arrowOffsetParent = await (platform2.getOffsetParent == null ? void 0 : platform2.getOffsetParent(element));
      let clientSize = arrowOffsetParent ? arrowOffsetParent[clientProp] : 0;
      if (!clientSize || !await (platform2.isElement == null ? void 0 : platform2.isElement(arrowOffsetParent))) {
        clientSize = elements.floating[clientProp] || rects.floating[length];
      }
      const centerToReference = endDiff / 2 - startDiff / 2;
      const largestPossiblePadding = clientSize / 2 - arrowDimensions[length] / 2 - 1;
      const minPadding = min(paddingObject[minProp], largestPossiblePadding);
      const maxPadding = min(paddingObject[maxProp], largestPossiblePadding);
      const max2 = clientSize - arrowDimensions[length] - maxPadding;
      const center = clientSize / 2 - arrowDimensions[length] / 2 + centerToReference;
      const offset3 = clamp(minPadding, center, max2);
      const shouldAddOffset = !middlewareData.arrow && getAlignment(placement) != null && center !== offset3 && rects.reference[length] / 2 - (center < minPadding ? minPadding : maxPadding) - arrowDimensions[length] / 2 < 0;
      const alignmentOffset = shouldAddOffset ? center < minPadding ? center - minPadding : center - max2 : 0;
      return {
        [axis]: coords[axis] + alignmentOffset,
        data: {
          [axis]: offset3,
          centerOffset: center - offset3 - alignmentOffset,
          ...shouldAddOffset && {
            alignmentOffset
          }
        },
        reset: shouldAddOffset
      };
    }
  });
  var flip = function(options) {
    if (options === void 0) {
      options = {};
    }
    return {
      name: "flip",
      options,
      async fn(state) {
        var _middlewareData$arrow, _middlewareData$flip;
        const {
          placement,
          middlewareData,
          rects,
          initialPlacement,
          platform: platform2,
          elements
        } = state;
        const {
          mainAxis: checkMainAxis = true,
          crossAxis: checkCrossAxis = true,
          fallbackPlacements: specifiedFallbackPlacements,
          fallbackStrategy = "bestFit",
          fallbackAxisSideDirection = "none",
          flipAlignment = true,
          ...detectOverflowOptions
        } = evaluate(options, state);
        if ((_middlewareData$arrow = middlewareData.arrow) != null && _middlewareData$arrow.alignmentOffset) {
          return {};
        }
        const side = getSide(placement);
        const initialSideAxis = getSideAxis(initialPlacement);
        const isBasePlacement = getSide(initialPlacement) === initialPlacement;
        const rtl = await (platform2.isRTL == null ? void 0 : platform2.isRTL(elements.floating));
        const fallbackPlacements = specifiedFallbackPlacements || (isBasePlacement || !flipAlignment ? [getOppositePlacement(initialPlacement)] : getExpandedPlacements(initialPlacement));
        const hasFallbackAxisSideDirection = fallbackAxisSideDirection !== "none";
        if (!specifiedFallbackPlacements && hasFallbackAxisSideDirection) {
          fallbackPlacements.push(...getOppositeAxisPlacements(initialPlacement, flipAlignment, fallbackAxisSideDirection, rtl));
        }
        const placements2 = [initialPlacement, ...fallbackPlacements];
        const overflow = await platform2.detectOverflow(state, detectOverflowOptions);
        const overflows = [];
        let overflowsData = ((_middlewareData$flip = middlewareData.flip) == null ? void 0 : _middlewareData$flip.overflows) || [];
        if (checkMainAxis) {
          overflows.push(overflow[side]);
        }
        if (checkCrossAxis) {
          const sides2 = getAlignmentSides(placement, rects, rtl);
          overflows.push(overflow[sides2[0]], overflow[sides2[1]]);
        }
        overflowsData = [...overflowsData, {
          placement,
          overflows
        }];
        if (!overflows.every((side2) => side2 <= 0)) {
          var _middlewareData$flip2, _overflowsData$filter;
          const nextIndex = (((_middlewareData$flip2 = middlewareData.flip) == null ? void 0 : _middlewareData$flip2.index) || 0) + 1;
          const nextPlacement = placements2[nextIndex];
          if (nextPlacement) {
            const ignoreCrossAxisOverflow = checkCrossAxis === "alignment" ? initialSideAxis !== getSideAxis(nextPlacement) : false;
            if (!ignoreCrossAxisOverflow || // We leave the current main axis only if every placement on that axis
            // overflows the main axis.
            overflowsData.every((d) => getSideAxis(d.placement) === initialSideAxis ? d.overflows[0] > 0 : true)) {
              return {
                data: {
                  index: nextIndex,
                  overflows: overflowsData
                },
                reset: {
                  placement: nextPlacement
                }
              };
            }
          }
          let resetPlacement = (_overflowsData$filter = overflowsData.filter((d) => d.overflows[0] <= 0).sort((a, b) => a.overflows[1] - b.overflows[1])[0]) == null ? void 0 : _overflowsData$filter.placement;
          if (!resetPlacement) {
            switch (fallbackStrategy) {
              case "bestFit": {
                var _overflowsData$filter2;
                const placement2 = (_overflowsData$filter2 = overflowsData.filter((d) => {
                  if (hasFallbackAxisSideDirection) {
                    const currentSideAxis = getSideAxis(d.placement);
                    return currentSideAxis === initialSideAxis || // Create a bias to the `y` side axis due to horizontal
                    // reading directions favoring greater width.
                    currentSideAxis === "y";
                  }
                  return true;
                }).map((d) => [d.placement, d.overflows.filter((overflow2) => overflow2 > 0).reduce((acc, overflow2) => acc + overflow2, 0)]).sort((a, b) => a[1] - b[1])[0]) == null ? void 0 : _overflowsData$filter2[0];
                if (placement2) {
                  resetPlacement = placement2;
                }
                break;
              }
              case "initialPlacement":
                resetPlacement = initialPlacement;
                break;
            }
          }
          if (placement !== resetPlacement) {
            return {
              reset: {
                placement: resetPlacement
              }
            };
          }
        }
        return {};
      }
    };
  };
  function getSideOffsets(overflow, rect) {
    return {
      top: overflow.top - rect.height,
      right: overflow.right - rect.width,
      bottom: overflow.bottom - rect.height,
      left: overflow.left - rect.width
    };
  }
  function isAnySideFullyClipped(overflow) {
    return sides.some((side) => overflow[side] >= 0);
  }
  var hide = function(options) {
    if (options === void 0) {
      options = {};
    }
    return {
      name: "hide",
      options,
      async fn(state) {
        const {
          rects,
          platform: platform2
        } = state;
        const {
          strategy = "referenceHidden",
          ...detectOverflowOptions
        } = evaluate(options, state);
        switch (strategy) {
          case "referenceHidden": {
            const overflow = await platform2.detectOverflow(state, {
              ...detectOverflowOptions,
              elementContext: "reference"
            });
            const offsets = getSideOffsets(overflow, rects.reference);
            return {
              data: {
                referenceHiddenOffsets: offsets,
                referenceHidden: isAnySideFullyClipped(offsets)
              }
            };
          }
          case "escaped": {
            const overflow = await platform2.detectOverflow(state, {
              ...detectOverflowOptions,
              altBoundary: true
            });
            const offsets = getSideOffsets(overflow, rects.floating);
            return {
              data: {
                escapedOffsets: offsets,
                escaped: isAnySideFullyClipped(offsets)
              }
            };
          }
          default: {
            return {};
          }
        }
      }
    };
  };
  var originSides = /* @__PURE__ */ new Set(["left", "top"]);
  async function convertValueToCoords(state, options) {
    const {
      placement,
      platform: platform2,
      elements
    } = state;
    const rtl = await (platform2.isRTL == null ? void 0 : platform2.isRTL(elements.floating));
    const side = getSide(placement);
    const alignment = getAlignment(placement);
    const isVertical = getSideAxis(placement) === "y";
    const mainAxisMulti = originSides.has(side) ? -1 : 1;
    const crossAxisMulti = rtl && isVertical ? -1 : 1;
    const rawValue = evaluate(options, state);
    let {
      mainAxis,
      crossAxis,
      alignmentAxis
    } = typeof rawValue === "number" ? {
      mainAxis: rawValue,
      crossAxis: 0,
      alignmentAxis: null
    } : {
      mainAxis: rawValue.mainAxis || 0,
      crossAxis: rawValue.crossAxis || 0,
      alignmentAxis: rawValue.alignmentAxis
    };
    if (alignment && typeof alignmentAxis === "number") {
      crossAxis = alignment === "end" ? alignmentAxis * -1 : alignmentAxis;
    }
    return isVertical ? {
      x: crossAxis * crossAxisMulti,
      y: mainAxis * mainAxisMulti
    } : {
      x: mainAxis * mainAxisMulti,
      y: crossAxis * crossAxisMulti
    };
  }
  var offset = function(options) {
    if (options === void 0) {
      options = 0;
    }
    return {
      name: "offset",
      options,
      async fn(state) {
        var _middlewareData$offse, _middlewareData$arrow;
        const {
          x,
          y,
          placement,
          middlewareData
        } = state;
        const diffCoords = await convertValueToCoords(state, options);
        if (placement === ((_middlewareData$offse = middlewareData.offset) == null ? void 0 : _middlewareData$offse.placement) && (_middlewareData$arrow = middlewareData.arrow) != null && _middlewareData$arrow.alignmentOffset) {
          return {};
        }
        return {
          x: x + diffCoords.x,
          y: y + diffCoords.y,
          data: {
            ...diffCoords,
            placement
          }
        };
      }
    };
  };
  var shift = function(options) {
    if (options === void 0) {
      options = {};
    }
    return {
      name: "shift",
      options,
      async fn(state) {
        const {
          x,
          y,
          placement,
          platform: platform2
        } = state;
        const {
          mainAxis: checkMainAxis = true,
          crossAxis: checkCrossAxis = false,
          limiter = {
            fn: (_ref) => {
              let {
                x: x2,
                y: y2
              } = _ref;
              return {
                x: x2,
                y: y2
              };
            }
          },
          ...detectOverflowOptions
        } = evaluate(options, state);
        const coords = {
          x,
          y
        };
        const overflow = await platform2.detectOverflow(state, detectOverflowOptions);
        const crossAxis = getSideAxis(placement);
        const mainAxis = getOppositeAxis(crossAxis);
        let mainAxisCoord = coords[mainAxis];
        let crossAxisCoord = coords[crossAxis];
        const clampCoord = (axis, coord) => clamp(coord + overflow[axis === "y" ? "top" : "left"], coord, coord - overflow[axis === "y" ? "bottom" : "right"]);
        if (checkMainAxis) {
          mainAxisCoord = clampCoord(mainAxis, mainAxisCoord);
        }
        if (checkCrossAxis) {
          crossAxisCoord = clampCoord(crossAxis, crossAxisCoord);
        }
        const limitedCoords = limiter.fn({
          ...state,
          [mainAxis]: mainAxisCoord,
          [crossAxis]: crossAxisCoord
        });
        return {
          ...limitedCoords,
          data: {
            x: limitedCoords.x - x,
            y: limitedCoords.y - y,
            enabled: {
              [mainAxis]: checkMainAxis,
              [crossAxis]: checkCrossAxis
            }
          }
        };
      }
    };
  };
  var limitShift = function(options) {
    if (options === void 0) {
      options = {};
    }
    return {
      options,
      fn(state) {
        var _rawOffset$mainAxis, _rawOffset$crossAxis;
        const {
          x,
          y,
          placement,
          rects,
          middlewareData
        } = state;
        const {
          offset: offset3 = 0,
          mainAxis: checkMainAxis = true,
          crossAxis: checkCrossAxis = true
        } = evaluate(options, state);
        const coords = {
          x,
          y
        };
        const crossAxis = getSideAxis(placement);
        const mainAxis = getOppositeAxis(crossAxis);
        let mainAxisCoord = coords[mainAxis];
        let crossAxisCoord = coords[crossAxis];
        const rawOffset = evaluate(offset3, state);
        const computedOffset = typeof rawOffset === "number" ? {
          mainAxis: rawOffset,
          crossAxis: 0
        } : {
          mainAxis: (_rawOffset$mainAxis = rawOffset.mainAxis) != null ? _rawOffset$mainAxis : 0,
          crossAxis: (_rawOffset$crossAxis = rawOffset.crossAxis) != null ? _rawOffset$crossAxis : 0
        };
        if (checkMainAxis) {
          const len = mainAxis === "y" ? "height" : "width";
          const limitMin = rects.reference[mainAxis] - rects.floating[len] + computedOffset.mainAxis;
          const limitMax = rects.reference[mainAxis] + rects.reference[len] - computedOffset.mainAxis;
          if (mainAxisCoord < limitMin) {
            mainAxisCoord = limitMin;
          } else if (mainAxisCoord > limitMax) {
            mainAxisCoord = limitMax;
          }
        }
        if (checkCrossAxis) {
          var _middlewareData$offse, _middlewareData$offse2;
          const len = mainAxis === "y" ? "width" : "height";
          const isOriginSide = originSides.has(getSide(placement));
          const limitMin = rects.reference[crossAxis] - rects.floating[len] + (isOriginSide ? ((_middlewareData$offse = middlewareData.offset) == null ? void 0 : _middlewareData$offse[crossAxis]) || 0 : 0) + (isOriginSide ? 0 : computedOffset.crossAxis);
          const limitMax = rects.reference[crossAxis] + rects.reference[len] + (isOriginSide ? 0 : ((_middlewareData$offse2 = middlewareData.offset) == null ? void 0 : _middlewareData$offse2[crossAxis]) || 0) - (isOriginSide ? computedOffset.crossAxis : 0);
          if (crossAxisCoord < limitMin) {
            crossAxisCoord = limitMin;
          } else if (crossAxisCoord > limitMax) {
            crossAxisCoord = limitMax;
          }
        }
        return {
          [mainAxis]: mainAxisCoord,
          [crossAxis]: crossAxisCoord
        };
      }
    };
  };
  var size = function(options) {
    if (options === void 0) {
      options = {};
    }
    return {
      name: "size",
      options,
      async fn(state) {
        const {
          placement,
          rects,
          platform: platform2,
          elements
        } = state;
        const {
          apply = () => {
          },
          ...detectOverflowOptions
        } = evaluate(options, state);
        const overflow = await platform2.detectOverflow(state, detectOverflowOptions);
        const side = getSide(placement);
        const alignment = getAlignment(placement);
        const isYAxis = getSideAxis(placement) === "y";
        const {
          width,
          height
        } = rects.floating;
        let heightSide;
        let widthSide;
        if (side === "top" || side === "bottom") {
          heightSide = side;
          widthSide = alignment === (await (platform2.isRTL == null ? void 0 : platform2.isRTL(elements.floating)) ? "start" : "end") ? "left" : "right";
        } else {
          widthSide = side;
          heightSide = alignment === "end" ? "top" : "bottom";
        }
        const maximumClippingHeight = height - overflow.top - overflow.bottom;
        const maximumClippingWidth = width - overflow.left - overflow.right;
        const overflowAvailableHeight = min(height - overflow[heightSide], maximumClippingHeight);
        const overflowAvailableWidth = min(width - overflow[widthSide], maximumClippingWidth);
        const shiftData = state.middlewareData.shift;
        const noShift = !shiftData;
        let availableHeight = overflowAvailableHeight;
        let availableWidth = overflowAvailableWidth;
        if (shiftData != null && shiftData.enabled.x) {
          availableWidth = maximumClippingWidth;
        }
        if (shiftData != null && shiftData.enabled.y) {
          availableHeight = maximumClippingHeight;
        }
        if (noShift && !alignment) {
          if (isYAxis) {
            availableWidth = width - 2 * max(overflow.left, overflow.right);
          } else {
            availableHeight = height - 2 * max(overflow.top, overflow.bottom);
          }
        }
        await apply({
          ...state,
          availableWidth,
          availableHeight
        });
        const nextDimensions = await platform2.getDimensions(elements.floating);
        if (width !== nextDimensions.width || height !== nextDimensions.height) {
          return {
            reset: {
              rects: true
            }
          };
        }
        return {};
      }
    };
  };

  // node_modules/@floating-ui/utils/dist/floating-ui.utils.dom.mjs
  function hasWindow() {
    return typeof window !== "undefined";
  }
  function getNodeName(node) {
    if (isNode2(node)) {
      return (node.nodeName || "").toLowerCase();
    }
    return "#document";
  }
  function getWindow(node) {
    var _node$ownerDocument;
    return (node == null || (_node$ownerDocument = node.ownerDocument) == null ? void 0 : _node$ownerDocument.defaultView) || window;
  }
  function getDocumentElement(node) {
    var _ref;
    return (_ref = (isNode2(node) ? node.ownerDocument : node.document) || window.document) == null ? void 0 : _ref.documentElement;
  }
  function isNode2(value) {
    if (!hasWindow()) {
      return false;
    }
    return value instanceof Node || value instanceof getWindow(value).Node;
  }
  function isElement(value) {
    if (!hasWindow()) {
      return false;
    }
    return value instanceof Element || value instanceof getWindow(value).Element;
  }
  function isHTMLElement(value) {
    if (!hasWindow()) {
      return false;
    }
    return value instanceof HTMLElement || value instanceof getWindow(value).HTMLElement;
  }
  function isShadowRoot(value) {
    if (!hasWindow() || typeof ShadowRoot === "undefined") {
      return false;
    }
    return value instanceof ShadowRoot || value instanceof getWindow(value).ShadowRoot;
  }
  function isOverflowElement(element) {
    const {
      overflow,
      overflowX,
      overflowY,
      display
    } = getComputedStyle2(element);
    return /auto|scroll|overlay|hidden|clip/.test(overflow + overflowY + overflowX) && display !== "inline" && display !== "contents";
  }
  function isTableElement(element) {
    return /^(table|td|th)$/.test(getNodeName(element));
  }
  function isTopLayer(element) {
    try {
      if (element.matches(":popover-open")) {
        return true;
      }
    } catch (_e) {
    }
    try {
      return element.matches(":modal");
    } catch (_e) {
      return false;
    }
  }
  var willChangeRe = /transform|translate|scale|rotate|perspective|filter/;
  var containRe = /paint|layout|strict|content/;
  var isNotNone = (value) => !!value && value !== "none";
  var isWebKitValue;
  function isContainingBlock(elementOrCss) {
    const css = isElement(elementOrCss) ? getComputedStyle2(elementOrCss) : elementOrCss;
    return isNotNone(css.transform) || isNotNone(css.translate) || isNotNone(css.scale) || isNotNone(css.rotate) || isNotNone(css.perspective) || !isWebKit() && (isNotNone(css.backdropFilter) || isNotNone(css.filter)) || willChangeRe.test(css.willChange || "") || containRe.test(css.contain || "");
  }
  function getContainingBlock(element) {
    let currentNode = getParentNode(element);
    while (isHTMLElement(currentNode) && !isLastTraversableNode(currentNode)) {
      if (isContainingBlock(currentNode)) {
        return currentNode;
      } else if (isTopLayer(currentNode)) {
        return null;
      }
      currentNode = getParentNode(currentNode);
    }
    return null;
  }
  function isWebKit() {
    if (isWebKitValue == null) {
      isWebKitValue = typeof CSS !== "undefined" && CSS.supports && CSS.supports("-webkit-backdrop-filter", "none");
    }
    return isWebKitValue;
  }
  function isLastTraversableNode(node) {
    return /^(html|body|#document)$/.test(getNodeName(node));
  }
  function getComputedStyle2(element) {
    return getWindow(element).getComputedStyle(element);
  }
  function getNodeScroll(element) {
    if (isElement(element)) {
      return {
        scrollLeft: element.scrollLeft,
        scrollTop: element.scrollTop
      };
    }
    return {
      scrollLeft: element.scrollX,
      scrollTop: element.scrollY
    };
  }
  function getParentNode(node) {
    if (getNodeName(node) === "html") {
      return node;
    }
    const result = (
      // Step into the shadow DOM of the parent of a slotted node.
      node.assignedSlot || // DOM Element detected.
      node.parentNode || // ShadowRoot detected.
      isShadowRoot(node) && node.host || // Fallback.
      getDocumentElement(node)
    );
    return isShadowRoot(result) ? result.host : result;
  }
  function getNearestOverflowAncestor(node) {
    const parentNode = getParentNode(node);
    if (isLastTraversableNode(parentNode)) {
      return (node.ownerDocument || node).body;
    }
    if (isHTMLElement(parentNode) && isOverflowElement(parentNode)) {
      return parentNode;
    }
    return getNearestOverflowAncestor(parentNode);
  }
  function getOverflowAncestors(node, list, traverseIframes) {
    var _node$ownerDocument2;
    if (list === void 0) {
      list = [];
    }
    if (traverseIframes === void 0) {
      traverseIframes = true;
    }
    const scrollableAncestor = getNearestOverflowAncestor(node);
    const isBody = scrollableAncestor === ((_node$ownerDocument2 = node.ownerDocument) == null ? void 0 : _node$ownerDocument2.body);
    const win = getWindow(scrollableAncestor);
    if (isBody) {
      const frameElement = getFrameElement(win);
      return list.concat(win, win.visualViewport || [], isOverflowElement(scrollableAncestor) ? scrollableAncestor : [], frameElement && traverseIframes ? getOverflowAncestors(frameElement) : []);
    } else {
      return list.concat(scrollableAncestor, getOverflowAncestors(scrollableAncestor, [], traverseIframes));
    }
  }
  function getFrameElement(win) {
    return win.parent && Object.getPrototypeOf(win.parent) ? win.frameElement : null;
  }

  // node_modules/@floating-ui/dom/dist/floating-ui.dom.mjs
  function getCssDimensions(element) {
    const css = getComputedStyle2(element);
    let width = parseFloat(css.width) || 0;
    let height = parseFloat(css.height) || 0;
    const hasOffset = isHTMLElement(element);
    const offsetWidth = hasOffset ? element.offsetWidth : width;
    const offsetHeight = hasOffset ? element.offsetHeight : height;
    const shouldFallback = round(width) !== offsetWidth || round(height) !== offsetHeight;
    if (shouldFallback) {
      width = offsetWidth;
      height = offsetHeight;
    }
    return {
      width,
      height,
      $: shouldFallback
    };
  }
  function unwrapElement(element) {
    return !isElement(element) ? element.contextElement : element;
  }
  function getScale(element) {
    const domElement = unwrapElement(element);
    if (!isHTMLElement(domElement)) {
      return createCoords(1);
    }
    const rect = domElement.getBoundingClientRect();
    const {
      width,
      height,
      $
    } = getCssDimensions(domElement);
    let x = ($ ? round(rect.width) : rect.width) / width;
    let y = ($ ? round(rect.height) : rect.height) / height;
    if (!x || !Number.isFinite(x)) {
      x = 1;
    }
    if (!y || !Number.isFinite(y)) {
      y = 1;
    }
    return {
      x,
      y
    };
  }
  var noOffsets = /* @__PURE__ */ createCoords(0);
  function getVisualOffsets(element) {
    const win = getWindow(element);
    if (!isWebKit() || !win.visualViewport) {
      return noOffsets;
    }
    return {
      x: win.visualViewport.offsetLeft,
      y: win.visualViewport.offsetTop
    };
  }
  function shouldAddVisualOffsets(element, isFixed, floatingOffsetParent) {
    if (isFixed === void 0) {
      isFixed = false;
    }
    return !!floatingOffsetParent && isFixed && floatingOffsetParent === getWindow(element);
  }
  function getBoundingClientRect(element, includeScale, isFixedStrategy, offsetParent) {
    if (includeScale === void 0) {
      includeScale = false;
    }
    if (isFixedStrategy === void 0) {
      isFixedStrategy = false;
    }
    const clientRect = element.getBoundingClientRect();
    const domElement = unwrapElement(element);
    let scale = createCoords(1);
    if (includeScale) {
      if (offsetParent) {
        if (isElement(offsetParent)) {
          scale = getScale(offsetParent);
        }
      } else {
        scale = getScale(element);
      }
    }
    const visualOffsets = shouldAddVisualOffsets(domElement, isFixedStrategy, offsetParent) ? getVisualOffsets(domElement) : createCoords(0);
    let x = (clientRect.left + visualOffsets.x) / scale.x;
    let y = (clientRect.top + visualOffsets.y) / scale.y;
    let width = clientRect.width / scale.x;
    let height = clientRect.height / scale.y;
    if (domElement && offsetParent) {
      const win = getWindow(domElement);
      const offsetWin = isElement(offsetParent) ? getWindow(offsetParent) : offsetParent;
      let currentWin = win;
      let currentIFrame = getFrameElement(currentWin);
      while (currentIFrame && offsetWin !== currentWin) {
        const iframeScale = getScale(currentIFrame);
        const iframeRect = currentIFrame.getBoundingClientRect();
        const css = getComputedStyle2(currentIFrame);
        const left = iframeRect.left + (currentIFrame.clientLeft + parseFloat(css.paddingLeft)) * iframeScale.x;
        const top = iframeRect.top + (currentIFrame.clientTop + parseFloat(css.paddingTop)) * iframeScale.y;
        x *= iframeScale.x;
        y *= iframeScale.y;
        width *= iframeScale.x;
        height *= iframeScale.y;
        x += left;
        y += top;
        currentWin = getWindow(currentIFrame);
        currentIFrame = getFrameElement(currentWin);
      }
    }
    return rectToClientRect({
      width,
      height,
      x,
      y
    });
  }
  function getWindowScrollBarX(element, rect) {
    const leftScroll = getNodeScroll(element).scrollLeft;
    if (!rect) {
      return getBoundingClientRect(getDocumentElement(element)).left + leftScroll;
    }
    return rect.left + leftScroll;
  }
  function getHTMLOffset(documentElement, scroll) {
    const htmlRect = documentElement.getBoundingClientRect();
    const x = htmlRect.left + scroll.scrollLeft - getWindowScrollBarX(documentElement, htmlRect);
    const y = htmlRect.top + scroll.scrollTop;
    return {
      x,
      y
    };
  }
  function convertOffsetParentRelativeRectToViewportRelativeRect(_ref) {
    let {
      elements,
      rect,
      offsetParent,
      strategy
    } = _ref;
    const isFixed = strategy === "fixed";
    const documentElement = getDocumentElement(offsetParent);
    const topLayer = elements ? isTopLayer(elements.floating) : false;
    if (offsetParent === documentElement || topLayer && isFixed) {
      return rect;
    }
    let scroll = {
      scrollLeft: 0,
      scrollTop: 0
    };
    let scale = createCoords(1);
    const offsets = createCoords(0);
    const isOffsetParentAnElement = isHTMLElement(offsetParent);
    if (isOffsetParentAnElement || !isFixed) {
      if (getNodeName(offsetParent) !== "body" || isOverflowElement(documentElement)) {
        scroll = getNodeScroll(offsetParent);
      }
      if (isOffsetParentAnElement) {
        const offsetRect = getBoundingClientRect(offsetParent);
        scale = getScale(offsetParent);
        offsets.x = offsetRect.x + offsetParent.clientLeft;
        offsets.y = offsetRect.y + offsetParent.clientTop;
      }
    }
    const htmlOffset = documentElement && !isOffsetParentAnElement && !isFixed ? getHTMLOffset(documentElement, scroll) : createCoords(0);
    return {
      width: rect.width * scale.x,
      height: rect.height * scale.y,
      x: rect.x * scale.x - scroll.scrollLeft * scale.x + offsets.x + htmlOffset.x,
      y: rect.y * scale.y - scroll.scrollTop * scale.y + offsets.y + htmlOffset.y
    };
  }
  function getClientRects(element) {
    return element.getClientRects ? Array.from(element.getClientRects()) : [];
  }
  function getDocumentRect(html) {
    const scroll = getNodeScroll(html);
    const body = html.ownerDocument.body;
    const width = max(html.scrollWidth, html.clientWidth, body.scrollWidth, body.clientWidth);
    const height = max(html.scrollHeight, html.clientHeight, body.scrollHeight, body.clientHeight);
    let x = -scroll.scrollLeft + getWindowScrollBarX(html);
    const y = -scroll.scrollTop;
    if (getComputedStyle2(body).direction === "rtl") {
      x += max(html.clientWidth, body.clientWidth) - width;
    }
    return {
      width,
      height,
      x,
      y
    };
  }
  var SCROLLBAR_MAX = 25;
  function getViewportRect(element, strategy, rootBoundary) {
    if (rootBoundary === void 0) {
      rootBoundary = "viewport";
    }
    const isLayoutViewport = rootBoundary === "layoutViewport";
    const win = getWindow(element);
    const html = getDocumentElement(element);
    const visualViewport = win.visualViewport;
    let width = html.clientWidth;
    let height = html.clientHeight;
    let x = 0;
    let y = 0;
    if (visualViewport) {
      const layoutRelativeClientCoords = !isWebKit() || strategy === "fixed";
      if (isLayoutViewport) {
        if (!layoutRelativeClientCoords) {
          x = -visualViewport.offsetLeft;
          y = -visualViewport.offsetTop;
        }
      } else {
        width = visualViewport.width;
        height = visualViewport.height;
        if (layoutRelativeClientCoords) {
          x = visualViewport.offsetLeft;
          y = visualViewport.offsetTop;
        }
      }
    }
    const windowScrollbarX = getWindowScrollBarX(html);
    if (windowScrollbarX <= 0) {
      const doc = html.ownerDocument;
      const body = doc.body;
      const bodyStyles = getComputedStyle(body);
      const bodyMarginInline = doc.compatMode === "CSS1Compat" ? parseFloat(bodyStyles.marginLeft) + parseFloat(bodyStyles.marginRight) || 0 : 0;
      const reservedWidth = Math.abs(html.clientWidth - body.clientWidth - bodyMarginInline);
      const gutter = getComputedStyle(html).scrollbarGutter === "stable both-edges" ? reservedWidth / 2 : reservedWidth;
      if (gutter <= SCROLLBAR_MAX) {
        width -= gutter;
      }
    }
    return {
      width,
      height,
      x,
      y
    };
  }
  function getInnerBoundingClientRect(element, strategy) {
    const clientRect = getBoundingClientRect(element, true, strategy === "fixed");
    const top = clientRect.top + element.clientTop;
    const left = clientRect.left + element.clientLeft;
    const scale = getScale(element);
    const width = element.clientWidth * scale.x;
    const height = element.clientHeight * scale.y;
    const x = left * scale.x;
    const y = top * scale.y;
    return {
      width,
      height,
      x,
      y
    };
  }
  function getClientRectFromClippingAncestor(element, clippingAncestor, strategy) {
    let rect;
    if (clippingAncestor === "viewport" || clippingAncestor === "layoutViewport") {
      rect = getViewportRect(element, strategy, clippingAncestor);
    } else if (clippingAncestor === "document") {
      rect = getDocumentRect(getDocumentElement(element));
    } else if (isElement(clippingAncestor)) {
      rect = getInnerBoundingClientRect(clippingAncestor, strategy);
    } else {
      const visualOffsets = getVisualOffsets(element);
      rect = {
        x: clippingAncestor.x - visualOffsets.x,
        y: clippingAncestor.y - visualOffsets.y,
        width: clippingAncestor.width,
        height: clippingAncestor.height
      };
    }
    return rectToClientRect(rect);
  }
  function getClippingElementAncestors(element, cache) {
    const cachedResult = cache.get(element);
    if (cachedResult) {
      return cachedResult;
    }
    let result = getOverflowAncestors(element, [], false).filter((el) => isElement(el) && getNodeName(el) !== "body");
    let lastKeptComputedStyle = null;
    const elementIsFixed = getComputedStyle2(element).position === "fixed";
    let currentNode = elementIsFixed ? getParentNode(element) : element;
    while (isElement(currentNode) && !isLastTraversableNode(currentNode)) {
      const computedStyle = getComputedStyle2(currentNode);
      const currentNodeIsContaining = isContainingBlock(currentNode);
      const lastPosition = lastKeptComputedStyle ? lastKeptComputedStyle.position : elementIsFixed ? "fixed" : "";
      const shouldDropCurrentNode = !currentNodeIsContaining && (lastPosition === "fixed" || lastPosition === "absolute" && computedStyle.position === "static");
      if (shouldDropCurrentNode) {
        result = result.filter((ancestor) => ancestor !== currentNode);
      } else {
        lastKeptComputedStyle = computedStyle;
      }
      currentNode = getParentNode(currentNode);
    }
    cache.set(element, result);
    return result;
  }
  function getClippingRect(_ref) {
    let {
      element,
      boundary,
      rootBoundary,
      strategy
    } = _ref;
    const elementClippingAncestors = boundary === "clippingAncestors" ? isTopLayer(element) ? [] : getClippingElementAncestors(element, this._c) : [].concat(boundary);
    const clippingAncestors = [...elementClippingAncestors, rootBoundary];
    const firstRect = getClientRectFromClippingAncestor(element, clippingAncestors[0], strategy);
    let top = firstRect.top;
    let right = firstRect.right;
    let bottom = firstRect.bottom;
    let left = firstRect.left;
    for (let i = 1; i < clippingAncestors.length; i++) {
      const rect = getClientRectFromClippingAncestor(element, clippingAncestors[i], strategy);
      top = max(rect.top, top);
      right = min(rect.right, right);
      bottom = min(rect.bottom, bottom);
      left = max(rect.left, left);
    }
    return {
      width: right - left,
      height: bottom - top,
      x: left,
      y: top
    };
  }
  function getDimensions(element) {
    const {
      width,
      height
    } = getCssDimensions(element);
    return {
      width,
      height
    };
  }
  function getRectRelativeToOffsetParent(element, offsetParent, strategy) {
    const isOffsetParentAnElement = isHTMLElement(offsetParent);
    const documentElement = getDocumentElement(offsetParent);
    const isFixed = strategy === "fixed";
    const rect = getBoundingClientRect(element, true, isFixed, offsetParent);
    let scroll = {
      scrollLeft: 0,
      scrollTop: 0
    };
    const offsets = createCoords(0);
    if (isOffsetParentAnElement || !isFixed) {
      if (getNodeName(offsetParent) !== "body" || isOverflowElement(documentElement)) {
        scroll = getNodeScroll(offsetParent);
      }
      if (isOffsetParentAnElement) {
        const offsetRect = getBoundingClientRect(offsetParent, true, isFixed, offsetParent);
        offsets.x = offsetRect.x + offsetParent.clientLeft;
        offsets.y = offsetRect.y + offsetParent.clientTop;
      }
    }
    if (!isOffsetParentAnElement && documentElement) {
      offsets.x = getWindowScrollBarX(documentElement);
    }
    const htmlOffset = documentElement && !isOffsetParentAnElement && !isFixed ? getHTMLOffset(documentElement, scroll) : createCoords(0);
    const x = rect.left + scroll.scrollLeft - offsets.x - htmlOffset.x;
    const y = rect.top + scroll.scrollTop - offsets.y - htmlOffset.y;
    return {
      x,
      y,
      width: rect.width,
      height: rect.height
    };
  }
  function isStaticPositioned(element) {
    return getComputedStyle2(element).position === "static";
  }
  function getTrueOffsetParent(element, polyfill) {
    if (!isHTMLElement(element) || getComputedStyle2(element).position === "fixed") {
      return null;
    }
    if (polyfill) {
      return polyfill(element);
    }
    let rawOffsetParent = element.offsetParent;
    if (getDocumentElement(element) === rawOffsetParent) {
      rawOffsetParent = rawOffsetParent.ownerDocument.body;
    }
    return rawOffsetParent;
  }
  function getOffsetParent(element, polyfill) {
    const win = getWindow(element);
    if (isTopLayer(element)) {
      return win;
    }
    if (!isHTMLElement(element)) {
      let svgOffsetParent = getParentNode(element);
      while (svgOffsetParent && !isLastTraversableNode(svgOffsetParent)) {
        if (isElement(svgOffsetParent) && !isStaticPositioned(svgOffsetParent)) {
          return svgOffsetParent;
        }
        svgOffsetParent = getParentNode(svgOffsetParent);
      }
      return win;
    }
    let offsetParent = getTrueOffsetParent(element, polyfill);
    while (offsetParent && isTableElement(offsetParent) && isStaticPositioned(offsetParent)) {
      offsetParent = getTrueOffsetParent(offsetParent, polyfill);
    }
    if (offsetParent && isLastTraversableNode(offsetParent) && isStaticPositioned(offsetParent) && !isContainingBlock(offsetParent)) {
      return win;
    }
    return offsetParent || getContainingBlock(element) || win;
  }
  var getElementRects = async function(data) {
    const getOffsetParentFn = this.getOffsetParent || getOffsetParent;
    const getDimensionsFn = this.getDimensions;
    const floatingDimensions = await getDimensionsFn(data.floating);
    return {
      reference: getRectRelativeToOffsetParent(data.reference, await getOffsetParentFn(data.floating), data.strategy),
      floating: {
        x: 0,
        y: 0,
        width: floatingDimensions.width,
        height: floatingDimensions.height
      }
    };
  };
  function isRTL(element) {
    return getComputedStyle2(element).direction === "rtl";
  }
  var platform = {
    convertOffsetParentRelativeRectToViewportRelativeRect,
    getDocumentElement,
    getClippingRect,
    getOffsetParent,
    getElementRects,
    getClientRects,
    getDimensions,
    getScale,
    isElement,
    isRTL
  };
  function rectsAreEqual(a, b) {
    return a.x === b.x && a.y === b.y && a.width === b.width && a.height === b.height;
  }
  function observeMove(element, onMove, ancestorResize) {
    let io = null;
    let timeoutId;
    const root = getDocumentElement(element);
    function cleanup() {
      var _io;
      clearTimeout(timeoutId);
      (_io = io) == null || _io.disconnect();
      io = null;
    }
    function refresh(skip, threshold) {
      if (skip === void 0) {
        skip = false;
      }
      if (threshold === void 0) {
        threshold = 1;
      }
      cleanup();
      const elementRectForRootMargin = element.getBoundingClientRect();
      const {
        left,
        top,
        width,
        height
      } = elementRectForRootMargin;
      if (!skip) {
        onMove();
      }
      if (!width || !height) {
        return;
      }
      const insetTop = floor(top);
      const insetRight = floor(root.clientWidth - (left + width));
      const insetBottom = floor(root.clientHeight - (top + height));
      const insetLeft = floor(left);
      const rootMargin = -insetTop + "px " + -insetRight + "px " + -insetBottom + "px " + -insetLeft + "px";
      const options = {
        rootMargin,
        threshold: max(0, min(1, threshold)) || 1
      };
      let isFirstUpdate = true;
      function handleObserve(entries) {
        const ratio = entries[0].intersectionRatio;
        if (!rectsAreEqual(elementRectForRootMargin, element.getBoundingClientRect())) {
          return refresh();
        }
        if (ratio !== threshold) {
          if (!isFirstUpdate) {
            return refresh();
          }
          if (!ratio) {
            timeoutId = setTimeout(() => {
              refresh(false, 1e-7);
            }, 1e3);
          } else {
            refresh(false, ratio);
          }
        }
        isFirstUpdate = false;
      }
      try {
        io = new IntersectionObserver(handleObserve, {
          ...options,
          // Handle <iframe>s
          root: root.ownerDocument
        });
      } catch (_e) {
        io = new IntersectionObserver(handleObserve, options);
      }
      io.observe(element);
    }
    const win = getWindow(element);
    const handleResize = () => refresh(ancestorResize);
    win.addEventListener("resize", handleResize);
    refresh(true);
    return () => {
      win.removeEventListener("resize", handleResize);
      cleanup();
    };
  }
  function autoUpdate(reference, floating, update, options) {
    if (options === void 0) {
      options = {};
    }
    const {
      ancestorScroll = true,
      ancestorResize = true,
      elementResize = typeof ResizeObserver === "function",
      layoutShift = typeof IntersectionObserver === "function",
      animationFrame = false
    } = options;
    const referenceEl = unwrapElement(reference);
    const ancestors = ancestorScroll || ancestorResize ? [...referenceEl ? getOverflowAncestors(referenceEl) : [], ...floating ? getOverflowAncestors(floating) : []] : [];
    ancestors.forEach((ancestor) => {
      ancestorScroll && ancestor.addEventListener("scroll", update);
      ancestorResize && ancestor.addEventListener("resize", update);
    });
    const cleanupIo = referenceEl && layoutShift ? observeMove(referenceEl, update, ancestorResize) : null;
    let reobserveFrame = -1;
    let resizeObserver = null;
    if (elementResize) {
      resizeObserver = new ResizeObserver((_ref) => {
        let [firstEntry] = _ref;
        if (firstEntry && firstEntry.target === referenceEl && resizeObserver && floating) {
          resizeObserver.unobserve(floating);
          cancelAnimationFrame(reobserveFrame);
          reobserveFrame = requestAnimationFrame(() => {
            var _resizeObserver;
            (_resizeObserver = resizeObserver) == null || _resizeObserver.observe(floating);
          });
        }
        update();
      });
      if (referenceEl && !animationFrame) {
        resizeObserver.observe(referenceEl);
      }
      if (floating) {
        resizeObserver.observe(floating);
      }
    }
    let frameId;
    let prevRefRect = animationFrame ? getBoundingClientRect(reference) : null;
    if (animationFrame) {
      frameLoop();
    }
    function frameLoop() {
      const nextRefRect = getBoundingClientRect(reference);
      if (prevRefRect && !rectsAreEqual(prevRefRect, nextRefRect)) {
        update();
      }
      prevRefRect = nextRefRect;
      frameId = requestAnimationFrame(frameLoop);
    }
    update();
    return () => {
      var _resizeObserver2;
      ancestors.forEach((ancestor) => {
        ancestorScroll && ancestor.removeEventListener("scroll", update);
        ancestorResize && ancestor.removeEventListener("resize", update);
      });
      cleanupIo == null || cleanupIo();
      (_resizeObserver2 = resizeObserver) == null || _resizeObserver2.disconnect();
      resizeObserver = null;
      if (animationFrame) {
        cancelAnimationFrame(frameId);
      }
    };
  }
  var offset2 = offset;
  var shift2 = shift;
  var flip2 = flip;
  var size2 = size;
  var hide2 = hide;
  var arrow2 = arrow;
  var limitShift2 = limitShift;
  var computePosition2 = (reference, floating, options) => {
    const cache = /* @__PURE__ */ new Map();
    const mergedOptions = options != null ? options : {};
    const platformWithCache = {
      ...platform,
      ...mergedOptions.platform,
      _c: cache
    };
    return computePosition(reference, floating, {
      ...mergedOptions,
      platform: platformWithCache
    });
  };

  // src/popper.ts
  function getSideAndAlignFromPlacement(placement) {
    const [side, align = "center"] = placement.split("-");
    return [side, align ?? "center"];
  }
  var OPPOSITE_SIDE = { top: "bottom", right: "left", bottom: "top", left: "right" };
  function roundByDPR(element, value) {
    const dpr = Math.max(1, window.devicePixelRatio || 1);
    return Math.round(value * dpr) / dpr;
  }
  function transformOriginMiddleware(options) {
    return {
      name: "transformOrigin",
      options,
      async fn(data) {
        const { placement, rects, middlewareData } = data;
        const cannotCenterArrow = middlewareData.arrow?.centerOffset !== 0;
        const isArrowHidden = cannotCenterArrow;
        const arrowWidth = isArrowHidden ? 0 : options.arrowWidth;
        const arrowHeight = isArrowHidden ? 0 : options.arrowHeight;
        const [placedSide, placedAlign] = getSideAndAlignFromPlacement(placement);
        const noArrowAlign = { start: "0%", center: "50%", end: "100%" };
        const arrowXCenter = (middlewareData.arrow?.x ?? 0) + arrowWidth / 2;
        const arrowYCenter = (middlewareData.arrow?.y ?? 0) + arrowHeight / 2;
        let x = "";
        let y = "";
        if (placedSide === "bottom") {
          x = isArrowHidden ? noArrowAlign[placedAlign] : `${arrowXCenter}px`;
          y = `${-arrowHeight}px`;
        } else if (placedSide === "top") {
          x = isArrowHidden ? noArrowAlign[placedAlign] : `${arrowXCenter}px`;
          y = `${rects.floating.height + arrowHeight}px`;
        } else if (placedSide === "right") {
          x = `${-arrowHeight}px`;
          y = isArrowHidden ? noArrowAlign[placedAlign] : `${arrowYCenter}px`;
        } else if (placedSide === "left") {
          x = `${rects.floating.width + arrowHeight}px`;
          y = isArrowHidden ? noArrowAlign[placedAlign] : `${arrowYCenter}px`;
        }
        return { data: { x, y } };
      }
    };
  }
  function positionPopper(elements, options = {}, onPlaced) {
    const {
      side = "bottom",
      sideOffset = 0,
      align = "center",
      alignOffset = 0,
      arrowPadding = 0,
      avoidCollisions = true,
      collisionBoundary = [],
      collisionPadding: collisionPaddingProp = 0,
      sticky = "partial",
      hideWhenDetached = false,
      updatePositionStrategy = "optimized"
    } = options;
    const { anchor, floating, content, arrow: arrow3 } = elements;
    const desiredPlacement = side + (align !== "center" ? `-${align}` : "");
    const collisionPadding = typeof collisionPaddingProp === "number" ? collisionPaddingProp : { top: 0, right: 0, bottom: 0, left: 0, ...collisionPaddingProp };
    const boundary = Array.isArray(collisionBoundary) ? collisionBoundary : [collisionBoundary];
    const hasExplicitBoundaries = boundary.length > 0;
    const detectOverflowOptions = {
      padding: collisionPadding,
      boundary: boundary.filter((b) => b !== null),
      altBoundary: hasExplicitBoundaries
    };
    const fs = floating.style;
    fs.setProperty("position", "fixed");
    fs.setProperty("left", "0");
    fs.setProperty("top", "0");
    fs.setProperty("transform", "translate(0, -200%)");
    fs.setProperty("min-width", "max-content");
    content.style.setProperty("animation", "none");
    const arrowStyleFor = (placedSide, arrowData) => {
      const entries = [
        ["position", "absolute"],
        ["left", arrowData ? `${arrowData.x ?? 0}px` : void 0],
        ["top", arrowData ? `${arrowData.y ?? 0}px` : void 0],
        [OPPOSITE_SIDE[placedSide], "0"],
        [
          "transform-origin",
          { top: "", right: "0 0", bottom: "center 0", left: "100% 0" }[placedSide] || void 0
        ],
        [
          "transform",
          {
            top: "translateY(100%)",
            right: "translateY(50%) rotate(90deg) translateX(-50%)",
            bottom: "rotate(180deg)",
            left: "translateY(50%) rotate(-90deg) translateX(50%)"
          }[placedSide]
        ],
        ["visibility", arrowData?.centerOffset !== void 0 && arrowData.centerOffset !== 0 ? "hidden" : void 0]
      ];
      return entries.filter(([k, v]) => v !== void 0 || k === "left" || k === "top" || k === "visibility");
    };
    const applyArrowStyles = (placedSide, arrowData) => {
      if (!arrow3) return;
      const as = arrow3.style;
      const next = arrowStyleFor(placedSide, arrowData);
      const nextKeys = new Set(next.filter(([, v]) => v !== void 0).map(([k]) => k));
      for (const [k, v] of next) if (v !== void 0) as.setProperty(k, v);
      for (const k of arrowKeys) if (!nextKeys.has(k)) as.removeProperty(k);
      arrowKeys = nextKeys;
    };
    let arrowKeys = /* @__PURE__ */ new Set();
    applyArrowStyles(side);
    let arrowWidth = 0;
    let arrowHeight = 0;
    if (arrow3) {
      const cs = window.getComputedStyle(arrow3);
      arrowWidth = parseFloat(cs.width) || 0;
      arrowHeight = parseFloat(cs.height) || 0;
    }
    let sizeVars = [];
    const middleware = [
      offset2({ mainAxis: sideOffset + arrowHeight, alignmentAxis: alignOffset }),
      avoidCollisions && shift2({
        mainAxis: true,
        crossAxis: false,
        limiter: sticky === "partial" ? limitShift2() : void 0,
        ...detectOverflowOptions
      }),
      avoidCollisions && flip2({ ...detectOverflowOptions }),
      size2({
        ...detectOverflowOptions,
        apply: ({ rects, availableWidth, availableHeight }) => {
          const { width: anchorWidth, height: anchorHeight } = rects.reference;
          sizeVars = [
            ["--radix-popper-available-width", `${availableWidth}px`],
            ["--radix-popper-available-height", `${availableHeight}px`],
            ["--radix-popper-anchor-width", `${anchorWidth}px`],
            ["--radix-popper-anchor-height", `${anchorHeight}px`]
          ];
        }
      }),
      arrow3 && arrow2({ element: arrow3, padding: arrowPadding }),
      transformOriginMiddleware({ arrowWidth, arrowHeight }),
      hideWhenDetached && hide2({ strategy: "referenceHidden", ...detectOverflowOptions })
    ].filter(Boolean);
    let placement;
    let destroyed = false;
    const update = () => {
      void computePosition2(anchor, floating, {
        strategy: "fixed",
        placement: desiredPlacement,
        middleware
      }).then((data) => {
        if (destroyed) return;
        placement = data.placement;
        const x = roundByDPR(floating, data.x);
        const y = roundByDPR(floating, data.y);
        const [placedSide, placedAlign] = getSideAndAlignFromPlacement(data.placement);
        fs.setProperty("transform", `translate(${x}px, ${y}px)`);
        const originX = data.middlewareData.transformOrigin?.x;
        const originY = data.middlewareData.transformOrigin?.y;
        fs.setProperty("--radix-popper-transform-origin", [originX, originY].join(" "));
        fs.setProperty("z-index", window.getComputedStyle(content).zIndex);
        for (const [prop, value] of sizeVars) fs.setProperty(prop, value);
        const hidden = data.middlewareData.hide?.referenceHidden;
        if (hidden) {
          fs.setProperty("visibility", "hidden");
          fs.setProperty("pointer-events", "none");
        } else {
          fs.removeProperty("visibility");
          fs.removeProperty("pointer-events");
        }
        content.setAttribute("data-side", placedSide);
        content.setAttribute("data-align", placedAlign);
        content.style.removeProperty("animation");
        const arrowData = data.middlewareData.arrow;
        applyArrowStyles(placedSide, arrowData);
        onPlaced?.(placedSide, placedAlign, data.placement);
      });
    };
    let cleanupAutoUpdate;
    if (typeof ResizeObserver === "undefined") {
      update();
      cleanupAutoUpdate = () => void 0;
    } else {
      cleanupAutoUpdate = autoUpdate(anchor, floating, update, {
        animationFrame: updatePositionStrategy === "always"
      });
    }
    return {
      update,
      destroy() {
        destroyed = true;
        cleanupAutoUpdate();
      },
      getPlacement: () => placement
    };
  }

  // src/features/popover.ts
  function wirePopover(options) {
    const { content, trigger, onClosed } = options;
    const anchor = options.anchor ?? trigger;
    const doc = content.ownerDocument;
    const body = doc.body;
    const wrapper = options.wrapper ?? (() => {
      const w = doc.createElement("div");
      w.setAttribute("data-radix-popper-content-wrapper", "");
      return w;
    })();
    if (!wrapper.contains(content)) wrapper.appendChild(content);
    const externalGuards = options.guards;
    const guardStart = externalGuards?.start ?? makeGuard2(doc);
    const guardEnd = externalGuards?.end ?? makeGuard2(doc);
    body.insertBefore(guardStart, body.firstChild);
    body.appendChild(wrapper);
    body.appendChild(guardEnd);
    if (options.contentStateFlip) content.setAttribute("data-state", "open");
    if (trigger) {
      trigger.setAttribute("data-state", "open");
      trigger.setAttribute("aria-expanded", "true");
      trigger.setAttribute("aria-controls", content.id);
    }
    const popper = positionPopper(
      { anchor: anchorAsReference(anchor), floating: wrapper, content },
      options.popperOptions ?? {},
      (side, align) => {
        anchor?.setAttribute("data-radix-popper-side", side);
        anchor?.setAttribute("data-radix-popper-align", align);
      }
    );
    if (anchor && options.popperOptions) {
      anchor.setAttribute("data-radix-popper-side", options.popperOptions.side ?? "bottom");
      anchor.setAttribute("data-radix-popper-align", options.popperOptions.align ?? "center");
    }
    const layer = registerDismissLayer(
      trigger ? [content, trigger] : [content],
      () => close(options.closeFocus === "parametrized" ? false : void 0)
    );
    const onPointerDown = (event) => {
      const target = event.target;
      if (!isNode3(target)) return;
      dismissLayersOutside(target);
    };
    const onKeyDown = (event) => {
      if (event.key === "Escape") {
        if (options.escapeClose === false) return;
        if (!isHighestDismissLayer(layer)) return;
        if (options.escapePreventDefault) event.preventDefault();
        close();
        return;
      }
      if (options.tabCycle && event.key === "Tab") {
        const focusables2 = focusableIn2(content);
        if (focusables2.length === 0) return;
        const active = doc.activeElement;
        const idx = focusables2.indexOf(active);
        if (idx === -1 && active !== content) return;
        event.preventDefault();
        const next = event.shiftKey ? idx <= 0 ? focusables2.length - 1 : idx - 1 : idx === focusables2.length - 1 ? 0 : idx + 1;
        next >= 0 && focusables2[next].focus?.();
      }
    };
    doc.addEventListener("pointerdown", onPointerDown);
    doc.addEventListener("keydown", onKeyDown);
    const focusables = focusableIn2(content);
    (focusables[0] ?? content).focus?.();
    let closed = false;
    const close = (restoreFocus) => {
      if (closed) return;
      closed = true;
      unregisterDismissLayer(layer);
      doc.removeEventListener("pointerdown", onPointerDown);
      doc.removeEventListener("keydown", onKeyDown);
      popper.destroy();
      if (options.contentStateFlip) content.setAttribute("data-state", "closed");
      if (trigger) {
        trigger.setAttribute("data-state", "closed");
        trigger.setAttribute("aria-expanded", "false");
        trigger.removeAttribute("aria-controls");
        const restore = options.closeFocus === "parametrized" ? restoreFocus !== false : true;
        if (restore) trigger.focus?.();
      }
      anchor?.removeAttribute("data-radix-popper-side");
      anchor?.removeAttribute("data-radix-popper-align");
      const removeDom = () => {
        wrapper.remove();
        if (!externalGuards) {
          guardStart.remove();
          guardEnd.remove();
        }
      };
      if (options.unmount === "exit-window") {
        presenceExit(content, options.exitDuration ?? 100, removeDom);
      } else {
        removeDom();
      }
      onClosed?.();
    };
    return { close, popper: () => popper };
  }
  function anchorAsReference(anchor) {
    if (anchor === void 0) {
      return {
        getBoundingClientRect: () => ({ x: 0, y: 0, top: 0, left: 0, right: 0, bottom: 0, width: 0, height: 0 })
      };
    }
    return anchor;
  }
  function makeGuard2(doc) {
    const guard = doc.createElement("span");
    guard.setAttribute("data-radix-focus-guard", "");
    guard.setAttribute("style", "outline: none; opacity: 0; position: fixed; pointer-events: none;");
    guard.setAttribute("tabindex", "0");
    return guard;
  }
  var FOCUSABLE_SELECTOR2 = 'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])';
  function focusableIn2(root) {
    const doc = root.ownerDocument;
    return [...root.querySelectorAll(FOCUSABLE_SELECTOR2)].filter(
      (el) => el.offsetParent !== null || el === doc.activeElement
    );
  }
  function isNode3(v) {
    return typeof v?.nodeType === "number";
  }

  // src/features/tooltip.ts
  var provider = {
    delayDuration: 700,
    // primitive default; themes root passes 200
    skipDelayDuration: 300,
    isOpenDelayed: true,
    isPointerInTransit: false,
    skipDelayTimer: 0
  };
  function configureTooltipProvider(options) {
    if (options.delayDuration !== void 0) provider.delayDuration = options.delayDuration;
    if (options.skipDelayDuration !== void 0) provider.skipDelayDuration = options.skipDelayDuration;
  }
  var openTooltips = /* @__PURE__ */ new Set();
  function wireTooltip(options) {
    const { trigger } = options;
    const doc = trigger.ownerDocument;
    const view = doc.defaultView ?? globalThis;
    const delayDuration = options.delayDuration ?? provider.delayDuration;
    const setState = (s) => {
      machineState = s;
      options.onStateChange?.(s);
    };
    let machineState = "closed";
    let mounted;
    const local = {
      openTimer: 0,
      wasOpenDelayed: false,
      hasPointerMoveOpened: false,
      isPointerDown: false
    };
    const instance = { options, close: () => close() };
    const clearOpenTimer = () => {
      view.clearTimeout(local.openTimer);
      local.openTimer = 0;
    };
    const open = (state) => {
      if (machineState !== "closed") return;
      const built = options.buildContent(state);
      const contentEl = built.content;
      const body = doc.body;
      setState(state);
      providerOnOpen();
      for (const other of openTooltips) if (other !== instance) other.close();
      const wrapper = doc.createElement("div");
      wrapper.setAttribute("data-radix-popper-content-wrapper", "");
      wrapper.appendChild(contentEl);
      body.appendChild(wrapper);
      trigger.setAttribute("data-state", state);
      trigger.setAttribute("aria-describedby", contentEl.id);
      const popper = positionPopper(
        {
          anchor: trigger,
          floating: wrapper,
          content: contentEl,
          arrow: built.arrow ?? null
        },
        options.popperOptions ?? {},
        (side, align) => {
          trigger.setAttribute("data-radix-popper-side", side);
          trigger.setAttribute("data-radix-popper-align", align);
        }
      );
      const listeners = [];
      let graceArea = null;
      let trackGrace = null;
      const removeGrace = () => {
        graceArea = null;
        if (trackGrace) {
          doc.removeEventListener("pointermove", trackGrace);
          trackGrace = null;
        }
        provider.isPointerInTransit = false;
      };
      const createGrace = (event, hoverTarget) => {
        const pointerEvent = event;
        const exitPoint = { x: pointerEvent.clientX, y: pointerEvent.clientY };
        const exitSide = getExitSideFromRect(exitPoint, pointerEvent.currentTarget.getBoundingClientRect());
        const paddedExitPoints = getPaddedExitPoints(exitPoint, exitSide);
        const hoverTargetPoints = getPointsFromRect(hoverTarget.getBoundingClientRect());
        graceArea = getHull([...paddedExitPoints, ...hoverTargetPoints]);
        provider.isPointerInTransit = true;
        if (!trackGrace) {
          trackGrace = (moveEvent) => {
            if (!graceArea) return;
            const target = moveEvent.target;
            const pointerPosition = { x: moveEvent.clientX, y: moveEvent.clientY };
            const hasEnteredTarget = isNode4(target) && trigger.contains(target) || isNode4(target) && contentEl.contains(target);
            const isOutside = !isPointInPolygon(pointerPosition, graceArea);
            if (hasEnteredTarget) removeGrace();
            else if (isOutside) {
              removeGrace();
              close();
            }
          };
          doc.addEventListener("pointermove", trackGrace);
        }
      };
      if (!options.disableHoverableContent) {
        const onTriggerLeave = (event) => createGrace(event, contentEl);
        const onContentLeave = (event) => createGrace(event, trigger);
        trigger.addEventListener("pointerleave", onTriggerLeave);
        contentEl.addEventListener("pointerleave", onContentLeave);
        listeners.push([trigger, "pointerleave", onTriggerLeave], [contentEl, "pointerleave", onContentLeave]);
      }
      const onPointerDown = (event) => {
        const target = event.target;
        if (isNode4(target) && contentEl.contains(target)) return;
        close();
      };
      const onKeyDown = (event) => {
        if (event.key === "Escape") close();
      };
      const onScroll = (event) => {
        const target = event.target;
        if (isNode4(target) && target.contains(trigger)) close();
      };
      doc.addEventListener("pointerdown", onPointerDown);
      doc.addEventListener("keydown", onKeyDown);
      view.addEventListener(
        "scroll",
        onScroll,
        { capture: true }
      );
      listeners.push([doc, "pointerdown", onPointerDown], [doc, "keydown", onKeyDown], [view, "scroll", onScroll]);
      mounted = {
        wrapper,
        popper,
        cleanup: () => {
          for (const [target, type, fn] of listeners) target.removeEventListener(type, fn);
          if (trackGrace) doc.removeEventListener("pointermove", trackGrace);
          removeGrace();
        }
      };
      openTooltips.add(instance);
    };
    const close = () => {
      if (machineState === "closed") return;
      clearOpenTimer();
      providerOnClose();
      openTooltips.delete(instance);
      mounted?.cleanup();
      mounted?.popper.destroy();
      mounted?.wrapper.remove();
      mounted = void 0;
      trigger.setAttribute("data-state", "closed");
      trigger.removeAttribute("aria-describedby");
      trigger.removeAttribute("data-radix-popper-side");
      trigger.removeAttribute("data-radix-popper-align");
      setState("closed");
      options.onClosed?.();
    };
    const handleOpen = () => {
      clearOpenTimer();
      local.wasOpenDelayed = false;
      open("instant-open");
    };
    const handleDelayedOpen = () => {
      clearOpenTimer();
      local.openTimer = view.setTimeout(() => {
        local.wasOpenDelayed = true;
        open("delayed-open");
        local.openTimer = 0;
      }, delayDuration);
    };
    const handleClose = () => {
      clearOpenTimer();
      close();
    };
    const pointerUp = () => {
      local.isPointerDown = false;
    };
    const handlers = {
      onpointermove: () => {
        if (!local.hasPointerMoveOpened && !provider.isPointerInTransit) {
          if (provider.isOpenDelayed) handleDelayedOpen();
          else handleOpen();
          local.hasPointerMoveOpened = true;
        }
      },
      onpointerleave: () => {
        if (options.disableHoverableContent) {
          handleClose();
        } else {
          clearOpenTimer();
        }
        local.hasPointerMoveOpened = false;
      },
      onpointerdown: () => {
        if (machineState !== "closed") handleClose();
        local.isPointerDown = true;
        doc.addEventListener("pointerup", pointerUp, { once: true });
      },
      onfocus: () => {
        if (!local.isPointerDown) handleOpen();
      },
      onblur: () => handleClose(),
      onclick: () => handleClose()
    };
    const providerOnOpen = () => {
      view.clearTimeout(provider.skipDelayTimer);
      provider.isOpenDelayed = false;
    };
    const providerOnClose = () => {
      view.clearTimeout(provider.skipDelayTimer);
      provider.skipDelayTimer = view.setTimeout(() => {
        provider.isOpenDelayed = true;
      }, provider.skipDelayDuration);
    };
    return {
      handlers,
      close,
      state: () => machineState
    };
  }
  function getExitSideFromRect(point, rect) {
    const top = Math.abs(rect.top - point.y);
    const bottom = Math.abs(rect.bottom - point.y);
    const right = Math.abs(rect.right - point.x);
    const left = Math.abs(rect.left - point.x);
    switch (Math.min(top, bottom, right, left)) {
      case left:
        return "left";
      case right:
        return "right";
      case top:
        return "top";
      case bottom:
        return "bottom";
      default:
        throw new Error("unreachable");
    }
  }
  function getPaddedExitPoints(exitPoint, exitSide, padding = 5) {
    const paddedExitPoints = [];
    switch (exitSide) {
      case "top":
        paddedExitPoints.push(
          { x: exitPoint.x - padding, y: exitPoint.y + padding },
          { x: exitPoint.x + padding, y: exitPoint.y + padding }
        );
        break;
      case "bottom":
        paddedExitPoints.push(
          { x: exitPoint.x - padding, y: exitPoint.y - padding },
          { x: exitPoint.x + padding, y: exitPoint.y - padding }
        );
        break;
      case "left":
        paddedExitPoints.push(
          { x: exitPoint.x + padding, y: exitPoint.y - padding },
          { x: exitPoint.x + padding, y: exitPoint.y + padding }
        );
        break;
      case "right":
        paddedExitPoints.push(
          { x: exitPoint.x - padding, y: exitPoint.y - padding },
          { x: exitPoint.x - padding, y: exitPoint.y + padding }
        );
        break;
    }
    return paddedExitPoints;
  }
  function getPointsFromRect(rect) {
    const { top, right, bottom, left } = rect;
    return [
      { x: left, y: top },
      { x: right, y: top },
      { x: right, y: bottom },
      { x: left, y: bottom }
    ];
  }
  function isPointInPolygon(point, polygon) {
    const { x, y } = point;
    let inside = false;
    for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
      const ii = polygon[i];
      const jj = polygon[j];
      const xi = ii.x;
      const yi = ii.y;
      const xj = jj.x;
      const yj = jj.y;
      const intersect = yi > y !== yj > y && x < (xj - xi) * (y - yi) / (yj - yi) + xi;
      if (intersect) inside = !inside;
    }
    return inside;
  }
  function getHull(points) {
    const newPoints = points.slice();
    newPoints.sort((a, b) => {
      if (a.x < b.x) return -1;
      else if (a.x > b.x) return 1;
      else if (a.y < b.y) return -1;
      else if (a.y > b.y) return 1;
      else return 0;
    });
    return getHullPresorted(newPoints);
  }
  function getHullPresorted(points) {
    if (points.length <= 1) return points.slice();
    const upperHull = [];
    for (let i = 0; i < points.length; i++) {
      const p = points[i];
      while (upperHull.length >= 2) {
        const q = upperHull[upperHull.length - 1];
        const r = upperHull[upperHull.length - 2];
        if ((q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x)) upperHull.pop();
        else break;
      }
      upperHull.push(p);
    }
    upperHull.pop();
    const lowerHull = [];
    for (let i = points.length - 1; i >= 0; i--) {
      const p = points[i];
      while (lowerHull.length >= 2) {
        const q = lowerHull[lowerHull.length - 1];
        const r = lowerHull[lowerHull.length - 2];
        if ((q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x)) lowerHull.pop();
        else break;
      }
      lowerHull.push(p);
    }
    lowerHull.pop();
    if (upperHull.length === 1 && lowerHull.length === 1 && upperHull[0].x === lowerHull[0].x && upperHull[0].y === lowerHull[0].y) {
      return upperHull;
    } else {
      return upperHull.concat(lowerHull);
    }
  }
  function isNode4(v) {
    return typeof v?.nodeType === "number";
  }

  // src/features/position.ts
  var CONTENT_MARGIN = 10;
  var clamp2 = (value, [min2, max2]) => Math.min(Math.max(value, min2), max2);
  var px = (n) => `${n}px`;
  function positionItemAligned(el) {
    const { trigger, content, viewport, wrapper, selectedItem, doc } = el;
    const view = doc.defaultView;
    if (!view || !selectedItem) return;
    const valueNode = trigger.querySelector(".rt-SelectTriggerInner > span");
    const itemText = selectedItem.querySelector("span[id]") ?? selectedItem;
    if (!valueNode || !itemText) return;
    const triggerRect = trigger.getBoundingClientRect();
    const contentRect = content.getBoundingClientRect();
    const valueNodeRect = valueNode.getBoundingClientRect();
    const itemTextRect = itemText.getBoundingClientRect();
    const innerW = view.innerWidth;
    const innerH = view.innerHeight;
    const itemTextOffset = itemTextRect.left - contentRect.left;
    const left = valueNodeRect.left - itemTextOffset;
    const leftDelta = triggerRect.left - left;
    const minContentWidth = triggerRect.width + leftDelta;
    const contentWidth = Math.max(minContentWidth, contentRect.width);
    const rightEdge = innerW - CONTENT_MARGIN;
    const clampedLeft = clamp2(left, [
      CONTENT_MARGIN,
      Math.max(CONTENT_MARGIN, rightEdge - contentWidth)
    ]);
    const ws = wrapper.style;
    ws.minWidth = px(minContentWidth);
    ws.left = px(clampedLeft);
    const availableHeight = innerH - CONTENT_MARGIN * 2;
    const itemsHeight = viewport.scrollHeight;
    const cs = view.getComputedStyle(content);
    const contentBorderTopWidth = parseInt(cs.borderTopWidth, 10) || 0;
    const contentPaddingTop = parseInt(cs.paddingTop, 10) || 0;
    const contentBorderBottomWidth = parseInt(cs.borderBottomWidth, 10) || 0;
    const contentPaddingBottom = parseInt(cs.paddingBottom, 10) || 0;
    const fullContentHeight = contentBorderTopWidth + contentPaddingTop + itemsHeight + contentPaddingBottom + contentBorderBottomWidth;
    const item = selectedItem;
    const minContentHeight = Math.min(item.offsetHeight * 5, fullContentHeight);
    const vs = view.getComputedStyle(viewport);
    const viewportPaddingTop = parseInt(vs.paddingTop, 10) || 0;
    const viewportPaddingBottom = parseInt(vs.paddingBottom, 10) || 0;
    const topEdgeToTriggerMiddle = triggerRect.top + triggerRect.height / 2 - CONTENT_MARGIN;
    const triggerMiddleToBottomEdge = availableHeight - topEdgeToTriggerMiddle;
    const selectedItemHalfHeight = item.offsetHeight / 2;
    const itemOffsetMiddle = item.offsetTop + selectedItemHalfHeight;
    const contentTopToItemMiddle = contentBorderTopWidth + contentPaddingTop + itemOffsetMiddle;
    const itemMiddleToContentBottom = fullContentHeight - contentTopToItemMiddle;
    const willAlignWithoutTopOverflow = contentTopToItemMiddle <= topEdgeToTriggerMiddle;
    const optionEls = Array.from(viewport.querySelectorAll('[role="option"]'));
    if (willAlignWithoutTopOverflow) {
      const isLastItem = selectedItem === optionEls[optionEls.length - 1];
      ws.top = "";
      ws.bottom = "0px";
      const viewportOffsetBottom = content.clientHeight - viewport.offsetTop - viewport.offsetHeight;
      const clampedTriggerMiddleToBottomEdge = Math.max(
        triggerMiddleToBottomEdge,
        selectedItemHalfHeight + (isLastItem ? viewportPaddingBottom : 0) + viewportOffsetBottom + contentBorderBottomWidth
      );
      const height = contentTopToItemMiddle + clampedTriggerMiddleToBottomEdge;
      ws.height = px(height);
    } else {
      const isFirstItem = selectedItem === optionEls[0];
      ws.bottom = "";
      ws.top = "0px";
      const clampedTopEdgeToTriggerMiddle = Math.max(
        topEdgeToTriggerMiddle,
        contentBorderTopWidth + viewport.offsetTop + (isFirstItem ? viewportPaddingTop : 0) + selectedItemHalfHeight
      );
      const height = clampedTopEdgeToTriggerMiddle + itemMiddleToContentBottom;
      ws.height = px(height);
      viewport.scrollTop = contentTopToItemMiddle - topEdgeToTriggerMiddle + viewport.offsetTop;
    }
    ws.margin = `${CONTENT_MARGIN}px 0`;
    ws.minHeight = px(minContentHeight);
    ws.maxHeight = px(availableHeight);
    ws.zIndex = "auto";
  }

  // src/features/select.ts
  function wireSelect(options) {
    const { trigger, content, viewport, onClosed } = options;
    const doc = trigger.ownerDocument;
    const view = doc.defaultView ?? globalThis;
    const w = view;
    const positionFn = options.position ?? positionItemAligned;
    let open = false;
    let mounted;
    let typeahead = "";
    let typeaheadTimer = 0;
    const items = () => Array.from(viewport.querySelectorAll('[role="option"]'));
    const selected = () => items().find((el) => el.getAttribute("aria-selected") === "true");
    const highlighted = () => viewport.querySelector('[role="option"][data-highlighted]') ?? void 0;
    const setHighlighted = (item) => {
      for (const el of items()) {
        if (el === item) el.setAttribute("data-highlighted", "");
        else el.removeAttribute("data-highlighted");
      }
      item?.focus?.();
    };
    const setText = (el, text) => {
      const span = el?.firstElementChild ?? el;
      if (span) span.textContent = text;
    };
    const openSelect = () => {
      if (open) return;
      open = true;
      const wrapper = options.wrapper ?? (() => {
        const div = doc.createElement("div");
        div.setAttribute("style", "display: flex; flex-direction: column; position: fixed;");
        return div;
      })();
      if (!wrapper.contains(content)) wrapper.appendChild(content);
      doc.body.appendChild(wrapper);
      const guard = () => {
        const s = doc.createElement("span");
        s.setAttribute("data-radix-focus-guard", "");
        s.setAttribute("tabindex", "0");
        s.setAttribute("data-aria-hidden", "true");
        s.setAttribute("aria-hidden", "true");
        s.setAttribute(
          "style",
          "outline: none; opacity: 0; position: fixed; pointer-events: none;"
        );
        return s;
      };
      const guardStart = guard();
      const guardEnd = guard();
      doc.body.insertBefore(guardStart, doc.body.firstChild);
      doc.body.appendChild(guardEnd);
      const hidden = [];
      for (const el of Array.from(doc.body.children)) {
        if (el === wrapper || el === guardStart || el === guardEnd) continue;
        if (el.hasAttribute("aria-hidden")) continue;
        el.setAttribute("aria-hidden", "true");
        el.setAttribute("data-aria-hidden", "true");
        hidden.push(el);
      }
      trigger.setAttribute("data-state", "open");
      trigger.setAttribute("aria-expanded", "true");
      trigger.setAttribute("aria-controls", content.id);
      content.setAttribute("data-state", "open");
      const sel = selected();
      setHighlighted(sel);
      positionFn({ trigger, content, viewport, wrapper, selectedItem: sel, doc });
      sel?.scrollIntoView?.({ block: "nearest" });
      const optionEls = Array.from(viewport.querySelectorAll('[role="option"]'));
      if (sel === optionEls[0]) viewport.scrollTop = 0;
      const onKeyDown = (event) => {
        if (!open) return;
        const list = items();
        const cur = highlighted();
        const idx = cur ? list.indexOf(cur) : -1;
        switch (event.key) {
          case "ArrowDown":
            event.preventDefault();
            setHighlighted(list[Math.min(idx + 1, list.length - 1)]);
            return;
          case "ArrowUp":
            event.preventDefault();
            setHighlighted(list[Math.max(idx - 1, 0)]);
            return;
          case "Home":
            event.preventDefault();
            setHighlighted(list[0]);
            return;
          case "End":
            event.preventDefault();
            setHighlighted(list[list.length - 1]);
            return;
          case "Enter":
          case " ":
            event.preventDefault();
            if (cur) handles.select(cur);
            return;
          case "Escape":
            event.preventDefault();
            handles.close(true);
            return;
          case "Tab":
            event.preventDefault();
            handles.close(false);
            return;
          default:
            break;
        }
        if (event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey) {
          typeahead += event.key.toLowerCase();
          w.clearTimeout(typeaheadTimer);
          typeaheadTimer = w.setTimeout(() => {
            typeahead = "";
          }, 500);
          const match = list.find(
            (el) => (el.textContent ?? "").trim().toLowerCase().startsWith(typeahead)
          );
          if (match) {
            event.preventDefault();
            setHighlighted(match);
          }
        }
      };
      const onPointerDown = (event) => {
        const target = event.target;
        if (target instanceof Node && (content.contains(target) || trigger.contains(target))) return;
        handles.close(false);
      };
      doc.addEventListener("keydown", onKeyDown);
      doc.addEventListener("pointerdown", onPointerDown);
      mounted = { wrapper, guards: [guardStart, guardEnd], hidden, onKeyDown, onPointerDown };
    };
    const closeSelect = (restoreFocus = true) => {
      if (!open) return;
      open = false;
      w.clearTimeout(typeaheadTimer);
      typeahead = "";
      doc.removeEventListener("keydown", mounted?.onKeyDown);
      doc.removeEventListener("pointerdown", mounted?.onPointerDown);
      for (const el of mounted?.hidden ?? []) {
        el.removeAttribute("aria-hidden");
        el.removeAttribute("data-aria-hidden");
      }
      mounted?.guards[0].remove();
      mounted?.guards[1].remove();
      mounted?.wrapper.remove();
      const hl = highlighted();
      hl?.removeAttribute("data-highlighted");
      trigger.setAttribute("data-state", "closed");
      trigger.setAttribute("aria-expanded", "false");
      trigger.removeAttribute("aria-controls");
      content.setAttribute("data-state", "closed");
      if (restoreFocus) trigger.focus?.();
      mounted = void 0;
      onClosed?.();
    };
    const selectItem = (item) => {
      for (const el of items()) {
        if (el === item) {
          el.setAttribute("aria-selected", "true");
          el.setAttribute("data-state", "checked");
        } else {
          el.setAttribute("aria-selected", "false");
          el.setAttribute("data-state", "unchecked");
        }
      }
      const text = (item.textContent ?? "").trim();
      const valueNode = options.valueNode ?? trigger.querySelector(".rt-SelectTriggerInner > span") ?? void 0;
      if (valueNode) valueNode.textContent = text;
      handles.close(true);
    };
    const handles = {
      open: openSelect,
      close: closeSelect,
      select: selectItem,
      isOpen: () => open
    };
    return handles;
  }

  // src/features/tabs.ts
  function adoptBody(panel) {
    const tpl = panel.querySelector(":scope > template");
    if (!tpl) return;
    const doc = panel.ownerDocument;
    const body = tpl.content;
    const holder = doc.createDocumentFragment();
    while (body.firstChild) holder.appendChild(body.firstChild);
    while (panel.firstChild) panel.firstChild.remove();
    panel.appendChild(holder);
  }
  function releaseBody(panel) {
    if (panel.querySelector(":scope > template")) return;
    const doc = panel.ownerDocument;
    const tpl = doc.createElement("template");
    while (panel.firstChild) tpl.content.appendChild(panel.firstChild);
    panel.appendChild(tpl);
  }
  function wireTabs(options) {
    const { list, triggers, panels } = options;
    const doc = list.ownerDocument;
    let current = options.initial;
    const activate = (index, interactive = false) => {
      const clamped = Math.max(0, Math.min(index, triggers.length - 1));
      current = clamped;
      triggers.forEach((t, i) => {
        t.setAttribute("aria-selected", String(i === clamped));
        t.setAttribute("data-state", i === clamped ? "active" : "inactive");
        if (interactive) t.setAttribute("tabindex", i === clamped ? "0" : "-1");
      });
      panels.forEach((p, i) => {
        if (i === clamped) {
          p.setAttribute("data-state", "active");
          p.removeAttribute("hidden");
          adoptBody(p);
        } else {
          p.setAttribute("data-state", "inactive");
          p.setAttribute("hidden", "");
          releaseBody(p);
        }
      });
      options.onChange?.(clamped);
    };
    const onKeyDown = (event) => {
      const key = event.key;
      const last = triggers.length - 1;
      let next;
      switch (key) {
        case "ArrowRight":
          next = current === last ? 0 : current + 1;
          break;
        case "ArrowLeft":
          next = current === 0 ? last : current - 1;
          break;
        case "Home":
          next = 0;
          break;
        case "End":
          next = last;
          break;
        default:
          return;
      }
      event.preventDefault();
      activate(next, true);
      triggers[next].focus?.();
    };
    list.addEventListener("keydown", onKeyDown);
    triggers.forEach((t, i) => {
      t.addEventListener("click", (event) => {
        event.preventDefault();
        activate(i, true);
        t.focus?.();
      });
    });
    activate(current);
    doc;
    return { activate, active: () => current };
  }

  // src/features/hover-card.ts
  function wireHoverCard(options) {
    const { trigger } = options;
    const doc = trigger.ownerDocument;
    const view = doc.defaultView ?? globalThis;
    const w = view;
    let open = false;
    let mounted;
    let contentEl;
    let hasSelection = false;
    let isPointerDownOnContent = false;
    let openTimer = 0;
    let closeTimer = 0;
    const clearTimers = () => {
      w.clearTimeout(openTimer);
      w.clearTimeout(closeTimer);
    };
    const openHoverCard = () => {
      if (open) return;
      open = true;
      const content = options.buildContent();
      contentEl = content;
      const wrapper = doc.createElement("div");
      wrapper.setAttribute("data-radix-popper-content-wrapper", "");
      wrapper.appendChild(content);
      doc.body.appendChild(wrapper);
      const tabbables = content.querySelectorAll(
        'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
      );
      tabbables.forEach((el) => el.setAttribute("tabindex", "-1"));
      trigger.setAttribute("data-state", "open");
      const popper = positionPopper(
        { anchor: trigger, floating: wrapper, content },
        options.popperOptions ?? {},
        (side, align) => {
          trigger.setAttribute("data-radix-popper-side", side);
          trigger.setAttribute("data-radix-popper-align", align);
        }
      );
      if (options.popperOptions) {
        trigger.setAttribute("data-radix-popper-side", options.popperOptions.side ?? "bottom");
        trigger.setAttribute("data-radix-popper-align", options.popperOptions.align ?? "center");
      }
      const handleOpen = () => {
        w.clearTimeout(closeTimer);
        openTimer = w.setTimeout(() => openHoverCard(), options.openDelay);
      };
      const handleClose = () => {
        w.clearTimeout(openTimer);
        if (!hasSelection && !isPointerDownOnContent) {
          closeTimer = w.setTimeout(() => closeHoverCard(), options.closeDelay);
        }
      };
      const onEnter = (event) => {
        if (event.pointerType === "touch") return;
        handleOpen();
      };
      const onLeave = (event) => {
        if (event.pointerType === "touch") return;
        handleClose();
      };
      const onContentPointerDown = (event) => {
        const current = event.currentTarget;
        if (current instanceof Element && current.contains(event.target)) {
          hasSelection = false;
          isPointerDownOnContent = true;
        }
      };
      const onPointerUp = () => {
        isPointerDownOnContent = false;
        w.setTimeout(() => {
          const sel = doc.getSelection()?.toString();
          if (sel && sel !== "") hasSelection = true;
        });
      };
      content.addEventListener("pointerenter", onEnter);
      content.addEventListener("pointerleave", onLeave);
      content.addEventListener("pointerdown", onContentPointerDown);
      doc.addEventListener("pointerup", onPointerUp);
      mounted = {
        wrapper,
        popper,
        cleanup: () => {
          content.removeEventListener("pointerenter", onEnter);
          content.removeEventListener("pointerleave", onLeave);
          content.removeEventListener("pointerdown", onContentPointerDown);
          doc.removeEventListener("pointerup", onPointerUp);
          w.clearTimeout(openTimer);
          w.clearTimeout(closeTimer);
        }
      };
      options.onOpen?.();
    };
    const closeHoverCard = () => {
      if (!open) return;
      open = false;
      clearTimers();
      mounted?.cleanup();
      mounted?.popper.destroy();
      mounted?.wrapper.remove();
      mounted = void 0;
      contentEl = void 0;
      hasSelection = false;
      isPointerDownOnContent = false;
      trigger.setAttribute("data-state", "closed");
      trigger.removeAttribute("data-radix-popper-side");
      trigger.removeAttribute("data-radix-popper-align");
      options.onClosed?.();
    };
    const triggerHandleOpen = () => {
      w.clearTimeout(closeTimer);
      openTimer = w.setTimeout(() => openHoverCard(), options.openDelay);
    };
    const triggerHandleClose = () => {
      w.clearTimeout(openTimer);
      if (!hasSelection && !isPointerDownOnContent) {
        closeTimer = w.setTimeout(() => closeHoverCard(), options.closeDelay);
      }
    };
    return {
      handlers: {
        onpointerenter: triggerHandleOpen,
        onpointerleave: triggerHandleClose,
        onfocus: triggerHandleOpen,
        onblur: triggerHandleClose
      },
      dismiss: closeHoverCard,
      openNow: openHoverCard
    };
  }

  // src/features/slider.ts
  var THUMB_ROLE = '[role="slider"]';
  function decimalCount(step) {
    const s = String(step);
    const i = s.indexOf(".");
    return i === -1 ? 0 : s.length - i - 1;
  }
  function roundValue(value, decimals) {
    const f = 10 ** decimals;
    return Math.round(value * f) / f;
  }
  function thumbInBoundsOffset(thumbWidth, percent, direction) {
    return (thumbWidth / 2 - percent * thumbWidth / 100 * direction) * direction;
  }
  function wireSlider(options) {
    const root = options.root;
    const doc = root.ownerDocument;
    const min2 = options.min ?? 0;
    const max2 = options.max ?? 100;
    const step = options.step ?? 1;
    const orientation = options.orientation ?? "horizontal";
    const direction = options.direction ?? 1;
    const decimals = decimalCount(step);
    const rangeEl = options.range ?? null;
    const trackEl = options.track ?? null;
    const geometry = options.geometry ?? {
      trackRect: () => {
        const track = trackEl ?? root;
        const r = track.getBoundingClientRect();
        return r.width > 0 ? { left: r.left, width: r.width } : null;
      },
      thumbWidth: () => {
        const thumb = root.querySelector(THUMB_ROLE);
        return thumb ? thumb.getBoundingClientRect().width : 0;
      }
    };
    let values = [...options.defaultValue ?? []].sort((a, b) => a - b);
    let disabled = options.disabled ?? false;
    let dragging = -1;
    const thumbs = () => Array.from(root.querySelectorAll(THUMB_ROLE));
    const clamp3 = (v) => Math.min(max2, Math.max(min2, v));
    function snapToStep(value) {
      return roundValue(Math.round((value - min2) / step) * step + min2, decimals);
    }
    function getNextSortedValues(prev, next, atIndex) {
      const nextValues = [...prev];
      nextValues[atIndex] = next;
      return nextValues.sort((a, b) => a - b);
    }
    function updateValues(value, atIndex, opts = {}) {
      if (disabled) return;
      const next = clamp3(snapToStep(value));
      const prev = values;
      const nextValues = getNextSortedValues(prev, next, atIndex);
      const hasChanged = String(nextValues) !== String(prev);
      values = nextValues;
      const newIndex = nextValues.indexOf(next);
      syncDom();
      if (opts.focusFollows !== false) {
        const ts = thumbs();
        if (newIndex < ts.length && doc.activeElement !== ts[newIndex] && doc.activeElement !== null) {
          const wasOnThumb = ts.some((t) => t === doc.activeElement || t.contains(doc.activeElement));
          if (wasOnThumb) ts[newIndex]?.focus?.();
        }
      }
      if (hasChanged) {
        options.onValueChange?.(values);
        if (opts.commit) options.onValueCommit?.(values);
      }
    }
    function thumbPercent(value) {
      return max2 === min2 ? 0 : value / (max2 - min2) * 100;
    }
    function syncDom() {
      const ts = thumbs();
      const multi = values.length > 1;
      ts.forEach((thumb, i) => {
        const value = values[i];
        if (value === void 0) {
          thumb.style.display = "none";
          return;
        }
        thumb.style.removeProperty("display");
        thumb.setAttribute("aria-valuenow", String(value));
        thumb.setAttribute("aria-valuemin", String(min2));
        thumb.setAttribute("aria-valuemax", String(max2));
        thumb.setAttribute("aria-orientation", orientation);
        thumb.setAttribute("data-orientation", orientation);
        if (multi) {
          if (values.length > 2) thumb.setAttribute("aria-label", `Value ${i + 1} of ${values.length}`);
          else thumb.setAttribute("aria-label", i === 0 ? "Minimum" : "Maximum");
        } else {
          thumb.removeAttribute("aria-label");
        }
        const wrapper = thumb.parentElement;
        if (wrapper && wrapper !== root) {
          const percent = thumbPercent(value);
          const edgeDir = orientation === "vertical" ? 1 : direction;
          const offset3 = thumbInBoundsOffset(geometry.thumbWidth(), percent, edgeDir);
          const edge = orientation === "vertical" ? "bottom" : direction === 1 ? "left" : "right";
          const style = wrapper.style;
          style.setProperty("transform", "var(--radix-slider-thumb-transform)");
          style.setProperty("position", "absolute");
          style.setProperty(edge, `calc(${roundValue(percent, 2)}% + ${roundValue(offset3, 2)}px)`);
        }
      });
      if (rangeEl) {
        const count = values.length;
        const start = count > 1 ? thumbPercent(values[0]) : 0;
        const end = count > 0 ? 100 - thumbPercent(values[count - 1]) : 100;
        const startEdge = orientation === "vertical" ? "bottom" : direction === 1 ? "left" : "right";
        const endEdge = orientation === "vertical" ? "top" : direction === 1 ? "right" : "left";
        const style = rangeEl.style;
        style.setProperty(startEdge, `${roundValue(start, 2)}%`);
        style.setProperty(endEdge, `${roundValue(end, 2)}%`);
      }
    }
    function applyDisabled() {
      const off = disabled ? "" : null;
      root.setAttribute("aria-disabled", String(disabled));
      if (off === null) root.removeAttribute("data-disabled");
      else root.setAttribute("data-disabled", off);
      if (trackEl) {
        if (off === null) trackEl.removeAttribute("data-disabled");
        else trackEl.setAttribute("data-disabled", off);
      }
      if (rangeEl) {
        if (off === null) rangeEl.removeAttribute("data-disabled");
        else rangeEl.setAttribute("data-disabled", off);
      }
      thumbs().forEach((thumb) => {
        if (off === null) {
          thumb.removeAttribute("data-disabled");
          thumb.setAttribute("tabindex", "0");
        } else {
          thumb.setAttribute("data-disabled", off);
          thumb.removeAttribute("tabindex");
        }
      });
    }
    function valueFromPoint(x) {
      const rect = geometry.trackRect();
      if (!rect || rect.width <= 0) return null;
      let percent = (x - rect.left) / rect.width * 100;
      percent = Math.min(100, Math.max(0, percent));
      if (direction === -1 && orientation === "horizontal") percent = 100 - percent;
      return min2 + percent / 100 * (max2 - min2);
    }
    function nearestThumbIndex(value) {
      let best = 0;
      let bestDist = Infinity;
      values.forEach((v, i) => {
        const d = Math.abs(v - value);
        if (d < bestDist) {
          bestDist = d;
          best = i;
        }
      });
      return best;
    }
    root.addEventListener("pointerdown", (event) => {
      if (disabled) return;
      const target = event.target;
      const thumb = target?.closest?.(THUMB_ROLE) ?? null;
      root.setPointerCapture?.(event.pointerId);
      event.preventDefault?.();
      if (thumb) {
        const index = thumbs().indexOf(thumb);
        if (index !== -1) {
          dragging = index;
          thumb.focus?.({ preventScroll: true });
        }
        return;
      }
      const value = valueFromPoint(event.clientX);
      if (value !== null) {
        const index = nearestThumbIndex(value);
        dragging = index;
        updateValues(value, index, { commit: true, focusFollows: false });
      }
    });
    root.addEventListener("pointermove", (event) => {
      if (dragging === -1 || disabled) return;
      const value = valueFromPoint(event.clientX);
      if (value !== null) updateValues(value, dragging, { commit: false });
    });
    const endDrag = () => {
      if (dragging === -1) return;
      dragging = -1;
      options.onValueCommit?.(values);
    };
    root.addEventListener("pointerup", endDrag);
    root.addEventListener("pointercancel", endDrag);
    const ARROW_KEYS = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"];
    const PAGE_KEYS = ["PageUp", "PageDown"];
    root.addEventListener("keydown", (event) => {
      const key = event.key;
      const target = event.target;
      const thumb = target?.closest?.(THUMB_ROLE) ?? null;
      if (!thumb || disabled) return;
      const ts = thumbs();
      const domIndex = ts.indexOf(thumb);
      const value = values[domIndex];
      if (value === void 0) return;
      const atIndex = values.indexOf(value);
      const posStep = () => updateValues(value + step, atIndex, { commit: true });
      const negStep = () => updateValues(value - step, atIndex, { commit: true });
      const skip = (dir) => updateValues(value + dir * step * 10, atIndex, { commit: true });
      let handled = true;
      const horizontal = orientation === "horizontal";
      if (key === "Home") updateValues(min2, 0, { commit: true });
      else if (key === "End") updateValues(max2, values.length - 1, { commit: true });
      else if (PAGE_KEYS.includes(key) || event.shiftKey && ARROW_KEYS.includes(key)) {
        if (key === "PageUp" || key === "ArrowUp" || key === "ArrowRight" && horizontal) skip(1);
        else if (key === "PageDown" || key === "ArrowDown" || key === "ArrowLeft" && horizontal) skip(-1);
        else if (key === "ArrowRight" || key === "ArrowLeft") skip(direction);
        else handled = false;
      } else if (key === "ArrowRight") horizontal ? direction === 1 ? posStep() : negStep() : handled = false;
      else if (key === "ArrowLeft") horizontal ? direction === 1 ? negStep() : posStep() : handled = false;
      else if (key === "ArrowUp") horizontal ? posStep() : posStep();
      else if (key === "ArrowDown") horizontal ? negStep() : negStep();
      else handled = false;
      if (handled) event.preventDefault();
    });
    syncDom();
    applyDisabled();
    return {
      values: () => [...values],
      setValue: (value, atIndex, opts) => updateValues(value, atIndex, { commit: opts?.commit ?? false }),
      setDisabled(d) {
        disabled = d;
        if (d) endDrag();
        applyDisabled();
      }
    };
  }

  // src/features/scroll-area.ts
  var MACHINE = {
    hidden: { SCROLL: "scrolling" },
    scrolling: { SCROLL_END: "idle", POINTER_ENTER: "interacting" },
    interacting: { SCROLL: "interacting", POINTER_LEAVE: "idle" },
    idle: { HIDE: "hidden", SCROLL: "scrolling", POINTER_ENTER: "interacting" }
  };
  var SCROLL_END_DEBOUNCE = 100;
  var RESIZE_DEBOUNCE = 10;
  var EXIT_FADE = 120;
  function toInt(value) {
    return value ? parseInt(value, 10) : 0;
  }
  function getThumbRatio(viewportSize, contentSize) {
    const ratio = viewportSize / contentSize;
    return isNaN(ratio) ? 0 : ratio;
  }
  function getThumbSize(sizes) {
    const ratio = getThumbRatio(sizes.viewport, sizes.content);
    const scrollbarPadding = sizes.paddingStart + sizes.paddingEnd;
    const thumbSize = (sizes.scrollbar - scrollbarPadding) * ratio;
    return Math.max(thumbSize, 18);
  }
  function linearScale(input, output) {
    return (value) => {
      if (input[0] === input[1] || output[0] === output[1]) return output[0];
      const ratio = (output[1] - output[0]) / (input[1] - input[0]);
      return output[0] + ratio * (value - input[0]);
    };
  }
  function getThumbOffsetFromScroll(scrollPos, sizes, dir) {
    const thumbSizePx = getThumbSize(sizes);
    const scrollbarPadding = sizes.paddingStart + sizes.paddingEnd;
    const scrollbar = sizes.scrollbar - scrollbarPadding;
    const maxScrollPos = sizes.content - sizes.viewport;
    const maxThumbPos = scrollbar - thumbSizePx;
    const scrollClampRange = dir === "ltr" ? [0, maxScrollPos] : [maxScrollPos * -1, 0];
    const scrollWithoutMomentum = Math.min(Math.max(scrollPos, scrollClampRange[0]), scrollClampRange[1]);
    const interpolate = linearScale([0, maxScrollPos], [0, maxThumbPos]);
    return interpolate(scrollWithoutMomentum);
  }
  function wireScrollArea(options) {
    const root = options.root;
    const viewport = options.viewport;
    const doc = root.ownerDocument;
    const win = doc.defaultView;
    const type = options.type ?? "hover";
    const scrollHideDelay = options.scrollHideDelay ?? 600;
    const dir = options.dir ?? "ltr";
    const refs = options.scrollbars;
    const cornerEl = options.corner ?? null;
    const thumbVar = { vertical: "--radix-scroll-area-thumb-height", horizontal: "--radix-scroll-area-thumb-width" };
    const geometry = options.geometry ?? {
      vertical: () => {
        const sb = refs.vertical?.scrollbar ?? null;
        const cs = sb && win ? win.getComputedStyle(sb) : null;
        return {
          content: viewport.scrollHeight,
          viewport: viewport.offsetHeight,
          scrollbar: sb ? sb.clientHeight : 0,
          paddingStart: cs ? toInt(cs.paddingTop) : 0,
          paddingEnd: cs ? toInt(cs.paddingBottom) : 0,
          cross: sb ? sb.offsetWidth : 0
        };
      },
      horizontal: () => {
        const sb = refs.horizontal?.scrollbar ?? null;
        const cs = sb && win ? win.getComputedStyle(sb) : null;
        return {
          content: viewport.scrollWidth,
          viewport: viewport.offsetWidth,
          scrollbar: sb ? sb.clientWidth : 0,
          paddingStart: cs ? toInt(cs.paddingLeft) : 0,
          paddingEnd: cs ? toInt(cs.paddingRight) : 0,
          cross: sb ? sb.offsetHeight : 0
        };
      }
    };
    const axes = {
      vertical: refs.vertical ? { present: false, fadeTimer: 0, machine: "hidden", debounceTimer: 0, hideTimer: 0, prevPos: viewport.scrollTop, sizes: { content: 0, viewport: 0, scrollbar: 0, paddingStart: 0, paddingEnd: 0, cross: 0 }, overflow: false, hasThumb: false } : null,
      horizontal: refs.horizontal ? { present: false, fadeTimer: 0, machine: "hidden", debounceTimer: 0, hideTimer: 0, prevPos: viewport.scrollLeft, sizes: { content: 0, viewport: 0, scrollbar: 0, paddingStart: 0, paddingEnd: 0, cross: 0 }, overflow: false, hasThumb: false } : null
    };
    let pointerOver = false;
    let hoverHideTimer = 0;
    let cornerPresent = false;
    const listeners = [];
    const timers = /* @__PURE__ */ new Set();
    const on = (target, eventType, handler, opts) => {
      target.addEventListener(eventType, handler, opts);
      listeners.push([target, eventType, handler]);
    };
    const later = (fn, ms) => {
      const id = setTimeout(() => {
        timers.delete(id);
        fn();
      }, ms);
      timers.add(id);
      return id;
    };
    const cancel = (id) => {
      if (id) {
        clearTimeout(id);
        timers.delete(id);
      }
    };
    function scrollbarEl(axis) {
      return refs[axis]?.scrollbar ?? null;
    }
    function thumbEl(axis) {
      return refs[axis]?.thumb ?? null;
    }
    function scrollPos(axis) {
      return axis === "vertical" ? viewport.scrollTop : viewport.scrollLeft;
    }
    function computeAxis(axis) {
      const st = axes[axis];
      if (!st) return;
      st.sizes = geometry[axis]();
      st.overflow = st.sizes.viewport < st.sizes.content;
      const ratio = getThumbRatio(st.sizes.viewport, st.sizes.content);
      st.hasThumb = ratio > 0 && ratio < 1;
      const sb = scrollbarEl(axis);
      if (sb) sb.style.setProperty(thumbVar[axis], getThumbSize(st.sizes) + "px");
      syncThumb(axis);
    }
    function syncThumb(axis) {
      const st = axes[axis];
      const thumb = thumbEl(axis);
      if (!st || !thumb) return;
      thumb.setAttribute("data-state", st.hasThumb ? "visible" : "hidden");
      const style = thumb.style;
      style.setProperty("width", "var(--radix-scroll-area-thumb-width)");
      style.setProperty("height", "var(--radix-scroll-area-thumb-height)");
      const offset3 = getThumbOffsetFromScroll(scrollPos(axis), st.sizes, axis === "vertical" ? "ltr" : dir);
      style.transform = axis === "vertical" ? `translate3d(0, ${offset3}px, 0)` : `translate3d(${offset3}px, 0, 0)`;
    }
    function dataStateOf(axis) {
      const st = axes[axis];
      if (type === "always") return "visible";
      if (type === "auto") return st.overflow ? "visible" : "hidden";
      if (type === "hover") return pointerOver ? "visible" : "hidden";
      return st.machine === "hidden" ? "hidden" : "visible";
    }
    function presenceOf(axis) {
      const st = axes[axis];
      if (type === "always") return true;
      if (type === "auto") return st.overflow;
      if (type === "hover") return pointerOver && st.overflow;
      return st.machine !== "hidden";
    }
    function syncAxis(axis) {
      const st = axes[axis];
      const sb = scrollbarEl(axis);
      if (!st || !sb) return;
      const next = presenceOf(axis);
      sb.setAttribute("data-state", dataStateOf(axis));
      if (next) {
        if (st.fadeTimer) {
          cancel(st.fadeTimer);
          st.fadeTimer = 0;
        }
        if (!st.present) {
          st.present = true;
          applyCorner();
          fireState();
        }
      } else if (st.present) {
        st.fadeTimer = later(() => {
          st.fadeTimer = 0;
          st.present = false;
          applyCorner();
          fireState();
        }, EXIT_FADE);
      }
    }
    function applyCorner() {
      const v = axes.vertical;
      const h = axes.horizontal;
      const next = v !== null && h !== null && type !== "scroll" && v.present && h.present && v.sizes.cross > 0 && h.sizes.cross > 0;
      cornerPresent = next;
      const width = next ? v.sizes.cross : 0;
      const height = next ? h.sizes.cross : 0;
      const style = root.style;
      style.setProperty("position", "relative");
      style.setProperty("--radix-scroll-area-corner-width", width + "px");
      style.setProperty("--radix-scroll-area-corner-height", height + "px");
      if (cornerEl && next) {
        const cs = cornerEl.style;
        if (next) {
          cs.setProperty("width", width + "px");
          cs.setProperty("height", height + "px");
          cs.setProperty("position", "absolute");
          cs.setProperty(dir === "ltr" ? "right" : "left", "0px");
          cs.setProperty("bottom", "0px");
        }
      }
    }
    function snapshot() {
      const snap = (axis) => {
        const st = axes[axis];
        return st ? { present: st.present, hasThumb: st.hasThumb } : null;
      };
      return { vertical: snap("vertical"), horizontal: snap("horizontal"), corner: cornerPresent };
    }
    function fireState() {
      options.onStateChange?.(snapshot());
    }
    function send(axis, event) {
      const st = axes[axis];
      if (!st) return;
      const next = MACHINE[st.machine][event];
      if (next === void 0 || next === st.machine) return;
      cancel(st.hideTimer);
      st.hideTimer = 0;
      st.machine = next;
      if (next === "idle") st.hideTimer = later(() => send(axis, "HIDE"), scrollHideDelay);
      syncAxis(axis);
    }
    function handleScroll() {
      for (const axis of ["vertical", "horizontal"]) {
        const st = axes[axis];
        if (!st) continue;
        const pos = scrollPos(axis);
        if (pos !== st.prevPos) {
          st.prevPos = pos;
          if (type === "scroll") {
            send(axis, "SCROLL");
            cancel(st.debounceTimer);
            st.debounceTimer = later(() => send(axis, "SCROLL_END"), SCROLL_END_DEBOUNCE);
          }
        }
      }
      syncThumb("vertical");
      syncThumb("horizontal");
    }
    on(viewport, "scroll", handleScroll);
    if (type === "hover") {
      on(root, "pointerenter", () => {
        cancel(hoverHideTimer);
        hoverHideTimer = 0;
        pointerOver = true;
        syncAxis("vertical");
        syncAxis("horizontal");
      });
      on(root, "pointerleave", () => {
        hoverHideTimer = later(() => {
          hoverHideTimer = 0;
          pointerOver = false;
          syncAxis("vertical");
          syncAxis("horizontal");
        }, 0);
      });
    }
    for (const axis of ["vertical", "horizontal"]) {
      const st = axes[axis];
      const sb = scrollbarEl(axis);
      if (!st || !sb) continue;
      if (type === "scroll") {
        on(sb, "pointerenter", () => send(axis, "POINTER_ENTER"));
        on(sb, "pointerleave", () => send(axis, "POINTER_LEAVE"));
      }
      on(
        sb,
        "wheel",
        (event) => {
          const e = event;
          if (axis === "vertical") viewport.scrollTop = viewport.scrollTop + e.deltaY;
          else viewport.scrollLeft = viewport.scrollLeft + e.deltaX;
          const maxScrollPos = st.sizes.content - st.sizes.viewport;
          const pos = scrollPos(axis);
          if (pos > 0 && pos < maxScrollPos) e.preventDefault();
        },
        { passive: false }
      );
    }
    const roFactory = options.createResizeObserver !== void 0 ? options.createResizeObserver : win && typeof win.ResizeObserver === "function" ? (callback) => new win.ResizeObserver(callback) : null;
    let resizeTimer = 0;
    const observer = roFactory ? roFactory(() => {
      cancel(resizeTimer);
      resizeTimer = later(recompute, RESIZE_DEBOUNCE);
    }) : null;
    if (observer) {
      observer.observe(viewport);
      if (options.content) observer.observe(options.content);
      for (const axis of ["vertical", "horizontal"]) {
        const sb = scrollbarEl(axis);
        if (sb) observer.observe(sb);
      }
    }
    function recompute() {
      const before = snapshot();
      computeAxis("vertical");
      computeAxis("horizontal");
      syncAxis("vertical");
      syncAxis("horizontal");
      applyCorner();
      if (JSON.stringify(before) !== JSON.stringify(snapshot())) fireState();
    }
    recompute();
    fireState();
    return {
      state: snapshot,
      recompute,
      destroy() {
        for (const [target, eventType, handler] of listeners) target.removeEventListener(eventType, handler);
        for (const axis of ["vertical", "horizontal"]) {
          const st = axes[axis];
          if (!st) continue;
          cancel(st.fadeTimer);
          cancel(st.debounceTimer);
          cancel(st.hideTimer);
        }
        cancel(hoverHideTimer);
        cancel(resizeTimer);
        observer?.disconnect();
      }
    };
  }

  // src/features/menu.ts
  function wireMenu(options) {
    const exitDuration = options.exitDuration ?? 100;
    const protocol = {
      menuTrigger: options.protocol?.menuTrigger ?? "data-radixuigo-menu-trigger",
      menuSubtrigger: options.protocol?.menuSubtrigger ?? "data-radixuigo-menu-subtrigger",
      contextTrigger: options.protocol?.contextTrigger ?? "data-radixuigo-context-trigger"
    };
    const collisionPadding = options.collisionPadding ?? 10;
    let documentRef = null;
    const layers2 = [];
    const exiting = [];
    let guards = null;
    const hiding = refcountHiding(options.isPortalMarker);
    const guardsAdd = (body) => {
      if (guards) return;
      const mk = () => {
        const s = body.ownerDocument.createElement("span");
        s.setAttribute("data-radix-focus-guard", "");
        s.setAttribute("tabindex", "0");
        s.setAttribute("aria-hidden", "true");
        s.setAttribute("style", "outline: none; opacity: 0; position: fixed; pointer-events: none;");
        return s;
      };
      guards = { start: mk(), end: mk() };
      body.insertBefore(guards.start, body.firstChild);
      body.appendChild(guards.end);
    };
    const guardsRemove = () => {
      if (!guards) return;
      guards.start.parentNode?.removeChild(guards.start);
      guards.end.parentNode?.removeChild(guards.end);
      guards = null;
    };
    const menuItems = (content) => Array.from(content.querySelectorAll('[role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"]'));
    const activeLayer = () => layers2[layers2.length - 1];
    const rootLayerRec = () => layers2[0];
    const setState = (el, state) => {
      el.setAttribute("data-state", state);
    };
    const setTriggerOpen = (trigger, content, side) => {
      if (trigger.hasAttribute(protocol.contextTrigger)) {
        setState(trigger, "open");
        return;
      }
      trigger.setAttribute("aria-expanded", "true");
      trigger.setAttribute("aria-controls", content.id);
      setState(trigger, "open");
      trigger.setAttribute("data-radix-popper-side", side);
      trigger.setAttribute("data-radix-popper-align", "start");
    };
    const setTriggerClosed = (trigger) => {
      if (trigger.hasAttribute("aria-haspopup")) {
        trigger.setAttribute("aria-expanded", "false");
        trigger.removeAttribute("aria-controls");
        trigger.removeAttribute("data-radix-popper-side");
        trigger.removeAttribute("data-radix-popper-align");
      }
      setState(trigger, "closed");
    };
    const highlightItem = (item) => {
      const content = activeLayer()?.layer.content;
      if (!content) return;
      for (const el of Array.from(content.querySelectorAll("[data-highlighted]"))) {
        el.removeAttribute("data-highlighted");
      }
      if (item) {
        item.setAttribute("data-highlighted", "");
        item.focus();
      }
    };
    const moveHighlight = (dir) => {
      const content = activeLayer()?.layer.content;
      if (!content) return;
      const items = menuItems(content);
      if (!items.length) return;
      let cur = -1;
      for (let i = 0; i < items.length; i++) {
        if (items[i].hasAttribute("data-highlighted")) {
          cur = i;
          break;
        }
      }
      let next;
      if (cur === -1) next = dir > 0 ? 0 : items.length - 1;
      else {
        next = cur + dir;
        if (next < 0 || next >= items.length) next = cur;
      }
      highlightItem(items[next]);
    };
    const makePopper = (layer, trigger, positioning) => {
      const anchor = positioning.point ? {
        getBoundingClientRect: () => ({
          x: positioning.point.x,
          y: positioning.point.y,
          top: positioning.point.y,
          left: positioning.point.x,
          right: positioning.point.x,
          bottom: positioning.point.y,
          width: 0,
          height: 0
        })
      } : trigger;
      return positionPopper(
        {
          anchor,
          floating: layer.wrapper,
          content: layer.content,
          arrow: null
        },
        {
          side: positioning.side,
          align: "start",
          sideOffset: positioning.sideOffset,
          alignOffset: positioning.alignOffset ?? 0,
          collisionPadding
        },
        (side) => {
          if (!positioning.point && !trigger.hasAttribute(protocol.contextTrigger)) {
            trigger.setAttribute("data-radix-popper-side", side);
            trigger.setAttribute("data-radix-popper-align", "start");
          }
        }
      );
    };
    const focusLayerIn = (layer, positioning) => {
      layer.content.style.removeProperty("animation");
      if (positioning.keyboard) highlightItem(menuItems(layer.content)[0]);
      else layer.content.focus();
    };
    const openMenuLayer = (id, trigger, positioning) => {
      if (layers2.some((r) => r.layer.id === id)) return;
      const adopt = exiting.findIndex((r) => r.layer.id === id);
      if (adopt !== -1) {
        const rec2 = exiting[adopt];
        exiting.splice(adopt, 1);
        rec2.exit?.cancel();
        rec2.exit = null;
        rec2.popper = makePopper(rec2.layer, trigger, positioning);
        setState(rec2.layer.content, "open");
        setTriggerOpen(trigger, rec2.layer.content, positioning.side);
        layers2.push(rec2);
        focusLayerIn(rec2.layer, positioning);
        return;
      }
      const layer = options.mountLayer(id);
      if (!layer) return;
      documentRef = layer.wrapper.ownerDocument;
      const rec = {
        layer,
        sub: layers2.length > 0,
        hidden: null,
        exit: null,
        popper: makePopper(layer, trigger, positioning)
      };
      setState(layer.content, "open");
      setTriggerOpen(trigger, layer.content, positioning.side);
      if (!rec.sub) {
        guardsAdd(layer.wrapper.ownerDocument.body);
        rec.hidden = hiding.hideBackground(layer.wrapper);
      }
      layers2.push(rec);
      focusLayerIn(layer, positioning);
    };
    const closeMenuLayer = (rec, silent) => {
      const idx = layers2.indexOf(rec);
      if (idx === -1) return;
      layers2.splice(idx, 1);
      rec.popper?.destroy();
      rec.popper = null;
      setState(rec.layer.content, "closed");
      setTriggerClosed(rec.layer.trigger);
      const wasRoot = !rec.sub && layers2.length === 0;
      rec.exit = presenceExit(
        rec.layer.content,
        exitDuration,
        () => {
          const ei = exiting.indexOf(rec);
          if (ei !== -1) exiting.splice(ei, 1);
          rec.layer.wrapper.remove();
          if (rec.hidden) hiding.restoreBackground(rec.hidden);
          if (wasRoot && guards && layers2.length === 0) {
            guardsRemove();
            options.onAllClosed?.();
          }
        }
      );
      exiting.push(rec);
      if (!silent) {
        const rootTrig = rec.sub ? rootLayerRec()?.layer.trigger ?? rec.layer.trigger : rec.layer.trigger;
        setTimeout(() => rootTrig.focus(), 0);
      }
    };
    const closeAll = (silent = false) => {
      for (let i = layers2.length - 1; i >= 0; i--) closeMenuLayer(layers2[i], silent);
    };
    const insideOpenMenu = (el) => layers2.some((r) => r.layer.wrapper.contains(el));
    const handles = {
      onDocumentClick(target, preventDefault) {
        const trig = target.closest(`[${protocol.menuTrigger}]`);
        if (trig) {
          preventDefault();
          const id = trig.getAttribute(protocol.menuTrigger);
          if (rootLayerRec()?.layer.id === id) closeAll();
          else {
            if (exiting.some((r) => !r.layer.trigger.hasAttribute(protocol.contextTrigger))) return;
            closeAll(true);
            openMenuLayer(id, trig, { side: "bottom", sideOffset: 4 });
          }
          return;
        }
        const subtrig = target.closest(`[${protocol.menuSubtrigger}]`);
        if (subtrig && insideOpenMenu(subtrig)) {
          preventDefault();
          openMenuLayer(subtrig.getAttribute(protocol.menuSubtrigger), subtrig, {
            side: "right",
            sideOffset: 1,
            alignOffset: -8
          });
          return;
        }
        if (insideOpenMenu(target)) {
          const role = target.closest('[role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"]');
          if (role) {
            preventDefault();
            if (options.onBeforeSelect?.(role, "pointer") === false) return;
            closeAll();
            return;
          }
        }
        if (layers2.length && !target.closest("[data-radix-popper-content-wrapper]")) {
          closeAll();
        }
      },
      onContextmenu(target, x, y, preventDefault) {
        const trig = target.closest(`[${protocol.contextTrigger}]`);
        if (trig) {
          preventDefault();
          const id = trig.getAttribute(protocol.contextTrigger);
          if (rootLayerRec()?.layer.id === id) closeAll(true);
          else {
            closeAll(true);
            openMenuLayer(id, trig, {
              side: "right",
              sideOffset: 2,
              alignOffset: -8,
              point: { x, y }
            });
          }
          return;
        }
        if (layers2.length && !target.closest("[data-radix-popper-content-wrapper]")) {
          closeAll(true);
        }
      },
      onKeydown(target, key, preventDefault) {
        if (layers2.length) {
          const top = activeLayer();
          if (key === "Escape") {
            preventDefault();
            closeAll();
            return;
          }
          if (key === "Tab") {
            preventDefault();
            return;
          }
          if (key === "ArrowDown") {
            preventDefault();
            moveHighlight(1);
            return;
          }
          if (key === "ArrowUp") {
            preventDefault();
            moveHighlight(-1);
            return;
          }
          if (key === "ArrowRight") {
            const cur = top.layer.content.querySelector("[data-highlighted]");
            if (cur && cur.hasAttribute(protocol.menuSubtrigger)) {
              preventDefault();
              openMenuLayer(cur.getAttribute(protocol.menuSubtrigger), cur, {
                side: "right",
                sideOffset: 1,
                alignOffset: -8,
                keyboard: true
              });
            }
            return;
          }
          if (key === "ArrowLeft") {
            if (top.sub) {
              preventDefault();
              closeMenuLayer(top, true);
              highlightItem(top.layer.trigger);
            }
            return;
          }
          if (key === "Enter" || key === " ") {
            const doc = top.layer.content.ownerDocument;
            const ae = doc.activeElement;
            const aeRole = ae?.getAttribute?.("role");
            if (ae && ae.closest && ae.closest('[role="menu"]') === top.layer.content && (aeRole === "menuitem" || aeRole === "menuitemcheckbox" || aeRole === "menuitemradio")) {
              preventDefault();
              if (options.onBeforeSelect?.(ae, "keyboard") === false) return;
              closeAll();
            }
          }
          return;
        }
        const trig = target?.closest?.(`[${protocol.menuTrigger}]`);
        if (trig && key === "ArrowDown") {
          preventDefault();
          openMenuLayer(trig.getAttribute(protocol.menuTrigger), trig, {
            side: "bottom",
            sideOffset: 4,
            keyboard: true
          });
        }
      },
      openMenu: (id, trigger, positioning) => openMenuLayer(id, trigger, positioning),
      toggleMenu: (id, trigger, positioning) => {
        if (rootLayerRec()?.layer.id === id) closeAll();
        else {
          closeAll(true);
          openMenuLayer(id, trigger, positioning);
        }
      },
      closeAll,
      depth: () => layers2.length,
      rootLayer: () => rootLayerRec()?.layer,
      destroy() {
        for (const rec of layers2.slice()) {
          rec.exit?.cancel();
          rec.exit = null;
          rec.popper?.destroy();
          rec.layer.wrapper.remove();
          setTriggerClosed(rec.layer.trigger);
        }
        for (const rec of exiting.splice(0)) {
          rec.exit?.cancel();
          rec.exit = null;
          rec.layer.wrapper.remove();
        }
        layers2.length = 0;
        hiding.reset();
        guardsRemove();
      }
    };
    return handles;
  }
  return __toCommonJS(index_exports);
})();
globalThis.RadixKernel = RadixKernel;

;
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

;
const shadless = globalThis.shadless
export default shadless
export const { init, initAll, destroy, refresh, start, stop, register, get, instances, h, theme } = shadless
