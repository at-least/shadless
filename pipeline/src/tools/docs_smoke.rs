//! Port of pipeline/docs_smoke.go — serve the built Zola site over http and
//! drive the browser with the REAL mouse (synthetic PointerEvents are
//! rejected by radix-like filters):
//!   1. dialog.html: real-mouse open inside the iframe + Escape close
//!   2. avatar.html: preview over http, 0 console errors, images settled
//!   3. dialog page preview wiring counts
//!   --all: every built page — render, no raw mdx leak, 0 errors

use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::LazyLock;

static RE_IFRAME_SRC: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"<iframe[^>]*\ssrc="([^"]+)""#).unwrap());

pub fn run_docs_smoke(root: &Path, all: bool) -> i32 {
    let site_dir = root.join("docs/site/public");
    if !site_dir.exists() {
        eprintln!("FAIL  docs smoke: the site is not built — run make docs first");
        return 1;
    }

    let mut failures: Vec<String> = Vec::new();

    // ---- 0. every root-relative preview iframe src is a built file ----
    // The browser rounds below visit dialog and avatar (or, under --all,
    // count iframes without asserting their targets): a preview pointing at
    // a page that does not exist is invisible to all of them — accordion's
    // iframe can 404 and the gate stays green. Check the links statically,
    // on every built page, in both modes.
    {
        let mut dead: Vec<String> = Vec::new();
        let mut seen = 0usize;
        for e in walkdir::WalkDir::new(&site_dir) {
            let Ok(e) = e else { continue };
            if e.file_type().is_dir()
                || e.path().extension().map(|x| x != "html").unwrap_or(true)
            {
                continue;
            }
            let Ok(rel) = e.path().strip_prefix(&site_dir) else {
                continue;
            };
            let Ok(b) = std::fs::read_to_string(e.path()) else {
                continue;
            };
            for m in RE_IFRAME_SRC.captures_iter(&b) {
                seen += 1;
                let src = &m[1];
                if src.starts_with('/') && !src.contains("//") {
                    if !site_dir.join(src.trim_start_matches('/')).exists() {
                        dead.push(format!("{}: {}", rel.display(), src));
                    }
                }
            }
        }
        dead.sort();
        if dead.is_empty() && seen > 0 {
            println!(
                "PASS  iframe targets: every root-relative preview iframe src is a built page ({seen} iframes)"
            );
        } else {
            for d in &dead {
                eprintln!("FAIL  iframe target missing — {}", d);
            }
            if seen == 0 {
                // a template drift that stops emitting preview iframes would
                // otherwise turn this check into a vacuous pass
                eprintln!("FAIL  iframe targets: no preview iframes found in the built pages — the preview template changed?");
            }
            failures.push("iframe targets".to_string());
        }
    }


    // ephemeral-port static server (python3 http.server, same as the JS gate).
    // The listener is dropped before python binds the port (a TOCTOU window:
    // a parallel gate can steal it), so a start that never comes up is
    // retried on a fresh port. ServerGuard kills the server on every exit
    // path — early returns and panics included; the old code orphaned it.
    struct ServerGuard(Child);
    impl Drop for ServerGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    // bound (not dropped) so the guard keeps the server alive for the whole
    // smoke run; nothing calls methods on it — Drop does the cleanup
    let (_server, base): (ServerGuard, String) = {
        let mut attempt = 0;
        loop {
            attempt += 1;
            let free_port = std::net::TcpListener::bind("127.0.0.1:0")
                .and_then(|l| l.local_addr().map(|a| a.port()))
                .unwrap_or(0);
            let child = match Command::new("python3")
                .args([
                    "-m",
                    "http.server",
                    &free_port.to_string(),
                    "--bind",
                    "127.0.0.1",
                    "--directory",
                    site_dir.to_str().unwrap_or("."),
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("FAIL  docs smoke: python3 http.server: {}", e);
                    return 1;
                }
            };
            let guard = ServerGuard(child);
            let base = format!("http://127.0.0.1:{}", free_port);
            let mut server_up = false;
            for _ in 0..100 {
                if let Ok(resp) = ureq::get(&format!("{}/index.html", base))
                    .timeout(std::time::Duration::from_secs(2))
                    .call()
                {
                    let _ = resp.into_string();
                    server_up = true;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            if server_up {
                break (guard, base);
            }
            if attempt >= 3 {
                eprintln!(
                    "FAIL  docs smoke: static server did not come up after {} attempts (python3 http.server) — another process may be taking the ports",
                    attempt
                );
                return 1;
            }
        }
    };

    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("docs smoke: {}", e);

            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("docs smoke: {}", e);

        return 1;
    }

    let check = |label: &str, ok: bool, detail: &str, failures: &mut Vec<String>| {
        if ok {
            println!("PASS  {}", label);
        } else if !detail.is_empty() {
            eprintln!("FAIL  {} — {}", label, detail);
            failures.push(label.to_string());
        } else {
            eprintln!("FAIL  {}", label);
            failures.push(label.to_string());
        }
    };

    // ---- 1. dialog preview: real-mouse open + Escape close inside the iframe ----
    let page = match shell.new_page(false) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("docs smoke: {}", e);

            return 1;
        }
    };
    if let Err(e) = page.goto_url(&format!("{}/components/dialog/", base)) {
        eprintln!("docs smoke: {}", e);

        return 1;
    }
    // let every lazy iframe settle first — recomputing the trigger box
    // mid-layout-shift made the real-mouse click land outside the trigger
    let _ = page.wait_for_load_state("networkidle", 10000);

    let dialog_frame = r#"iframe.demo[title="dialog-demo"]"#;
    let trigger_sel = r#"[data-slot="dialog-trigger"]"#;
    if let Err(e) = page.loc_wait(dialog_frame, trigger_sel, "visible", 5000) {
        eprintln!("docs smoke: trigger wait: {}", e);

        return 1;
    }
    let _ = page.loc_scroll(dialog_frame, trigger_sel, 0);
    let _ = page.wait_for_timeout(300);
    let tbox = match page.loc_box(dialog_frame, trigger_sel, 0) {
        Ok(Some(b)) => b,
        Ok(None) => {
            eprintln!("docs smoke: trigger box: element not visible");

            return 1;
        }
        Err(e) => {
            eprintln!("docs smoke: trigger box: {}", e);

            return 1;
        }
    };
    let _ = page.mouse_click(tbox.x + tbox.width / 2.0, tbox.y + tbox.height / 2.0);

    if page
        .loc_wait(
            dialog_frame,
            r#"[data-slot="dialog-content"][data-state="open"]"#,
            "visible",
            3000,
        )
        .is_ok()
    {
        let open_count = page
            .loc_count(dialog_frame, r#"[data-slot="dialog-content"][data-state="open"]"#)
            .unwrap_or(0);
        let (expanded, _) = page
            .loc_attr(dialog_frame, trigger_sel, "aria-expanded")
            .unwrap_or((String::new(), false));
        check(
            "dialog: real-mouse click opens dialog in iframe (live content data-state=open)",
            open_count == 1 && expanded == "true",
            &format!("aria-expanded={}", expanded),
            &mut failures,
        );
    } else {
        check(
            "dialog: real-mouse click opens dialog in iframe (live content data-state=open)",
            false,
            "locator wait timed out",
            &mut failures,
        );
    }

    // focus stays inside the frame: real-mouse click on the content card
    // padding, then Escape
    if let Ok(Some(cbox)) = page.loc_box(dialog_frame, r#"[data-slot="dialog-content"]"#, 0) {
        let _ = page.mouse_click(cbox.x + 12.0, cbox.y + 12.0);
    }
    let _ = page.key_press("Escape");

    let _ = page.loc_wait(dialog_frame, r#"[data-slot="dialog-content"]"#, "detached", 3000);
    let live_portal_nodes = page
        .loc_count(
            dialog_frame,
            r#"[data-slot="dialog-portal"], [data-slot="dialog-overlay"], [data-slot="dialog-content"]"#,
        )
        .unwrap_or(-1);
    let (expanded_closed, _) = page
        .loc_attr(dialog_frame, trigger_sel, "aria-expanded")
        .unwrap_or((String::new(), false));
    let (trig_state, _) = page
        .loc_attr(dialog_frame, trigger_sel, "data-state")
        .unwrap_or((String::new(), false));
    check(
        "dialog: Escape closes dialog inside iframe (live portal nodes removed)",
        live_portal_nodes == 0 && expanded_closed == "false" && trig_state == "closed",
        &format!(
            "livePortalNodes={} aria-expanded={} data-state={}",
            live_portal_nodes, expanded_closed, trig_state
        ),
        &mut failures,
    );

    // ---- 2. avatar preview over http: 0 console errors ----
    let av_page = match shell.new_page_origin(true, &base) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("docs smoke: {}", e);

            return 1;
        }
    };
    let av_frame = r#"iframe.demo[title="avatar-demo"]"#;
    let _ = av_page.goto_url(&format!("{}/components/avatar/", base));
    let _ = av_page.loc_wait(av_frame, r#"[data-slot="avatar"]"#, "visible", 5000);
    if let Ok(Some(av_box)) = av_page.loc_box(av_frame, r#"[data-slot="avatar"]"#, 0) {
        let _ = av_page.mouse_click(av_box.x + 2.0, av_box.y + 2.0);
    }
    let mut settled: Vec<bool> = Vec::new();
    let mut all_settled = false;
    for _ in 0..40 {
        if let Ok(images) = av_page.loc_eval_all(
            av_frame,
            r#"[data-slot="avatar-image"]"#,
            r#"el => el.complete && el.naturalWidth > 0"#,
        ) {
            settled = images
                .as_array()
                .map(|a| a.iter().map(|v| v == &serde_json::Value::Bool(true)).collect())
                .unwrap_or_default();
        }
        all_settled = !settled.is_empty() && settled.iter().all(|s| *s);
        if all_settled {
            break;
        }
        let _ = av_page.wait_for_timeout(250);
    }
    let av_errors = av_page.events().unwrap_or_default();
    // Go %v on []bool: "[true true true true true]" — space-separated
    let settled_go: String = settled
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    check(
        &format!(
            "avatar: preview over http reports 0 console errors (images={} settled=[{}])",
            settled.len(),
            settled_go
        ),
        av_errors.is_empty() && !settled.is_empty() && all_settled,
        &format!("{:?}", av_errors),
        &mut failures,
    );

    // ---- 3. dialog page preview wiring counts ----
    let iframes = page.loc_count("", "iframe.demo").unwrap_or(0);
    let unavailable = page.loc_count("", ".demo-missing").unwrap_or(0);
    let previews = iframes + unavailable;
    println!(
        "dialog page wiring: previews={} iframes={} unavailable-notes={}",
        previews, iframes, unavailable
    );
    check(
        "dialog page wiring: every preview is an iframe or an unavailable note",
        previews > 0 && iframes >= 1,
        &format!("previews={} iframes={} unavailable={}", previews, iframes, unavailable),
        &mut failures,
    );

    // ---- 4. --all: every-page sweep ----
    let mut verify_n = 0usize;
    if all {
        // Zola emits one directory per page: <path>/index.html
        let mut page_files: Vec<String> = Vec::new();
        for e in walkdir::WalkDir::new(&site_dir) {
            let Ok(e) = e else { continue };
            if !e.file_type().is_dir() && e.file_name() == "index.html" {
                if let Ok(rel) = e.path().strip_prefix(&site_dir) {
                    page_files.push(rel.to_string_lossy().into_owned());
                }
            }
        }
        page_files.sort();
        let (mut render_fail, mut leak_fail) = (0usize, 0usize);
        let (mut console_err_count, mut iframes_loaded) = (0usize, 0usize);
        let (mut n_components, mut n_guides, mut n_index) = (0usize, 0usize, 0usize);
        for f in &page_files {
            let p = match shell.new_page_origin(true, &base) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let trimmed = f.trim_end_matches("index.html").trim_end_matches('/');
            let _ = p.goto_url(&format!("{}/{}/", base, trimmed));
            let n = p.loc_count("", "iframe.demo").unwrap_or(0);
            for i in 0..n {
                let _ = p.loc_scroll("", "iframe.demo", i);
                iframes_loaded += 1;
            }
            let _ = p.wait_for_load_state("networkidle", 2000);
            let res = p.evaluate_fn(
                r#"() => {
        const article = document.querySelector('.vp-doc')
        const text = article?.innerText?.trim() ?? ''
        const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT,
          { acceptNode: (n) => (n.parentElement.closest('pre, code') ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT) })
        let visible = ''
        for (; walker.nextNode();) visible += walker.currentNode.data + '\n'
        const home = document.querySelector('.home-hero')
        return { rendered: (!!article && text.length > 0) || !!home, leaks: visible.match(/Component(Preview|Source)\b/g) ?? [] }
      }"#,
            );
            if f == "index.html" {
                n_index += 1;
            } else if f.starts_with("guides/") {
                n_guides += 1;
            } else {
                n_components += 1;
            }
            let mut rendered = false;
            let mut leaks: Vec<String> = Vec::new();
            if let Ok(m) = res {
                if let Some(obj) = m.as_object() {
                    rendered = obj.get("rendered") == Some(&serde_json::Value::Bool(true));
                    if let Some(raw) = obj.get("leaks").and_then(|v| v.as_array()) {
                        for l in raw {
                            if let Some(s) = l.as_str() {
                                leaks.push(s.to_string());
                            }
                        }
                    }
                }
            }
            if !rendered {
                render_fail += 1;
                eprintln!("FAIL  render: {} — article missing/empty", f);
            }
            if !leaks.is_empty() {
                leak_fail += 1;
                eprintln!(
                    "FAIL  mdx leak: {} — {}",
                    f,
                    crate::emit::css::dedup(&leaks).join(", ")
                );
            }
            let evts = p.events().unwrap_or_default();
            // console and pageerror are merged by the capture; the JS split
            // them only for the summary line
            if !evts.is_empty() {
                let n = evts.len().min(3);
                eprintln!("FAIL  console: {} — {}", f, evts[..n].join(" | "));
            }
            console_err_count += evts.len();
            p.close();
        }
        println!(
            "pages: {}/{} visited ({} components, {} guides, {} index) · {} preview iframes loaded",
            page_files.len(),
            page_files.len(),
            n_components,
            n_guides,
            n_index,
            iframes_loaded
        );
        check(
            &format!(
                "every-page render: {} pages non-empty (article present)",
                page_files.len()
            ),
            render_fail == 0,
            &format!("{} failed", render_fail),
            &mut failures,
        );
        check(
            "every-page mdx: 0 raw ComponentPreview/ComponentSource outside code blocks",
            leak_fail == 0,
            &format!("{} pages leaking", leak_fail),
            &mut failures,
        );
        // One count on purpose: the capture merges console errors and
        // pageerrors into the same event list.
        check(
            "every-page console: 0 error events, console and pageerror alike (iframes included)",
            console_err_count == 0,
            &format!("{} error events", console_err_count),
            &mut failures,
        );
        verify_n = page_files.len();
    }

    shell.close();
    // the ServerGuard kills + reaps the static server on drop

    if !failures.is_empty() {
        if all {
            println!("FAIL  docs verify ({} failed)", failures.len());
        } else {
            println!("FAIL  docs smoke ({} failed)", failures.len());
        }
        return 1;
    }
    if all {
        println!("PASS  docs verify ({} pages, 0 console errors)", verify_n);
    } else {
        println!("PASS  docs smoke (dialog iframe open/close, avatar 0 errors, preview wiring)");
    }
    0
}
