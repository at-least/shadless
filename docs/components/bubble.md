---
title: "Bubble"
description: "Displays conversational content in a message bubble. Supports variants, alignment, grouping, reactions, and collapsible content."
---

# Bubble

Displays conversational content in a message bubble. Supports variants, alignment, grouping, reactions, and collapsible content.

::::demo bubble-demo
<iframe class="demo" src="/demos/bubble-demo.html" title="bubble-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-demo.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="bubble"
    data-variant="default"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Hey there! what's up?
    </div>
  </div>
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
        Hey! Want to see chat bubbles?
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
        I can group messages, switch sides, and keep the whole thread easy to scan.
      </div>
      <div
        data-slot="bubble-reactions"
        data-align="end"
        data-side="bottom"
        class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
        role="img"
        aria-label="Reaction: thumbs up"
      >
        <span>👍</span>
      </div>
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="default"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Sure. Hit me with your best demo.
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
      Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
      role="img"
      aria-label="Reactions: thumbs up, fire, eyes, and 2 more"
    >
      <span>👍</span><span>🔥</span><span>👀</span><span>+2</span>
    </div>
  </div>
</div>
```
:::

::::


The `Bubble` component displays framed conversational content. Use it for chat text, short structured output, quoted replies, suggestions, and reactions.

For full-featured chat interfaces, use the [         ](/components/message) component. `Bubble` is intentionally scoped to the bubble surface. Place avatars, names, timestamps, metadata, and message-level actions in [         ](/components/message).

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/bubble.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/bubble.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/bubble.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/bubble.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
| `side="outline"` (JSX prop) | `data-side="outline"` (markup) |
| `align="outline"` (JSX prop) | `data-align="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
bubble
├── bubble-content
└── bubble-reactions
```

## Features

- Seven visual variants, from a strong primary bubble to unframed ghost content
- Start and end alignment for sender and receiver bubbles
- Reactions that anchor to the bubble edge with configurable side and alignment
- Bubbles size to their content, up to 80% of the container width
- Polymorphic content via `asChild` for link and button bubbles
- Customizable styling through the `className` prop on every part

## Variants

Use `variant` to change the visual treatment of the bubble.

::::demo bubble-variants
<iframe class="demo" src="/demos/bubble-variants.html" title="bubble-variants" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-variants.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-variants.html]
<div class="flex w-full max-w-sm flex-col gap-12 py-12">
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
      This is the default primary bubble.
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="secondary"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-secondary *:data-[slot=bubble-content]:text-secondary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      This is the secondary variant.
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
      This one is muted. It uses a lower emphasis color for the chat bubble.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
      role="img"
      aria-label="Reaction: thumbs up"
    >
      <span>👍</span>
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="tinted"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] dark:*:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] *:data-[slot=bubble-content]:text-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      This one is tinted. The tint is a softer color derived from the primary color.
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="outline"
    data-align="start"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-background *:data-[slot=bubble-content]:border-border [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:text-foreground dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-input/30"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      We can also use an outlined variant.
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="destructive"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-destructive/10 dark:*:data-[slot=bubble-content]:bg-destructive/20 *:data-[slot=bubble-content]:text-destructive [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/20 dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/30"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Or a destructive variant with a reaction.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
      role="img"
      aria-label="Reaction: fire"
    >
      <span>🔥</span>
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="ghost"
    data-align="start"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:rounded-none *:data-[slot=bubble-content]:bg-transparent *:data-[slot=bubble-content]:p-0 [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:text-foreground dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-muted/50 border-none"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      <div data-slot="markdown">
        Ghost bubbles work for assistant text, **markdown**, and other content that should not be
        framed. This is perfect for assistant messages that should not have a frame and can take the
        full width of the container. You can also render `code` in it. Ghost bubbles are full width
        and can take the full width of the container.
      </div>
    </div>
  </div>
</div>
```
:::

::::


| Variant       | Description                                            |
| ------------- | ------------------------------------------------------ |
| `default`     | A strong primary bubble, usually for the current user. |
| `secondary`   | The standard neutral bubble for conversation content.  |
| `muted`       | A lower-emphasis bubble for quiet supporting content.  |
| `tinted`      | A subtle primary-tinted bubble.                        |
| `outline`     | A bordered bubble for secondary or rich content.       |
| `ghost`       | Unframed content for assistant text or rich content.   |
| `destructive` | A destructive bubble for error or failed actions.      |

