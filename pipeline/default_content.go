// Code generated from src/emitter/index.mjs DEFAULT_CONTENT. DO NOT EDIT;
// regenerate with tools/default-content-dump.mjs.
package main

// defaultContentEntry mirrors one entry of DEFAULT_CONTENT. Set marks the key
// present-and-null (the Go equivalent of JS undefined-vs-null).
type defaultContentEntry struct {
	Inner    string
	Attrs    map[string]string
	Children map[string]string
	Set      bool
}

var DEFAULT_CONTENT = map[string]map[string]defaultContentEntry{
	"badge": {
		"Badge": {Set: true, Inner: escHtml("Badge")},
	},
	"button": {
		"Button": {Set: true, Inner: escHtml("Button")},
	},
	"input": {
		"Input": {Set: true, Attrs: map[string]string{"placeholder": "Type here…"}},
	},
	"textarea": {
		"Textarea": {Set: true, Attrs: map[string]string{"placeholder": "Type here…"}},
	},
	"skeleton": {
		"Skeleton": {Set: true, Attrs: map[string]string{"style": "width:250px;height:1rem;display:block"}},
	},
	"spinner": {
		"Spinner": {Set: true, Inner: "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1rem;height:1rem\" aria-hidden=\"true\"><path d=\"M21 12a9 9 0 1 1-6.219-8.56\"/></svg>"},
	},
	"alert": {
		"Alert": {Set: true, Inner: "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" aria-hidden=\"true\"><circle cx=\"12\" cy=\"12\" r=\"10\"></circle><line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"12\"></line><line x1=\"12\" y1=\"16\" x2=\"12.01\" y2=\"16\"></line></svg><div data-slot=\"alert-title\">Heads up!</div><div data-slot=\"alert-description\">You can add components to your app using the cli.</div>", Attrs: map[string]string{"role": "alert"}},
		"AlertTitle": {Set: true, Inner: escHtml("Heads up!")},
		"AlertDescription": {Set: true, Inner: escHtml("You can add components to your app using the cli.")},
	},
	"attachment": {
		"Attachment": {Set: true, Inner: "<div data-slot=\"attachment-media\" style=\"display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1.25rem;height:1.25rem\" aria-hidden=\"true\"><path d=\"M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48\"></path></svg></div><div data-slot=\"attachment-content\" style=\"display:flex;flex-direction:column;gap:0.125rem\"><span data-slot=\"attachment-title\" style=\"font-weight:500\">document.pdf</span><span data-slot=\"attachment-description\" style=\"font-size:0.75rem;color:var(--muted-foreground)\">2.4 MB</span></div>"},
		"AttachmentMedia": {Set: true, Inner: "<div style=\"display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1.25rem;height:1.25rem\" aria-hidden=\"true\"><path d=\"M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48\"></path></svg></div>"},
		"AttachmentContent": {Set: true, Inner: "<span data-slot=\"attachment-title\" style=\"font-weight:500\">document.pdf</span><span data-slot=\"attachment-description\" style=\"font-size:0.75rem;color:var(--muted-foreground)\">2.4 MB</span>"},
		"AttachmentTitle": {Set: true, Inner: escHtml("document.pdf")},
		"AttachmentDescription": {Set: true, Inner: escHtml("2.4 MB")},
		"AttachmentActions": {Set: true},
		"AttachmentTrigger": {Set: true},
		"AttachmentGroup": {Set: true},
	},
	"breadcrumb": {
		"Breadcrumb": {Set: true, Inner: "<ol data-slot=\"breadcrumb-list\"><li data-slot=\"breadcrumb-item\"><a data-slot=\"breadcrumb-link\" href=\"#\" style=\"transition:color;hover:{color:var(--foreground)}\">Home</a></li><li data-slot=\"breadcrumb-separator\" role=\"presentation\" aria-hidden=\"true\" style=\"display:inline-flex;align-items:center;color:var(--muted-foreground)\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:0.875rem;height:0.875rem\"><path d=\"m9 18 6-6-6-6\"></path></svg></li><li data-slot=\"breadcrumb-item\"><a data-slot=\"breadcrumb-link\" href=\"#\">Components</a></li><li data-slot=\"breadcrumb-separator\" role=\"presentation\" aria-hidden=\"true\" style=\"display:inline-flex;align-items:center;color:var(--muted-foreground)\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:0.875rem;height:0.875rem\"><path d=\"m9 18 6-6-6-6\"></path></svg></li><li data-slot=\"breadcrumb-item\"><span data-slot=\"breadcrumb-page\" style=\"font-weight:normal;color:var(--foreground)\">Breadcrumb</span></li></ol>"},
		"BreadcrumbList": {Set: true, Inner: "<li data-slot=\"breadcrumb-item\"><a data-slot=\"breadcrumb-link\" href=\"#\">Home</a></li><li data-slot=\"breadcrumb-separator\" role=\"presentation\" aria-hidden=\"true\">/</li><li data-slot=\"breadcrumb-item\"><span data-slot=\"breadcrumb-page\">Current</span></li>"},
		"BreadcrumbItem": {Set: true, Inner: "<a data-slot=\"breadcrumb-link\" href=\"#\">Home</a>"},
		"BreadcrumbLink": {Set: true, Inner: escHtml("Home")},
		"BreadcrumbPage": {Set: true, Inner: escHtml("Current")},
		"BreadcrumbSeparator": {Set: true, Inner: escHtml("/")},
		"BreadcrumbEllipsis": {Set: true, Inner: escHtml("…")},
	},
	"bubble": {
		"BubbleGroup": {Set: true},
		"Bubble": {Set: true, Inner: "<div data-slot=\"bubble-content\" style=\"display:inline-block;border-radius:1rem;padding:0.5rem 0.75rem;background:var(--muted)\">Did you remove the stale route?</div>"},
		"BubbleContent": {Set: true, Inner: escHtml("Did you remove the stale route?")},
		"BubbleReactions": {Set: true},
	},
	"card": {
		"Card": {Set: true, Inner: "<div data-slot=\"card-header\"><div data-slot=\"card-title\" style=\"font-weight:600\">Create project</div><div data-slot=\"card-description\" style=\"font-size:0.875rem;color:var(--muted-foreground)\">Deploy your new project in one-click.</div></div><div data-slot=\"card-content\" style=\"margin-top:1rem\"><p>Set up your project with our intuitive wizard.</p></div><div data-slot=\"card-footer\" style=\"display:flex;justify-content:space-between;align-items:center;margin-top:1rem\"><button>Cancel</button><button style=\"background:var(--primary);color:var(--primary-foreground);padding:0.375rem 0.75rem;border-radius:0.375rem\">Deploy</button></div>"},
		"CardHeader": {Set: true, Inner: "<div data-slot=\"card-title\" style=\"font-weight:600\">Title</div><div data-slot=\"card-description\" style=\"font-size:0.875rem;color:var(--muted-foreground)\">Description</div>"},
		"CardTitle": {Set: true, Inner: escHtml("Title")},
		"CardDescription": {Set: true, Inner: escHtml("Description")},
		"CardAction": {Set: true},
		"CardContent": {Set: true, Inner: escHtml("Content")},
		"CardFooter": {Set: true, Inner: escHtml("Footer")},
	},
	"direction": {
		"DirectionProvider": {Set: true},
	},
	"empty": {
		"Empty": {Set: true, Inner: "<div data-slot=\"empty-header\"><div data-slot=\"empty-icon\" style=\"display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted);margin-bottom:0.75rem\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1.25rem;height:1.25rem\" aria-hidden=\"true\"><circle cx=\"11\" cy=\"11\" r=\"8\"></circle><path d=\"m21 21-4.3-4.3\"></path></svg></div><div data-slot=\"empty-title\" style=\"font-weight:600\">No results</div><div data-slot=\"empty-description\" style=\"font-size:0.875rem;color:var(--muted-foreground)\">Try adjusting your search or filters.</div></div><div data-slot=\"empty-content\" style=\"margin-top:0.75rem\"><button style=\"background:var(--primary);color:var(--primary-foreground);padding:0.375rem 0.75rem;border-radius:0.375rem\">Clear filters</button></div>"},
		"EmptyHeader": {Set: true, Inner: "<div data-slot=\"empty-icon\" style=\"display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted);margin-bottom:0.5rem\">⌕</div><div data-slot=\"empty-title\" style=\"font-weight:600\">No results</div><div data-slot=\"empty-description\" style=\"font-size:0.875rem;color:var(--muted-foreground)\">Adjust your search.</div>"},
		"EmptyMedia": {Set: true, Inner: escHtml("⌕")},
		"EmptyTitle": {Set: true, Inner: escHtml("No results")},
		"EmptyDescription": {Set: true, Inner: escHtml("Try adjusting your search or filters.")},
		"EmptyContent": {Set: true, Inner: escHtml("Content")},
	},
	"input-group": {
		"InputGroup": {Set: true, Inner: "<div data-slot=\"input-group-addon\" style=\"display:flex;align-items:center;padding:0 0.75rem;border:1px solid var(--input);border-right:0;border-radius:0.375rem 0 0 0.375rem;background:var(--muted);color:var(--muted-foreground)\">@</div><input data-slot=\"input-group-control\" placeholder=\"Username\" style=\"border-radius:0;border-left:0;border-right:0\"><div data-slot=\"input-group-addon\" style=\"display:flex;align-items:center;padding:0 0.75rem;border:1px solid var(--input);border-left:0;border-radius:0 0.375rem 0.375rem 0;background:var(--muted);color:var(--muted-foreground)\">@example.com</div>"},
		"InputGroupAddon": {Set: true, Inner: escHtml("@")},
		"InputGroupButton": {Set: true, Inner: escHtml("Button")},
		"InputGroupText": {Set: true, Inner: escHtml("Text")},
		"InputGroupInput": {Set: true, Attrs: map[string]string{"placeholder": "Type here…"}},
		"InputGroupTextarea": {Set: true, Attrs: map[string]string{"placeholder": "Type here…"}},
	},
	"item": {
		"ItemGroup": {Set: true},
		"ItemSeparator": {Set: true},
		"Item": {Set: true, Inner: "<div data-slot=\"item-media\" style=\"display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1.25rem;height:1.25rem\" aria-hidden=\"true\"><circle cx=\"12\" cy=\"12\" r=\"10\"></circle></svg></div><div data-slot=\"item-content\" style=\"display:flex;flex-direction:column;gap:0.125rem\"><div data-slot=\"item-title\" style=\"font-weight:500\">Item title</div><p data-slot=\"item-description\" style=\"font-size:0.875rem;color:var(--muted-foreground);margin:0\">Item description.</p></div><div data-slot=\"item-actions\" style=\"display:flex;align-items:center;gap:0.25rem\"><button style=\"padding:0.25rem 0.5rem;border-radius:0.375rem;font-size:0.875rem\">Edit</button></div>"},
		"ItemMedia": {Set: true, Inner: "<div style=\"display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1.25rem;height:1.25rem\"><circle cx=\"12\" cy=\"12\" r=\"10\"></circle></svg></div>"},
		"ItemContent": {Set: true, Inner: "<div data-slot=\"item-title\" style=\"font-weight:500\">Title</div><p data-slot=\"item-description\" style=\"font-size:0.875rem;color:var(--muted-foreground);margin:0\">Description</p>"},
		"ItemTitle": {Set: true, Inner: escHtml("Title")},
		"ItemDescription": {Set: true, Inner: escHtml("Description")},
		"ItemActions": {Set: true},
		"ItemHeader": {Set: true, Inner: escHtml("Header")},
		"ItemFooter": {Set: true, Inner: escHtml("Footer")},
	},
	"kbd": {
		"Kbd": {Set: true, Inner: "⌘<span style=\"margin:0 0.25rem\">+</span>K"},
		"KbdGroup": {Set: true, Inner: "<kbd data-slot=\"kbd\">⌘</kbd><kbd data-slot=\"kbd\">⇧</kbd><kbd data-slot=\"kbd\">K</kbd>"},
	},
	"marker": {
		"Marker": {Set: true, Inner: "<span data-slot=\"marker-icon\" style=\"display:inline-flex;align-items:center;justify-content:center;width:1.25rem;height:1.25rem;border-radius:9999px;background:var(--destructive);color:white;font-size:0.75rem\">1</span><span data-slot=\"marker-content\">New</span>"},
		"MarkerIcon": {Set: true, Inner: escHtml("1")},
		"MarkerContent": {Set: true, Inner: escHtml("New")},
	},
	"native-select": {
		"NativeSelect": {Set: true, Children: map[string]string{"native-select": "<option>Choose a fruit</option><option>Apple</option><option>Banana</option><option>Blueberry</option>"}},
		"NativeSelectOption": {Set: true, Inner: escHtml("Option")},
		"NativeSelectOptGroup": {Set: true},
	},
	"pagination": {
		"Pagination": {Set: true, Inner: "<ul data-slot=\"pagination-content\"><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\" aria-label=\"Previous\">‹</a></li><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\">1</a></li><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\" aria-current=\"page\" data-active=\"true\">2</a></li><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\">3</a></li><li data-slot=\"pagination-item\"><span data-slot=\"pagination-ellipsis\">…</span></li><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\" aria-label=\"Next\">›</a></li></ul>"},
		"PaginationContent": {Set: true, Inner: "<li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\" aria-current=\"page\" data-active=\"true\">1</a></li><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\">2</a></li><li data-slot=\"pagination-item\"><a data-slot=\"pagination-link\" href=\"#\">3</a></li>"},
		"PaginationItem": {Set: true, Inner: "<a data-slot=\"pagination-link\" href=\"#\">1</a>"},
		"PaginationLink": {Set: true, Inner: escHtml("1")},
		"PaginationPrevious": {Set: true, Inner: "‹", Attrs: map[string]string{"href": "#", "aria-label": "Go to the previous page"}},
		"PaginationNext": {Set: true, Inner: "›", Attrs: map[string]string{"href": "#", "aria-label": "Go to the next page"}},
		"PaginationEllipsis": {Set: true, Inner: escHtml("…")},
	},
	"table": {
		"Table": {Set: true, Inner: "<table data-slot=\"table\" style=\"width:100%;caption-side:bottom;font-size:0.875rem\"><thead data-slot=\"table-header\"><tr data-slot=\"table-row\"><th data-slot=\"table-head\" style=\"text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)\">Name</th><th data-slot=\"table-head\" style=\"text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)\">Status</th><th data-slot=\"table-head\" style=\"text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)\">Amount</th></tr></thead><tbody data-slot=\"table-body\"><tr data-slot=\"table-row\"><td data-slot=\"table-cell\" style=\"padding:0.5rem;border-bottom:1px solid var(--border)\">Alice</td><td data-slot=\"table-cell\" style=\"padding:0.5rem;border-bottom:1px solid var(--border)\">Active</td><td data-slot=\"table-cell\" style=\"text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)\">$250</td></tr><tr data-slot=\"table-row\"><td data-slot=\"table-cell\" style=\"padding:0.5rem;border-bottom:1px solid var(--border)\">Bob</td><td data-slot=\"table-cell\" style=\"padding:0.5rem;border-bottom:1px solid var(--border)\">Inactive</td><td data-slot=\"table-cell\" style=\"text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)\">$150</td></tr></tbody></table>"},
		"TableHeader": {Set: true, Inner: "<tr data-slot=\"table-row\"><th data-slot=\"table-head\" style=\"text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)\">Header</th></tr>"},
		"TableBody": {Set: true, Inner: "<tr data-slot=\"table-row\"><td data-slot=\"table-cell\" style=\"padding:0.5rem;border-bottom:1px solid var(--border)\">Cell</td></tr>"},
		"TableFooter": {Set: true, Inner: "<tr data-slot=\"table-row\"><td data-slot=\"table-cell\" style=\"padding:0.5rem\">Footer</td></tr>"},
		"TableRow": {Set: true, Inner: "<td data-slot=\"table-cell\" style=\"padding:0.5rem;border-bottom:1px solid var(--border)\">Cell</td>"},
		"TableHead": {Set: true, Inner: escHtml("Header")},
		"TableCell": {Set: true, Inner: escHtml("Cell")},
		"TableCaption": {Set: true, Inner: escHtml("Caption")},
	},
	"button-group": {
		"ButtonGroup": {Set: true, Inner: "<button style=\"display:inline-flex;align-items:center;border:1px solid var(--border);border-radius:0.375rem;padding:0.25rem 0.75rem;font-size:0.875rem;background:transparent\">Text</button><div data-slot=\"button-group-separator\" style=\"display:inline-block;width:1px;height:1.25rem;background:var(--input)\"></div><button style=\"display:inline-flex;align-items:center;justify-content:center;width:2.25rem;height:2.25rem;border:1px solid var(--border);border-radius:0.375rem;background:transparent\" aria-label=\"Add\">+</button>"},
		"ButtonGroupText": {Set: true, Inner: escHtml("Text")},
		"ButtonGroupSeparator": {Set: true},
	},
	"message": {
		"MessageGroup": {Set: true},
		"Message": {Set: true, Inner: "<div data-slot=\"message-avatar\" style=\"display:flex;align-items:flex-start;gap:0.75rem\"><span style=\"display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:9999px;background:var(--muted);font-size:0.875rem;font-weight:500\">CN</span></div><div data-slot=\"message-content\" style=\"display:flex;flex-direction:column;gap:0.25rem\"><div data-slot=\"message-header\" style=\"font-size:0.875rem;font-weight:600\">Header</div><div data-slot=\"message-footer\" style=\"font-size:0.75rem;color:var(--muted-foreground)\">Footer</div></div>"},
		"MessageAvatar": {Set: true, Inner: "<span style=\"display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:9999px;background:var(--muted);font-size:0.875rem;font-weight:500\">CN</span>"},
		"MessageContent": {Set: true, Inner: "<div data-slot=\"message-header\" style=\"font-weight:600\">Header</div><div data-slot=\"message-footer\" style=\"font-size:0.75rem;color:var(--muted-foreground)\">Footer</div>"},
		"MessageHeader": {Set: true, Inner: escHtml("Header")},
		"MessageFooter": {Set: true, Inner: escHtml("Footer")},
	},
	"message-scroller": {
		"MessageScrollerProvider": {Set: true},
		"MessageScroller": {Set: true, Inner: "<div data-slot=\"message-scroller-viewport\" style=\"height:160px;overflow:hidden;border:1px solid var(--border);border-radius:0.5rem;padding:0.75rem;background:color-mix(in oklab, var(--muted) 30%, transparent)\"><div data-slot=\"message-scroller-content\"><div data-slot=\"message-scroller-item\" style=\"margin-bottom:0.5rem\">Top message</div><div data-slot=\"message-scroller-item\" style=\"margin-bottom:0.5rem;margin-top:3rem\">Middle message</div><div data-slot=\"message-scroller-item\" style=\"margin-top:6rem\">Bottom message</div></div></div>"},
		"MessageScrollerViewport": {Set: true, Inner: "<div style=\"padding:0.75rem\">Scrollable content</div>"},
		"MessageScrollerContent": {Set: true, Inner: "<div style=\"padding:0.75rem\">Item content</div>"},
		"MessageScrollerItem": {Set: true, Inner: "<div style=\"padding:0.5rem;border:1px solid var(--border);border-radius:0.375rem\">Item</div>"},
		"MessageScrollerButton": {Set: true, Inner: "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" style=\"width:1rem;height:1rem\"><path d=\"M12 5v14M5 12l7 7 7-7\"></path></svg>"},
	},
}
