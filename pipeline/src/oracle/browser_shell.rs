//! Port of pipeline/browser_shell.go — the Go half of the chromium thin
//! shell (tools/browser-shell.mjs). One long-lived node process; JSON line
//! requests in, one JSON line response out. The driver owns every judgement.
//! Errors from the shell surface as Rust errors; a shell crash fails the node.

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Mutex;

pub struct BrowserShell {
    child: Mutex<Child>,
    stdin: Mutex<Option<ChildStdin>>,
    stdout: Mutex<BufReader<std::process::ChildStdout>>,
    reqs: Mutex<usize>,
}

impl BrowserShell {
    pub fn start() -> Result<BrowserShell, String> {
        let mut cmd = Command::new("node")
            .arg("tools/browser-shell.mjs")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()) // playwright diagnostics flow to the node log
            .spawn()
            .map_err(|e| e.to_string())?;
        let stdin = cmd.stdin.take().expect("piped stdin");
        let stdout = cmd.stdout.take().expect("piped stdout");
        Ok(BrowserShell {
            child: Mutex::new(cmd),
            stdin: Mutex::new(Some(stdin)),
            stdout: Mutex::new(BufReader::new(stdout)),
            reqs: Mutex::new(0),
        })
    }

    /// Sends req and returns the raw top-level response, erroring out on a
    /// shell-reported "error" field.
    pub fn round_trip(&self, req: &Value) -> Result<serde_json::Map<String, Value>, String> {
        *self.reqs.lock().unwrap() += 1;
        let n = *self.reqs.lock().unwrap();
        let mut line = serde_json::to_string(req).map_err(|e| e.to_string())?;
        line.push('\n');
        {
            let mut guard = self.stdin.lock().unwrap();
            match guard.as_mut() {
                Some(si) => {
                    si.write_all(line.as_bytes())
                        .map_err(|e| format!("browser-shell: write: {}", e))?;
                    si.flush().ok();
                }
                None => return Err("browser-shell: stdin closed".to_string()),
            }
        }
        let mut resp = String::new();
        {
            let mut so = self.stdout.lock().unwrap();
            so.read_line(&mut resp)
                .map_err(|e| format!("browser-shell: read (after {} reqs): {}", n, e))?;
        }
        let top: serde_json::Map<String, Value> = serde_json::from_str(resp.trim())
            .map_err(|e| format!("browser-shell: bad response {:?}: {}", resp, e))?;
        if let Some(e) = top.get("error") {
            let msg = e.as_str().unwrap_or("unknown").to_string();
            return Err(format!("browser-shell: {}", msg));
        }
        Ok(top)
    }

    pub fn call(&self, req: &Value) -> Result<Value, String> {
        let top = self.round_trip(req)?;
        Ok(Value::Object(top))
    }

    pub fn call_err(&self, req: &Value) -> Result<(), String> {
        self.call(req).map(|_| ())
    }

    /// call, but returns the "value" field's raw bytes instead of decoding
    /// through a map — which would lose the browser's own key order.
    pub fn call_raw_value(&self, req: &Value) -> Result<Value, String> {
        let top = self.round_trip(req)?;
        Ok(top.get("value").cloned().unwrap_or(Value::Null))
    }

    pub fn launch(&self) -> Result<(), String> {
        self.call_err(&json!({"op": "launch"}))
    }

    pub fn new_page(&self, capture: bool) -> Result<BPage<'_>, String> {
        self.new_page_origin(capture, "")
    }

    pub fn new_page_origin(&self, capture: bool, origin: &str) -> Result<BPage<'_>, String> {
        let res = self.call(&json!({"op": "newPage", "capture": capture, "origin": origin}))?;
        let id = res.get("pageId").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(BPage { s: self, id: id as i32 })
    }

    /// capture pageerror events only (example-fixture pins itself to uncaught
    /// errors, not the console/requestfailed net).
    pub fn new_page_errors_only(&self) -> Result<BPage<'_>, String> {
        let res = self.call(&json!({"op": "newPage", "capture": true, "captureOnly": "pageerror"}))?;
        let id = res.get("pageId").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(BPage { s: self, id: id as i32 })
    }

    pub fn close(&self) {
        let _ = self.call(&json!({"op": "close"}));
        // drop stdin so the shell sees EOF and exits (Go: stdin.Close())
        let _ = self.stdin.lock().unwrap().take();
        if let Ok(mut child) = self.child.lock() {
            let _ = child.wait();
        }
    }
}