A bubble sizes to its content, up to 80% of the container width. The `ghost` variant removes the max-width so assistant text and rich content can span the full row.

## Alignment

Use `align` on `Bubble` to align the bubble to the start or end of the conversation.

::::demo bubble-alignment
<iframe class="demo" src="/demos/bubble-alignment.html" title="bubble-alignment" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-alignment.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-alignment.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
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
      This bubble is aligned to the start. This is the default alignment.
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="default"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      This bubble is aligned to the end. Use this for user messages.
    </div>
  </div>
</div>
```
:::

::::


| align   | Description                                        |
| ------- | -------------------------------------------------- |
| `start` | Align the bubble to the start of the conversation. |
| `end`   | Align the bubble to the end of the conversation.   |

**Note:** When building chat interfaces, you probably want to use alignment on the `Message` component itself, not the `Bubble` component. You can use the `role` prop on the `Message` component to automatically align the bubble to the start or end of the conversation.

## Bubble Group

Use `BubbleGroup` to group consecutive bubbles from the same sender. Note the `align` prop should be set on the `Bubble` component itself, not the `BubbleGroup` component.

```text
BubbleGroup
├── Bubble
│   └── BubbleContent
└── Bubble
    └── BubbleContent
```

::::demo bubble-group-demo
<iframe class="demo" src="/demos/bubble-group-demo.html" title="bubble-group-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-group-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-group-demo.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
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
      Can you tell me what's the issue?
    </div>
  </div>
  <div data-slot="bubble-group" class="gap-2 flex min-w-0 flex-col">
    <div
      data-slot="bubble"
      data-variant="default"
      data-align="end"
      class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
    >
      <div
        data-slot="bubble-content"
        class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
      >
        You tell me!
      </div>
    </div>
    <div
      data-slot="bubble"
      data-variant="default"
      data-align="end"
      class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
    >
      <div
        data-slot="bubble-content"
        class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
      >
        It worked yesterday. You broke it!
      </div>
    </div>
    <div
      data-slot="bubble"
      data-variant="default"
      data-align="end"
      class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
    >
      <div
        data-slot="bubble-content"
        class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
      >
        Find the bug and fix it.
      </div>
      <div
        data-slot="bubble-reactions"
        data-align="start"
        data-side="bottom"
        class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 left-3"
        aria-label="Reactions: eyes"
      >
        <span>👀</span>
      </div>
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
      Want me to diff yesterday's you against today's you? It's a bit embarrassing.
    </div>
  </div>
</div>
```
:::

::::


## Links and Buttons

You can turn a bubble into a link or button by using the `asChild` prop on `BubbleContent`.

::::demo bubble-link-button
<iframe class="demo" src="/demos/bubble-link-button.html" title="bubble-link-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-link-button.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-link-button.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
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
      How can I help you today?
    </div>
  </div>
  <div data-slot="bubble-group" class="gap-2 flex min-w-0 flex-col">
    <div
      data-slot="bubble"
      data-variant="tinted"
      data-align="end"
      class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] dark:*:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] *:data-[slot=bubble-content]:text-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
    >
      <button
        data-slot="bubble-content"
        class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
      >
        I forgot my password
      </button>
    </div>
    <div
      data-slot="bubble"
      data-variant="tinted"
      data-align="end"
      class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] dark:*:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] *:data-[slot=bubble-content]:text-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
    >
      <button
        data-slot="bubble-content"
        class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
      >
        I need help with my subscription
      </button>
    </div>
    <div
      data-slot="bubble"
      data-variant="tinted"
      data-align="end"
      class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] dark:*:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] *:data-[slot=bubble-content]:text-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
    >
      <button
        data-slot="bubble-content"
        class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
      >
        Something else. Talk to a human.
      </button>
    </div>
  </div>
