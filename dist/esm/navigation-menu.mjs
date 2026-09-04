import "./shadless.mjs"
;
// generated: shadless navigation-menu behavior — shared-viewport menu on the
// kernel. Measured radix semantics (oracle-driven):
//   - the viewport lives INSIDE the nav root (in flow, below the list) and
//     is fully unmounted when closed (no empty container in the DOM);
//   - content mounts into the viewport from the item's template, carries
//     data-state + aria-labelledby and receives aria-controls on the trigger;
//   - focus STAYS on the trigger while open (unlike menubar, which moves it
//     into the content); horizontal arrows only move focus between triggers
//     (no auto-switch, no roving tabindex — triggers keep natural focus);
//   - Escape / outside click close; size reaches CSS through the
//     --radix-navigation-menu-viewport-{width,height} vars.
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["navigation-menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["navigation-menu"] = true
  shadless.register("navigation-menu", { init: function () {
    if (shadless.__menuWired.installed_navigation_menu) return
    shadless.__menuWired.installed_navigation_menu = true
    var EXIT = 120;
    var idOf = function (t) { return t.getAttribute("data-radixuigo-nav-trigger"); };

    // One state set PER ROOT. This used to be a single
    // document.querySelector("[data-slot=navigation-menu]"), so a second nav
    // on the page got no handle and clicking its trigger mounted the content
    // into the FIRST nav's viewport — while the protocol table promised
    // "wires every instance it finds — several per page".
    var navs = [];
    function navOf(el) {
      var r = el && el.closest && el.closest("[data-slot=navigation-menu]");
      for (var i = 0; i < navs.length; i++) if (navs[i].root === r) return navs[i];
      return null;
    }

    function wire(root) {
    var triggers = function () {
      return Array.prototype.slice.call(root.querySelectorAll("[data-radixuigo-nav-trigger]"));
    };
    var viewport = function () { return root.querySelector("[data-slot=navigation-menu-viewport]"); };
    var openId = null;
    var exitTimer = null;

    function ensureViewport() {
      var vp = viewport()
      if (vp) return vp
      vp = document.createElement("div")
      // data-slot alone is enough: dist/css/navigation-menu.css already
      // ships [data-slot="navigation-menu-viewport"] { @apply … } — the
      // rule is compiled from upstream regardless of whether any static
      // page happens to render this element, same as every other
      // dynamically-mounted node in this codebase (no className to keep in
      // sync by hand, and hand-syncing it had already drifted: it carried
      // origin-top-center, which the shipped rule does not).
      vp.setAttribute("data-slot", "navigation-menu-viewport")
      vp.id = (openId || idOf(triggers()[0]) || "nav") + "-viewport"
      vp.setAttribute("data-orientation", "horizontal")
      vp.setAttribute("data-state", "closed")
      root.appendChild(vp)
      return vp
    }

    function close(silent) {
      if (!openId) return
      var trig = root.querySelector('[data-radixuigo-nav-trigger="' + openId + '"]')
      var content = viewport() && viewport().querySelector('[data-slot=navigation-menu-content]')
      if (trig) {
        trig.setAttribute("data-state", "closed")
        trig.setAttribute("aria-expanded", "false")
        trig.removeAttribute("aria-controls")
      }
      if (content) content.setAttribute("data-state", "closed")
      var vp = viewport()
      if (vp) {
        vp.setAttribute("data-state", "closed")
        clearTimeout(exitTimer)
        exitTimer = setTimeout(function () { if (vp.parentNode && vp.getAttribute("data-state") === "closed") vp.remove() }, EXIT)
      }
      openId = null
      if (trig) shadless.h.emit(trig, "close", "navigation-menu")
    }

    function open(id, trig) {
      clearTimeout(exitTimer)
      if (openId && openId !== id) close(true)
      var tpl = document.getElementById(id + "-content-tpl")
      if (!tpl) return
      var vp = ensureViewport()
      vp.setAttribute("data-state", "open")
      var content = tpl.content.firstElementChild.cloneNode(true)
      vp.replaceChildren(content)
      content.setAttribute("data-state", "open")
      trig.setAttribute("data-state", "open")
      trig.setAttribute("aria-expanded", "true")
      trig.setAttribute("aria-controls", content.id)
      var w = content.offsetWidth, h = content.offsetHeight
      vp.style.setProperty("--radix-navigation-menu-viewport-width", w + "px")
      vp.style.setProperty("--radix-navigation-menu-viewport-height", h + "px")
      openId = id
      shadless.h.emit(trig, "open", "navigation-menu")
    }

    triggers().forEach(function (t) {
      var id = idOf(t)
      shadless.instances.set(t, { component: "navigation-menu",
        open: function () { if (openId !== id) open(id, t) },
        close: function () { if (openId === id) close() },
        toggle: function () { openId === id ? close() : open(id, t) },
        isOpen: function () { return openId === id },
      })
    })
      return { root: root, triggers: triggers,
        openId: function () { return openId }, open: open, close: close };
    }

    Array.prototype.forEach.call(document.querySelectorAll("[data-slot=navigation-menu]"), function (r) {
      navs.push(wire(r));
    });

    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-nav-trigger]")
      if (trig) {
        var n = navOf(trig)
        if (!n) return
        e.preventDefault()
        var id = idOf(trig)
        if (id === n.openId()) n.close()
        else n.open(id, trig)
        return
      }
      var link = e.target.closest && e.target.closest("[data-slot=navigation-menu-link]")
      if (link) { var ln = navOf(link); if (ln && ln.openId()) ln.close(); return }
      navs.forEach(function (n) { if (n.openId() && !n.root.contains(e.target)) n.close() })
    })

    document.addEventListener("keydown", function (e) {
      var key = e.key
      if (key === "Escape") {
        var opened = navs.filter(function (n) { return n.openId() })
        if (opened.length) { e.preventDefault(); opened.forEach(function (n) { n.close() }) }
        return
      }
      if (key !== "ArrowRight" && key !== "ArrowLeft" && key !== "Home" && key !== "End") return
      var trig = e.target.closest && e.target.closest("[data-radixuigo-nav-trigger]")
      if (!trig) return
      var kn = navOf(trig)
      if (!kn) return
      e.preventDefault()
      var ts = kn.triggers()
      var i = ts.indexOf(trig)
      var next = key === "Home" ? ts[0]
        : key === "End" ? ts[ts.length - 1]
        : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length]
      next.focus()
    })
  } })
})()
