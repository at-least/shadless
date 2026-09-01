([h]) => {
    const doc = new DOMParser().parseFromString(`<body>${h}</body>`, "text/html")
    const canon = (node, parentRole = null, inCarousel = false) => {
      if (node.nodeType === 3) {
        // whitespace-only text nodes: JSX/serializer spacing artifacts —
        // they differ between SSR strings and innerHTML round-trips while
        // carrying no content
        const t = node.nodeValue.replace(/\s+/g, " ").trim()
        return t ? { t } : null
      }
      if (node.nodeType !== 1) return null
      const attrs = {}
      const ownRole = [...(node.attributes || [])].find((x) => x.name === "role")?.value ?? null
      // roving-focus containers (radiogroup/toolbar/tablist/menubar): radix
      // rotates tabindex across the subtree at runtime; the SSR initial
      // state and CSR settled state differ by design — tabindex inside such
      // subtrees is focus state, not structure
      const roving = ["radiogroup", "toolbar", "tablist", "menubar", "group"].includes(parentRole ?? ownRole ?? "")
      const hasPopup = [...(node.attributes || [])].some((x) => x.name === "aria-haspopup")
      const hasState = [...(node.attributes || [])].some((x) => x.name === "data-state")
      const carousel = inCarousel || [...(node.attributes || [])].some((x) => x.name === "data-slot" && x.value === "carousel-content")
      for (const a of node.attributes || []) {
        if (roving && a.name === "tabindex") continue
        // asChild Slot-merged triggers (deploy lag): our pin renders the
        // wrapper slot ("dialog-trigger"), live renders the child's
        // ("button") — same interactive element either way. The button
        // signature is gated on data-state (radix triggers carry it,
        // plain buttons don't); aria-haspopup is NOT a reliable gate —
        // newer radix dropped it from tooltip triggers.
        if (a.name === "data-slot" && (/-trigger$/.test(a.value) || (a.value === "button" && (hasPopup || hasState)))) { attrs["data-slot"] = "trigger"; continue }
        // aria-controls targets radix content ids — CSR drops it when the
        // controlled content is unmounted (closed), SSR renders it; the value
        // is a runtime id either way, so the key carries no comparable info
        if (a.name === "aria-controls") continue
        // srcset: Next-server responsive variants (/_next/image?...&w=640
        // 640w, …) — CDN artifact like the src indirection below
        if (a.name === "srcset") continue
        let v = a.value
        // next/image CDN indirection: SSR rewrites src through
        // /_next/image?url=<enc>&w=&q= — a Next-server concern, not DOM
        // semantics; recover the raw url both sides share
        if (a.name === "src" && v.startsWith("/_next/image?")) {
          try { v = new URLSearchParams(v.split("?")[1]).get("url") ?? v } catch {}
        }
        if (a.name === "class") v = [...new Set(v.split(/\s+/).filter(Boolean))].sort().join(" ")
        if (a.name === "style") {
          let decls = v.split(";").map((s) => s.trim()).filter(Boolean)
            .flatMap(normDecl)
          // radix Presence injects measured/animated declarations AFTER
          // mount (transition-duration, animation-name, resolved
          // --radix-collapsible-content-* measurements) that the SSR
          // snapshot cannot carry — runtime state noise, dropped on both
          // sides. SSR serializes without spaces (--a:var(--b)); CSR with
          // them.
          const RUNTIME_STYLE = new Set([
            "transition-duration", "animation-name", "animation-duration",
            "--radix-collapsible-content-height", "--radix-collapsible-content-width",
          ])
          // embla writes translate3d on the container once it measures —
          // the SSR frame cannot carry it; position-from-measurement is
          // runtime state, not structure
          if (carousel) RUNTIME_STYLE.add("transform")
          decls = decls
            .filter((s) => !RUNTIME_STYLE.has(s.split(":")[0].trim()))
            .map((s) => { const i = s.indexOf(":"); return s.slice(0, i).trim() + ":" + s.slice(i + 1).trim() })
            // React client collapses top/right/bottom/left:0 into `inset`; the
            // SSR serializer does not — expand inset so both sides canonicalize
            .flatMap((d) => d.startsWith("inset:") ? ["bottom:" + d.slice(6), "left:" + d.slice(6), "right:" + d.slice(6), "top:" + d.slice(6)] : [d])
            // unit normalization: CSR serializes 0 as "0px", SSR as "0"
            .map((d) => d.replace(/:0(px|rem|em|%|ch|vh|vw)$/, ":0"))
            // percentage precision: Chrome CSSOM serializes 3 decimals
            // (177.778%), React SSR writes the full float
            // (177.77777777777377%) — round both to 3
            .map((d) => d.replace(/:(\d+(\.\d+)?)%/, (m, num) => ":" + parseFloat(Number(num).toFixed(3)) + "%"))
          v = decls.sort().join(";")
          // CSR may leave a literally empty style="" behind (radix Presence
          // on a closed collapsible); SSR omits the attribute entirely
          if (v === "") continue
        }
        attrs[a.name] = v
      }
      // radix form-participation hidden controls (Switch/Checkbox/Radio
      // bubble <input>, Select's native <select>, Field's sr-only input):
      // rendered while the control ref is unsettled — the SSR frame sees
      // isFormControl=true — and removed after hydration when the demo has
      // no form ancestor. Identified by aria-hidden plus one of the two
      // hidden-control style fingerprints. Presence-only runtime state,
      // dropped wherever it appears.
      if ((node.tagName === "INPUT" || node.tagName === "SELECT") && attrs["aria-hidden"] === "true") {
        const s = attrs.style ?? ""
        if ((s.includes("opacity:0") && s.includes("pointer-events:none")) || s.includes("clip:rect(")) return null
      }
      // attrs as a SORTED map — SSR and CSR order attributes differently
      const sorted = {}
      for (const k of Object.keys(attrs).sort()) sorted[k] = attrs[k]
      const myRole = attrs.role ?? parentRole
      // adjacent text nodes: React SSR separates them with <!-- -->
      // comments (dropped by canon), while an innerHTML round-trip merges
      // them — join with a space on both sides so the boundary carries no
      // signal
      const kids = [...node.childNodes].map((c) => canon(c, myRole, carousel)).filter(Boolean)
      for (let i = kids.length - 1; i > 0; i--) {
        if (kids[i].t !== undefined && kids[i - 1].t !== undefined) {
          kids[i - 1] = { t: (kids[i - 1].t + " " + kids[i].t).replace(/\s+/g, " ").trim() }
          kids.splice(i, 1)
        }
      }
      return { tag: node.tagName, attrs: sorted, kids }
    }
    // Chrome CSSOM re-serializes the overflow longhand pair as the
    // shorthand ("overflow:hidden scroll"); React SSR writes the longhands
    // verbatim — same declarations, different spelling. Expand before the
    // RUNTIME_STYLE filter so both spellings meet as longhands.
    function normDecl(s) {
      const i = s.indexOf(":")
      const prop = s.slice(0, i).trim(), val = s.slice(i + 1).trim()
      if (prop !== "overflow") return [s]
      const parts = val.split(/\s+/)
      return parts.length === 1
        ? [`overflow-x:${parts[0]}`, `overflow-y:${parts[0]}`]
        : [`overflow-x:${parts[0]}`, `overflow-y:${parts[1]}`]
    }
    return canon(doc.body)
}