</div>
```
:::

::::




## Reactions

Use `BubbleReactions` for bubble reactions. You can use it to display reactions or quick action buttons. Use `side` and `align` to position the row — `side="top"` anchors it to the upper edge. Reactions overlap the bubble edge, so leave vertical space between rows — the examples below use a larger `gap` for this reason.

::::demo bubble-reactions
<iframe class="demo" src="/demos/bubble-reactions.html" title="bubble-reactions" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-reactions.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-reactions.html]
<div class="flex w-full max-w-sm flex-col gap-12 py-12">
  <div
    data-slot="bubble"
    data-variant="muted"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      I don't need tests, I know my code works.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="start"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 left-3"
      role="img"
      aria-label="Reactions: thumbs up, surprised"
    >
      <span>👍</span><span>😮</span>
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
      Bold. Fine I'll add some tests. I'll let you know when they're done.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
      role="img"
      aria-label="Reactions: eyes, rocket, and 2 more"
    >
      <span>👀</span><span>🚀</span><span>+2</span>
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="default"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Tests passed on the first try. All 142 of them. Looking good!
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="start"
      data-side="top"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center top-0 -translate-y-3/4 left-3"
      role="img"
      aria-label="Reactions: party popper, clapping hands"
    >
      <span>🎉</span><span>👏</span>
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="destructive"
    data-align="start"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-destructive/10 dark:*:data-[slot=bubble-content]:bg-destructive/20 *:data-[slot=bubble-content]:text-destructive [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/20 dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/30"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Are you sure I can run this command?
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
    >
      <button
        data-slot="button"
        data-variant="ghost"
        data-size="xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-6 gap-1 rounded-[min(var(--radius-md),10px)] px-2 text-xs in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3"
      >
        Yes, run it
      </button>
    </div>
  </div>
</div>
```
:::

::::


## Show More / Collapsible

Long bubble content can be composed with [             ](/components/collapsible) to allow for a show more or show less interaction. Use the `CollapsibleTrigger` component to trigger the collapsible content.

::::demo bubble-collapsible
<iframe class="demo" src="/demos/bubble-collapsible.html" title="bubble-collapsible" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-collapsible.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-collapsible.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
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
      How can I help you today?
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="muted"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-muted [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors whitespace-pre-line"
    >
      <div data-state="closed" data-slot="collapsible">
        <div>
          The accessibility review found two focus states that were visually too subtle in dark
          mode. I checked the dialog, menu, and drawer paths because each one renders focusable
          control...
        </div>
        <button
          data-slot="collapsible-trigger"
          data-variant="link"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-8 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-1 p-0 text-muted-foreground"
          type="button"
          aria-controls="radix-<auto>_"
          aria-expanded="false"
          data-state="closed"
        >
          Show more<svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-chevron-down group-data-open/button:rotate-180"
            data-icon="inline-end"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## Tooltip

Wrap a bubble in a [         ](/components/tooltip) to reveal metadata on hover, such as when a message was read.

::::demo bubble-tooltip
<iframe class="demo" src="/demos/bubble-tooltip.html" title="bubble-tooltip" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-tooltip.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-tooltip.html]
<div class="flex w-full max-w-sm flex-col gap-4 py-12">
  <div
    data-slot="bubble"
    data-variant="secondary"
    data-align="start"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-secondary *:data-[slot=bubble-content]:text-secondary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Did you remove the stale route?
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="default"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Yes, removed it from the registry.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3 p-0"
    >
      <button
        data-slot="tooltip-trigger"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        data-state="closed"
        id="k0-trigger"
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
          class="lucide lucide-check"
        >
          <path d="M20 6 9 17l-5-5"></path>
        </svg>
      </button>
    </div>
  </div>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 21px);
      min-width: max-content;
      --radix-popper-transform-origin: 27.836px 23px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 39px;
      --radix-popper-anchor-width: 40px;
      --radix-popper-anchor-height: 33px;
    "
  >
    <div
      data-side="top"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      Read on Jan 5, 2026 at 4:32 PM<span
        style="position: absolute; bottom: 0px; transform: translateY(100%); left: 22.836px"
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >Read on Jan 5, 2026 at 4:32 PM</span
      >
    </div>
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
```
:::

::::


## Popover

Pair a bubble with a [         ](/components/popover) to surface more information on demand, such as the full error message for a failed action.

::::demo bubble-popover
<iframe class="demo" src="/demos/bubble-popover.html" title="bubble-popover" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/bubble-popover.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [bubble-popover.html]
<div class="flex w-full max-w-sm flex-col gap-4 py-12">
  <div
    data-slot="bubble"
    data-variant="default"
    data-align="end"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Run the build script.
    </div>
  </div>
  <div
    data-slot="bubble"
    data-variant="destructive"
    data-align="start"
    class="gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col *:data-[slot=bubble-content]:bg-destructive/10 dark:*:data-[slot=bubble-content]:bg-destructive/20 *:data-[slot=bubble-content]:text-destructive [&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/20 dark:[&amp;&gt;[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/30"
  >
    <div
      data-slot="bubble-content"
      class="rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors"
    >
      Failed to run the command.
    </div>
    <div
      data-slot="bubble-reactions"
      data-align="end"
      data-side="bottom"
      class="rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center bottom-0 translate-y-3/4 right-3"
    >
      <button
        data-slot="popover-trigger"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3 aria-expanded:text-destructive"
        aria-label="Show error details"
        type="button"
        aria-haspopup="dialog"
        aria-expanded="false"
        aria-controls="k0"
        data-state="closed"
        id="k0-trigger"
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
          class="lucide lucide-info"
        >
          <circle cx="12" cy="12" r="10"></circle>
          <path d="M12 16v-4"></path>
          <path d="M12 8h.01"></path>
        </svg>
      </button>
    </div>
  </div>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 81px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 639px;
      --radix-popper-anchor-width: 40px;
      --radix-popper-anchor-height: 33px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium text-sm">
          Command failed with exit code 1
        </div>
        <p data-slot="popover-description" class="text-muted-foreground text-sm">
          ENOENT: no such file or directory, open pnpm-lock.yaml
        </p>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
    });
  } })
})()
```
:::

