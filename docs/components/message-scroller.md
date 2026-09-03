---
title: "Message Scroller"
description: "A chat scroll container that anchors turns, opens saved transcripts, follows streamed responses, loads history without jumping, and jumps to any message."
---

# Message Scroller

A chat scroll container that anchors turns, opens saved transcripts, follows streamed responses, loads history without jumping, and jumps to any message.

::::demo message-scroller-demo
<iframe class="demo" src="/demos/message-scroller.html" title="message-scroller-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller.html]
<div></div>
<div data-slot="message-scroller" class="group/message-scroller">
  <div
    data-slot="message-scroller-viewport"
    style="
      height: 160px;
      overflow: hidden;
      border: 1px solid var(--border);
      border-radius: 0.5rem;
      padding: 0.75rem;
      background: color-mix(in oklab, var(--muted) 30%, transparent);
    "
  >
    <div data-slot="message-scroller-content">
      <div data-slot="message-scroller-item" style="margin-bottom: 0.5rem">Top message</div>
      <div data-slot="message-scroller-item" style="margin-bottom: 0.5rem; margin-top: 3rem">
        Middle message
      </div>
      <div data-slot="message-scroller-item" style="margin-top: 6rem">Bottom message</div>
    </div>
  </div>
</div>
<div data-slot="message-scroller-viewport">
  <div style="padding: 0.75rem">Scrollable content</div>
</div>
<div data-slot="message-scroller-content"><div style="padding: 0.75rem">Item content</div></div>
<div data-slot="message-scroller-item">
  <div style="padding: 0.5rem; border: 1px solid var(--border); border-radius: 0.375rem">Item</div>
</div>
<button data-slot="message-scroller-button">
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    style="width: 1rem; height: 1rem"
  >
    <path d="M12 5v14M5 12l7 7 7-7"></path>
  </svg>
</button>
```
:::

::::


## What Makes a Great Streaming Chat Experience

Building a chat interface used to be simple. You create an inverted list with
an input. Type a message, it appends at the bottom. When a reply comes in, the
list grows and scrolls. Done.

Streaming breaks that model. Messages arrive in chunks while you may still be
reading, scrolling, or looking somewhere else entirely.

Now the challenge is preserving the reader's place while the conversation keeps
changing. Get that wrong and the experience feels jumpy: people are pulled to
the bottom, lose context, and have to find their way back.

In practice, this comes down to scroll: when to follow, when to hold, and when
to let the reader decide. A great streaming chat should:

1. **Move only when the reader asked to move.** If someone is reading, don’t pull them somewhere else. Auto-scroll should never be the default.
2. **Follow only while they’re following.** If they’re at the live edge, keep the stream in view. If they scroll away, leave them there.
3. **Every interaction is a signal.** Scrolling is not the only one. Selecting text, using the keyboard, opening a link, or searching should all stop the interface from moving.
4. **Start a new turn near the top of the viewport.** This gives the new turn somewhere it can be read from the beginning.
5. **Then stream in the answer.** The answer should grow into the screen, not immediately push everything away.
6. **Keep part of the previous conversation in context.** The prompt and reply should stay visually connected, and enough of the previous turn should remain visible so the reader knows where they are.
7. **Let new content arrive offscreen.** The conversation can keep streaming without changing what the reader is looking at.
8. **Show what’s happening out of view.** Make it clear when a response is still streaming or when new messages have arrived.
9. **Make it easy to return to the latest reply.** A “Jump to latest” action should bring the reader back and resume following.
10. **Let people jump anywhere in the conversation.** Long threads need message links, search, unread markers, and direct navigation.
11. **Reopen where the reader left off.** A saved conversation should open at the last meaningful turn. Often this is the last user message. Not the absolute bottom.
12. **Keep the reader’s place when layout changes.** Images load. Markdown expands. Code blocks render. Older messages appear above. None of that should make the reader lose their place.
13. **Handle interruptions without stealing position.** Stopping, retrying, regenerating, branching, or errors should not unexpectedly move the conversation.
14. **Stay responsive in long threads.** Streaming text, markdown, code, images, and long history should still feel responsive.
15. **Be accessible without the noise.** Keep the transcript navigable, preserve keyboard focus, and announce important events at a comfortable pace.

**Never move the reader against their intent.**

## MessageScroller

MessageScroller is a chat transcript scroller built for these behaviors.
`MessageScrollerProvider` owns the scroll state and transcript-row behavior:
opening position, streamed output, new-turn anchoring, prepended history,
visibility, and scroll controls. `MessageScroller` is the styled frame that
renders inside it.

MessageScroller is scoped to the scroll viewport. It does not own messages, AI state,
transport, persistence, branching, or model state. Your product code stays
focused on composing messages, markers, tools, attachments, and prompt inputs.

It gives you the scroll behavior that chat needs, without taking over the rest
of the chat UI. And it stays fast, even in long conversations with rich
markdown.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/message-scroller.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/message-scroller.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/message-scroller.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                         into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/message-scroller.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

See the demos for real compositions — every slot is a `data-slot` attribute in the shipped markup.

## Core Concepts

### Anchoring Turns

A turn is the part of the conversation that starts a new exchange. In a simple
AI chat, that is usually the user's message and the assistant reply that follows.

An anchor is the row the viewport should treat as the start of that turn. Mark
that row with `scrollAnchor`. When a new anchor is appended, the viewport moves
it near the top and keeps a peek of the previous item above it, so the new turn
does not feel detached from its context.

```tsx
// This tells the scroller to anchor the user's message for the next turn.
<MessageScrollerItem
  messageId={message.id}
  scrollAnchor={message.role === "user"}
/>
```

Scroll anchors are not tied to message role. You can turn any row into an anchor:
a user message, a system marker, a handoff event, or anything else that starts a
meaningful turn. `MessageScroller` only needs to know which row should anchor the
viewport.

In the following example, the user's message is anchored. When you send a new message, the viewport anchors it near the top and appends the assistant reply below it. Toggle the anchor to the assistant's message to see the difference.

::::demo message-scroller-anchoring
<iframe class="demo" src="/demos/message-scroller-anchoring.html" title="message-scroller-anchoring" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-anchoring.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-anchoring.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b">
      <div data-slot="card-title">Anchoring Turns</div>
      <div data-slot="card-description">Choose which role settles near the top edge.</div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="outline"
          data-size="icon"
          id="reset-btn"
          aria-label="Reset anchored turns"
          disabled
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
            <path d="M21 3v5h-5"></path>
          </svg>
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="empty" id="empty-state" class="h-full">
        <div data-slot="empty-header">
          <div data-slot="empty-icon">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <path d="M10.1 2.182a10 10 0 0 1 3.8 0"></path>
              <path d="M13.9 21.818a10 10 0 0 1-3.8 0"></path>
              <path d="M17.609 3.72a10 10 0 0 1 2.69 2.7"></path>
              <path d="M2.182 13.9a10 10 0 0 1 0-3.8"></path>
              <path d="M20.28 17.61a10 10 0 0 1-2.7 2.69"></path>
              <path d="M21.818 10.1a10 10 0 0 1 0 3.8"></path>
              <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"></path>
              <path d="m6.163 21.117-2.906.85a1 1 0 0 1-1.236-1.169l.965-2.98"></path>
            </svg>
          </div>
          <div data-slot="empty-title">No anchored messages yet</div>
          <div data-slot="empty-description">
            Send the first message to see the selected role anchor.
          </div>
        </div>
      </div>
      <div data-slot="message-scroller" class="group/message-scroller" id="scroller" hidden>
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div data-slot="card-footer">
      <div
        data-slot="toggle-group"
        id="tg1"
        dir="ltr"
        role="radiogroup"
        aria-label="Select scroll anchor role"
        data-spacing="2"
        style="--gap: 2"
        tabindex="0"
      >
        <button
          data-slot="toggle-group-item"
          type="button"
          value="user"
          aria-label="Anchor user messages"
          data-state="on"
          role="radio"
          aria-checked="true"
          data-spacing="2"
          tabindex="0"
        >
          User
        </button>
        <button
          data-slot="toggle-group-item"
          type="button"
          value="assistant"
          aria-label="Anchor assistant messages"
          data-state="off"
          role="radio"
          aria-checked="false"
          data-spacing="2"
          tabindex="-1"
        >
          Assistant
        </button>
      </div>
      <button
        data-slot="button"
        type="button"
        data-size="icon"
        id="send-btn"
        style="margin-inline-start: auto"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="m5 12 7-7 7 7"></path>
          <path d="M12 19V5"></path>
        </svg>
        <span class="sr-only">Send Message</span>
      </button>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Toggle the anchor role, then send messages to compare where turns settle.
  </p>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

(function () {
      var scripted = [
        { id: "anchor-1-user", role: "user", text: "Can you show me how anchoring behaves when a new prompt starts the turn?" },
        { id: "anchor-1-assistant", role: "assistant", text: "Append the user prompt first, then append the assistant response. With User selected, the prompt settles near the top and the assistant response fills in below it." },
        { id: "anchor-2-user", role: "user", text: "What changes when assistant messages are the anchor?" },
        { id: "anchor-2-assistant", role: "assistant", text: "Now each assistant response is the item MessageScroller keeps in view. This is useful when the reply is the moment you want readers to land on after each turn." },
        { id: "anchor-3-user", role: "user", text: "Can I switch roles and keep adding turns?" },
        { id: "anchor-3-assistant", role: "assistant", text: "Yes. The next appended message with the selected role becomes the anchor, so you can compare user and assistant anchoring without resetting the demo." },
      ]
      var anchorRole = "user"
      var messages = []
      var index = 0
      var PEEK = 32
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var sendBtn = document.getElementById("send-btn")
      var resetBtn = document.getElementById("reset-btn")
      var emptyState = document.getElementById("empty-state")
      var scroller = document.getElementById("scroller")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function rowHTML(m) {
        var isUser = m.role === "user"
        var isAnchor = m.role === anchorRole
        var body = '<div data-slot="bubble-content">' + (isUser ? m.text : paras(m.text).map(function (p) { return '<p style="white-space:pre-wrap">' + p + "</p>" }).join("")) + "</div>"
        return '<div data-slot="message-scroller-item" data-message-id="' + m.id + '"' + (isAnchor ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isUser ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + (isUser ? "muted" : "ghost") + '"' + (isUser ? ' data-align="end"' : "") + ">" + body + "</div>" +
          "</div></div></div>"
      }
      function syncSend() {
        sendBtn.disabled = index >= scripted.length
        if (sendBtn.disabled) sendBtn.setAttribute("data-disabled", "true"); else sendBtn.removeAttribute("data-disabled")
        resetBtn.disabled = messages.length === 0
        if (resetBtn.disabled) resetBtn.setAttribute("data-disabled", "true"); else resetBtn.removeAttribute("data-disabled")
      }
      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      function appendNext() {
        if (index >= scripted.length) return
        var m = scripted[index]
        messages.push(m)
        index++
        emptyState.hidden = true
        scroller.hidden = false
        var wasEmpty = messages.length === 1
        content.insertAdjacentHTML("beforeend", rowHTML(m))
        var newItem = content.lastElementChild
        if (m.role === anchorRole) {
          var prev = newItem.previousElementSibling
          var peek = prev ? PEEK : 0
          viewport.scrollTop = Math.max(0, newItem.offsetTop - peek)
        } else if (wasEmpty) {
          viewport.scrollTop = 0
        }
        update()
        syncSend()
      }
      sendBtn.addEventListener("click", appendNext)

      function reset() {
        messages = []
        index = 0
        content.innerHTML = ""
        scroller.hidden = true
        emptyState.hidden = false
        syncSend()
      }
      resetBtn.addEventListener("click", reset)

      // toggle-group selection (shadless.js manages visual state; react here)
      var items = document.querySelectorAll("[data-slot=toggle-group-item]")
      items.forEach(function (it) {
        it.addEventListener("click", function () {
          var v = it.getAttribute("value")
          if (v !== "user" && v !== "assistant") return
          if (v === anchorRole) return
          anchorRole = v
          reset()
        })
      })

      syncSend()
      shadless.initAll()
    })()
```
:::

