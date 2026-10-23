//! The code-tab serializer. html5ever's stock serializer re-escapes `&`
//! (and `<`/`>`) inside attribute values (html5ever serialize/mod.rs
//! write_escaped — the arms are unguarded), which is exactly what stage 2a
//! removed from the shipped tabs: a consumer's Tailwind content scan
//! extracts nothing from `[&amp;_a]`. This module re-implements the stock
//! `Serializer` verbatim — the traversal stays rcdom's `Serialize` impl
//! (void elements via the ignore_children stack, raw-text elements via the
//! parent-html-name check, foreign-content attribute prefixes) — and
//! overrides ONLY attribute-value escaping:
//!
//! - `"` → `&quot;` (the value is double-quoted);
//! - `&` → `&amp;` ONLY where the next character is ASCII alphanumeric or
//!   `#` — the stage-2a rule. Decoded and re-parsed, that pair would start
//!   a character reference (`&amp;copy;` → `©`), so it must stay escaped;
//!   every other `&` round-trips raw, and a raw `&` is what a consumer's
//!   scan reads (`[&_a]:underline`). `&nbsp;` for U+00A0 is kept in both
//!   modes like the stock serializer;
//! - `<` and `>` are never escaped in attribute values: raw, they cannot
//!   reparse as markup inside a quoted value.
//!
//! Text nodes keep the standard escaping (`&` `<` `>` and nbsp).

use std::io::{self, Write};

use html5ever::serialize::{AttrRef, Serializer};
use html5ever::{LocalName, QualName, local_name, ns};
use markup5ever_rcdom::{Handle, NodeData};

struct ElemInfo {
    html_name: Option<LocalName>,
    ignore_children: bool,
}

pub struct TabSerializer<'w> {
    writer: &'w mut Vec<u8>,
    stack: Vec<ElemInfo>,
}

impl<'w> TabSerializer<'w> {
    pub fn new(writer: &'w mut Vec<u8>) -> Self {
        TabSerializer {
            writer,
            stack: vec![ElemInfo {
                html_name: None, // ChildrenOnly(None): no parent element
                ignore_children: false,
            }],
        }
    }

    fn parent(&mut self) -> &mut ElemInfo {
        self.stack.last_mut().expect("ElemInfo stack never empties")
    }

    fn write_escaped(&mut self, text: &str, attr_mode: bool) -> io::Result<()> {
        if !attr_mode {
            for c in text.chars() {
                match c {
                    '&' => self.writer.write_all(b"&amp;"),
                    '\u{00A0}' => self.writer.write_all(b"&nbsp;"),
                    '<' => self.writer.write_all(b"&lt;"),
                    '>' => self.writer.write_all(b"&gt;"),
                    c => self.writer.write_fmt(format_args!("{c}")),
                }?;
            }
            return Ok(());
        }
        let b = text.as_bytes();
        for (i, c) in text.char_indices() {
            match c {
                '"' => self.writer.write_all(b"&quot;"),
                '&' => {
                    let reference_shaped = b
                        .get(i + 1)
                        .is_some_and(|n| n.is_ascii_alphanumeric() || *n == b'#');
                    if reference_shaped {
                        self.writer.write_all(b"&amp;")
                    } else {
                        self.writer.write_all(b"&")
                    }
                }
                '\u{00A0}' => self.writer.write_all(b"&nbsp;"),
                c => self.writer.write_fmt(format_args!("{c}")),
            }?;
        }
        Ok(())
    }

    fn tagname(name: &QualName) -> LocalName {
        match name.ns {
            ns!(html) | ns!(mathml) | ns!(svg) => (),
            ref ns => {
                // html5ever warns here; the demo DOM never carries one and a
                // silently-wrong name would be worse than the noise
                eprintln!("warn: node with weird namespace {ns:?}");
            }
        }
        name.local.clone()
    }
}

impl<'w> Serializer for TabSerializer<'w> {
    fn start_elem<'a, AttrIter>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
    where
        AttrIter: Iterator<Item = AttrRef<'a>>,
    {
        let html_name = match name.ns {
            ns!(html) => Some(name.local.clone()),
            _ => None,
        };

        if self.parent().ignore_children {
            self.stack.push(ElemInfo {
                html_name,
                ignore_children: true,
            });
            return Ok(());
        }

        self.writer.write_all(b"<")?;
        self.writer.write_all(Self::tagname(&name).as_bytes())?;
        for (name, value) in attrs {
            self.writer.write_all(b" ")?;

            match name.ns {
                ns!() => (),
                ns!(xml) => self.writer.write_all(b"xml:")?,
                ns!(xmlns) => {
                    if name.local != local_name!("xmlns") {
                        self.writer.write_all(b"xmlns:")?;
                    }
                }
                ns!(xlink) => self.writer.write_all(b"xlink:")?,
                ref ns => {
                    eprintln!("warn: attr with weird namespace {ns:?}");
                    self.writer.write_all(b"unknown_namespace:")?;
                }
            }

            self.writer.write_all(name.local.as_bytes())?;
            self.writer.write_all(b"=\"")?;
            self.write_escaped(value, true)?;
            self.writer.write_all(b"\"")?;
        }
        self.writer.write_all(b">")?;

        let ignore_children = name.ns == ns!(html)
            && matches!(
                name.local,
                local_name!("area")
                    | local_name!("base")
                    | local_name!("basefont")
                    | local_name!("bgsound")
                    | local_name!("br")
                    | local_name!("col")
                    | local_name!("embed")
                    | local_name!("frame")
                    | local_name!("hr")
                    | local_name!("img")
                    | local_name!("input")
                    | local_name!("keygen")
                    | local_name!("link")
                    | local_name!("meta")
                    | local_name!("param")
                    | local_name!("source")
                    | local_name!("track")
                    | local_name!("wbr")
            );

        self.stack.push(ElemInfo {
            html_name,
            ignore_children,
        });

        Ok(())
    }

