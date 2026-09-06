// shadless dropdown-menu behavior — registers with shadless.h.installMenuFamily,
// the shared wireMenu glue also used by context-menu.js (core.js has the full
// story: the two files' bodies were byte-identical but for this line, so the
// body now lives once and both files just point at it).
(function () {
  shadless.register("dropdown-menu", { init: shadless.h.installMenuFamily })
})()