::::


### Group Chat

In a group chat, the turn boundary is more specific than "the user message". It is often
the message that asks the model to respond, or a marker like "Marcus joined the
chat". Typing indicators and history controls usually should not anchor.

Because anchoring is role-independent, you can anchor a marker just as easily as
a message.

```tsx
<MessageScrollerItem messageId="marcus-joined" scrollAnchor>
  <Marker variant="separator">
    <MarkerContent>Marcus joined the chat</MarkerContent>
  </Marker>
</MessageScrollerItem>
```

::::demo message-scroller-group-chat
<iframe class="demo" src="/demos/message-scroller-group-chat.html" title="message-scroller-group-chat" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-group-chat.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-group-chat.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Group Chat</div>
      <div data-slot="card-description">
        A group chat with several participants and an assistant. The Marker is marked as a turn.
      </div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="outline"
          data-size="icon"
          id="t1-trigger"
          aria-label="Reset conversation"
          data-state="closed"
          disabled
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
            <path d="M21 3v5h-5"></path>
          </svg>
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="message-scroller" class="group/message-scroller">
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div
      data-slot="card-footer"
      class="border-t"
      style="flex-direction: column; align-items: center; gap: calc(var(--spacing) * 2)"
    >
      <button
        data-slot="button"
        type="button"
        data-variant="secondary"
        id="action-btn"
        style="width: 100%"
      >
        Add Rocky
      </button>
      <p id="footer-hint" class="text-xs" style="color: var(--muted-foreground)">
        This will create a marker and make it the anchor.
      </p>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    When a user joins, a marker is created. scrollAnchor on the marker marks it as the next turn.
  </p>
</div>
<template id="t1-portal">
  <div data-slot="tooltip-content" id="d1" role="tooltip" data-state="closed">
    Reset
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
      <path d="M12 5v14M5 12h14"></path>
    </svg>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()