    fn end_elem(&mut self, name: QualName) -> io::Result<()> {
        let info = self.stack.pop().expect("no ElemInfo");
        if info.ignore_children {
            return Ok(());
        }

        self.writer.write_all(b"</")?;
        self.writer.write_all(Self::tagname(&name).as_bytes())?;
        self.writer.write_all(b">")
    }

    fn write_text(&mut self, text: &str) -> io::Result<()> {
        let escape = match self.parent().html_name {
            Some(local_name!("style"))
            | Some(local_name!("script"))
            | Some(local_name!("xmp"))
            | Some(local_name!("iframe"))
            | Some(local_name!("noembed"))
            | Some(local_name!("noframes"))
            | Some(local_name!("plaintext")) => false,

            Some(local_name!("noscript")) => false, // scripting enabled (the parse default)

            _ => true,
        };

        if escape {
            self.write_escaped(text, false)
        } else {
            self.writer.write_all(text.as_bytes())
        }
    }

    fn write_comment(&mut self, text: &str) -> io::Result<()> {
        self.writer.write_all(b"<!--")?;
        self.writer.write_all(text.as_bytes())?;
        self.writer.write_all(b"-->")
    }

    fn write_doctype(&mut self, name: &str) -> io::Result<()> {
        self.writer.write_all(b"<!DOCTYPE ")?;
        self.writer.write_all(name.as_bytes())?;
        self.writer.write_all(b">")
    }

    fn write_processing_instruction(&mut self, target: &str, data: &str) -> io::Result<()> {
        self.writer.write_all(b"<?")?;
        self.writer.write_all(target.as_bytes())?;
        self.writer.write_all(b" ")?;
        self.writer.write_all(data.as_bytes())?;
        self.writer.write_all(b">")
    }
}

/// Serializes the CHILDREN of `node` — the body-fragment serialization the
/// code tab carries. The traversal mirrors rcdom's own `Serialize` impl
/// (the standard fragment-serialization walk: children in order, void
/// elements through the serializer's ignore_children stack, raw-text
/// elements through the parent-html-name check) with one addition the
/// browser's serializer makes and rcdom's walk does not: a `<template>`'s
/// contents (rcdom keeps them in `template_contents`, not `children`) are
/// serialized inside the element, the way every browser writes them.
pub fn serialize_children(node: &Handle) -> String {
    use NodeData::Element;
    use std::collections::VecDeque;

    enum Op {
        Open(Handle),
        Close(QualName),
    }

    let mut buf: Vec<u8> = Vec::new();
    let walk = |buf: &mut Vec<u8>| -> io::Result<()> {
        let mut ser = TabSerializer::new(buf);
        let mut ops: VecDeque<Op> = VecDeque::new();
        ops.extend(node.children.borrow().iter().map(|h| Op::Open(h.clone())));
        while let Some(op) = ops.pop_front() {
            match op {
                Op::Open(handle) => match &handle.data {
                    Element {
                        name,
                        attrs,
                        template_contents,
                        ..
                    } => {
                        ser.start_elem(
                            name.clone(),
                            attrs.borrow().iter().map(|at| (&at.name, &at.value[..])),
                        )?;
                        ops.push_front(Op::Close(name.clone()));
                        for child in handle.children.borrow().iter().rev() {
                            ops.push_front(Op::Open(child.clone()));
                        }
                        if let Some(tc) = template_contents.borrow().as_ref() {
                            for child in tc.children.borrow().iter().rev() {
                                ops.push_front(Op::Open(child.clone()));
                            }
                        }
                    }
                    NodeData::Doctype { name, .. } => ser.write_doctype(name)?,
                    NodeData::Text { contents } => ser.write_text(&contents.borrow())?,
                    NodeData::Comment { contents } => ser.write_comment(contents)?,
                    NodeData::ProcessingInstruction { target, contents } => {
                        ser.write_processing_instruction(target, contents)?
                    }
                    NodeData::Document => {
                        panic!("cannot serialize a Document node as an element")
                    }
                },
                Op::Close(name) => ser.end_elem(name)?,
            }
        }
        Ok(())
    };
    walk(&mut buf).expect("writing to a Vec<u8> cannot fail");
    String::from_utf8(buf).expect("the serializer only writes UTF-8")
}