/// A typed handle over one shell page.
pub struct BPage<'a> {
    s: &'a BrowserShell,
    pub id: i32,
}

impl<'a> BPage<'a> {
    pub fn goto_url(&self, url: &str) -> Result<(), String> {
        self.s.call_err(&json!({"op": "goto", "pageId": self.id, "url": url}))
    }

    /// Runs expr, a plain (non-function-shaped) expression, in the page.
    pub fn evaluate(&self, expr: &str) -> Result<Value, String> {
        let res = self
            .s
            .call(&json!({"op": "evaluate", "pageId": self.id, "expr": expr}))?;
        Ok(res.get("value").cloned().unwrap_or(Value::Null))
    }

    /// evaluate but preserves the result's own JSON key order.
    pub fn evaluate_ordered(&self, expr: &str) -> Result<Value, String> {
        self.s
            .call_raw_value(&json!({"op": "evaluate", "pageId": self.id, "expr": expr}))
    }

    /// Runs a function-shaped expression ("() => {…}") in the page; arg
    /// becomes the function's single parameter.
    pub fn evaluate_fn(&self, expr: &str) -> Result<Value, String> {
        self.evaluate_fn_arg(expr, Value::Null)
    }

    pub fn evaluate_fn_arg(&self, expr: &str, arg: Value) -> Result<Value, String> {
        let res = self.s.call(&json!({
            "op": "evaluateFn", "pageId": self.id, "expr": expr, "arg": arg
        }))?;
        Ok(res.get("value").cloned().unwrap_or(Value::Null))
    }

