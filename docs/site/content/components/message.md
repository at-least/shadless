---
title: "Message"
description: "Displays a message in a conversation, with optional avatar, header, footer, and alignment."
weight: 30
---

# Message

Displays a message in a conversation, with optional avatar, header, footer, and alignment.

{% <demo name="message-demo" status="authored"> %}
<iframe class="demo" src="/demos/message-demo.html" title="message-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=message-demo.html
<div class="flex w-full max-w-sm flex-col gap-6 py-12">
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">ME</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">Deploying to prod real quick.</div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">R</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">It's 4:55 PM. On a Friday.</div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">ME</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">It's a one-line change.</div>
      </div>
      <div data-slot="message-footer">Delivered</div>
    </div>
  </div>
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">R</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble-group">
        <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
          <div data-slot="bubble-content">It's always a one-line change 😭.</div>
        </div>
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="group/bubble *:data-[slot=bubble-content]:bg-muted [&>[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div data-slot="bubble-content">Alright, let me take a look.</div>
          <div
            data-slot="bubble-reactions"
            data-align="end"
            data-side="bottom"
            class="bg-muted"
            aria-label="Reactions: thumbs up"
          >
            <span>👍</span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div data-slot="marker" data-variant="default" class="group/marker" role="status">
    <span data-slot="marker-content" class="shimmer"
      ><span class="font-medium">Oliver</span> is typing...</span
    >
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    // avatar: settle image vs fallback from load state (radix Presence),
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
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

The `Message` component lays out a single message in a conversation. It handles the avatar, alignment, header, and footer around the message surface.

For AI apps, you can render reasoning steps, tool calls and assistant messages using the `Message` component.

## Installation

**Add shadless and this component to your Tailwind v4 (>= 4.3) entry:**

```css
@import "shadless";
@import "shadless/message.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build (>= 4.3) |
| `dist/css/message.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
message
├── message-avatar
└── message-content
    ├── message-header
    ├── Bubble
    └── message-footer
