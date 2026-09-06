/* shadcn-docs — interactions for the shadless docs theme.
   Forked from vitezola's app.js (search, code groups, copy buttons,
   scrollspy); the appearance toggle and mobile drawer were rewritten for
   this theme's markup. The localStorage key is unchanged so visitors keep
   their light/dark preference across the theme swap. */
(function () {
  "use strict";

  var d = document;
  var root = d.documentElement;

  /* ── Appearance ─────────────────────────────────────────────────────── */

  var modeToggle = d.getElementById("ModeToggle");

  function isDark() {
    return root.classList.contains("dark");
  }

  function toggleAppearance() {
    try {
      localStorage.setItem("vitepress-theme-appearance", isDark() ? "light" : "dark");
    } catch (e) {}
    root.classList.toggle("dark", !isDark());
  }

  if (modeToggle) modeToggle.addEventListener("click", toggleAppearance);

  /* ── Mobile nav drawer ──────────────────────────────────────────────── */

  var mobileToggle = d.getElementById("MobileNavToggle");
  var mobileNav = d.getElementById("MobileNav");

  function setMobileNav(open) {
    if (!mobileNav) return;
    mobileNav.hidden = !open;
    if (mobileToggle) mobileToggle.setAttribute("aria-expanded", open ? "true" : "false");
    d.body.style.overflow = open ? "hidden" : "";
  }

  if (mobileToggle) {
    mobileToggle.addEventListener("click", function () {
      setMobileNav(mobileNav.hidden);
    });
  }
  if (mobileNav) {
    mobileNav.addEventListener("click", function (e) {
      if (e.target.closest("[data-mobile-nav-close]") || e.target.closest("a")) {
        setMobileNav(false);
      }
    });
  }

  /* ── Code groups ────────────────────────────────────────────────────── */
  /* Builds radio tabs from the `name=` fence annotation; the .vp-code-group
     markup comes from the site's content pipeline. */

  d.querySelectorAll(".vp-code-group").forEach(function (group, gi) {
    var blocks = group.querySelectorAll(".blocks > pre");
    var allNamed = Array.prototype.every.call(blocks, function (pre) {
      return pre.querySelector("code[data-name]");
    });
    if (blocks.length < 2 || !allNamed) {
      // No tab bar to build — show every block stacked instead of leaving
      // them all hidden behind the .active gate.
      blocks.forEach(function (b) {
        b.classList.add("active");
      });
      return;
    }
    var tabs = d.createElement("div");
    tabs.className = "tabs";
    var groupName = "vp-tab-" + gi;
    blocks.forEach(function (pre, i) {
      var code = pre.querySelector("code[data-name]");
      var name = code ? code.getAttribute("data-name") : "";
      if (!name) return;
      var input = d.createElement("input");
      input.type = "radio";
      input.name = groupName;
      input.id = groupName + "-" + i;
      if (i === 0) input.checked = true;
      var label = d.createElement("label");
      label.htmlFor = input.id;
      label.textContent = name;
      input.addEventListener("change", function () {
        blocks.forEach(function (b) {
          b.classList.remove("active");
        });
        pre.classList.add("active");
      });
      tabs.appendChild(input);
      tabs.appendChild(label);
    });
    group.prepend(tabs);
    blocks[0].classList.add("active");
  });

  /* ── Code copy buttons + lang labels ────────────────────────────────── */

  function copyText(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      return navigator.clipboard.writeText(text);
    }
    // Non-secure origins (plain http): execCommand fallback
    return new Promise(function (resolve, reject) {
      var ta = d.createElement("textarea");
      ta.value = text;
      ta.style.position = "fixed";
      ta.style.opacity = "0";
      d.body.appendChild(ta);
      ta.select();
      try {
        d.execCommand("copy") ? resolve() : reject(new Error("copy failed"));
      } catch (err) {
        reject(err);
      } finally {
        ta.remove();
      }
    });
  }

  var LANG_ALIASES = {
    shellscript: "sh",
    typescript: "ts",
    javascript: "js",
    markdown: "md",
    plain: "text"
  };

  d.querySelectorAll("pre.giallo").forEach(function (pre) {
    var code = pre.querySelector("code");
    var rawLang = code ? code.getAttribute("data-lang") : null;
    if (code && rawLang && rawLang !== "plain") {
      pre.classList.add("language-" + rawLang);
      var lang = d.createElement("span");
      lang.className = "lang";
      lang.textContent = LANG_ALIASES[rawLang] || rawLang;
      pre.appendChild(lang);
    }
    var button = d.createElement("button");
    button.type = "button";
    button.className = "vp-copy-button copy";
    button.setAttribute("aria-label", "Copy code");
    button.addEventListener("click", function () {
      if (!code) return;
      copyText(code.textContent).then(function () {
        button.classList.add("copied");
        setTimeout(function () {
          button.classList.remove("copied");
        }, 1200);
      });
    });
    pre.appendChild(button);
  });

  /* ── Scrollspy: "On this page" active link ──────────────────────────── */

  var tocNav = d.querySelector(".docs-toc");
  var tocLinks = tocNav
    ? Array.prototype.slice.call(tocNav.querySelectorAll("a"))
    : [];

  function scrollTick() {
    if (!tocLinks.length) return;
    var activeLink = null;
    tocLinks.forEach(function (link) {
      var id = decodeURIComponent(link.hash.slice(1));
      var heading = d.getElementById(id);
      if (!heading) return;
      link.classList.remove("active");
      if (heading.getBoundingClientRect().top < 96) activeLink = link;
    });
    if (activeLink) activeLink.classList.add("active");
  }

  var ticking = false;
  window.addEventListener(
    "scroll",
    function () {
      if (!ticking) {
        ticking = true;
        requestAnimationFrame(function () {
          scrollTick();
          ticking = false;
        });
      }
    },
    { passive: true }
  );
  scrollTick();

  /* ── Local search ───────────────────────────────────────────────────── */
  /* Modal markup is in partials/search_modal.html; the IDs are kept from
     vitezola so this block is a verbatim port. */

  var searchBox = d.getElementById("VPLocalSearchBox");
  var searchButton = d.getElementById("VPSearchButton");
  var searchInput = d.getElementById("localsearch-input");
  var searchResults = d.getElementById("VPSearchResults");
  var searchClear = d.getElementById("VPSearchClear");
  var indexPromise = null;
  var selectedIndex = -1;
  var currentResults = [];
  var lastFocus = null;

  function searchIndexUrl() {
    if (window.shadcnDocs && window.shadcnDocs.searchIndexUrl) {
      return window.shadcnDocs.searchIndexUrl;
    }
    return null;
  }

  function loadIndex() {
    if (!indexPromise) {
      indexPromise = fetch(searchIndexUrl())
        .then(function (r) {
          if (!r.ok) throw new Error("search index " + r.status);
          return r.json();
        })
        .catch(function (err) {
          indexPromise = null;
          throw err;
        });
    }
    return indexPromise;
  }

  function esc(s) {
    return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  function mark(text, tokens) {
    if (!tokens.length) return esc(text);
    var pattern = new RegExp(
      "(" + tokens.map(function (t) { return t.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"); }).join("|") + ")",
      "gi"
    );
    // split on the raw text first, then escape each piece, so tokens can
    // never match inside HTML entities produced by esc()
    var parts = text.split(pattern);
    return parts
      .map(function (part) {
        var isToken = tokens.some(function (t) {
          return part.toLowerCase() === t.toLowerCase();
        });
        return isToken ? "<mark>" + esc(part) + "</mark>" : esc(part);
      })
      .join("");
  }

  function makeExcerpt(content, tokens) {
    var lower = content.toLowerCase();
    var pos = -1;
    for (var i = 0; i < tokens.length && pos < 0; i++) {
      pos = lower.indexOf(tokens[i]);
    }
    var start = Math.max(0, pos - 60);
    var end = Math.min(content.length, (pos < 0 ? 0 : pos) + 240);
    return (start > 0 ? "…" : "") + content.slice(start, end) + (end < content.length ? "…" : "");
  }

  function runSearch(q) {
    if (!searchIndexUrl()) return Promise.resolve([]);
    return loadIndex().then(function (index) {
      var tokens = q.toLowerCase().split(/\s+/).filter(Boolean);
      if (!tokens.length) return [];
      return index
        .map(function (entry) {
          var title = (entry.title || "").toLowerCase();
          var content = (entry.body || entry.content || entry.contents || "").toLowerCase();
          var score = 0;
          tokens.forEach(function (t) {
            if (title === t) score += 20;
            while (title.indexOf(t) !== -1) {
              score += 5;
              title = title.replace(t, " ");
            }
            var count = 0;
            var idx = content.indexOf(t);
            while (idx !== -1 && count < 50) {
              count++;
              idx = content.indexOf(t, idx + t.length);
            }
            score += Math.min(count, 20);
          });
          return { entry: entry, score: score };
        })
        .filter(function (r) {
          return r.score > 0;
        })
        .sort(function (a, b) {
          return b.score - a.score;
        })
        .slice(0, 20);
    });
  }

  function renderResults(results, q) {
    currentResults = results;
    selectedIndex = -1;
    searchResults.innerHTML = "";
    if (!results.length) {
      searchResults.innerHTML =
        '<li class="no-results">' +
        (q ? 'No results for "<b>' + esc(q) + '</b>"' : "") +
        "</li>";
      return;
    }
    var tokens = q.toLowerCase().split(/\s+/).filter(Boolean);
    results.forEach(function (r) {
      var li = d.createElement("li");
      li.className = "result";
      li.setAttribute("role", "option");
      var div = d.createElement("div");
      var path = (r.entry.url || "").replace(/^https?:\/\/[^/]+/, "").split("/").filter(Boolean);
      var titles = '<div class="titles">';
      path.slice(0, -1).forEach(function (seg) {
        titles += '<p class="title">' + esc(seg) + "</p>";
      });
      titles += '<p class="title main">' + mark(r.entry.title || r.entry.url, tokens) + "</p></div>";
      var excerpt = '<p class="excerpt">' + mark(makeExcerpt(r.entry.body || r.entry.content || "", tokens), tokens) + "</p>";
      div.innerHTML = titles + excerpt;
      li.appendChild(div);
      li.addEventListener("click", function () {
        window.location.href = r.entry.url;
      });
      searchResults.appendChild(li);
    });
  }

  function openSearch() {
    if (!searchBox || !searchIndexUrl()) return;
    lastFocus = d.activeElement;
    searchBox.hidden = false;
    d.body.style.overflow = "hidden";
    searchInput.focus();
    searchInput.select();
  }

  function closeSearch() {
    searchBox.hidden = true;
    d.body.style.overflow = "";
    if (lastFocus && lastFocus.focus) lastFocus.focus();
  }

  if (searchButton) searchButton.addEventListener("click", openSearch);
  if (searchBox) {
    d.getElementById("VPSearchBackdrop").addEventListener("click", closeSearch);
    searchInput.addEventListener("input", function () {
      var q = searchInput.value.trim();
      searchClear.disabled = !q;
      if (!q) {
        renderResults([], "");
        return;
      }
      runSearch(q)
        .then(function (results) {
          renderResults(results, q);
        })
        .catch(function () {
          searchResults.innerHTML =
            '<li class="no-results">Search index unavailable.</li>';
        });
    });
    searchClear.addEventListener("click", function () {
      searchInput.value = "";
      searchInput.dispatchEvent(new Event("input"));
      searchInput.focus();
    });
    searchInput.addEventListener("keydown", function (e) {
      var items = searchResults.querySelectorAll(".result");
      if (e.key === "ArrowDown" || e.key === "ArrowUp") {
        e.preventDefault();
        if (!items.length) return;
        selectedIndex =
          e.key === "ArrowDown"
            ? Math.min(selectedIndex + 1, items.length - 1)
            : Math.max(selectedIndex - 1, 0);
        items.forEach(function (item, i) {
          item.classList.toggle("selected", i === selectedIndex);
        });
        items[selectedIndex].scrollIntoView({ block: "nearest" });
      } else if (e.key === "Enter") {
        e.preventDefault();
        if (selectedIndex >= 0 && currentResults[selectedIndex]) {
          window.location.href = currentResults[selectedIndex].entry.url;
        }
      }
    });
  }

  d.addEventListener("keydown", function (e) {
    if (searchBox && !searchBox.hidden) {
      if (e.key === "Escape") {
        closeSearch();
        return;
      }
      // minimal focus trap for the aria-modal dialog
      if (e.key === "Tab") {
        var focusables = [searchInput, searchClear].filter(function (el) {
          return el && !el.disabled;
        });
        if (focusables.length < 2) {
          e.preventDefault();
          searchInput.focus();
          return;
        }
        var first = focusables[0];
        var last = focusables[focusables.length - 1];
        if (e.shiftKey && d.activeElement === first) {
          e.preventDefault();
          last.focus();
        } else if (!e.shiftKey && d.activeElement === last) {
          e.preventDefault();
          first.focus();
        }
      }
      return;
    }
    if (!searchBox || searchBox.hidden) {
      if (e.key === "k" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        openSearch();
      } else if (e.key === "/" && d.activeElement && !/^(INPUT|TEXTAREA)$/.test(d.activeElement.tagName)) {
        e.preventDefault();
        openSearch();
      }
    }
  });
})();
