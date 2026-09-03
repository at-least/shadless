---
title: "Message"
description: "Displays a message in a conversation, with optional avatar, header, footer, and alignment."
---

# Message

Displays a message in a conversation, with optional avatar, header, footer, and alignment.

::::demo message-demo
<iframe class="demo" src="/demos/message-demo.html" title="message-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-demo.html]
<div class="flex w-full max-w-sm flex-col gap-6 py-12">
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >ME</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Deploying to prod real quick.
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >R</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="muted"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          It's 4:55 PM. On a Friday.
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >ME</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          It's a one-line change.
        </div>
      </div>
      <div
        data-slot="message-footer"
        class="text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center group-data-[align=end]/message:justify-end"
      >
        Delivered
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >R</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div data-slot="bubble-group" class="gap-2 flex min-w-0 flex-col">
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div
            data-slot="bubble-content"
            class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
          >
            It's always a one-line change 😭.
          </div>
        </div>
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div
            data-slot="bubble-content"
            class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
          >
            Alright, let me take a look.
          </div>
          <div
            data-slot="bubble-reactions"
            data-align="end"
            data-side="bottom"
            class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
            aria-label="Reactions: thumbs up"
          >
            <span>👍</span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
    role="status"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word shimmer"
      ><span class="font-medium">Oliver</span> is typing...</span
    >
  </div>
</div>
```
:::

::::


The `Message` component lays out a single message in a conversation. It handles the avatar, alignment, header, and footer around the message surface.

For AI apps, you can render reasoning steps, tool calls and assistant messages using the `Message` component.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/message.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/message.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/message.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from `dist/components/message.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/message.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
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

