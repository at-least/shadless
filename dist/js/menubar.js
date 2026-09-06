// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    // NOT routed through shadless.h.nextIndex: nextIndex locates "current"
    // via document.activeElement, but while a menu is open focus has moved
    // into its content (RadixKernel's focusLayerIn) — the open trigger this
    // needs to step from is handles.rootLayer().trigger, which nextIndex has
    // no way to receive. This wrap-around IS a third copy of the same
    // arithmetic, kept separate on purpose rather than fed a synthetic
    // "current index" that would silently stop matching real focus.
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        // nextIndex also skips disabled triggers and swaps the horizontal
        // arrows under dir=rtl — the shipped oracle-menubar-rtl fixture
        // exercises exactly this, which the old hand-rolled formula never did
        var n = shadless.h.nextIndex(e, ts);
        if (n < 0) return;
        var next = ts[n];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
    });
  } })
})()
