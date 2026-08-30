---
title: "Attachment"
description: "Displays a file or image attachment with media, metadata, upload state, and actions."
---

# Attachment

Displays a file or image attachment with media, metadata, upload state, and actions.

<iframe class="demo" src="/demos/attachment-demo.html" title="attachment-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

The `Attachment` component displays a file or image attachment, its media, name, and metadata, with optional actions and upload state. Use it for files and images in chat composers, message threads, and upload lists.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/attachment.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/attachment.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/attachment.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                   into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/attachment.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
| `orientation="outline"` (JSX prop) | `data-orientation="outline"` (markup) |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
attachment
├── attachment-media
├── attachment-content
│   ├── attachment-title
│   └── attachment-description
├── attachment-actions
│   └── attachment-action
└── attachment-trigger
```

## Features

- Icon and image media through `AttachmentMedia`
- Upload states: `idle`, `uploading`, `processing`, `error`, and `done` with built-in styling and a shimmer while in progress
- Three sizes and horizontal or vertical orientation
- A full-card `AttachmentTrigger` that opens a link or dialog while the actions stay independently clickable
- Scrollable, snapping `AttachmentGroup` with an edge fade
- Customizable styling through the `className` prop on every part

## Image

Set `variant="image"` on `AttachmentMedia` and render an `<img>` inside it. Use `orientation="vertical"` to stack the media above the content.

<iframe class="demo" src="/demos/attachment-image.html" title="attachment-image" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## States

Set `state` to reflect the upload lifecycle. `uploading` and `processing` shimmer the title, and `error` switches to a destructive treatment.

<iframe class="demo" src="/demos/attachment-states.html" title="attachment-states" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Sizes

Use `size` to switch between `default`, `sm`, and `xs`.

<iframe class="demo" src="/demos/attachment-sizes.html" title="attachment-sizes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Group

Wrap attachments in `AttachmentGroup` to lay them out in a horizontally scrollable, snapping row with an edge fade.

<iframe class="demo" src="/demos/attachment-group.html" title="attachment-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Trigger

Add an `AttachmentTrigger` to make the whole card open a link or dialog. It fills the card behind the actions, so the actions stay clickable.

<iframe class="demo" src="/demos/attachment-trigger.html" title="attachment-trigger" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

```tsx showLineNumbers
<Dialog>
  <Attachment>
    {/* media, content, actions */}
    <DialogTrigger asChild>
      <AttachmentTrigger aria-label="Preview research-summary.pdf" />
    </DialogTrigger>
  </Attachment>
  <DialogContent>{/* ... */}</DialogContent>
</Dialog>
```

## Accessibility

`AttachmentAction` renders a `Button`, and `AttachmentTrigger` renders a real `<button>` (or your element via `asChild`). Follow the guidance below so both are operable and announced.

### Label icon-only actions

`AttachmentAction` is usually icon-only, so give each one an `aria-label` describing the action and its target.

```tsx showLineNumbers
<AttachmentAction aria-label="Remove sales-dashboard.pdf">
  <XIcon />
</AttachmentAction>
```

### Label the trigger

`AttachmentTrigger` covers the card with no text of its own, so give it an `aria-label` for what activating it does.

```tsx showLineNumbers
<AttachmentTrigger asChild>
  <a
    href={url}
    target="_blank"
    rel="noreferrer"
    aria-label="Open workspace.png"
  />
</AttachmentTrigger>
```

The trigger sits behind the actions in the stacking order, so an `AttachmentAction` and the `AttachmentTrigger` never trap each other — both remain separately focusable and clickable.

### Keyboard scrolling

An `AttachmentGroup` scrolls horizontally. When its attachments are interactive: a trigger or actions, keyboard users reach off-screen items by tabbing to them. For a row of presentational attachments, make the group itself focusable and scrollable by adding `tabIndex={0}`, `role="group"`, and an `aria-label`.

### Meaning beyond color

The `error` state uses a destructive color. Keep the failure reason in `AttachmentDescription` so the state is not conveyed by color alone.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="attachment"` |
| `data-slot="attachment-media"` |
| `data-slot="attachment-content"` |
| `data-slot="attachment-title"` |
| `data-slot="attachment-description"` |
| `data-slot="attachment-actions"` |
| `data-slot="attachment-action"` |
| `data-slot="attachment-trigger"` |
| `data-slot="attachment-group"` |

### Attachment

The root attachment container.

| Prop          | Type                                                         | Default        | Description                                       |
| ------------- | ------------------------------------------------------------ | -------------- | ------------------------------------------------- |
| `state`       | `"idle" \| "uploading" \| "processing" \| "error" \| "done"` | `"done"`       | The upload state. Drives styling and the shimmer. |
| `size`        | `"default" \| "sm" \| "xs"`                                  | `"default"`    | The attachment size.                              |
| `orientation` | `"horizontal" \| "vertical"`                                 | `"horizontal"` | Lay the media beside or above the content.        |
| `className`   | `string`                                                     | -              | Additional classes to apply to the root element.  |

### AttachmentMedia

The media slot for an icon or image preview.

| Prop        | Type                | Default  | Description                                    |
| ----------- | ------------------- | -------- | ---------------------------------------------- |
| `variant`   | `"icon" \| "image"` | `"icon"` | Whether the media holds an icon or an `<img>`. |
| `className` | `string`            | -        | Additional classes to apply to the media slot. |

### AttachmentContent

Wraps the title and description.

| Prop        | Type     | Default | Description                                      |
| ----------- | -------- | ------- | ------------------------------------------------ |
| `className` | `string` | -       | Additional classes to apply to the content slot. |

### AttachmentTitle

The attachment name. Shimmers while the attachment is `uploading` or `processing`.

| Prop        | Type     | Default | Description                               |
| ----------- | -------- | ------- | ----------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the title. |

### AttachmentDescription

Secondary metadata such as the file type, size, or upload status.

| Prop        | Type     | Default | Description                                     |
| ----------- | -------- | ------- | ----------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the description. |

### AttachmentActions

A container for one or more actions, aligned to the end of the attachment.

| Prop        | Type     | Default | Description                                 |
| ----------- | -------- | ------- | ------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the actions. |

### AttachmentAction

An action button. Renders a [        ](/components/button) and accepts all of its props.

| Prop       | Type                                  | Default     | Description                              |
| ---------- | ------------------------------------- | ----------- | ---------------------------------------- |
| `size`     | `Button["size"]`                      | `"icon-xs"` | The button size.                         |
| `...props` | `React.ComponentProps<typeof Button>` | -           | Props spread to the underlying `Button`. |

### AttachmentTrigger

A full-card overlay that activates the attachment. Renders a `<button>` by default.

| Prop       | Type                             | Default | Description                                  |
| ---------- | -------------------------------- | ------- | -------------------------------------------- |
| `asChild`  | `boolean`                        | `false` | Render as the child element, such as a link. |
| `...props` | `React.ComponentProps<"button">` | -       | Props spread to the trigger element.         |

### AttachmentGroup

Lays out attachments in a horizontally scrollable, snapping row.

| Prop        | Type     | Default | Description                               |
| ----------- | -------- | ------- | ----------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the group. |
