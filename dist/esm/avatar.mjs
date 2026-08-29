import "./shadless.mjs"
;
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