```

## Features

- Start and end alignment for sender and receiver rows via the `align` prop
- Avatar slot that anchors to the bottom of the message and stays clear of the footer
- Header and footer slots for sender names, status, and message actions
- Footer follows the message side; actions stay aligned on `align="end"` rows
- Group wrapper for stacking consecutive messages from the same sender
- Customizable styling through the `className` prop on every part

## Avatar

Use `MessageAvatar` to render an avatar next to the message. Set `align="end"` on the message to align the avatar to the end of the message.

{% <demo name="message-avatar" status="authored"> %}
<iframe class="demo" src="/demos/message-avatar.html" title="message-avatar" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-avatar.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=message-avatar.html
<div class="flex w-full max-w-sm flex-col gap-6 py-12">
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">R</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">The build failed during dependency installation.</div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">R</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">Can you share the exact error?</div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-avatar">
      <span data-slot="avatar" data-size="default" class="group/avatar"
        ><span data-slot="avatar-fallback">R</span></span
      >
    </div>
    <div data-slot="message-content">
      <div data-slot="bubble-group">
        <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
          <div data-slot="bubble-content">Here's the error from the logs</div>
        </div>
        <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
          <div data-slot="bubble-content">
            Something went wrong with the build. The libraries are not installed correctly. Try
            running the build again.
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    // avatar: settle image vs fallback from load state (radix Presence),
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
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

| align   | Description                                         |
| ------- | --------------------------------------------------- |
| `start` | Align the message to the start of the conversation. |
| `end`   | Align the message to the end of the conversation.   |

## Group

Use `MessageGroup` to stack consecutive messages from the same sender. Render an empty `MessageAvatar` on the earlier messages to keep them aligned with the avatar on the last one.

{% <demo name="message-group" status="authored"> %}
<iframe class="demo" src="/demos/message-group.html" title="message-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-group.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=message-group.html
<div class="flex w-full max-w-sm flex-col gap-6 py-12">
  <div data-slot="message-group">
    <div data-slot="message" data-align="start" class="group/message">
      <div data-slot="message-avatar"></div>
      <div data-slot="message-content">
        <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
          <div data-slot="bubble-content">I checked the registry addresses.</div>
        </div>
      </div>
    </div>
    <div data-slot="message" data-align="start" class="group/message">
      <div data-slot="message-avatar">
        <span data-slot="avatar" data-size="default" class="group/avatar"
          ><span data-slot="avatar-fallback">CN</span></span
        >
      </div>
      <div data-slot="message-content">
        <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
          <div data-slot="bubble-content">
            The component and example JSON now live under the UI registry.
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    // avatar: settle image vs fallback from load state (radix Presence),
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
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Header and Footer

Use `MessageHeader` for a sender name and `MessageFooter` for metadata such as a delivery or read status.

{% <demo name="message-header-footer" status="authored"> %}
<iframe class="demo" src="/demos/message-header-footer.html" title="message-header-footer" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-header-footer.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=message-header-footer.html
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-content">
      <div data-slot="message-header">Olivia</div>
      <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">I already checked the logs.</div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">
          Send the report to the team. Ping @shadcn if you need help.
        </div>
      </div>
      <div data-slot="message-footer">
        <div>Read <span class="font-normal">Yesterday</span></div>
      </div>
    </div>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Actions

Place message-level actions in `MessageFooter`, such as copy, retry, or feedback buttons.

{% <demo name="message-actions" status="authored"> %}
<iframe class="demo" src="/demos/message-actions.html" title="message-actions" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-actions.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=message-actions.html
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">
          The install failure is coming from the workspace package.
        </div>
      </div>
      <div data-slot="message-footer">
        <button
          data-slot="button"
          data-variant="ghost"
          data-size="icon"
          class="group/button"
          aria-label="Copy"
          title="Copy"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-copy"
          >
            <rect width="14" height="14" x="8" y="8" rx="2" ry="2"></rect>
            <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>
          </svg></button
        ><button
          data-slot="button"
          data-variant="ghost"
          data-size="icon"
          class="group/button"
          aria-label="Like"
          title="Like"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-thumbs-up"
          >
            <path d="M7 10v12"></path>
            <path
              d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z"
            ></path>
          </svg></button
        ><button
          data-slot="button"
          data-variant="ghost"
          data-size="icon"
          class="group/button"
          aria-label="Dislike"
          title="Dislike"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-thumbs-down"
          >
            <path d="M17 14V2"></path>
            <path
              d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22a3.13 3.13 0 0 1-3-3.88Z"
            ></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">Okay drop me a link. Taking a look...</div>
      </div>
      <div data-slot="message-footer" class="gap-2">
        <span class="font-normal text-destructive">Failed to send</span
        ><button
          data-slot="button"
          data-variant="ghost"
          data-size="icon-xs"
          class="group/button"
          title="Retry"
          aria-label="Retry"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-refresh-ccw"
          >
            <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
            <path d="M3 3v5h5"></path>
            <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
            <path d="M16 16h5v5"></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Attachment

{% <demo name="message-attachment" status="authored"> %}
<iframe class="demo" src="/demos/message-attachment.html" title="message-attachment" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-attachment.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=message-attachment.html
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-content">
      <div
        data-slot="attachment"
        data-state="done"
        data-size="default"
        data-orientation="vertical"
        class="group/attachment"
      >
        <div
          data-slot="attachment-media"
          data-variant="image"
          class="[&_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! *:[img]:w-full"
        >
          <img
            alt="Workspace"
            src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
          />
        </div>
      </div>
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">
          Here's the image. Can you add it to the PDF? Use it for the cover page.
        </div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="start" class="group/message">
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="muted" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">
          Done. Here's the PDF with the image added as the cover page.
        </div>
      </div>
      <div
        data-slot="attachment"
        data-state="done"
        data-size="default"
        data-orientation="horizontal"
        class="group/attachment"
      >
        <div
          data-slot="attachment-media"
          data-variant="icon"
          class="[&_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6!"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-file-text"
          >
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
            <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
            <path d="M10 9H8"></path>
            <path d="M16 13H8"></path>
            <path d="M16 17H8"></path>
          </svg>
        </div>
        <div data-slot="attachment-content">
          <span data-slot="attachment-title">sales-dashboard.pdf</span
          ><span data-slot="attachment-description">PDF · 2.4 MB</span>
        </div>
        <div data-slot="attachment-actions">
          <button
            data-slot="attachment-action"
            data-variant="secondary"
            data-size="icon-sm"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg"
            type="button"
            title="Download"
            aria-label="Download"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-download"
            >
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="7 10 12 15 17 10"></polyline>
              <line x1="12" x2="12" y1="15" y2="3"></line>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
  <div data-slot="message" data-align="end" class="group/message">
    <div data-slot="message-content">
      <div data-slot="bubble" data-variant="default" data-align="start" class="group/bubble">
        <div data-slot="bubble-content">Thanks. Looks good.</div>
      </div>
    </div>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Accessibility

`Message` is a presentational layout wrapper. Accessibility comes from the content you place inside it.

### Label icon-only actions

Action buttons in `MessageFooter` are usually icon-only, so give each one an `aria-label`.

```html
<div data-slot="message-footer">
  <button data-slot="button" class="group/button" data-variant="ghost" data-size="icon" aria-label="Copy">
    <!-- lucide "copy" icon -->
  </button>
</div>
```

### Status updates

For in-progress messages, use a [`Marker`](/components/marker) with `role="status"` so assistive tech announces the update as it appears.

```html
<div data-slot="message" class="group/message">
  <div data-slot="marker" class="group/marker" role="status">
    <!-- lucide "marker" icon -->
      <svg data-slot="spinner" />
    <!-- lucide "marker" icon -->
    <span data-slot="marker-content">Checking the logs...</span>
  </div>
</div>
```

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="message-group"` |
| `data-slot="message"` |
| `data-slot="message-avatar"` |
| `data-slot="message-content"` |
| `data-slot="message-header"` |
| `data-slot="message-footer"` |

**Runtime:** no JavaScript — this is markup + CSS. No `cva`-declared variants — the stylesheet embedded below is the whole default look; its `data-*` selectors are the attributes each slot's styling depends on.

**The stylesheet** — `shadless/message.css`, the exact bytes of `dist/css/message.css`, verbatim. Every rule keys on the `data-slot` / `data-*` attributes in the tables above, so this is the authoritative default look of each slot:

```css
/* message */
@layer components {
  [data-slot="message-group"] { @apply gap-2 flex min-w-0 flex-col; }
  [data-slot="message"] { @apply text-sm gap-2 relative flex w-full min-w-0 data-[align=end]:flex-row-reverse; }
  [data-slot="message-avatar"] { @apply min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted; }
  [data-slot="message-content"] { @apply gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word; }
  [data-slot="message-header"] { @apply text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center; }
  [data-slot="message-footer"] { @apply text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center group-data-[align=end]/message:justify-end; }
}
```