    pub fn set_content(&self, html: &str) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "setContent", "pageId": self.id, "html": html}))
    }

    pub fn add_style_tag_path(&self, path: &str) -> Result<(), String> {
        let abs = std::fs::canonicalize(path)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| path.to_string());
        self.s
            .call_err(&json!({"op": "addStyleTag", "pageId": self.id, "path": abs}))
    }

    pub fn add_style_tag(&self, content: &str) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "addStyleTag", "pageId": self.id, "content": content}))
    }

    pub fn add_script_tag(&self, content: &str) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "addScriptTag", "pageId": self.id, "content": content}))
    }

    pub fn focus(&self, selector: &str, timeout_ms: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "focus", "pageId": self.id, "selector": selector, "timeout": timeout_ms
        }))
    }

    pub fn wheel(&self, dx: f64, dy: f64) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "wheel", "pageId": self.id, "dx": dx, "dy": dy}))
    }

    /// Blocks http(s) subresource loads (initial-render pin).
    pub fn route_abort_external(&self) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "routeAbortExternal", "pageId": self.id}))
    }

    pub fn wait_for_function(&self, expr: &str, timeout_ms: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "waitForFunction", "pageId": self.id, "expr": expr, "timeout": timeout_ms
        }))
    }

    pub fn events(&self) -> Result<Vec<String>, String> {
        let res = self.s.call(&json!({"op": "events", "pageId": self.id}))?;
        let mut out = Vec::new();
        if let Some(errors) = res.get("errors").and_then(|v| v.as_array()) {
            for e in errors {
                if let Some(s) = e.as_str() {
                    out.push(s.to_string());
                }
            }
        }
        Ok(out)
    }

    pub fn close(&self) {
        let _ = self.s.call(&json!({"op": "close", "pageId": self.id}));
    }

    // ---- locator primitives (frame = an iframe selector; "" = the page) ----

    pub fn loc_count(&self, frame: &str, selector: &str) -> Result<i64, String> {
        let res = self.s.call(&json!({
            "op": "locCount", "pageId": self.id, "frame": frame, "selector": selector
        }))?;
        Ok(res.get("value").and_then(|v| v.as_i64()).unwrap_or(0))
    }

    pub fn loc_wait(&self, frame: &str, selector: &str, state: &str, timeout_ms: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "locWait", "pageId": self.id, "frame": frame,
            "selector": selector, "state": state, "timeout": timeout_ms
        }))
    }

    pub fn loc_scroll(&self, frame: &str, selector: &str, index: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "locScroll", "pageId": self.id, "frame": frame,
            "selector": selector, "index": index
        }))
    }

    /// Returns (value, present).
    pub fn loc_attr(&self, frame: &str, selector: &str, attr: &str) -> Result<(String, bool), String> {
        let res = self.s.call(&json!({
            "op": "locAttr", "pageId": self.id, "frame": frame, "selector": selector, "attr": attr
        }))?;
        match res.get("value").and_then(|v| v.as_str()) {
            Some(v) => Ok((v.to_string(), true)),
            None => Ok((String::new(), false)),
        }
    }

    pub fn loc_box(&self, frame: &str, selector: &str, index: i64) -> Result<Option<BBox>, String> {
        let res = self.s.call(&json!({
            "op": "locBox", "pageId": self.id, "frame": frame,
            "selector": selector, "index": index
        }))?;
        let Some(m) = res.get("value").and_then(|v| v.as_object()) else {
            return Ok(None);
        };
        let g = |k: &str| m.get(k).and_then(|v| v.as_f64()).unwrap_or(0.0);
        Ok(Some(BBox { x: g("x"), y: g("y"), width: g("width"), height: g("height") }))
    }

    /// Evaluates expr over every match (playwright evaluateAll).
    pub fn loc_eval_all(&self, frame: &str, selector: &str, expr: &str) -> Result<Value, String> {
        self.loc_eval_all_arg(frame, selector, expr, Value::Null)
    }

    pub fn loc_eval_all_arg(&self, frame: &str, selector: &str, expr: &str, arg: Value) -> Result<Value, String> {
        let res = self.s.call(&json!({
            "op": "locEvalAll", "pageId": self.id, "frame": frame,
            "selector": selector, "expr": expr, "arg": arg
        }))?;
        Ok(res.get("value").cloned().unwrap_or(Value::Null))
    }

    /// Clicks the nth match; button "right" for context-menu triggers.
    pub fn loc_click(&self, frame: &str, selector: &str, index: i64, button: &str) -> Result<(), String> {
        self.loc_click_timeout(frame, selector, index, button, 15000)
    }

    /// locClick with an explicit actionability timeout (15000ms shared
    /// default: example-fixture overran the 5000ms locator default twice
    /// under 16-way browser load).
    pub fn loc_click_timeout(&self, frame: &str, selector: &str, index: i64, button: &str, timeout_ms: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "locClick", "pageId": self.id, "frame": frame, "selector": selector,
            "index": index, "button": button, "timeout": timeout_ms
        }))
    }

    /// Evaluates expr on the nth match only (playwright $eval semantics).
    pub fn loc_eval(&self, frame: &str, selector: &str, expr: &str, index: i64) -> Result<Value, String> {
        let res = self.s.call(&json!({
            "op": "locEval", "pageId": self.id, "frame": frame,
            "selector": selector, "expr": expr, "index": index
        }))?;
        Ok(res.get("value").cloned().unwrap_or(Value::Null))
    }

    pub fn mouse_move(&self, x: f64, y: f64, steps: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "mouseMove", "pageId": self.id, "x": x, "y": y, "steps": steps
        }))
    }

    pub fn wait_for_load_state(&self, state: &str, timeout_ms: i64) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "waitForLoadState", "pageId": self.id, "state": state, "timeout": timeout_ms
        }))
    }

    pub fn wait_for_timeout(&self, ms: i64) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "waitForTimeout", "pageId": self.id, "ms": ms}))
    }

    pub fn mouse_click(&self, x: f64, y: f64) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "mouseClick", "pageId": self.id, "x": x, "y": y}))
    }

    /// button "right" drives context-menu triggers.
    pub fn mouse_click_button(&self, x: f64, y: f64, button: &str) -> Result<(), String> {
        self.s.call_err(&json!({
            "op": "mouseClick", "pageId": self.id, "x": x, "y": y, "button": button
        }))
    }

    /// Runs contract-def open/openShadless playwright code against the page.
    pub fn driver(&self, code: &str) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "driver", "pageId": self.id, "code": code}))
    }

    pub fn key_press(&self, key: &str) -> Result<(), String> {
        self.s
            .call_err(&json!({"op": "keyPress", "pageId": self.id, "key": key}))
    }
}

pub struct BBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Drop for BrowserShell {
    fn drop(&mut self) {
        // Best effort: closing stdin trips the driver's EOF handler, which
        // closes chromium — an error path that never reached close() must
        // not leak a browser. Never blocks long on a wedged child.
        if let Some(si) = self.stdin.lock().unwrap().take() {
            drop(si); // dropping the ChildStdin closes the pipe
        }
        let Ok(child) = self.child.get_mut() else {
            return;
        };
        for _ in 0..10 {
            match child.try_wait() {
                Ok(Some(_)) => return,
                Ok(None) => std::thread::sleep(std::time::Duration::from_millis(100)),
                Err(_) => return,
            }
        }
    }
}