(function () {
      var CURRENT = "Grace"
      var initialItems = [
        { type: "message", id: "group-1", sender: "Grace", role: "participant", text: "@mary, the astrophage line keeps matching Venus energy output. Can you check my math?" },
        { type: "message", id: "group-2", sender: "Mary (Agent)", role: "assistant", text: "Yes. Confirmed. The curve points to a microorganism harvesting stellar energy and breeding near carbon dioxide. If @rocky agrees, this is the clue we need." },
        { type: "message", id: "group-3", sender: "Grace", role: "participant", text: "ping @rocky", scrollAnchor: true },
      ]
      var rockyMarker = { type: "event", id: "group-4", text: "Rocky has joined the chat", scrollAnchor: true }
      var rockyMessage = { type: "message", id: "group-5", sender: "Rocky", role: "participant", text: "Amaze. Astrophage eats light, makes heat, goes to carbon dioxide. Rocky has fuel model. Grace is smart." }

      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var actionBtn = document.getElementById("action-btn")
      var hint = document.getElementById("footer-hint")
      var resetBtn = document.getElementById("t1-trigger")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")
      var PEEK = 32

      var turn = "idle" // idle | marker | message
      function currentItems() {
        return turn === "message" ? initialItems.concat([rockyMarker, rockyMessage])
          : turn === "marker" ? initialItems.concat([rockyMarker])
          : initialItems.slice()
      }
      function msgRow(it) {
        var isCurrent = it.sender === CURRENT
        var variant = isCurrent ? "muted" : it.role === "assistant" ? "ghost" : "tinted"
        var body = '<div data-slot="bubble-content">' + it.text + "</div>"
        var header = (!isCurrent ? '<div data-slot="message-header">' + it.sender + "</div>" : "")
        return '<div data-slot="message-scroller-item" data-message-id="' + it.id + '"' + (it.scrollAnchor ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isCurrent ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' + header +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + variant + '"' + (isCurrent ? ' data-align="end"' : "") + ">" + body + "</div>" +
          "</div></div></div>"
      }
      function markerRow(it) {
        return '<div data-slot="message-scroller-item" data-message-id="' + it.id + '"' + (it.scrollAnchor ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="marker" class="group/marker" data-variant="separator"><span data-slot="marker-content">' + it.text + "</span></div>" +
          "</div>"
      }
      function render() {
        var items = currentItems()
        content.innerHTML = items.map(function (it) { return it.type === "message" ? msgRow(it) : markerRow(it) }).join("")
      }
      render()

      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      function syncUI() {
        actionBtn.textContent = turn === "idle" ? "Add Rocky" : "Send Message as Rocky"
        var complete = turn === "message"
        actionBtn.disabled = complete
        if (complete) actionBtn.setAttribute("data-disabled", "true"); else actionBtn.removeAttribute("data-disabled")
        resetBtn.disabled = turn === "idle"
        if (turn === "idle") resetBtn.setAttribute("data-disabled", "true"); else resetBtn.removeAttribute("data-disabled")
        hint.textContent = turn === "idle" ? "This will create a marker and make it the anchor" : "Now send Rocky's reply into the conversation"
      }
      actionBtn.addEventListener("click", function () {
        if (turn === "message") return
        var prevTurn = turn
        turn = prevTurn === "idle" ? "marker" : "message"
        var items = currentItems()
        var newItems = prevTurn === "idle" ? [rockyMarker] : [rockyMessage]
        newItems.forEach(function (it) {
          var html = it.type === "message" ? msgRow(it) : markerRow(it)
          content.insertAdjacentHTML("beforeend", html)
        })
        var lastNew = content.lastElementChild
        if (lastNew && lastNew.getAttribute("data-scroll-anchor") !== null) {
          var prev = lastNew.previousElementSibling
          viewport.scrollTop = Math.max(0, lastNew.offsetTop - (prev ? PEEK : 0))
        }
        syncUI()
        update()
      })
      resetBtn.addEventListener("click", function () {
        if (turn === "idle") return
        turn = "idle"
        render()
        viewport.scrollTop = 0
        syncUI()
        update()
      })
      syncUI()
    })()
```
:::

::::


### Keeping Context Visible

When a new turn starts, it should still feel like part of the same continuous
thread. `scrollPreviousItemPeek` keeps a slice of the previous item visible
above the anchor, so the reader keeps their context instead of feeling like the
conversation restarted on a blank page.

```tsx
// Keep 64px of the previous turn visible above the newly anchored row.
<MessageScrollerProvider scrollPreviousItemPeek={64}>
  <MessageScroller>{/* anchored turns */}</MessageScroller>
</MessageScrollerProvider>
```

Adjust the peek amount in the example below to see how it affects the conversation.

::::demo message-scroller-previous-context
<iframe class="demo" src="/demos/message-scroller-previous-context.html" title="message-scroller-previous-context" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-previous-context.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-previous-context.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Keeping Context Visible</div>
      <div data-slot="card-description">New turns keep part of the previous reply in view.</div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="outline"
          data-size="icon"
          id="t1-trigger"
          aria-label="Reset context example"
          data-state="closed"
          disabled
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
            <path d="M21 3v5h-5"></path>
          </svg>
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="message-scroller" class="group/message-scroller" id="scroller">
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div data-slot="card-footer" style="flex-direction: column; gap: calc(var(--spacing) * 2)">
      <form id="composer" style="width: 100%">
        <div
          data-slot="input-group"
          class="group/input-group"
          style="height: auto; flex-direction: column; align-items: stretch"
        >
          <div style="height: 3.5rem; width: 100%; padding: 0.625rem 0.75rem">
            <span
              id="next-msg"
              class="line-clamp-2"
              style="
                display: -webkit-box;
                -webkit-line-clamp: 2;
                -webkit-box-orient: vertical;
                overflow: hidden;
              "
            ></span>
          </div>
          <div
            data-slot="input-group-addon"
            role="group"
            data-align="block-end"
            style="
              padding-top: 0.25rem;
              justify-content: flex-start;
              width: 100%;
              gap: calc(var(--spacing) * 2);
            "
          >
            <button
              data-slot="button"
              type="button"
              data-variant="outline"
              data-size="icon-sm"
              id="d1-trigger"
              aria-label="Add files"
              aria-haspopup="menu"
              aria-expanded="false"
              data-state="closed"
              data-radixuigo-menu-trigger="d1"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="M5 12h14"></path>
                <path d="M12 5v14"></path>
              </svg>
            </button>
            <div
              style="display: flex; width: 7rem; align-items: center; gap: calc(var(--spacing) * 2)"
            >
              <span class="text-xs tabular-nums" id="peek-px" style="color: var(--muted-foreground)"
                >64px</span
              >
              <span
                data-slot="slider"
                id="s1"
                data-orientation="horizontal"
                dir="ltr"
                class="w-full"
              >
                <span data-slot="slider-track" data-orientation="horizontal">
                  <span
                    data-slot="slider-range"
                    data-orientation="horizontal"
                    style="left: 0%; right: 100%"
                  ></span>
                </span>
                <span
                  style="
                    position: absolute;
                    transform: var(--radix-slider-thumb-transform);
                    left: 0%;
                  "
                >
                  <span
                    data-slot="slider-thumb"
                    role="slider"
                    tabindex="0"
                    aria-label="Previous context peek"
                    aria-valuemin="64"
                    aria-valuemax="128"
                    aria-valuenow="64"
                    aria-orientation="horizontal"
                  ></span>
                </span>
              </span>
            </div>
            <button
              data-slot="button"
              type="submit"
              data-variant="default"
              data-size="icon-sm"
              id="send-btn"
              style="margin-inline-start: auto"
              disabled
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="m5 12 7-7 7 7"></path>
                <path d="M12 19V5"></path>
              </svg>
              <span class="sr-only">Send</span>
            </button>
          </div>
        </div>
      </form>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Adjust the slider and send. Observe the previous message peek.
  </p>
</div>
<template id="t1-portal">
  <div data-slot="tooltip-content" id="tt1" role="tooltip" data-state="closed">
    Reset
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
      <path d="M12 5v14M5 12h14"></path>
    </svg>
  </div>
</template>
<template id="d1-tpl">
  <div
    data-slot="dropdown-menu-content"
    id="d1"
    role="menu"
    tabindex="-1"
    data-state="closed"
    dir="ltr"
    aria-orientation="vertical"
    data-orientation="vertical"
    aria-labelledby="d1-trigger"
    style="width: 11rem"
  >
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <path
          d="m16 6-8.414 8.586a2 2 0 0 0 2.829 2.829l8.414-8.586a4 4 0 1 0-5.657-5.657l-8.379 8.551a6 6 0 1 0 8.485 8.485l8.379-8.551"
        ></path></svg
      >Add Photos &amp; Files
    </div>
    <div data-slot="dropdown-menu-separator" role="separator" aria-orientation="horizontal"></div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <rect width="18" height="18" x="3" y="3" rx="2" ry="2"></rect>
        <circle cx="9" cy="9" r="2"></circle>
        <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path></svg
      >Create Image
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <path
          d="m10.065 12.493-6.18 1.318a.934.934 0 0 1-1.108-.702l-.537-2.15a1.07 1.07 0 0 1 .691-1.265l13.504-4.44"
        ></path>
        <path d="m13.56 11.747 4.332-.924"></path>
        <path d="m16 21-3.105-6.21"></path>
        <path
          d="M16.485 5.94a2 2 0 0 1 1.455-2.425l1.09-.272a1 1 0 0 1 1.212.727l1.515 6.06a1 1 0 0 1-.727 1.213l-1.09.272a2 2 0 0 1-2.425-1.455z"
        ></path>
        <path d="m6.158 8.633 1.114 4.456"></path>
        <path d="m8 21 3.105-6.21"></path></svg
      >Deep Research
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="12" cy="12" r="10"></circle>
        <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"></path>
        <path d="M2 12h20"></path></svg
      >Web Search
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
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
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()

(function () {
      var script = [
        { u: "I'm building a chat for our app and the scroll behavior is driving me nuts. Every time the AI streams a reply, the whole thread jumps around.", a: "That's the classic streaming scroll problem. Wrap your message list in MessageScroller and turn on autoScroll — the viewport pins to the bottom as tokens arrive, so users always see the latest text land in place.\n\nThe important part: it only auto-scrolls while the reader is already at the bottom. The moment they scroll up to read something earlier, auto-scroll backs off and their position is preserved. You get smooth streaming without fighting the user's intent." },
        { u: "Okay, but when someone sends a new message the view still feels jarring — like the whole conversation reloads from the top.", a: "MessageScrollerItem fixes that with turn anchoring. Set scrollAnchor on the turn that should settle near the top instead of blindly snapping to the document bottom.\n\nIt also leaves a small peek of the previous exchange visible above the anchor, so context isn't lost. The reply starts in view without that disorienting jump you get from a plain overflow container." },
        { u: "And if they've scrolled up to re-read an older answer? I don't want to yank them back down.", a: "You won't. Auto-scroll only runs when the viewport is already pinned to the bottom, so scrolling up is a deliberate opt-out — their place in the thread stays put even as new tokens keep arriving below.\n\nWhen there is content they haven't seen yet, MessageScrollerButton appears at the bottom of the viewport. One tap jumps them back to the newest message and re-engages auto-scroll. Same pattern as Slack or iMessage: quiet when you're caught up, helpful when you're not." },
        { u: "Last one — does this work with assistive tech?", a: "MessageScrollerContent sets role=\"log\" and aria-relevant=\"additions\" by default, so screen readers announce new messages as they stream in.\n\nThe scroll button is a real button with an sr-only label, and it's removed from the tab order when you're already at the bottom — no ghost focus stops." },
      ]
      var DEFAULT_PEEK = 64
      var peek = DEFAULT_PEEK
      var nextIdx = 2
      var busy = false
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var sendBtn = document.getElementById("send-btn")
      var resetBtn = document.getElementById("t1-trigger")
      var nextMsg = document.getElementById("next-msg")
      var peekPx = document.getElementById("peek-px")
      var form = document.getElementById("composer")
      var scroller = document.getElementById("scroller")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function userRow(text, n) {
        var id = "prev-u-" + n
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '" data-scroll-anchor="">' +
          '<div data-slot="message" class="group/message" data-align="end"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="muted" data-align="end"><div data-slot="bubble-content">' + text + "</div></div>" +
          "</div></div></div>"
      }
      function assistantRow(text, n) {
        var id = "prev-a-" + n
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '">' +
          '<div data-slot="message" class="group/message"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="ghost"><div data-slot="bubble-content">' +
          paras(text).map(function (p) { return '<p style="white-space:pre-wrap">' + p + "</p>" }).join("") + "</div></div>" +
          "</div></div></div>"
      }
      function assistantShell(id) {
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '">' +
          '<div data-slot="message" class="group/message"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="ghost"><div data-slot="bubble-content" id="' + id + '-c"></div></div>' +
          "</div></div></div>"
      }
      function renderInitial() {
        content.innerHTML = userRow(script[0].u, 1) + assistantRow(script[1].a, 1)
      }
      renderInitial()

      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      function syncUI() {
        var next = nextIdx < script.length
        sendBtn.disabled = !next || busy
        if (sendBtn.disabled) sendBtn.setAttribute("data-disabled", "true"); else sendBtn.removeAttribute("data-disabled")
        resetBtn.disabled = busy
        if (resetBtn.disabled) resetBtn.setAttribute("data-disabled", "true"); else resetBtn.removeAttribute("data-disabled")
        content.setAttribute("aria-busy", String(busy))
        nextMsg.style.opacity = busy ? "0.6" : "1"
        if (next) nextMsg.textContent = script[nextIdx].u
        else nextMsg.innerHTML = '<span style="color:var(--muted-foreground)">No messages queued. Reset the context.</span>'
      }

      function send() {
        if (nextIdx >= script.length || busy) return
        var uText = script[nextIdx].u
        var aText = script[nextIdx + 1].a
        var uN = (nextIdx / 2) + 1
        var tmp = document.createElement("div")
        tmp.innerHTML = userRow(uText, uN)
        var uItem = tmp.firstElementChild
        content.appendChild(uItem)
        viewport.scrollTop = Math.max(0, uItem.offsetTop - peek)
        nextIdx += 2
        busy = true
        syncUI()
        var aId = "prev-a-" + uN
        tmp.innerHTML = assistantShell(aId)
        content.appendChild(tmp.firstElementChild)
        var sink = document.getElementById(aId + "-c")
        var parts = paras(aText)
        var pi = 0, ti = 0
        var full = parts[0] || ""
        sink.innerHTML = '<p style="white-space:pre-wrap"></p>'
        var pEl = sink.firstElementChild
        var timer = setInterval(function () {
          if (ti < full.length) { pEl.textContent = full.slice(0, ti + 3); ti += 3 }
          else {
            pi++
            if (pi < parts.length) {
              sink.insertAdjacentHTML("beforeend", '<p style="white-space:pre-wrap"></p>')
              pEl = sink.lastElementChild; full = parts[pi]; ti = 0
            } else { clearInterval(timer); busy = false; syncUI() }
          }
          update()
        }, 35)
      }
      form.addEventListener("submit", function (e) { e.preventDefault(); send() })
      resetBtn.addEventListener("click", function () {
        if (busy) return
        nextIdx = 2; peek = DEFAULT_PEEK
        peekPx.textContent = DEFAULT_PEEK + "px"
        renderInitial()
        viewport.scrollTop = 0
        syncUI(); update()
      })

      var sliderRoot = document.querySelector("[data-slot=slider]")
      RadixKernel.wireSlider({
        root: sliderRoot,
        track: sliderRoot.querySelector("[data-slot=slider-track]"),
        range: sliderRoot.querySelector("[data-slot=slider-range]"),
        defaultValue: [DEFAULT_PEEK],
        min: 64, max: 128, step: 1,
        onValueChange: function (v) { peek = v[0] || DEFAULT_PEEK; peekPx.textContent = peek + "px" },
      })

      requestAnimationFrame(function () { viewport.scrollTop = 0; update() })
      syncUI()
    })()
```
:::

::::


### Following the Live Edge

When the reader is at the live edge, either because they stayed there or
returned there, `autoScroll` keeps streamed replies in view as they grow.
Scrolling away from the live edge releases the view, whether by wheel, touch,
keyboard scroll keys, or dragging the scrollbar. An explicit message jump
releases it too. New chunks can then arrive without moving the reader.

`autoScroll` composes with turn anchoring. When a new turn anchors near the
top, the view stays put while the reply streams into the room below it. Once
the reply fills the viewport, the reader is back at the live edge and
follow-output takes over from the anchor.

```tsx
<MessageScrollerProvider autoScroll>
  <MessageScroller>{/* streamed turns */}</MessageScroller>
</MessageScrollerProvider>
```

::::demo message-scroller-streaming
<iframe class="demo" src="/demos/message-scroller-streaming.html" title="message-scroller-streaming" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-streaming.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-streaming.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Streaming Messages</div>
      <div data-slot="card-description">Auto-scroll follows the live edge of the conversation.</div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="outline"
          data-size="icon"
          id="t1-trigger"
          aria-label="Reset stream"
          data-state="closed"
          disabled
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
            <path d="M21 3v5h-5"></path>
          </svg>
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="empty" id="empty-state" class="h-full">
        <div data-slot="empty-header">
          <div data-slot="empty-icon">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <path d="M10.1 2.182a10 10 0 0 1 3.8 0"></path>
              <path d="M13.9 21.818a10 10 0 0 1-3.8 0"></path>
              <path d="M17.609 3.72a10 10 0 0 1 2.69 2.7"></path>
              <path d="M2.182 13.9a10 10 0 0 1 0-3.8"></path>
              <path d="M20.28 17.61a10 10 0 0 1-2.7 2.69"></path>
              <path d="M21.818 10.1a10 10 0 0 1 0 3.8"></path>
              <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"></path>
              <path d="m6.163 21.117-2.906.85a1 1 0 0 1-1.236-1.169l.965-2.98"></path>
            </svg>
          </div>
          <div data-slot="empty-title">Ready to Stream</div>
          <div data-slot="empty-description">Press send to stream a scripted launch summary.</div>
        </div>
      </div>
      <div
        data-slot="message-scroller"
        class="group/message-scroller"
        id="scroller"
        hidden
        data-autoscroll=""
      >
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div data-slot="card-footer" style="flex-direction: column; gap: calc(var(--spacing) * 2)">
      <form id="composer" style="width: 100%">
        <div
          data-slot="input-group"
          class="group/input-group"
          style="height: auto; flex-direction: column; align-items: stretch"
        >
          <div style="height: 3.5rem; width: 100%; padding: 0.625rem 0.75rem">
            <span
              id="next-msg"
              class="line-clamp-2"
              style="
                display: -webkit-box;
                -webkit-line-clamp: 2;
                -webkit-box-orient: vertical;
                overflow: hidden;
              "
            ></span>
          </div>
          <div
            data-slot="input-group-addon"
            role="group"
            data-align="block-end"
            style="padding-top: 0.25rem; justify-content: flex-start; width: 100%"
          >
            <button
              data-slot="button"
              type="button"
              data-variant="outline"
              data-size="icon-sm"
              id="d1-trigger"
              aria-label="Add files"
              aria-haspopup="menu"
              aria-expanded="false"
              data-state="closed"
              data-radixuigo-menu-trigger="d1"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="M5 12h14"></path>
                <path d="M12 5v14"></path>
              </svg>
            </button>
            <button
              data-slot="button"
              type="submit"
              data-variant="default"
              data-size="icon-sm"
              id="send-btn"
              style="margin-inline-start: auto"
              disabled
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="m5 12 7-7 7 7"></path>
                <path d="M12 19V5"></path>
              </svg>
              <span class="sr-only">Send</span>
            </button>
          </div>
        </div>
      </form>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Streaming is simulated. autoScroll is enabled.
  </p>
</div>
<template id="t1-portal">
  <div data-slot="tooltip-content" id="tt1" role="tooltip" data-state="closed">
    Reset
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
      <path d="M12 5v14M5 12h14"></path>
    </svg>
  </div>
</template>
<template id="d1-tpl">
  <div
    data-slot="dropdown-menu-content"
    id="d1"
    role="menu"
    tabindex="-1"
    data-state="closed"
    dir="ltr"
    aria-orientation="vertical"
    data-orientation="vertical"
    aria-labelledby="d1-trigger"
    style="width: 11rem"
  >
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <path
          d="m16 6-8.414 8.586a2 2 0 0 0 2.829 2.829l8.414-8.586a4 4 0 1 0-5.657-5.657l-8.379 8.551a6 6 0 1 0 8.485 8.485l8.379-8.551"
        ></path></svg
      >Add Photos &amp; Files
    </div>
    <div data-slot="dropdown-menu-separator" role="separator" aria-orientation="horizontal"></div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <rect width="18" height="18" x="3" y="3" rx="2" ry="2"></rect>
        <circle cx="9" cy="9" r="2"></circle>
        <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path></svg
      >Create Image
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <path
          d="m10.065 12.493-6.18 1.318a.934.934 0 0 1-1.108-.702l-.537-2.15a1.07 1.07 0 0 1 .691-1.265l13.504-4.44"
        ></path>
        <path d="m13.56 11.747 4.332-.924"></path>
        <path d="m16 21-3.105-6.21"></path>
        <path
          d="M16.485 5.94a2 2 0 0 1 1.455-2.425l1.09-.272a1 1 0 0 1 1.212.727l1.515 6.06a1 1 0 0 1-.727 1.213l-1.09.272a2 2 0 0 1-2.425-1.455z"
        ></path>
        <path d="m6.158 8.633 1.114 4.456"></path>
        <path d="m8 21 3.105-6.21"></path></svg
      >Deep Research
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="12" cy="12" r="10"></circle>
        <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"></path>
        <path d="M2 12h20"></path></svg
      >Web Search
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
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
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()

(function () {
      var script = [
        { u: "I'm building a chat for our app and the scroll behavior is driving me nuts. Every time the AI streams a reply, the whole thread jumps around.", a: "That's the classic streaming scroll problem. Wrap your message list in MessageScroller and turn on autoScroll — the viewport pins to the bottom as tokens arrive, so users always see the latest text land in place.\n\nThe important part: it only auto-scrolls while the reader is already at the bottom. The moment they scroll up to read something earlier, auto-scroll backs off and their position is preserved. You get smooth streaming without fighting the user's intent." },
        { u: "Okay, but when someone sends a new message the view still feels jarring — like the whole conversation reloads from the top.", a: "MessageScrollerItem fixes that with turn anchoring. Set scrollAnchor on the turn that should settle near the top instead of blindly snapping to the document bottom.\n\nIt also leaves a small peek of the previous exchange visible above the anchor, so context isn't lost. The reply starts in view without that disorienting jump you get from a plain overflow container." },
        { u: "And if they've scrolled up to re-read an older answer? I don't want to yank them back down.", a: "You won't. Auto-scroll only runs when the viewport is already pinned to the bottom, so scrolling up is a deliberate opt-out — their place in the thread stays put even as new tokens keep arriving below.\n\nWhen there is content they haven't seen yet, MessageScrollerButton appears at the bottom of the viewport. One tap jumps them back to the newest message and re-engages auto-scroll. Same pattern as Slack or iMessage: quiet when you're caught up, helpful when you're not." },
        { u: "Last one — does this work with assistive tech?", a: "MessageScrollerContent sets role=\"log\" and aria-relevant=\"additions\" by default, so screen readers announce new messages as they stream in.\n\nThe scroll button is a real button with an sr-only label, and it's removed from the tab order when you're already at the bottom — no ghost focus stops." },
      ]
      var sent = 0, busy = false
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var sendBtn = document.getElementById("send-btn")
      var resetBtn = document.getElementById("t1-trigger")
      var emptyState = document.getElementById("empty-state")
      var scroller = document.getElementById("scroller")
      var nextMsg = document.getElementById("next-msg")
      var form = document.getElementById("composer")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function userRow(text) {
        var id = "strm-u-" + sent
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '" data-scroll-anchor="">' +
          '<div data-slot="message" class="group/message" data-align="end"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="muted" data-align="end"><div data-slot="bubble-content">' + text + "</div></div>" +
          "</div></div></div>"
      }
      function assistantShell(id) {
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '">' +
          '<div data-slot="message" class="group/message"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="ghost"><div data-slot="bubble-content" id="' + id + '-c"></div></div>' +
          "</div></div></div>"
      }
      function atBottom() { return viewport.scrollTop + viewport.clientHeight >= viewport.scrollHeight - 4 }
      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
        if (end) scroller.setAttribute("data-autoscrolling", ""); else scroller.removeAttribute("data-autoscrolling")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () {
        viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" })
      })

      function syncUI() {
        var next = sent < script.length
        sendBtn.disabled = !next || busy
        if (sendBtn.disabled) sendBtn.setAttribute("data-disabled", "true"); else sendBtn.removeAttribute("data-disabled")
        resetBtn.disabled = sent === 0 || busy
        if (resetBtn.disabled) resetBtn.setAttribute("data-disabled", "true"); else resetBtn.removeAttribute("data-disabled")
        content.setAttribute("aria-busy", String(busy))
        nextMsg.setAttribute("data-status", busy ? "busy" : "ready")
        nextMsg.style.opacity = busy ? "0.6" : "1"
        if (next) nextMsg.textContent = script[sent].u
        else nextMsg.innerHTML = '<span style="color:var(--muted-foreground)">No messages queued. Reset the stream.</span>'
      }

      function send() {
        if (sent >= script.length || busy) return
        var pair = script[sent]
        emptyState.hidden = true
        scroller.hidden = false
        var tmp = document.createElement("div")
        tmp.innerHTML = userRow(pair.u)
        content.appendChild(tmp.firstElementChild)
        viewport.scrollTop = viewport.scrollHeight
        sent++
        busy = true
        syncUI()
        var aId = "strm-a-" + sent
        tmp.innerHTML = assistantShell(aId)
        content.appendChild(tmp.firstElementChild)
        var sink = document.getElementById(aId + "-c")
        var parts = paras(pair.a)
        var pi = 0, ti = 0
        var full = parts[0] || ""
        sink.innerHTML = '<p style="white-space:pre-wrap"></p>'
        var pEl = sink.firstElementChild
        var timer = setInterval(function () {
          if (ti < full.length) { pEl.textContent = full.slice(0, ti + 3); ti += 3 }
          else {
            pi++
            if (pi < parts.length) {
              sink.insertAdjacentHTML("beforeend", '<p style="white-space:pre-wrap"></p>')
              pEl = sink.lastElementChild; full = parts[pi]; ti = 0
            } else { clearInterval(timer); busy = false; syncUI() }
          }
          if (atBottom()) viewport.scrollTop = viewport.scrollHeight
          update()
        }, 20)
      }
      form.addEventListener("submit", function (e) { e.preventDefault(); send() })
      resetBtn.addEventListener("click", function () {
        if (busy) return
        sent = 0; content.innerHTML = ""
        scroller.hidden = true; emptyState.hidden = false
        syncUI(); update()
      })
      syncUI(); update()
    })()
```
:::

::::


Calling `scrollToEnd`, or pressing `MessageScrollerButton`, re-engages
follow-output when `autoScroll` is enabled, so a reader who scrolled away can
return to the live edge and keep following. The root and viewport expose
`data-autoscrolling` while that programmatic scroll to the latest message runs,
so you can conditionally apply styles during the transition.

### Opening Saved Threads

It can seem reasonable to reopen a saved thread at the absolute end of the
transcript, but that often drops the reader into the conversation without enough
context. A better default is `"last-anchor"`: show the last meaningful turn,
like the user's latest message, with the reply below it.

That gives the reader an immediate place in the thread. They can see what they
asked, where the answer starts, and continue from there without reconstructing
the conversation from the bottom edge.

```tsx
<MessageScrollerProvider defaultScrollPosition="last-anchor">
  <MessageScroller>{/* transcript */}</MessageScroller>
</MessageScrollerProvider>
```

::::demo message-scroller-opening-position
<iframe class="demo" src="/demos/message-scroller-opening-position.html" title="message-scroller-opening-position" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-opening-position.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-opening-position.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Opening Position</div>
      <div data-slot="card-description">Choose where a saved transcript opens.</div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="message-scroller" class="group/message-scroller">
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div data-slot="card-footer" class="border-t" style="justify-content: center">
      <div
        data-slot="tabs"
        data-orientation="horizontal"
        dir="ltr"
        class="group/tabs"
        style="width: 100%"
      >
        <div
          data-slot="tabs-list"
          role="tablist"
          aria-orientation="horizontal"
          data-orientation="horizontal"
          data-variant="default"
          style="width: 100%"
        >
          <button
            data-slot="tabs-trigger"
            type="button"
            role="tab"
            data-pos="start"
            aria-selected="false"
            data-state="inactive"
          >
            start
          </button>
          <button
            data-slot="tabs-trigger"
            type="button"
            role="tab"
            data-pos="end"
            aria-selected="false"
            data-state="inactive"
          >
            end
          </button>
          <button
            data-slot="tabs-trigger"
            type="button"
            role="tab"
            data-pos="last-anchor"
            aria-selected="true"
            data-state="active"
          >
            last-anchor
          </button>
        </div>
      </div>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Toggle the defaultScrollPosition to see where the transcript starts when you open the thread.
  </p>
</div>
```

```js:line-numbers [behavior]
(function () {
      var msgs = [
        { id: "open-1", role: "user", text: "This is the first message the user sent in the conversation." },
        { id: "open-2", role: "assistant", text: "Workspace creation rose 8%, but first invite completion only rose 2%." },
        { id: "open-3", role: "user", text: "This is the last message the user sent in the conversation." },
        { id: "open-4", role: "assistant", text: "Start with the invite step. Teams are creating workspaces but waiting to add collaborators.\n\nRecommended follow-up:\n\n1. Compare invite drop-off by account size.\n2. Check whether users who skip invites still return within 24 hours.\n3. Review the empty-state copy on the first project screen.\n4. Segment activation by template, since template users may not need invites right away.\n\nIf that pattern holds, the next experiment should make collaboration useful earlier instead of prompting for invites harder." },
      ]
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var triggers = document.querySelectorAll("[data-slot=tabs-trigger]")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function rowHTML(m) {
        var isUser = m.role === "user"
        var body = '<div data-slot="bubble-content">' + (isUser ? m.text : paras(m.text).map(function (p) { return '<p style="white-space:pre-wrap">' + p + "</p>" }).join("")) + "</div>"
        return '<div data-slot="message-scroller-item" data-message-id="' + m.id + '"' + (isUser ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isUser ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + (isUser ? "muted" : "ghost") + '"' + (isUser ? ' data-align="end"' : "") + ">" + body + "</div>" +
          "</div></div></div>"
      }
      content.innerHTML = msgs.map(rowHTML).join("")

      function applyPosition(pos) {
        if (pos === "start") { viewport.scrollTop = 0 }
        else if (pos === "end") { viewport.scrollTop = viewport.scrollHeight }
        else {
          var t = content.querySelector('[data-message-id="open-3"]')
          viewport.scrollTop = t ? Math.max(0, t.offsetTop - 64) : 0
        }
        update()
      }
      function activate(value) {
        triggers.forEach(function (t) {
          var on = t.getAttribute("data-pos") === value
          t.setAttribute("aria-selected", String(on))
          t.setAttribute("data-state", on ? "active" : "inactive")
        })
        applyPosition(value)
      }
      triggers.forEach(function (t) {
        t.addEventListener("click", function () { activate(t.getAttribute("data-pos")) })
      })
      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      requestAnimationFrame(function () { applyPosition("last-anchor") })
    })()
```
:::

::::


`"last-anchor"` is keyed on `scrollAnchor`, not message role. If no anchor
exists, or the last anchored turn already fits in the viewport, it falls back to
`"end"`.

Use `"start"` when you want to resume at the beginning of a conversation, or
`"end"` when the absolute latest message is the right place to land.

### Loading Earlier Messages

Loading earlier messages should not move the conversation the reader is already
looking at. When older rows are prepended above the current transcript,
`MessageScrollerViewport` preserves the visible row so the reader stays in the
same place while history loads above them.

This is enabled by default through `preserveScrollOnPrepend`.

::::demo message-scroller-load-history
<iframe class="demo" src="/demos/message-scroller-load-history.html" title="message-scroller-load-history" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-load-history.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-load-history.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Load History</div>
      <div data-slot="card-description">Prepended messages keep your place.</div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="outline"
          data-size="icon"
          id="t1-trigger"
          aria-label="Reset loaded messages"
          data-state="closed"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
            <path d="M21 3v5h-5"></path>
          </svg>
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="message-scroller" class="group/message-scroller">
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div
      data-slot="card-footer"
      class="border-t"
      style="flex-direction: column; align-items: center; gap: calc(var(--spacing) * 2)"
    >
      <button
        data-slot="button"
        type="button"
        data-variant="secondary"
        id="load-btn"
        style="width: 100%"
      >
        Load History
      </button>
      <p class="text-xs" style="color: var(--muted-foreground)">
        Restore earlier messages while keeping your place.
      </p>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Click Load History to load the entire conversation.
  </p>
</div>
<template id="t1-portal">
  <div data-slot="tooltip-content" id="d1" role="tooltip" data-state="closed">
    Reset
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
      <path d="M12 5v14M5 12h14"></path>
    </svg>
  </div>
</template>
<div id="toast" class="toast" hidden></div>
<style>
  .toast {
    position: fixed;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 12px 16px;
    border-radius: 8px;
    background: var(--popover);
    color: var(--popover-foreground);
    border: 1px solid var(--border);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.18);
    font-size: 0.875rem;
  }
  .toast strong {
    font-weight: 600;
  }
  .toast span {
    color: var(--muted-foreground);
  }