#[cfg(test)]
mod tests {
    // markup5ever_rcdom's Drop for Node STEALS the children of every
    // descendant while the document drops, so any Handle is only valid
    // while the RcDom it came from is alive — every helper keeps the dom.
    use super::serialize_children;
    use html5ever::parse_document;
    use html5ever::tendril::TendrilSink;
    use markup5ever_rcdom::{NodeData, RcDom};

    fn parse_body(html: &str) -> (RcDom, markup5ever_rcdom::Handle) {
        let dom: RcDom =
            parse_document(RcDom::default(), html5ever::ParseOpts::default()).one(html);
        let html_el = dom
            .document
            .children
            .borrow()
            .iter()
            .find(|c| {
                matches!(
                    &c.data,
                    NodeData::Element { name, .. } if name.local == html5ever::local_name!("html")
                )
            })
            .expect("html element")
            .clone();
        let body = html_el
            .children
            .borrow()
            .iter()
            .find(|c| {
                matches!(
                    &c.data,
                    NodeData::Element { name, .. } if name.local == html5ever::local_name!("body")
                )
            })
            .expect("body element")
            .clone();
        (dom, body)
    }

    fn ser(html: &str) -> String {
        let (dom, body) = parse_body(html);
        let _ = dom; // alive for the whole call
        serialize_children(&body)
    }

    #[test]
    fn unit_writer_bracket_utilities_stay_raw() {
        // the shape stage 2a shipped: raw & and > inside class values, so a
        // consumer's tailwind scan reads the utility
        let out =
            ser(r#"<div><span class="[&amp;_a]:underline has-[&gt;svg]:grid">x</span></div>"#);
        assert!(
            out.contains(r#"class="[&_a]:underline has-[>svg]:grid""#),
            "{out}"
        );
    }

    #[test]
    fn unit_writer_reference_shaped_amp_stays_escaped() {
        // & followed by alnum/# would start a character reference on
        // reparse — it must stay &amp;
        let out = ser(r#"<div><span class="a&amp;copy b&amp;#38;c">x</span></div>"#);
        assert!(out.contains(r#"class="a&amp;copy b&amp;#38;c""#), "{out}");
        // trailing / punctuation-followed & stays raw
        let out = ser(r#"<div><span class="a&amp; b&amp;">x</span></div>"#);
        assert!(out.contains(r#"class="a& b&""#), "{out}");
    }

    #[test]
    fn unit_writer_never_escapes_angles_in_attrs() {
        let out = ser(r#"<div><a class="p&gt;q [&lt;i]:size-4" href="/x?a=1&amp;b=2">l</a></div>"#);
        assert!(out.contains(r#"class="p>q [<i]:size-4""#), "{out}");
        // query & in href is followed by alnum — stays escaped there
        assert!(out.contains(r#"href="/x?a=1&amp;b=2""#), "{out}");
        // non-class attributes are escaped by the same rule, and text nodes
        // keep standard escaping
        assert!(out.contains(">l</a>"), "{out}");
    }

    #[test]
    fn unit_writer_text_escaping_and_nbsp() {
        let out = ser("<div><p>a &amp; b &lt; c &gt; d e</p></div>");
        assert!(out.contains("<p>a &amp; b &lt; c &gt; d e</p>"), "{out}");
        let out = ser("<div><p>a\u{00A0}b</p></div>");
        assert!(out.contains("<p>a&nbsp;b</p>"), "{out}");
    }

    #[test]
    fn unit_writer_void_element_and_quote() {
        let out = ser(
            r#"<div><input data-slot="input" required="" type="email" placeholder="say &quot;hi&quot;"></div>"#,
        );
        assert!(out.contains("<input data-slot="), "{out}");
        assert!(!out.contains("</input>"), "{out}");
        assert!(out.contains(r#"placeholder="say &quot;hi&quot;""#), "{out}");
    }

    #[test]
    fn unit_writer_svg_camelcase_and_xlink() {
        let out = ser(
            r##"<div><svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><use xlink:href="#a"></use></svg></div>"##,
        );
        assert!(out.contains("<svg viewBox="), "{out}");
        assert!(out.contains("xlink:href"), "{out}");
    }

    #[test]
    fn unit_writer_template_contents_serialize() {
        // rcdom keeps template children in template_contents, not children; the projection
        // relies on portal templates landing in the tab
        let out = ser(
            r#"<div><template id="d1-portal"><div data-slot="dialog-overlay" class="fixed"></div></template></div>"#,
        );
        assert!(out.contains("<template id=\"d1-portal\">"), "{out}");
        assert!(out.contains("</template>"), "{out}");
        assert!(out.contains(r#"data-slot="dialog-overlay""#), "{out}");
    }

    #[test]
    fn unit_writer_raw_roundtrip_is_identity() {
        // a serialized raw-& attribute reparses to the same value: the
        // writer is idempotent on its own output
        let once = ser(r#"<div><span class="[&_svg]:size-4">x</span></div>"#);
        let twice = ser(&once);
        assert_eq!(once, twice);
    }
}