::::


## Accessibility

`Bubble` renders the presentational message surface. Keep conversation-level semantics on the surrounding container and follow the guidelines below.

### Labeling Reactions

Reactions render as a row of emoji. A screen reader reads each glyph with no context, and counters like `+8` are announced as "plus eight". Group the row as a single image with a descriptive `aria-label` so it announces once. `role="img"` also hides the individual emoji from assistive tech, so no `aria-hidden` is needed.

```tsx showLineNumbers
<BubbleReactions role="img" aria-label="Reactions: thumbs up, fire, and 8 more">
  <span>👍</span>
  <span>🔥</span>
  <span>+8</span>
</BubbleReactions>
```

When reactions are interactive, render buttons instead and give icon-only buttons an `aria-label`.

```tsx showLineNumbers
<BubbleReactions>
  <Button aria-label="Thumbs up" variant="secondary" size="icon-xs">
    <ThumbsUpIcon />
  </Button>
</BubbleReactions>
```

### Interactive Bubbles

When a bubble is clickable, render it as a real `<button>` or `<a>` with the `asChild` prop so it is focusable and exposes the correct role. `BubbleContent` ships a visible focus ring for interactive elements, and the accessible name comes from the bubble text. No extra label is needed.

```tsx showLineNumbers
<Bubble variant="muted" align="end">
  <BubbleContent asChild>
    <button type="button" onClick={onReply}>
      I forgot my password
    </button>
  </BubbleContent>
</Bubble>
```

### Meaning Beyond Color

Bubble variants signal role and tone with color. Pair them with text, alignment, or icons so meaning is not conveyed by color alone. For a `destructive` bubble, keep the error context in the message text rather than relying on the color treatment.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="bubble-group"` |
| `data-slot="bubble"` |
| `data-slot="bubble-content"` |
| `data-slot="bubble-reactions"` |

**Runtime:** no JavaScript — this is markup + CSS. Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/bubble.css` for any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `bubble` | `data-variant` | `default`, `secondary`, `muted`, `tinted`, `outline`, `ghost`, `destructive` | `default` |
| `bubble-reactions` | `data-side` | `top`, `bottom` | `bottom` |
| `bubble-reactions` | `data-align` | `start`, `end` | `end` |
