---
title: "Message"
description: "Displays a message in a conversation, with optional avatar, header, footer, and alignment."
---

# Message

Displays a message in a conversation, with optional avatar, header, footer, and alignment.

<iframe class="demo" src="/demos/message-demo.html" title="message-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

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


**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



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

<iframe class="demo" src="/demos/message-avatar.html" title="message-avatar" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

| align   | Description                                         |
| ------- | --------------------------------------------------- |
| `start` | Align the message to the start of the conversation. |
| `end`   | Align the message to the end of the conversation.   |

## Group

Use `MessageGroup` to stack consecutive messages from the same sender. Render an empty `MessageAvatar` on the earlier messages to keep them aligned with the avatar on the last one.

<iframe class="demo" src="/demos/message-group.html" title="message-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Header and Footer

Use `MessageHeader` for a sender name and `MessageFooter` for metadata such as a delivery or read status.

<iframe class="demo" src="/demos/message-header-footer.html" title="message-header-footer" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Actions

Place message-level actions in `MessageFooter`, such as copy, retry, or feedback buttons.

<iframe class="demo" src="/demos/message-actions.html" title="message-actions" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Attachment

<iframe class="demo" src="/demos/message-attachment.html" title="message-attachment" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

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

For in-progress messages, use a [        ](/components/marker) with `role="status"` so assistive tech announces the update as it appears.

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


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="message-group"` |
| `data-slot="message"` |
| `data-slot="message-avatar"` |
| `data-slot="message-content"` |
| `data-slot="message-header"` |
| `data-slot="message-footer"` |

### Message

The message row wrapper.

| Prop        | Type               | Default   | Description                                       |
| ----------- | ------------------ | --------- | ------------------------------------------------- |
| `align`     | `"start" \| "end"` | `"start"` | The alignment of the message in the conversation. |
| `className` | `string`           | -         | Additional classes to apply to the row.           |

### MessageGroup

Groups consecutive messages from the same sender.

| Prop        | Type     | Default | Description                                    |
| ----------- | -------- | ------- | ---------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the group root. |

### MessageAvatar

The avatar slot, aligned to the bottom of the message. When the message has a `MessageFooter`, the avatar shifts up to stay aligned with the message surface instead of the footer.

| Prop        | Type     | Default | Description                                     |
| ----------- | -------- | ------- | ----------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the avatar slot. |

### MessageContent

Wraps the header, message surface, and footer.

| Prop        | Type     | Default | Description                                      |
| ----------- | -------- | ------- | ------------------------------------------------ |
| `className` | `string` | -       | Additional classes to apply to the content slot. |

### MessageHeader

Displays content above the message, such as a sender name. Stays aligned to the start regardless of `align`.

| Prop        | Type     | Default | Description                                |
| ----------- | -------- | ------- | ------------------------------------------ |
| `className` | `string` | -       | Additional classes to apply to the header. |

### MessageFooter

Displays content below the message, such as status or actions. Aligns to the message side.

| Prop        | Type     | Default | Description                                |
| ----------- | -------- | ------- | ------------------------------------------ |
| `className` | `string` | -       | Additional classes to apply to the footer. |
