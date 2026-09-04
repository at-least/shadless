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
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

See the demos for real compositions — every slot is a `data-slot` attribute in the shipped markup.

## Core Concepts

shadless's `message-scroller` is pure static markup and CSS — `generated/ir/message-scroller.json` reports tier `static`, and there is no `dist/js/message-scroller.js`. Upstream's virtualization, scroll anchoring, `useMessageScrollerVisibility`, and `scrollToMessage` are all React-only behavior this port does not include; build them yourself on top of the shipped markup if you need them.

## Accessibility

`MessageScroller` keeps the scroll container keyboard reachable and the
transcript announceable without forcing a specific message UI.

`MessageScrollerViewport` is a labelled, keyboard-focusable scroll region by
default. It uses `role="region"`, `aria-label="Messages"`, and `tabIndex={0}`,
so keyboard users can focus the transcript and scroll it directly.

`MessageScrollerContent` marks the transcript as a live region with
`role="log"` and `aria-relevant="additions"`. New rows can be announced, but
streamed text mutations do not have to be announced token by token.

```html
<div data-slot="message-scroller-content" aria-busy="true">
  <!-- messages -->
</div>
```

Toggle `aria-busy` yourself while a turn streams — there is no built-in status tracking.
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