::::demo message-avatar
<iframe class="demo" src="/demos/message-avatar.html" title="message-avatar" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-avatar.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-avatar.html]
<div class="flex w-full max-w-sm flex-col gap-6 py-12">
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >R</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="muted"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          The build failed during dependency installation.
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >R</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Can you share the exact error?
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-avatar"
      class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><span
          data-slot="avatar-fallback"
          class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
          >R</span
        ></span
      >
    </div>
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div data-slot="bubble-group" class="gap-2 flex min-w-0 flex-col">
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div
            data-slot="bubble-content"
            class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
          >
            Here's the error from the logs
          </div>
        </div>
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div
            data-slot="bubble-content"
            class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
          >
            Something went wrong with the build. The libraries are not installed correctly. Try
            running the build again.
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
```
:::

::::


| align   | Description                                         |
| ------- | --------------------------------------------------- |
| `start` | Align the message to the start of the conversation. |
| `end`   | Align the message to the end of the conversation.   |

## Group

Use `MessageGroup` to stack consecutive messages from the same sender. Render an empty `MessageAvatar` on the earlier messages to keep them aligned with the avatar on the last one.

::::demo message-group
<iframe class="demo" src="/demos/message-group.html" title="message-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-group.html]
<div class="flex w-full max-w-sm flex-col gap-6 py-12">
  <div data-slot="message-group" class="gap-2 flex min-w-0 flex-col">
    <div
      data-slot="message"
      data-align="start"
      class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
    >
      <div
        data-slot="message-avatar"
        class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
      ></div>
      <div
        data-slot="message-content"
        class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
      >
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div
            data-slot="bubble-content"
            class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
          >
            I checked the registry addresses.
          </div>
        </div>
      </div>
    </div>
    <div
      data-slot="message"
      data-align="start"
      class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
    >
      <div
        data-slot="message-avatar"
        class="min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted"
      >
        <span
          data-slot="avatar"
          data-size="default"
          class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
          ><span
            data-slot="avatar-fallback"
            class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
            >CN</span
          ></span
        >
      </div>
      <div
        data-slot="message-content"
        class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
      >
        <div
          data-slot="bubble"
          data-variant="muted"
          data-align="start"
          class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        >
          <div
            data-slot="bubble-content"
            class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
          >
            The component and example JSON now live under the UI registry.
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## Header and Footer

Use `MessageHeader` for a sender name and `MessageFooter` for metadata such as a delivery or read status.

::::demo message-header-footer
<iframe class="demo" src="/demos/message-header-footer.html" title="message-header-footer" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-header-footer.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-header-footer.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="message-header"
        class="text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center"
      >
        Olivia
      </div>
      <div
        data-slot="bubble"
        data-variant="muted"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          I already checked the logs.
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Send the report to the team. Ping @shadcn if you need help.
        </div>
      </div>
      <div
        data-slot="message-footer"
        class="text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center group-data-[align=end]/message:justify-end"
      >
        <div>Read <span class="font-normal">Yesterday</span></div>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## Actions

Place message-level actions in `MessageFooter`, such as copy, retry, or feedback buttons.

::::demo message-actions
<iframe class="demo" src="/demos/message-actions.html" title="message-actions" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-actions.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-actions.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="muted"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          The install failure is coming from the workspace package.
        </div>
      </div>
      <div
        data-slot="message-footer"
        class="text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center group-data-[align=end]/message:justify-end"
      >
        <button
          data-slot="button"
          data-variant="ghost"
          data-size="icon"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Okay drop me a link. Taking a look...
        </div>
      </div>
      <div
        data-slot="message-footer"
        class="text-xs font-medium text-muted-foreground px-3 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center group-data-[align=end]/message:justify-end gap-2"
      >
        <span class="font-normal text-destructive">Failed to send</span
        ><button
          data-slot="button"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
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
:::

::::


## Attachment

::::demo message-attachment
<iframe class="demo" src="/demos/message-attachment.html" title="message-attachment" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/message-attachment.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [message-attachment.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="attachment"
        data-state="done"
        data-size="default"
        data-orientation="vertical"
        class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
      >
        <div
          data-slot="attachment-media"
          data-variant="image"
          class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
        >
          <img
            alt="Workspace"
            src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
          />
        </div>
      </div>
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Here's the image. Can you add it to the PDF? Use it for the cover page.
        </div>
      </div>
    </div>
  </div>
  <div
    data-slot="message"
    data-align="start"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="muted"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Done. Here's the PDF with the image added as the cover page.
        </div>
      </div>
      <div
        data-slot="attachment"
        data-state="done"
        data-size="default"
        data-orientation="horizontal"
        class="rounded-xl w-fit focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center"
      >
        <div
          data-slot="attachment-media"
          data-variant="icon"
          class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        <div
          data-slot="attachment-content"
          class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
        >
          <span
            data-slot="attachment-title"
            class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
            >sales-dashboard.pdf</span
          ><span
            data-slot="attachment-description"
            class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
            >PDF · 2.4 MB</span
          >
        </div>
        <div
          data-slot="attachment-actions"
          class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
        >
          <button
            data-slot="attachment-action"
            data-variant="secondary"
            data-size="icon-sm"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg"
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
  <div
    data-slot="message"
    data-align="end"
    class="text-sm gap-2 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse"
  >
    <div
      data-slot="message-content"
      class="gap-2.5 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word"
    >
      <div
        data-slot="bubble"
        data-variant="default"
        data-align="start"
        class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
      >
        <div
          data-slot="bubble-content"
          class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
        >
          Thanks. Looks good.
        </div>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## Accessibility

`Message` is a presentational layout wrapper. Accessibility comes from the content you place inside it.

### Label icon-only actions

Action buttons in `MessageFooter` are usually icon-only, so give each one an `aria-label`.

```tsx showLineNumbers
<MessageFooter>
  <Button variant="ghost" size="icon" aria-label="Copy">
    <CopyIcon />
  </Button>
</MessageFooter>
```

### Status updates

For in-progress messages, use a [`Marker`](/components/marker) with `role="status"` so assistive tech announces the update as it appears.

```tsx showLineNumbers
<Message>
  <Marker role="status">
    <MarkerIcon>
      <Spinner />
    </MarkerIcon>
    <MarkerContent>Checking the logs...</MarkerContent>
  </Marker>
</Message>
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

**Runtime:** no JavaScript — this is markup + CSS. No `cva`-declared variants. Check `dist/css/message.css` for any `data-*` attribute this slot's styling depends on.
