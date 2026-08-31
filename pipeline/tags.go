package main

import (
	"regexp"
	"strings"
)

// Tag normalization, shared by converter / emitter / css — the Go twin of
// src/tags.mjs. Resolution order:
//
//	1. native tag            → itself
//	2. <ternary:A/B>         → alternate B (asChild ? Slot.Root : "div"; the
//	                           Slot branch is React-only)
//	3. ir.tagHints[tag]      → explicit converter hint
//	4. anything else         → false (UNRESOLVED — callers must fail loudly)
var nativeTags = map[string]bool{}

func init() {
	for _, t := range strings.Fields(`div span p a button h1 h2 h3 h4 h5 h6
ul ol li nav table thead tbody tfoot tr th td caption
input select option optgroup textarea label form img svg path
circle line rect polygon polyline ellipse g defs use
section header footer main article aside small strong em kbd
dl dt dd fieldset legend output datalist meter progress
details summary picture time mark sub sup i b u s
abbr address hgroup dialog search blockquote code pre
template style script title head body html
figure figcaption`) {
		nativeTags[t] = true
	}
}

// voidTags: HTML5 void elements — never emit a closing tag.
var voidTags = map[string]bool{}

func init() {
	for _, t := range strings.Fields("br hr img input meta link area base col embed source track wbr") {
		voidTags[t] = true
	}
}

var knownMemberTags = map[string]string{
	"LabelPrimitive.Root": "label", // @radix-ui/react-label renders <label>
}

func externalMemberTag(tag string) string {
	if v, ok := knownMemberTags[tag]; ok {
		return v
	}
	suffix := tag[strings.LastIndex(tag, ".")+1:]
	switch suffix {
	case "Button", "Trigger", "Link":
		return "button"
	}
	return "div"
}

var kebabBoundary = regexp.MustCompile(`([a-z0-9])([A-Z])`)
var kebabSpace = regexp.MustCompile(`[\s.]+`)

func kebab(s string) string {
	return strings.ToLower(kebabSpace.ReplaceAllString(
		kebabBoundary.ReplaceAllString(s, "$1-$2"), "-"))
}

var ternaryRe = regexp.MustCompile(`^<ternary:([^/]+)/(.+)>$`)

// normalizeTag resolves a raw IR tag to a native tag. Returns "" when the
// tag is unresolvable — the caller (a converter or the emitter) must fail
// loudly; the old silent <button> coercion is the bug this pins down.
func normalizeTag(tag string, hints map[string]string) (string, bool) {
	if tag == "" {
		return "", false
	}
	if nativeTags[tag] {
		return tag, true
	}
	if m := ternaryRe.FindStringSubmatch(tag); m != nil {
		return normalizeTag(m[2], hints)
	}
	if h, ok := hints[tag]; ok {
		if nativeTags[h] {
			return h, true
		}
		return "", false
	}
	return "", false
}
