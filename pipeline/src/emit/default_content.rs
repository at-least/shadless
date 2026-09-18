//! Port of pipeline/default_content.go — Code generated from
//! src/emitter/index.mjs DEFAULT_CONTENT. Entry.set marks the key
//! present-and-null (the Go equivalent of JS undefined-vs-null).
//! escHtml is applied at data construction; identity on this corpus's
//! plain words, kept for parity.

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub inner: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<(String, String)>,
    pub set: bool,
}

pub fn default_content() -> &'static HashMap<&'static str, HashMap<&'static str, Entry>> {
    static M: OnceLock<HashMap<&'static str, HashMap<&'static str, Entry>>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m: HashMap<&'static str, HashMap<&'static str, Entry>> = HashMap::new();
        let mut badge: HashMap<&'static str, Entry> = HashMap::new();
        let badge_e = Entry {
            inner: r##"Badge"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        badge.insert(r##"Badge"##, badge_e);
        m.insert(r##"badge"##, badge);
        let mut button: HashMap<&'static str, Entry> = HashMap::new();
        let button_e = Entry {
            inner: r##"Button"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        button.insert(r##"Button"##, button_e);
        m.insert(r##"button"##, button);
        let mut input: HashMap<&'static str, Entry> = HashMap::new();
        let input_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![("placeholder".to_string(), "Type here…".to_string())],
            children: vec![],
            set: true,
        };
        input.insert(r##"Input"##, input_e);
        m.insert(r##"input"##, input);
        let mut textarea: HashMap<&'static str, Entry> = HashMap::new();
        let textarea_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![("placeholder".to_string(), "Type here…".to_string())],
            children: vec![],
            set: true,
        };
        textarea.insert(r##"Textarea"##, textarea_e);
        m.insert(r##"textarea"##, textarea);
        let mut skeleton: HashMap<&'static str, Entry> = HashMap::new();
        let skeleton_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![("style".to_string(), "width:250px;height:1rem;display:block".to_string())],
            children: vec![],
            set: true,
        };
        skeleton.insert(r##"Skeleton"##, skeleton_e);
        m.insert(r##"skeleton"##, skeleton);
        let mut spinner: HashMap<&'static str, Entry> = HashMap::new();
        let spinner_e = Entry {
            inner: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1rem;height:1rem" aria-hidden="true"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        spinner.insert(r##"Spinner"##, spinner_e);
        m.insert(r##"spinner"##, spinner);
        let mut alert: HashMap<&'static str, Entry> = HashMap::new();
        let alert_e = Entry {
            inner: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg><div data-slot="alert-title">Heads up!</div><div data-slot="alert-description">You can add components to your app using the cli.</div>"##.to_string(),
            attrs: vec![("role".to_string(), "alert".to_string())],
            children: vec![],
            set: true,
        };
        alert.insert(r##"Alert"##, alert_e);
        let alert_title_e = Entry {
            inner: r##"Heads up!"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        alert.insert(r##"AlertTitle"##, alert_title_e);
        let alert_description_e = Entry {
            inner: r##"You can add components to your app using the cli."##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        alert.insert(r##"AlertDescription"##, alert_description_e);
        m.insert(r##"alert"##, alert);
        let mut attachment: HashMap<&'static str, Entry> = HashMap::new();
        let attachment_e = Entry {
            inner: r##"<div data-slot="attachment-media" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg></div><div data-slot="attachment-content" style="display:flex;flex-direction:column;gap:0.125rem"><span data-slot="attachment-title" style="font-weight:500">document.pdf</span><span data-slot="attachment-description" style="font-size:0.75rem;color:var(--muted-foreground)">2.4 MB</span></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"Attachment"##, attachment_e);
        let attachment_media_e = Entry {
            inner: r##"<div style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentMedia"##, attachment_media_e);
        let attachment_content_e = Entry {
            inner: r##"<span data-slot="attachment-title" style="font-weight:500">document.pdf</span><span data-slot="attachment-description" style="font-size:0.75rem;color:var(--muted-foreground)">2.4 MB</span>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentContent"##, attachment_content_e);
        let attachment_title_e = Entry {
            inner: r##"document.pdf"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentTitle"##, attachment_title_e);
        let attachment_description_e = Entry {
            inner: r##"2.4 MB"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentDescription"##, attachment_description_e);
        let attachment_actions_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentActions"##, attachment_actions_e);
        let attachment_trigger_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentTrigger"##, attachment_trigger_e);
        let attachment_group_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        attachment.insert(r##"AttachmentGroup"##, attachment_group_e);
        m.insert(r##"attachment"##, attachment);
        let mut breadcrumb: HashMap<&'static str, Entry> = HashMap::new();
        let breadcrumb_e = Entry {
            inner: r##"<ol data-slot="breadcrumb-list"><li data-slot="breadcrumb-item"><a data-slot="breadcrumb-link" href="#" style="transition:color">Home</a></li><li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" style="display:inline-flex;align-items:center;color:var(--muted-foreground)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:0.875rem;height:0.875rem"><path d="m9 18 6-6-6-6"></path></svg></li><li data-slot="breadcrumb-item"><a data-slot="breadcrumb-link" href="#">Components</a></li><li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" style="display:inline-flex;align-items:center;color:var(--muted-foreground)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:0.875rem;height:0.875rem"><path d="m9 18 6-6-6-6"></path></svg></li><li data-slot="breadcrumb-item"><span data-slot="breadcrumb-page" style="font-weight:normal;color:var(--foreground)">Breadcrumb</span></li></ol>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"Breadcrumb"##, breadcrumb_e);
        let breadcrumb_list_e = Entry {
            inner: r##"<li data-slot="breadcrumb-item"><a data-slot="breadcrumb-link" href="#">Home</a></li><li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true">/</li><li data-slot="breadcrumb-item"><span data-slot="breadcrumb-page">Current</span></li>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"BreadcrumbList"##, breadcrumb_list_e);
        let breadcrumb_item_e = Entry {
            inner: r##"<a data-slot="breadcrumb-link" href="#">Home</a>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"BreadcrumbItem"##, breadcrumb_item_e);
        let breadcrumb_link_e = Entry {
            inner: r##"Home"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"BreadcrumbLink"##, breadcrumb_link_e);
        let breadcrumb_page_e = Entry {
            inner: r##"Current"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"BreadcrumbPage"##, breadcrumb_page_e);
        let breadcrumb_separator_e = Entry {
            inner: r##"/"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"BreadcrumbSeparator"##, breadcrumb_separator_e);
        let breadcrumb_ellipsis_e = Entry {
            inner: r##"…"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        breadcrumb.insert(r##"BreadcrumbEllipsis"##, breadcrumb_ellipsis_e);
        m.insert(r##"breadcrumb"##, breadcrumb);
        let mut bubble: HashMap<&'static str, Entry> = HashMap::new();
        let bubble_group_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        bubble.insert(r##"BubbleGroup"##, bubble_group_e);
        let bubble_e = Entry {
            inner: r##"<div data-slot="bubble-content" style="display:inline-block;border-radius:1rem;padding:0.5rem 0.75rem;background:var(--muted)">Did you remove the stale route?</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        bubble.insert(r##"Bubble"##, bubble_e);
        let bubble_content_e = Entry {
            inner: r##"Did you remove the stale route?"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        bubble.insert(r##"BubbleContent"##, bubble_content_e);
        let bubble_reactions_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        bubble.insert(r##"BubbleReactions"##, bubble_reactions_e);
        m.insert(r##"bubble"##, bubble);
        let mut card: HashMap<&'static str, Entry> = HashMap::new();
        let card_e = Entry {
            inner: r##"<div data-slot="card-header"><div data-slot="card-title" style="font-weight:600">Create project</div><div data-slot="card-description" style="font-size:0.875rem;color:var(--muted-foreground)">Deploy your new project in one-click.</div></div><div data-slot="card-content" style="margin-top:1rem"><p>Set up your project with our intuitive wizard.</p></div><div data-slot="card-footer" style="display:flex;justify-content:space-between;align-items:center;margin-top:1rem"><button>Cancel</button><button style="background:var(--primary);color:var(--primary-foreground);padding:0.375rem 0.75rem;border-radius:0.375rem">Deploy</button></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"Card"##, card_e);
        let card_header_e = Entry {
            inner: r##"<div data-slot="card-title" style="font-weight:600">Title</div><div data-slot="card-description" style="font-size:0.875rem;color:var(--muted-foreground)">Description</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"CardHeader"##, card_header_e);
        let card_title_e = Entry {
            inner: r##"Title"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"CardTitle"##, card_title_e);
        let card_description_e = Entry {
            inner: r##"Description"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"CardDescription"##, card_description_e);
        let card_action_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"CardAction"##, card_action_e);
        let card_content_e = Entry {
            inner: r##"Content"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"CardContent"##, card_content_e);
        let card_footer_e = Entry {
            inner: r##"Footer"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        card.insert(r##"CardFooter"##, card_footer_e);
        m.insert(r##"card"##, card);
        let mut direction: HashMap<&'static str, Entry> = HashMap::new();
        let direction_provider_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        direction.insert(r##"DirectionProvider"##, direction_provider_e);
        m.insert(r##"direction"##, direction);
        let mut empty: HashMap<&'static str, Entry> = HashMap::new();
        let empty_e = Entry {
            inner: r##"<div data-slot="empty-header"><div data-slot="empty-icon" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted);margin-bottom:0.75rem"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></svg></div><div data-slot="empty-title" style="font-weight:600">No results</div><div data-slot="empty-description" style="font-size:0.875rem;color:var(--muted-foreground)">Try adjusting your search or filters.</div></div><div data-slot="empty-content" style="margin-top:0.75rem"><button style="background:var(--primary);color:var(--primary-foreground);padding:0.375rem 0.75rem;border-radius:0.375rem">Clear filters</button></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        empty.insert(r##"Empty"##, empty_e);
        let empty_header_e = Entry {
            inner: r##"<div data-slot="empty-icon" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted);margin-bottom:0.5rem">⌕</div><div data-slot="empty-title" style="font-weight:600">No results</div><div data-slot="empty-description" style="font-size:0.875rem;color:var(--muted-foreground)">Adjust your search.</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        empty.insert(r##"EmptyHeader"##, empty_header_e);
        let empty_media_e = Entry {
            inner: r##"⌕"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        empty.insert(r##"EmptyMedia"##, empty_media_e);
        let empty_title_e = Entry {
            inner: r##"No results"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        empty.insert(r##"EmptyTitle"##, empty_title_e);
        let empty_description_e = Entry {
            inner: r##"Try adjusting your search or filters."##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        empty.insert(r##"EmptyDescription"##, empty_description_e);
        let empty_content_e = Entry {
            inner: r##"Content"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        empty.insert(r##"EmptyContent"##, empty_content_e);
        m.insert(r##"empty"##, empty);
        let mut input_group: HashMap<&'static str, Entry> = HashMap::new();
        let input_group_e = Entry {
            inner: r##"<div data-slot="input-group-addon" style="display:flex;align-items:center;padding:0 0.75rem;border:1px solid var(--input);border-right:0;border-radius:0.375rem 0 0 0.375rem;background:var(--muted);color:var(--muted-foreground)">@</div><input data-slot="input-group-control" placeholder="Username" style="border-radius:0;border-left:0;border-right:0"><div data-slot="input-group-addon" style="display:flex;align-items:center;padding:0 0.75rem;border:1px solid var(--input);border-left:0;border-radius:0 0.375rem 0.375rem 0;background:var(--muted);color:var(--muted-foreground)">@example.com</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        input_group.insert(r##"InputGroup"##, input_group_e);
        let input_group_addon_e = Entry {
            inner: r##"@"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        input_group.insert(r##"InputGroupAddon"##, input_group_addon_e);
        let input_group_button_e = Entry {
            inner: r##"Button"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        input_group.insert(r##"InputGroupButton"##, input_group_button_e);
        let input_group_text_e = Entry {
            inner: r##"Text"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        input_group.insert(r##"InputGroupText"##, input_group_text_e);
        let input_group_input_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![("placeholder".to_string(), "Type here…".to_string())],
            children: vec![],
            set: true,
        };
        input_group.insert(r##"InputGroupInput"##, input_group_input_e);
        let input_group_textarea_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![("placeholder".to_string(), "Type here…".to_string())],
            children: vec![],
            set: true,
        };
        input_group.insert(r##"InputGroupTextarea"##, input_group_textarea_e);
        m.insert(r##"input-group"##, input_group);
        let mut item: HashMap<&'static str, Entry> = HashMap::new();
        let item_group_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemGroup"##, item_group_e);
        let item_separator_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemSeparator"##, item_separator_e);
        let item_e = Entry {
            inner: r##"<div data-slot="item-media" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><circle cx="12" cy="12" r="10"></circle></svg></div><div data-slot="item-content" style="display:flex;flex-direction:column;gap:0.125rem"><div data-slot="item-title" style="font-weight:500">Item title</div><p data-slot="item-description" style="font-size:0.875rem;color:var(--muted-foreground);margin:0">Item description.</p></div><div data-slot="item-actions" style="display:flex;align-items:center;gap:0.25rem"><button style="padding:0.25rem 0.5rem;border-radius:0.375rem;font-size:0.875rem">Edit</button></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"Item"##, item_e);
        let item_media_e = Entry {
            inner: r##"<div style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem"><circle cx="12" cy="12" r="10"></circle></svg></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemMedia"##, item_media_e);
        let item_content_e = Entry {
            inner: r##"<div data-slot="item-title" style="font-weight:500">Title</div><p data-slot="item-description" style="font-size:0.875rem;color:var(--muted-foreground);margin:0">Description</p>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemContent"##, item_content_e);
        let item_title_e = Entry {
            inner: r##"Title"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemTitle"##, item_title_e);
        let item_description_e = Entry {
            inner: r##"Description"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemDescription"##, item_description_e);
        let item_actions_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemActions"##, item_actions_e);
        let item_header_e = Entry {
            inner: r##"Header"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemHeader"##, item_header_e);
        let item_footer_e = Entry {
            inner: r##"Footer"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        item.insert(r##"ItemFooter"##, item_footer_e);
        m.insert(r##"item"##, item);
        let mut kbd: HashMap<&'static str, Entry> = HashMap::new();
        let kbd_e = Entry {
            inner: r##"⌘<span style="margin:0 0.25rem">+</span>K"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        kbd.insert(r##"Kbd"##, kbd_e);
        let kbd_group_e = Entry {
            inner: r##"<kbd data-slot="kbd">⌘</kbd><kbd data-slot="kbd">⇧</kbd><kbd data-slot="kbd">K</kbd>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        kbd.insert(r##"KbdGroup"##, kbd_group_e);
        m.insert(r##"kbd"##, kbd);
        let mut marker: HashMap<&'static str, Entry> = HashMap::new();
        let marker_e = Entry {
            inner: r##"<span data-slot="marker-icon" style="display:inline-flex;align-items:center;justify-content:center;width:1.25rem;height:1.25rem;border-radius:9999px;background:var(--destructive);color:white;font-size:0.75rem">1</span><span data-slot="marker-content">New</span>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        marker.insert(r##"Marker"##, marker_e);
        let marker_icon_e = Entry {
            inner: r##"1"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        marker.insert(r##"MarkerIcon"##, marker_icon_e);
        let marker_content_e = Entry {
            inner: r##"New"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        marker.insert(r##"MarkerContent"##, marker_content_e);
        m.insert(r##"marker"##, marker);
        let mut native_select: HashMap<&'static str, Entry> = HashMap::new();
        let native_select_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![("native-select".to_string(), "<option>Choose a fruit</option><option>Apple</option><option>Banana</option><option>Blueberry</option>".to_string())],
            set: true,
        };
        native_select.insert(r##"NativeSelect"##, native_select_e);
        let native_select_option_e = Entry {
            inner: r##"Option"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        native_select.insert(r##"NativeSelectOption"##, native_select_option_e);
        let native_select_opt_group_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        native_select.insert(r##"NativeSelectOptGroup"##, native_select_opt_group_e);
        m.insert(r##"native-select"##, native_select);
        let mut pagination: HashMap<&'static str, Entry> = HashMap::new();
        let pagination_e = Entry {
            inner: r##"<ul data-slot="pagination-content"><li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-label="Previous">‹</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">1</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-current="page" data-active="true">2</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">3</a></li><li data-slot="pagination-item"><span data-slot="pagination-ellipsis">…</span></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-label="Next">›</a></li></ul>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"Pagination"##, pagination_e);
        let pagination_content_e = Entry {
            inner: r##"<li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-current="page" data-active="true">1</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">2</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">3</a></li>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"PaginationContent"##, pagination_content_e);
        let pagination_item_e = Entry {
            inner: r##"<a data-slot="pagination-link" href="#">1</a>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"PaginationItem"##, pagination_item_e);
        let pagination_link_e = Entry {
            inner: r##"1"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"PaginationLink"##, pagination_link_e);
        let pagination_previous_e = Entry {
            inner: r##"‹"##.to_string(),
            attrs: vec![("href".to_string(), "#".to_string()), ("aria-label".to_string(), "Go to the previous page".to_string())],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"PaginationPrevious"##, pagination_previous_e);
        let pagination_next_e = Entry {
            inner: r##"›"##.to_string(),
            attrs: vec![("href".to_string(), "#".to_string()), ("aria-label".to_string(), "Go to the next page".to_string())],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"PaginationNext"##, pagination_next_e);
        let pagination_ellipsis_e = Entry {
            inner: r##"…"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        pagination.insert(r##"PaginationEllipsis"##, pagination_ellipsis_e);
        m.insert(r##"pagination"##, pagination);
        let mut table: HashMap<&'static str, Entry> = HashMap::new();
        let table_e = Entry {
            inner: r##"<table data-slot="table" style="width:100%;caption-side:bottom;font-size:0.875rem"><thead data-slot="table-header"><tr data-slot="table-row"><th data-slot="table-head" style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)">Name</th><th data-slot="table-head" style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)">Status</th><th data-slot="table-head" style="text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)">Amount</th></tr></thead><tbody data-slot="table-body"><tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Alice</td><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Active</td><td data-slot="table-cell" style="text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)">$250</td></tr><tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Bob</td><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Inactive</td><td data-slot="table-cell" style="text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)">$150</td></tr></tbody></table>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"Table"##, table_e);
        let table_header_e = Entry {
            inner: r##"<tr data-slot="table-row"><th data-slot="table-head" style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)">Header</th></tr>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableHeader"##, table_header_e);
        let table_body_e = Entry {
            inner: r##"<tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Cell</td></tr>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableBody"##, table_body_e);
        let table_footer_e = Entry {
            inner: r##"<tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem">Footer</td></tr>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableFooter"##, table_footer_e);
        let table_row_e = Entry {
            inner: r##"<td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Cell</td>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableRow"##, table_row_e);
        let table_head_e = Entry {
            inner: r##"Header"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableHead"##, table_head_e);
        let table_cell_e = Entry {
            inner: r##"Cell"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableCell"##, table_cell_e);
        let table_caption_e = Entry {
            inner: r##"Caption"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        table.insert(r##"TableCaption"##, table_caption_e);
        m.insert(r##"table"##, table);
        let mut button_group: HashMap<&'static str, Entry> = HashMap::new();
        let button_group_e = Entry {
            inner: r##"<button style="display:inline-flex;align-items:center;border:1px solid var(--border);border-radius:0.375rem;padding:0.25rem 0.75rem;font-size:0.875rem;background:transparent">Text</button><div data-slot="button-group-separator" style="display:inline-block;width:1px;height:1.25rem;background:var(--input)"></div><button style="display:inline-flex;align-items:center;justify-content:center;width:2.25rem;height:2.25rem;border:1px solid var(--border);border-radius:0.375rem;background:transparent" aria-label="Add">+</button>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        button_group.insert(r##"ButtonGroup"##, button_group_e);
        let button_group_text_e = Entry {
            inner: r##"Text"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        button_group.insert(r##"ButtonGroupText"##, button_group_text_e);
        let button_group_separator_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        button_group.insert(r##"ButtonGroupSeparator"##, button_group_separator_e);
        m.insert(r##"button-group"##, button_group);
        let mut message: HashMap<&'static str, Entry> = HashMap::new();
        let message_group_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message.insert(r##"MessageGroup"##, message_group_e);
        let message_e = Entry {
            inner: r##"<div data-slot="message-avatar" style="display:flex;align-items:flex-start;gap:0.75rem"><span style="display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:9999px;background:var(--muted);font-size:0.875rem;font-weight:500">CN</span></div><div data-slot="message-content" style="display:flex;flex-direction:column;gap:0.25rem"><div data-slot="message-header" style="font-size:0.875rem;font-weight:600">Header</div><div data-slot="message-footer" style="font-size:0.75rem;color:var(--muted-foreground)">Footer</div></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message.insert(r##"Message"##, message_e);
        let message_avatar_e = Entry {
            inner: r##"<span style="display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:9999px;background:var(--muted);font-size:0.875rem;font-weight:500">CN</span>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message.insert(r##"MessageAvatar"##, message_avatar_e);
        let message_content_e = Entry {
            inner: r##"<div data-slot="message-header" style="font-weight:600">Header</div><div data-slot="message-footer" style="font-size:0.75rem;color:var(--muted-foreground)">Footer</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message.insert(r##"MessageContent"##, message_content_e);
        let message_header_e = Entry {
            inner: r##"Header"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message.insert(r##"MessageHeader"##, message_header_e);
        let message_footer_e = Entry {
            inner: r##"Footer"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message.insert(r##"MessageFooter"##, message_footer_e);
        m.insert(r##"message"##, message);
        let mut message_scroller: HashMap<&'static str, Entry> = HashMap::new();
        let message_scroller_provider_e = Entry {
            inner: r##""##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message_scroller.insert(r##"MessageScrollerProvider"##, message_scroller_provider_e);
        let message_scroller_e = Entry {
            inner: r##"<div data-slot="message-scroller-viewport" style="height:160px;overflow:hidden;border:1px solid var(--border);border-radius:0.5rem;padding:0.75rem;background:color-mix(in oklab, var(--muted) 30%, transparent)"><div data-slot="message-scroller-content"><div data-slot="message-scroller-item" style="margin-bottom:0.5rem">Top message</div><div data-slot="message-scroller-item" style="margin-bottom:0.5rem;margin-top:3rem">Middle message</div><div data-slot="message-scroller-item" style="margin-top:6rem">Bottom message</div></div></div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message_scroller.insert(r##"MessageScroller"##, message_scroller_e);
        let message_scroller_viewport_e = Entry {
            inner: r##"<div style="padding:0.75rem">Scrollable content</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message_scroller.insert(r##"MessageScrollerViewport"##, message_scroller_viewport_e);
        let message_scroller_content_e = Entry {
            inner: r##"<div style="padding:0.75rem">Item content</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message_scroller.insert(r##"MessageScrollerContent"##, message_scroller_content_e);
        let message_scroller_item_e = Entry {
            inner: r##"<div style="padding:0.5rem;border:1px solid var(--border);border-radius:0.375rem">Item</div>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message_scroller.insert(r##"MessageScrollerItem"##, message_scroller_item_e);
        let message_scroller_button_e = Entry {
            inner: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1rem;height:1rem"><path d="M12 5v14M5 12l7 7 7-7"></path></svg>"##.to_string(),
            attrs: vec![],
            children: vec![],
            set: true,
        };
        message_scroller.insert(r##"MessageScrollerButton"##, message_scroller_button_e);
        m.insert(r##"message-scroller"##, message_scroller);
        m
    })
}