</style>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()

(function () {
      var history = [
        { id: "load-1", role: "user", text: "Can you summarize the incident channel?" },
        { id: "load-2", role: "assistant", text: "The first alert was a delayed export job. It started backing up around 09:42 UTC and triggered the warning once the retry queue crossed the threshold.\n\nNo customer-facing checkout paths were affected, but exports for larger workspaces were running about 12 minutes behind." },
        { id: "load-3", role: "user", text: "Was checkout affected?" },
        { id: "load-4", role: "assistant", text: "No checkout errors were reported. Payment authorization, order creation, and confirmation emails stayed inside their normal latency bands.\n\nThe only elevated metric was export queue depth, which maps to analytics downloads instead of checkout." },
        { id: "load-5", role: "user", text: "What changed in the last deploy?" },
        { id: "load-6", role: "assistant", text: "Only the export queue worker changed. The deploy moved large CSV jobs onto the shared retry policy, which made each failed attempt hold a worker slot longer than before.\n\nThe app deploy did not include checkout, pricing, or billing API changes." },
        { id: "load-7", role: "user", text: "Do we need to roll back?" },
        { id: "load-8", role: "assistant", text: "Not yet. Queue depth is recovering after we reduced retry concurrency, and the oldest pending job is now under five minutes old.\n\nKeep rollback ready if the queue starts climbing again, but the current trend points toward recovery." },
        { id: "load-9", role: "user", text: "Keep watching for customer-visible issues." },
        { id: "load-10", role: "assistant", text: "I will watch the queue and support tags for another 15 minutes. I am tracking export failures, delayed download requests, and any support thread that mentions missing reports.\n\nIf those stay quiet through the next batch window, we can close this as an internal degradation." },
      ]
      var INITIAL = 5
      var visibleCount = INITIAL
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var loadBtn = document.getElementById("load-btn")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function rowHTML(m) {
        var isUser = m.role === "user"
        var body = '<div data-slot="bubble-content">' + (isUser ? m.text : paras(m.text).map(function (p) { return '<p style="white-space:pre-wrap">' + p + "</p>" }).join("")) + "</div>"
        return '<div data-slot="message-scroller-item" data-message-id="' + m.id + '"' + (isUser ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isUser ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + (isUser ? "muted" : "ghost") + '"' + (isUser ? ' data-align="end"' : "") + ">" + body + "</div>" +
          "</div></div></div>"
      }
      function markerHTML() {
        return '<div data-slot="message-scroller-item">' +
          '<div data-slot="marker" class="group/marker" data-variant="separator"><span data-slot="marker-content">End of Conversation</span></div>' +
          "</div>"
      }
      function render() {
        var visible = history.slice(-visibleCount)
        content.innerHTML = visible.map(rowHTML).join("") + markerHTML()
      }
      render()

      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      var toastEl = document.getElementById("toast")
      var toastTimer = null
      function toast(title, desc) {
        toastEl.innerHTML = "<strong>" + title + "</strong><span>" + desc + "</span>"
        toastEl.hidden = false
        clearTimeout(toastTimer)
        toastTimer = setTimeout(function () { toastEl.hidden = true }, 2600)
      }

      loadBtn.addEventListener("click", function () {
        if (visibleCount >= history.length) return
        var firstItem = content.querySelector("[data-slot=message-scroller-item][data-message-id]")
        var anchorId = firstItem ? firstItem.getAttribute("data-message-id") : null
        var oldOffset = firstItem ? firstItem.offsetTop : 0
        var oldScroll = viewport.scrollTop
        visibleCount = history.length
        render()
        var ref = anchorId ? content.querySelector('[data-message-id="' + anchorId + '"]') : null
        if (ref) viewport.scrollTop = ref.offsetTop - (oldOffset - oldScroll)
        loadBtn.textContent = "History Loaded"
        loadBtn.setAttribute("data-disabled", "true")
        loadBtn.disabled = true
        toast("History loaded", "Scroll up to see earlier messages.")
        update()
      })
      loadBtn.disabled = false

      var resetBtn = document.getElementById("t1-trigger")
      resetBtn.addEventListener("click", function () {
        if (visibleCount === INITIAL) return
        visibleCount = INITIAL
        render()
        loadBtn.textContent = "Load History"
        loadBtn.removeAttribute("data-disabled")
        loadBtn.disabled = false
        viewport.scrollTop = viewport.scrollHeight
        update()
      })

      requestAnimationFrame(function () { viewport.scrollTop = viewport.scrollHeight; update() })
    })()
```
:::

::::


Use stable `messageId` values for message rows. That gives the scroller a
specific row to preserve instead of guessing from whichever pixel happens to sit
at the viewport edge.

### Animating New Messages

`MessageScrollerItem` can be animated directly. Create a motion version of the
item, keep `messageId` and `scrollAnchor` on it, and use transform and opacity
for the entrance.

A common chat pattern is to animate the user's message when it is sent, then let
the assistant reply stream into a regular row below it. Start the user row below
its final position so it feels like it rises from the live edge of the viewport.

```tsx
const MotionMessageScrollerItem = motion.create(MessageScrollerItem)
```

::::demo message-scroller-animation
<iframe class="demo" src="/demos/message-scroller-animation.html" title="message-scroller-animation" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-animation.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-animation.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b">
      <div data-slot="card-title">Animation</div>
      <div data-slot="card-description">
        Choose how user messages are animated when they are added to the conversation.
      </div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="outline"
          data-size="icon"
          id="reset-btn"
          aria-label="Reset animated messages"
          disabled
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
            <path d="M21 3v5h-5"></path>
          </svg>
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="empty" id="empty-state" class="h-full">
        <div data-slot="empty-header">
          <div data-slot="empty-icon">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <path d="M10.1 2.182a10 10 0 0 1 3.8 0"></path>
              <path d="M13.9 21.818a10 10 0 0 1-3.8 0"></path>
              <path d="M17.609 3.72a10 10 0 0 1 2.69 2.7"></path>
              <path d="M2.182 13.9a10 10 0 0 1 0-3.8"></path>
              <path d="M20.28 17.61a10 10 0 0 1-2.7 2.69"></path>
              <path d="M21.818 10.1a10 10 0 0 1 0 3.8"></path>
              <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"></path>
              <path d="m6.163 21.117-2.906.85a1 1 0 0 1-1.236-1.169l.965-2.98"></path>
            </svg>
          </div>
          <div data-slot="empty-title">No Messages Yet</div>
          <div data-slot="empty-description">Click the button below to send the first message.</div>
        </div>
      </div>
      <div data-slot="message-scroller" class="group/message-scroller" id="scroller" hidden>
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div data-slot="card-footer">
      <button
        data-slot="select-trigger"
        id="s1-trigger"
        type="button"
        role="combobox"
        aria-expanded="false"
        data-state="closed"
        data-size="default"
        dir="ltr"
        aria-autocomplete="none"
        aria-label="Animation preset"
      >
        <span data-slot="select-value" style="pointer-events: none">Fade</span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="m6 9 6 6 6-6"></path>
        </svg>
      </button>
      <button
        data-slot="button"
        type="button"
        data-size="icon"
        id="send-btn"
        style="margin-inline-start: auto"
        disabled
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="m5 12 7-7 7 7"></path>
          <path d="M12 19V5"></path>
        </svg>
        <span class="sr-only">Send Message</span>
      </button>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Select an animation then click send to see it in action.
  </p>
</div>
<template id="s1-tpl">
  <div
    data-slot="select-content"
    id="d1"
    role="listbox"
    tabindex="-1"
    data-state="closed"
    dir="ltr"
  >
    <div data-slot="select-scroll-up-button">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="m18 15-6-6-6 6"></path>
      </svg>
    </div>
    <div>
      <div
        data-slot="select-item"
        role="option"
        aria-selected="true"
        tabindex="-1"
        data-state="checked"
        data-highlighted=""
        data-value="fade"
        aria-labelledby="d1-a"
      >
        <span data-slot="select-item-indicator"
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M20 6 9 17l-5-5"></path></svg
        ></span>
        <span id="d1-a">Fade</span>
      </div>
      <div
        data-slot="select-item"
        role="option"
        aria-selected="false"
        tabindex="-1"
        data-state="unchecked"
        data-value="pop"
        aria-labelledby="d1-b"
      >
        <span data-slot="select-item-indicator"></span>
        <span id="d1-b">Pop</span>
      </div>
      <div
        data-slot="select-item"
        role="option"
        aria-selected="false"
        tabindex="-1"
        data-state="unchecked"
        data-value="tilt"
        aria-labelledby="d1-c"
      >
        <span data-slot="select-item-indicator"></span>
        <span id="d1-c">Tilt</span>
      </div>
    </div>
    <div data-slot="select-scroll-down-button">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="m6 9 6 6 6-6"></path>
      </svg>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()

(function () {
      var script = [
        { u: "Can user messages pop in like iMessage without breaking anchoring?", a: "Yes. Animate the user row with transform and opacity, and let the assistant response stream normally below it.\n\nThat keeps the row measurement predictable while still giving the newly sent bubble a more tactile entrance." },
        { u: "What makes the animation feel more like iMessage?", a: "Use a quick spring from the trailing edge: a little scale, a small upward move, and no layout animation.\n\nThe bubble feels tactile, but the measured row stays predictable, so anchoring and auto-scroll do not have to fight a changing layout." },
        { u: "Can I switch between presets while testing the same thread?", a: "Yes. Keep the conversation in place while you change the preset, then send the next message to compare the new entrance against the same context.\n\nThat makes it easier to judge the difference between a subtle fade, a snappy pop, and a more dramatic 3D tilt without rebuilding the scenario each time." },
      ]
      var PRESETS = {
        fade: { from: "opacity:0;transform:translateY(6px)", to: "opacity:1;transform:none" },
        pop: { from: "opacity:0;transform:scale(.92)", to: "opacity:1;transform:none" },
        tilt: { from: "opacity:0;transform:translateY(8px) rotate(-3deg)", to: "opacity:1;transform:none" },
      }
      var presetId = "fade"
      var sent = 0
      var busy = false
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var sendBtn = document.getElementById("send-btn")
      var resetBtn = document.getElementById("reset-btn")
      var emptyState = document.getElementById("empty-state")
      var scroller = document.getElementById("scroller")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function userRow(text, animate) {
        var id = "anim-u-" + sent
        var style = animate ? PRESETS[presetId].from : ""
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '" data-scroll-anchor=""' + (style ? ' style="' + style + '"' : "") + ">" +
          '<div data-slot="message" class="group/message" data-align="end"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="muted" data-align="end"><div data-slot="bubble-content">' + text + "</div></div>" +
          "</div></div></div>"
      }
      function assistantRowHTML(id) {
        return '<div data-slot="message-scroller-item" data-message-id="' + id + '">' +
          '<div data-slot="message" class="group/message"><div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="ghost"><div data-slot="bubble-content" id="' + id + '-content"></div></div>' +
          "</div></div></div>"
      }
      function atBottom() { return viewport.scrollTop + viewport.clientHeight >= viewport.scrollHeight - 4 }
      function follow() { if (atBottom() || busy) viewport.scrollTop = viewport.scrollHeight }
      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      function syncUI() {
        var next = sent < script.length
        sendBtn.disabled = !next || busy
        if (sendBtn.disabled) sendBtn.setAttribute("data-disabled", "true"); else sendBtn.removeAttribute("data-disabled")
        resetBtn.disabled = sent === 0 || busy
        if (resetBtn.disabled) resetBtn.setAttribute("data-disabled", "true"); else resetBtn.removeAttribute("data-disabled")
        content.setAttribute("aria-busy", String(busy))
      }

      function animateIn(item) {
        var p = PRESETS[presetId]
        item.style.transition = "opacity 240ms cubic-bezier(.23,1,.32,1), transform 240ms cubic-bezier(.23,1,.32,1)"
        requestAnimationFrame(function () { item.setAttribute("style", p.to + ";transition:opacity 240ms cubic-bezier(.23,1,.32,1), transform 240ms cubic-bezier(.23,1,.32,1)") })
      }

      function send() {
        if (sent >= script.length || busy) return
        var pair = script[sent]
        emptyState.hidden = true
        scroller.hidden = false
        // user message
        var tmp = document.createElement("div")
        tmp.innerHTML = userRow(pair.u, true)
        var uItem = tmp.firstElementChild
        content.appendChild(uItem)
        viewport.scrollTop = viewport.scrollHeight
        animateIn(uItem)
        sent++
        // assistant streaming
        busy = true
        syncUI()
        var aId = "anim-a-" + sent
        tmp.innerHTML = assistantRowHTML(aId)
        var aItem = tmp.firstElementChild
        content.appendChild(aItem)
        var sink = document.getElementById(aId + "-content")
        var parts = paras(pair.a)
        var pi = 0, ti = 0
        var fullPara = parts[0] || ""
        sink.innerHTML = '<p style="white-space:pre-wrap"></p>'
        var pEl = sink.firstElementChild
        var timer = setInterval(function () {
          if (ti < fullPara.length) {
            pEl.textContent = fullPara.slice(0, ti + 2)
            ti += 2
          } else {
            pi++
            if (pi < parts.length) {
              sink.insertAdjacentHTML("beforeend", '<p style="white-space:pre-wrap"></p>')
              pEl = sink.lastElementChild
              fullPara = parts[pi]; ti = 0
            } else {
              clearInterval(timer)
              busy = false
              syncUI()
            }
          }
          follow()
          update()
        }, 15)
      }
      sendBtn.addEventListener("click", send)
      resetBtn.addEventListener("click", function () {
        if (busy) return
        sent = 0
        content.innerHTML = ""
        scroller.hidden = true
        emptyState.hidden = false
        syncUI()
        update()
      })

      // react to select value changes (select-glue updates valueNode text)
      var valueNode = document.querySelector("[data-slot=select-value]")
      new MutationObserver(function () {
        var t = (valueNode.textContent || "").trim().toLowerCase()
        if (PRESETS[t]) presetId = t
      }).observe(valueNode, { childList: true, characterData: true, subtree: true })

      syncUI()
      update()
    })()
```
:::

::::


Avoid animating height, margin, or padding for row entrances; those changes can
fight the scroller's positioning work. If the reader prefers reduced motion,
skip the entrance animation and keep the scroll behavior the same.

### Jumping to Messages

Search results, permalinks, outline items, and toolbar buttons often need to
drive the transcript from outside the message list. Use `useMessageScroller` for
those controls. Because the hooks read from `MessageScrollerProvider`, they work
in any component inside the provider, including controls rendered outside the
`MessageScroller` frame.


```tsx
const { scrollToMessage, scrollToEnd, scrollToStart } = useMessageScroller()
```

::::demo message-scroller-commands
<iframe class="demo" src="/demos/message-scroller-commands.html" title="message-scroller-commands" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-commands.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-commands.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Commands</div>
      <div data-slot="card-description">Drive the transcript from outside.</div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          type="button"
          data-variant="secondary"
          id="d1-trigger"
          aria-haspopup="menu"
          aria-expanded="false"
          data-state="closed"
          data-radixuigo-menu-trigger="d1"
        >
          Jump to...
        </button>
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="message-scroller" class="group/message-scroller">
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            id="ms-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Use the controls to jump to any message in the conversation.
  </p>
</div>
<template id="d1-tpl">
  <div
    data-slot="dropdown-menu-content"
    id="d1"
    role="menu"
    tabindex="-1"
    data-state="closed"
    dir="ltr"
    aria-orientation="vertical"
    data-orientation="vertical"
    aria-labelledby="d1-trigger"
    style="width: calc(var(--spacing) * 40)"
  >
    <div data-slot="dropdown-menu-label">Conversations</div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
      data-jump="command-activation"
    >
      <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0"
        >We're seeing activation dip after workspace creation...</span
      >
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
      data-jump="command-compare"
    >
      <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0"
        >What should I compare before we change the onboarding...</span
      >
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
      data-jump="command-experiment"
    >
      <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0"
        >Can you turn that into an experiment?</span
      >
    </div>
    <div
      data-slot="dropdown-menu-item"
      role="menuitem"
      tabindex="-1"
      data-variant="default"
      data-orientation="vertical"
      data-jump="command-risk"
    >
      <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0"
        >What's the risk if we delay the invite prompt?</span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
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
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()

(function () {
      var msgs = [
        { id: "command-activation", role: "user", text: "We're seeing activation dip after workspace creation. Can you help me find the likely step?" },
        { id: "command-activation-a", role: "assistant", text: "The sharpest drop is between creating the workspace and inviting the first teammate.\n\nWorkspace creation is still healthy, but the invite step is where users pause. That suggests the product is asking for collaboration before the user has enough confidence in the workspace." },
        { id: "command-compare", role: "user", text: "What should I compare before we change the onboarding flow?" },
        { id: "command-compare-a", role: "assistant", text: "Compare three cohorts:\n\n1. Users who choose a template before inviting teammates.\n2. Users who start from a blank workspace.\n3. Users who skip invites and return within 24 hours.\n\nIf template users invite faster, the fix is probably better first-run guidance rather than a louder invite prompt." },
        { id: "command-experiment", role: "user", text: "Can you turn that into an experiment?" },
        { id: "command-experiment-a", role: "assistant", text: "Yes. Create a variant that shows a short checklist after workspace creation:\n\n- Pick a template.\n- Add one project detail.\n- Invite a teammate when the workspace has context.\n\nMeasure first invite completion, 24-hour return rate, and whether teams create a second project." },
        { id: "command-risk", role: "user", text: "What's the risk if we delay the invite prompt?" },
        { id: "command-risk-a", role: "assistant", text: "The main risk is reducing team creation for accounts that already know who they want to invite.\n\nTo protect that path, keep the invite action visible in the header and only change the primary empty-state guidance. That gives confident teams a direct route without forcing uncertain users through the invite step too early." },
      ]
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function rowHTML(m) {
        var isUser = m.role === "user"
        var body = '<div data-slot="bubble-content">' + (isUser ? m.text : paras(m.text).map(function (p) { return '<p style="white-space:pre-wrap">' + p + "</p>" }).join("")) + "</div>"
        return '<div data-slot="message-scroller-item" data-message-id="' + m.id + '"' + (isUser ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isUser ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + (isUser ? "muted" : "ghost") + '"' + (isUser ? ' data-align="end"' : "") + ">" + body + "</div>" +
          "</div></div></div>"
      }
      content.innerHTML = msgs.map(rowHTML).join("")

      function update() {
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      // jump menu items are cloned into live DOM by menu-glue — delegate clicks
      document.addEventListener("click", function (e) {
        var item = e.target.closest('[data-slot="dropdown-menu-item"][data-jump]')
        if (!item) return
        var id = item.getAttribute("data-jump")
        var target = content.querySelector('[data-message-id="' + id + '"]')
        if (target) target.scrollIntoView({ behavior: "smooth", block: "start" })
      })

      // open at the end (last anchor)
      requestAnimationFrame(function () {
        var lastUser = content.querySelector('[data-message-id="command-risk"]')
        if (lastUser) viewport.scrollTop = lastUser.offsetTop - 12
        update()
      })
    })()
```
:::

::::


`scrollToMessage` targets the `messageId` on `MessageScrollerItem`, so rows that
need to be addressable should have stable ids. `scrollToMessage` returns `false`
when the target is not mounted and cannot be queued.

`scrollToMessage` can queue a target before items exist, which covers
client-resolved permalinks while the transcript mounts. After rows have mounted,
a missing id returns `false` instead of starting a guessed retry loop. A `true`
result means the scroll ran or was queued, not that the row is already in view.

### Tracking the Reader's Position

Use `useMessageScrollerVisibility` to track the reader's position in the
conversation. A common example is a table-of-contents or a jump menu that
highlights the current anchored turn.


```tsx
const { currentAnchorId, visibleMessageIds } = useMessageScrollerVisibility()
```

::::demo message-scroller-visibility
<iframe class="demo" src="/demos/message-scroller-visibility.html" title="message-scroller-visibility" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-visibility.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-visibility.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div style="position: relative">
    <div data-slot="card" class="w-full" style="height: 35rem; gap: 0">
      <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
        <div data-slot="card-title">Transcript Outline</div>
        <div data-slot="card-description">Track the current anchored turn.</div>
      </div>
      <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
        <div data-slot="message-scroller" class="group/message-scroller">
          <div
            data-slot="message-scroller-viewport"
            id="ms-viewport"
            role="region"
            aria-label="Messages"
            tabindex="0"
          >
            <div
              data-slot="message-scroller-content"
              id="ms-content"
              role="log"
              aria-relevant="additions"
              style="padding: var(--card-spacing)"
            ></div>
          </div>
          <button
            data-slot="message-scroller-button"
            id="ms-button"
            type="button"
            data-active="false"
            aria-label="Scroll to latest"
            style="bottom: calc(var(--spacing) * 2)"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <path d="m6 9 6 6 6-6"></path>
            </svg>
            <span class="sr-only">Scroll to latest</span>
          </button>
        </div>
      </div>
    </div>
    <div style="position: absolute; top: 50%; right: -3rem; transform: translateY(-50%)">
      <button
        id="outline-trigger"
        type="button"
        aria-label="Open transcript outline"
        class="outline-btn"
      ></button>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Open the outline to jump between anchored turns as you read.
  </p>
</div>
<template id="outline-portal">
  <div
    data-slot="hover-card-content"
    data-state="closed"
    data-side="left"
    class="outline-card"
  ></div>
</template>
<style>
  .outline-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    width: 36px;
    height: 36px;
    border: none;
    border-radius: 8px;
    background: transparent;
    cursor: pointer;
    outline: none;
  }
  .outline-btn span {
    display: block;
    height: 2px;
    width: 16px;
    border-radius: 9999px;
    background: color-mix(in oklab, var(--muted-foreground) 40%, transparent);
  }
  .outline-btn span[data-current="true"] {
    background: var(--foreground);
  }
  .outline-card {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 16rem;
    padding: 4px;
    border-radius: 1rem;
  }
  .outline-card button {
    display: flex;
    min-height: 1.75rem;
    align-items: center;
    border: none;
    border-radius: 0.75rem;
    padding: 6px 8px;
    text-align: left;
    font-size: 0.875rem;
    background: transparent;
    color: var(--foreground);
    cursor: pointer;
    outline: none;
  }
  .outline-card button:hover {
    background: var(--accent);
    color: var(--accent-foreground);
  }
  .outline-card button[aria-current] {
    background: var(--accent);
    color: var(--accent-foreground);
  }
</style>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

(function () {
      var msgs = [
        { id: "vis-brief", role: "user", text: "Review the incident handoff and tell me what to read first." },
        { id: "vis-brief-a", role: "assistant", text: "Start with the summary and the impact section. The regression affected the upload queue, but the recovery path completed for every queued job." },
        { id: "vis-impact", role: "user", text: "What was the customer impact?" },
        { id: "vis-impact-a", role: "assistant", text: "Impact was limited to delayed processing.\n\nNo records were dropped, and the reconciliation worker confirmed each retry batch. Support saw confusion from two customers, but there were no checkout or billing errors." },
        { id: "vis-actions", role: "user", text: "What actions are open?" },
        { id: "vis-actions-a", role: "assistant", text: "Keep the retry window enabled until the next deploy, then add a queue-depth alert as the long-term fix.\n\nThe alert should fire on sustained queue growth, not a single short spike." },
        { id: "vis-checklist", role: "user", text: "Give me the follow-up checklist." },
        { id: "vis-checklist-a", role: "assistant", text: "After that, compare the queue recovery graph with the deploy timeline so the handoff shows exactly when processing returned to baseline. That makes it easier for support and engineering to answer the same customer questions without re-reading the whole incident thread.\n\nI would also add a short owner note beside each follow-up item. The checklist is small, but ownership keeps the retry-window decision, alert tuning, and support macro from drifting into separate follow-up conversations.\n\nKeep the retry window enabled until the next deploy, then add a queue-depth alert as the long-term fix.\n\nThe alert should fire on sustained queue growth, not a single short spike." },
      ]
      var userMsgs = msgs.filter(function (m) { return m.role === "user" })
      var viewport = document.getElementById("ms-viewport")
      var content = document.getElementById("ms-content")
      var button = document.getElementById("ms-button")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(t) { return t.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean) }
      function rowHTML(m) {
        var isUser = m.role === "user"
        var body = isUser
          ? '<div data-slot="bubble-content">' + m.text + "</div>"
          : '<div data-slot="bubble-content">' + paras(m.text).map(function (p) { return '<p style="white-space:pre-wrap">' + p + "</p>" }).join("") + "</div>"
        return '<div data-slot="message-scroller-item" data-message-id="' + m.id + '"' + (isUser ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isUser ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + (isUser ? "muted" : "ghost") + '"' + (isUser ? ' data-align="end"' : "") + ">" + body + "</div>" +
          "</div></div></div>"
      }
      content.innerHTML = msgs.map(rowHTML).join("")

      // outline trigger dots
      var trigger = document.getElementById("outline-trigger")
      trigger.innerHTML = userMsgs.map(function (m) {
        return '<span data-msg="' + m.id + '" data-current="false"></span>'
      }).join("")

      var currentAnchorId = userMsgs[0].id
      function trim(t) { return t.length > 42 ? t.slice(0, 39) + "..." : t }

      function update() {
        var items = content.querySelectorAll("[data-slot=message-scroller-item]")
        var anchor = userMsgs[0].id
        for (var i = 0; i < items.length; i++) {
          var it = items[i]
          if (it.offsetTop - viewport.scrollTop <= 12) {
            var mid = it.getAttribute("data-message-id")
            if (userMsgs.some(function (u) { return u.id === mid })) anchor = mid
          }
        }
        currentAnchorId = anchor
        trigger.querySelectorAll("span").forEach(function (s) {
          s.setAttribute("data-current", String(s.getAttribute("data-msg") === anchor))
        })
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        button.setAttribute("data-active", end ? "true" : "false")
      }
      viewport.addEventListener("scroll", update)

      // build hover-card content on open with live current-anchor
      var tpl = document.getElementById("outline-portal")
      function buildOutline() {
        var frag = tpl.content.cloneNode(true)
        var host = document.createElement("div"); host.appendChild(frag)
        var c = host.querySelector("[data-slot=hover-card-content]")
        c.innerHTML = userMsgs.map(function (m) {
          return '<button type="button" data-jump="' + m.id + '"' +
            (m.id === currentAnchorId ? ' aria-current="location"' : "") +
            '><span style="overflow:hidden;text-overflow:ellipsis;white-space:nowrap;min-width:0">' + trim(m.text) + "</span></button>"
        }).join("")
        c.setAttribute("data-state", "open")
        return c
      }
      var open = false
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 0, closeDelay: 0,
        popperOptions: { sideOffset: -28, side: "left" },
        buildContent: function () { return buildOutline() },
        onOpen: function () { open = true },
        onClosed: function () { open = false },
      })
      for (var ev in wired.handlers) trigger.addEventListener(ev.slice(2), wired.handlers[ev])
      document.addEventListener("keydown", function (e) { if (open && e.key === "Escape") wired.dismiss() })
      document.addEventListener("pointerdown", function (e) {
        if (!open) return
        var c = document.querySelector("[data-slot=hover-card-content]")
        if (c && !c.contains(e.target) && !trigger.contains(e.target)) wired.dismiss()
      })

      // jump on outline button click (delegation — content is cloned into live DOM)
      document.addEventListener("click", function (e) {
        var b = e.target.closest('[data-slot="hover-card-content"] button[data-jump]')
        if (!b) return
        var id = b.getAttribute("data-jump")
        var target = content.querySelector('[data-message-id="' + id + '"]')
        if (target) target.scrollIntoView({ behavior: "smooth", block: "start" })
        wired.dismiss()
      })
      button.addEventListener("click", function () { viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" }) })

      update()
    })()
```
:::

::::


`currentAnchorId` answers "where am I" by reporting the current anchored turn,
and it stays set after that anchor scrolls above the viewport. `visibleMessageIds`
answers "what is on screen", in document order.

Visibility is pay-for-what-you-use. Tracking only runs while something
subscribes to `useMessageScrollerVisibility`, and rows need a `messageId` to
participate.

### Reading Scroll State

Use `useMessageScrollerScrollable` when you need scroll state in JavaScript, such
as a status indicator or a custom "jump to latest" control. It reports which
edges the viewport can still scroll toward; "at the start/end" is the negation
(`!start` / `!end`), and "scrollable at all" is `start || end`. For styling the
scroller itself, prefer the `data-scrollable` attribute.


```tsx
const { start, end } = useMessageScrollerScrollable()
```

::::demo message-scroller-scrollable
<iframe class="demo" src="/demos/message-scroller-scrollable.html" title="message-scroller-scrollable" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-scroller-scrollable.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-scroller-scrollable.html]
<div class="mx-auto flex flex-col gap-4 w-full max-w-sm">
  <div data-slot="card" class="w-full" style="height: 35rem; gap: 0; overflow: hidden">
    <div data-slot="card-header" class="border-b" style="gap: calc(var(--spacing) * 1)">
      <div data-slot="card-title">Scroll Status</div>
      <div data-slot="card-description">
        Where the reader can scroll to based on current scroll position.
      </div>
    </div>
    <div data-slot="card-content" class="flex-1 min-h-0" style="overflow: hidden; padding: 0">
      <div data-slot="message-scroller" class="group/message-scroller">
        <div
          data-slot="message-scroller-viewport"
          id="ms-viewport"
          role="region"
          aria-label="Messages"
          tabindex="0"
        >
          <div
            data-slot="message-scroller-content"
            role="log"
            aria-relevant="additions"
            style="padding: var(--card-spacing); gap: calc(var(--spacing) * 4)"
          ></div>
        </div>
        <button
          data-slot="message-scroller-button"
          id="ms-button"
          type="button"
          data-active="false"
          aria-label="Scroll to latest"
          style="bottom: calc(var(--spacing) * 2)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
          <span class="sr-only">Scroll to latest</span>
        </button>
      </div>
    </div>
    <div
      data-slot="card-footer"
      class="border-t"
      style="justify-content: center; text-align: center"
    >
      <p id="scroll-status" class="text-sm" style="color: var(--muted-foreground)">
        You are at the top. You can only scroll down.
      </p>
    </div>
  </div>
  <p
    class="text-center text-xs"
    style="color: var(--muted-foreground); padding-inline: calc(var(--spacing) * 0.5)"
  >
    Scroll the transcript to see the footer update.
  </p>
</div>
```

```js:line-numbers [behavior]
(function () {
      var messages = Array.from({ length: 12 }, function (_, i) {
        return {
          id: "scrollable-" + (i + 1),
          role: i % 2 === 0 ? "user" : "assistant",
          text:
            i % 2 === 0
              ? "Review scroll checkpoint " + (i + 1) + "."
              : "Checkpoint " + (i + 1) + " is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
        }
      })
      var viewport = document.getElementById("ms-viewport")
      var content = viewport.querySelector("[data-slot=message-scroller-content]")
      var button = document.getElementById("ms-button")
      var statusEl = document.getElementById("scroll-status")
      var card = document.querySelector("[data-slot=card]")
      card.style.setProperty("--card-spacing", "calc(var(--spacing) * 6)")

      function paras(text) {
        return text.split(/\n\s*\n/).map(function (p) { return p.trim() }).filter(Boolean)
      }
      function rowHTML(m) {
        var isUser = m.role === "user"
        var body = isUser
          ? '<div data-slot="bubble-content">' + m.text + "</div>"
          : '<div data-slot="bubble-content">' + paras(m.text).map(function (p) {
              return '<p style="white-space:pre-wrap">' + p + "</p>"
            }).join("") + "</div>"
        return (
          '<div data-slot="message-scroller-item" data-message-id="' + m.id + '"' +
          (isUser ? ' data-scroll-anchor=""' : "") + ">" +
          '<div data-slot="message" class="group/message"' + (isUser ? ' data-align="end"' : "") + ">" +
          '<div data-slot="message-content">' +
          '<div data-slot="bubble" class="group/bubble" data-variant="' + (isUser ? "muted" : "ghost") + '"' + (isUser ? ' data-align="end"' : "") + ">" +
          body +
          "</div></div></div></div>"
        )
      }
      content.innerHTML = messages.map(rowHTML).join("")

      function getScrollStatus(start, end) {
        if (start && end) return "You can scroll both ways."
        if (end) return "You are at the top. You can only scroll down."
        if (start) return "You are at the bottom. You can only scroll up."
        return "All messages fit in the viewport."
      }
      function update() {
        var start = viewport.scrollTop > 1
        var end = viewport.scrollTop + viewport.clientHeight < viewport.scrollHeight - 1
        statusEl.textContent = getScrollStatus(start, end)
        var atBottom = !end
        button.setAttribute("data-active", atBottom ? "false" : "true")
      }
      viewport.addEventListener("scroll", update)
      button.addEventListener("click", function () {
        viewport.scrollTo({ top: viewport.scrollHeight, behavior: "smooth" })
      })
      viewport.scrollTop = 0
      update()
    })()
```
:::

::::


## Performance

`MessageScroller` is benchmarked against large transcripts with markdown and
composed message rows.

Our performance goal for `MessageScroller` is to keep the scroll hot path outside of React state: no React rerenders for
transcript rows, no forced layout on every scroll, and as little off-screen paint
work as the browser can avoid.

Scroll position, anchoring, and follow-output are tracked imperatively and mirrored onto the root and viewport through `data-*` attributes, so scrolling and streaming do not rerender transcript rows.

The styled `MessageScrollerItem` also ships with `content-visibility: auto` and
`contain-intrinsic-size`. Rows stay in the DOM for selection, copy,
find-in-page, SSR, and assistive tech, but the browser can skip rendering work
for rows far outside the viewport.

Visibility tracking is pay-for-what-you-use. A jump menu or active
turn indicator costs nothing until something subscribes to
`useMessageScrollerVisibility`.

This is comfortable for the expected range of a chat transcript: hundreds to low
thousands of turns, including messages with markdown and composed components.

## Virtualization

Virtualization is intentionally left outside the primitive. `MessageScroller`
renders real DOM rows and stays fast well into the thousands of turns (see
[Performance](#performance)), so most transcripts never need it.

When a transcript is large enough to need virtualization, use
`MessageScrollerViewport` as the scroll element and let the virtualizer own the
rows.

```tsx showLineNumbers

function VirtualizedTranscript({
  messages,
}: {
  messages: Array<{ id: string; content: React.ReactNode }>
}) {
  const viewportRef = React.useRef<HTMLDivElement>(null)

  const virtualizer = useVirtualizer({
    count: messages.length,
    getScrollElement: () => viewportRef.current,
    estimateSize: () => 86,
    getItemKey: (index) => messages[index]?.id ?? index,
    overscan: 8,
  })

  return (
    <MessageScrollerProvider>
      <MessageScroller>
        <MessageScrollerViewport ref={viewportRef}>
          <MessageScrollerContent className="block min-h-full">
            <div
              className="relative w-full"
              style={{ height: virtualizer.getTotalSize() }}
            >
              {virtualizer.getVirtualItems().map((virtualItem) => {
                const message = messages[virtualItem.index]

                if (!message) {
                  return null
                }

                return (
                  <div
                    key={virtualItem.key}
                    ref={virtualizer.measureElement}
                    data-index={virtualItem.index}
                    className="absolute start-0 top-0 w-full"
                    style={{
                      transform: `translateY(${virtualItem.start}px)`,
                    }}
                  >
                    <Message>{message.content}</Message>
                  </div>
                )
              })}
            </div>
          </MessageScrollerContent>
        </MessageScrollerViewport>
        <MessageScrollerButton />
      </MessageScroller>
    </MessageScrollerProvider>
  )
}
```

## Accessibility

`MessageScroller` keeps the scroll container keyboard reachable and the
transcript announceable without forcing a specific message UI.

`MessageScrollerViewport` is a labelled, keyboard-focusable scroll region by
default. It uses `role="region"`, `aria-label="Messages"`, and `tabIndex={0}`,
so keyboard users can focus the transcript and scroll it directly.

`MessageScrollerContent` marks the transcript as a live region with
`role="log"` and `aria-relevant="additions"`. New rows can be announced, but
streamed text mutations do not have to be announced token by token.

```tsx
<MessageScrollerContent aria-busy={status === "streaming"}>
  {/* messages */}
</MessageScrollerContent>
```

Pass `aria-busy` while a turn streams if announcements should wait for the
completed message row.

`MessageScrollerButton` renders a real button. When there is nothing to scroll
toward, it sets `inert`, uses `tabIndex={-1}`, and exposes `data-active="false"`
so inactive scroll controls do not create extra focus stops.

## Unstyled

The behavior in `MessageScroller` comes from the `@shadcn/react` package. To use
it directly with your own markup and styles, see
Message Scroller under @shadcn/react.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="message-scroller"` |
| `data-slot="message-scroller-viewport"` |
| `data-slot="message-scroller-content"` |
| `data-slot="message-scroller-item"` |
| `data-slot="message-scroller-button"` |

The props, data attributes, and hooks for every part are documented on the
@shadcn/react Message Scroller page.
They are identical for the styled component and the unstyled parts.
