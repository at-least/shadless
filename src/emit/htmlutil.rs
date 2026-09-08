//! Port of pipeline/emitter_html.go — escHtml mirrors src/emitter/index.mjs's
//! escHtml; the order load-bears (& first, so &lt; never becomes &amp;lt;).

pub fn esc_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
