//! Port of pipeline/mutations.go + meta.go — the mutation set, the snapshot
//! harness, and the meta-gate wiring.
//!
//! Every mutation is a deliberate, minimal break in a real file that its gate
//! MUST notice. Two invariants hold for the whole set and both are enforced:
//! every helper ASSERTS it changed something, and a mutation touches only
//! what it claims to (the JSON-shaped mutations are textual splices, not
//! parse/re-serialize round trips).

use super::pin::truncate;
use regex::{Captures, Regex};
use std::path::Path;
use std::sync::{Mutex, OnceLock};

pub const UPSTREAM_EXAMPLES: &str = ".upstream/shadcn-ui/apps/v4/examples/radix";

// ------------------------------------------------------------- helpers

/// mutEdit reads path, applies fn, and asserts the result differs. A missing
/// target and a no-op edit are both errors: the first means the tree was not
/// built, the second means the anchor text has moved and the mutation has
/// quietly stopped proving anything.
pub fn mut_edit(
    root: &Path,
    path: &str,
    f: impl Fn(&str) -> Result<String, String>,
) -> Result<(), String> {
    let full = root.join(path);
    let before =
        std::fs::read_to_string(&full).map_err(|_| format!("mutation target missing: {} (build first)", path))?;
    let after = f(&before)?;
    if after == before {
        return Err(format!(
            "mutation no-op on {} — the anchor text is gone; fix the mutation",
            path
        ));
    }
    std::fs::write(full, after).map_err(|e| e.to_string())
}

/// Replaces the first occurrence of an exact anchor, asserting it was present.
pub fn mut_replace_once(root: &Path, path: &str, find: &str, repl: &str) -> Result<(), String> {
    mut_edit(root, path, |s| {
        if !s.contains(find) {
            return Err(format!("anchor not found in {}: {}", path, truncate(find, 60)));
        }
        Ok(s.replacen(find, repl, 1))
    })
}

/// Replaces every occurrence of an exact anchor.
pub fn mut_replace_all(root: &Path, path: &str, find: &str, repl: &str) -> Result<(), String> {
    mut_edit(root, path, |s| {
        if !s.contains(find) {
            return Err(format!("anchor not found in {}: {}", path, truncate(find, 60)));
        }
        Ok(s.replace(find, repl))
    })
}

/// Replaces the FIRST regexp match, matching JS `String.replace` with a
/// non-global regex. repl may use $1 etc.
pub fn mut_replace_re(root: &Path, path: &str, re: &Regex, repl: &str) -> Result<(), String> {
    mut_edit(root, path, |s| {
        let Some(c) = re.captures(s) else {
            return Err(format!("pattern not found in {}: {}", path, re.as_str()));
        };
        let m = c.get(0).unwrap();
        let mut out = String::from(&s[..m.start()]);
        c.expand(repl, &mut out);
        out.push_str(&s[m.end()..]);
        Ok(out)
    })
}

/// Returns the first file under dir (recursively, sorted, so the choice is
/// deterministic) whose content satisfies pred.
pub fn mut_find_file(
    root: &Path,
    dir: &str,
    pred: impl Fn(&str) -> bool,
    exts: &[&str],
) -> Result<String, String> {
    let extsOwned: Vec<String> = if exts.is_empty() {
        vec![".html".to_string()]
    } else {
        exts.iter().map(|s| s.to_string()).collect()
    };
    let base = root.join(dir);
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();
    for e in walkdir::WalkDir::new(&base) {
        let Ok(e) = e else { continue };
        if e.file_type().is_dir() {
            continue;
        }
        let name = e.file_name().to_string_lossy();
        if extsOwned.iter().any(|x| name.ends_with(x.as_str())) {
            candidates.push(e.path().to_path_buf());
        }
    }
    candidates.sort();
    for p in candidates {
        let Ok(b) = std::fs::read_to_string(&p) else {
            continue;
        };
        if pred(&b) {
            let rel = p
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .into_owned();
            return Ok(rel);
        }
    }
    Err(format!(
        "no file under {} matched the mutation's predicate",
        dir
    ))
}

/// Splices text in immediately after the first occurrence of anchor. This is
/// how the JSON mutations add a key: everything else in the file keeps its
/// exact bytes, so the gate reacts to the new entry and not to a reformat.
pub fn insert_after(s: &str, anchor: &str, text: &str) -> (String, bool) {
    match s.find(anchor) {
        Some(i) => {
            let j = i + anchor.len();
            let mut out = String::with_capacity(s.len() + text.len());
            out.push_str(&s[..j]);
            out.push_str(text);
            out.push_str(&s[j..]);
            (out, true)
        }
        None => (s.to_string(), false),
    }
}

// ------------------------------------------------------------- regexes

pub fn re_commit() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""commit": "[0-9a-f]{40}""#).unwrap())
}
pub fn re_primary_token() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"--primary: oklch\([^)]*\);").unwrap())
}
pub fn re_padding() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r" px-[0-9.]+").unwrap())
}
pub fn re_dialog_script() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"<script src="[^"]*/js/dialog\.js"></script>"#).unwrap())
}
pub fn re_export_default_fn() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(export default function [0-9A-Za-z_]+\([^)]*\)[\t\n\f\r ]*\{)").unwrap())
}
pub fn re_first_reason() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""reason": "((?:[^"\\]|\\.)*)""#).unwrap())
}
pub fn re_sheet_hash() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""behavior:sheet"(?s:.){0,400}?"hash": "[0-9a-f]{64}""#).unwrap())
}
pub fn re_first_shadless_cell() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""shadless": "((?:[^"\\]|\\.)*)""#).unwrap())
}

// ------------------------------------------------------------- the set

/// One break, its target gate, and the bug class it proves.
///
/// `files` is the snapshot set: the harness saves these before apply and
/// restores them afterwards, including on crash. `resolve` replaces files for
/// a mutation whose target can only be located on a built tree — it runs
/// first and its result is both the snapshot set and apply's input, so the
/// target cannot drift between the two.
pub struct Mutation {
    pub id: &'static str,
    pub gate: &'static str,
    pub why: &'static str,
    pub files: &'static [&'static str],
    pub resolve: Option<fn(&Path) -> Result<Vec<String>, String>>,
    pub apply: fn(&Path, &[String]) -> Result<(), String>,
}

impl Mutation {
    fn targets(&self, root: &Path) -> Result<Vec<String>, String> {
        if let Some(r) = self.resolve {
            return r(root);
        }
        Ok(self.files.iter().map(|s| s.to_string()).collect())
    }
}

/// meta.go resolveOwnedExample: the first page in the ownership manifest that
/// still has an upstream example source, so a re-pin that retires one file
/// does not silently disarm the mutation that targets it.
pub fn resolve_owned_example(root: &Path) -> Result<Vec<String>, String> {
    let b = std::fs::read_to_string(root.join("docs/example-oracle.json"))
        .map_err(|e| format!("docs/example-oracle.json missing (build first): {}", e))?;
    #[derive(serde::Deserialize)]
    struct Owned {
        #[serde(default)]
        name: String,
    }
    let owned: Vec<Owned> = serde_json::from_str(&b)
        .map_err(|e| format!("docs/example-oracle.json: {}", e))?;
    for o in owned {
        let rel = Path::new(UPSTREAM_EXAMPLES).join(format!("{}.tsx", o.name));
        if root.join(&rel).exists() {
            return Ok(vec![rel.to_string_lossy().into_owned()]);
        }
    }
    Err(format!(
        "no page in docs/example-oracle.json has an upstream example under {}",
        UPSTREAM_EXAMPLES
    ))
}

fn apply_consumer_sim_unknown_utility(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(root, &f[0], "@apply ", "@apply mutation-not-a-real-utility ")
}

fn apply_contracts_strip_glue(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        r#"<script src="../../dist/js/dialog.js">"#,
        r#"<script data-mutated src="../../dist/js/MISSING.js">"#,
    )
}

fn apply_coverage_drop_contract(root: &Path, f: &[String]) -> Result<(), String> {
    let full = root.join(&f[0]);
    if !full.exists() {
        return Err(format!("mutation target missing: {}", f[0]));
    }
    std::fs::remove_file(full).map_err(|e| e.to_string())
}

fn apply_css_direction_new_physical(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(root, &f[0], "@apply ", "@apply right-[13px] ")
}

fn apply_demo_parity_token_drift(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_re(root, &f[0], re_primary_token(), "--primary: oklch(0.5 0.2 250);")
}

fn apply_demo_smoke_console_error(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "</body>",
        r#"<script>throw new Error("mutation: page error")</script></body>"#,
    )
}

fn apply_dist_complete_drop_component(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_all(
        root,
        &f[0],
        r#"[data-slot="dialog-content"]"#,
        r#"[data-slot="dialog-MUTATED"]"#,
    )
}

fn apply_docs_consistency_react_import(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "## Installation",
        "```tsx\nimport { Accordion } from \"@/components/ui/accordion\"\n```\n\n## Installation",
    )
}

fn apply_docs_fidelity_drop_heading(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(root, &f[0], "## Installation", "## Installation-mutated")
}

fn apply_docs_smoke_broken_iframe(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        r#"src="/demos/accordion-demo.html""#,
        r#"src="/demos/mutation-missing.html""#,
    )
}

fn apply_pin_base_drift(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(root, &f[0], "registry/bases/radix/ui", "registry/bases/base/ui")
}

fn apply_rtl_dict_missing_dictionary(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "const translations",
        "const translationsRenamedByMutation",
    )
}

fn apply_example_oracle_render_failure(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_re(
        root,
        &f[0],
        re_export_default_fn(),
        "$1\n  throw new Error(\"example-oracle mutation: render failure\")",
    )
}

fn apply_example_perturb_shipped(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        r#"data-slot="badge""#,
        r#"data-slot="badge" data-mutation="1""#,
    )
}

fn apply_golden_perturb_oracle(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        r#"data-slot=\"accordion\""#,
        r#"data-slot=\"accordion-mutated\""#,
    )
}

fn apply_interactivity_strip_script(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_re(root, &f[0], re_dialog_script(), "")
}

fn apply_ledger_budget_exceeded(root: &Path, f: &[String]) -> Result<(), String> {
    mut_edit(root, &f[0], |s| {
        let m = re_first_reason()
            .captures(s)
            .ok_or_else(|| format!("{} has no exemption to copy a reason from", f[0]))?;
        let (out, ok) = insert_after(
            s,
            "\"examples\": {",
            &format!("\n  \"__mutation-extra-demo\": {{\n   \"reason\": \"{}\"\n  }},", &m[1]),
        );
        if !ok {
            return Err(format!("{} has no \"examples\" object", f[0]));
        }
        Ok(out)
    })
}

fn apply_ledger_undocumented_exemption(root: &Path, f: &[String]) -> Result<(), String> {
    mut_edit(root, &f[0], |s| {
        let (out, ok) = insert_after(
            s,
            "\"examples\": {",
            "\n  \"__mutation-demo\": {\n   \"reason\": \"mutation: an undocumented brand-new reason\"\n  },",
        );
        if !ok {
            return Err(format!("{} has no \"examples\" object", f[0]));
        }
        Ok(out)
    })
}

fn apply_overlay_orphaned_rule(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "  ignoreAttrs: {\n    \"accordion\": [\"text\"],",
        "  ignoreAttrs: {\n    \"accordion\": [\"text\"],\n    \"accordion-phantom\": [\"text\"],",
    )
}

fn apply_overlay_stale_authored(root: &Path, f: &[String]) -> Result<(), String> {
    mut_edit(root, &f[0], |s| {
        let loc = re_sheet_hash()
            .find(s)
            .ok_or_else(|| format!("{} has no \"behavior:sheet\" unit with a hash", f[0]))?;
        let zero_hash = format!("\"hash\": \"{}\"", "0".repeat(64));
        let re = Regex::new(r#""hash": "[0-9a-f]{64}""#).unwrap();
        let zeroed = re.replace_all(&s[loc.start()..loc.end()], zero_hash.as_str()).into_owned();
        let mut out = String::from(&s[..loc.start()]);
        out.push_str(&zeroed);
        out.push_str(&s[loc.end()..]);
        Ok(out)
    })
}

fn apply_script_refs_dead_node_call(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "\"unit\": \"node tools/unit-check.mjs\"",
        "\"unit\": \"node tools/unit-check-MUTATED.mjs\"",
    )
}

fn apply_pack_broken_export(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "\"./dist/shadless.full.min.css\"",
        "\"./dist/shadless.full.MUTATED.css\"",
    )
}

fn apply_path_parity_drop_utility(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_re(root, &f[0], re_padding(), "")
}

fn apply_pin_commit_drift(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_re(
        root,
        &f[0],
        re_commit(),
        &format!("\"commit\": \"{}\"", "0".repeat(40)),
    )
}

fn apply_product_drop_slot_rule(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_all(
        root,
        &f[0],
        r#"[data-slot="badge"]"#,
        r#"[data-slot="badge-MUTATED"]"#,
    )
}

fn apply_reproducible_hand_edit(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(root, &f[0], "</body>", "<!-- hand edit --></body>")
}

fn apply_style_parity_perturb_padding(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        r#"data-slot="dialog-content""#,
        r#"data-slot="dialog-content" style="padding-top:37px""#,
    )
}

fn apply_style_parity_recorded_value_drift(root: &Path, f: &[String]) -> Result<(), String> {
    mut_edit(root, &f[0], |s| {
        if !s.contains("\"cells\"") {
            return Err(format!("{} has no cells array", f[0]));
        }
        let loc = re_first_shadless_cell()
            .find(s)
            .ok_or_else(|| {
                format!(
                    "{} records no cell values to perturb (no cells, or still bare ids — re-record it with values)",
                    f[0]
                )
            })?;
        let mut out = String::from(&s[..loc.start()]);
        out.push_str("\"shadless\": \"999px /* mutation */\"");
        out.push_str(&s[loc.end()..]);
        Ok(out)
    })
}

fn apply_unit_break_pure_fn(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "apply: toks.filter((t) => !MARKER.test(t) &&",
        "apply: toks.filter((t) => (true) &&",
    )
}

fn apply_typecheck_break_ir_contract(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        // exact match — mut_replace_once does no format expansion, so these
        // braces are single, as in the file and in Go's mutations.go
        "* @param {Record<string, string>} hints",
        "* @param {number} hints",
    )
}

fn apply_variant_merge_defaults(root: &Path, f: &[String]) -> Result<(), String> {
    mut_replace_once(
        root,
        &f[0],
        "[data-slot=\"button\"] { @apply ",
        "[data-slot=\"button\"] { @apply bg-primary text-primary-foreground ",
    )
}

/// The whole set, in id order. TestMeta cross-checks it against the graph in
/// both directions: every gate must be named by at least one mutation, and
/// every mutation must name a gate that lists it.
pub static MUTATIONS: &[Mutation] = &[
    Mutation {
        id: "consumer-sim-unknown-utility",
        gate: "consumer-sim",
        why: "a component stylesheet references a utility that does not exist",
        files: &["dist/css/switch.css"],
        resolve: None,
        apply: apply_consumer_sim_unknown_utility,
    },
    Mutation {
        id: "contracts-strip-glue",
        gate: "contracts",
        why: "a kernel-tier page ships without its behavior file — the oracle opens, the page does not",
        files: &["src/kernel/dialog.html"],
        resolve: None,
        apply: apply_contracts_strip_glue,
    },
    Mutation {
        id: "coverage-drop-contract",
        gate: "coverage",
        why: "a contract def disappears and its component's cells go uncovered",
        files: &["tools/contracts/components/dialog.mjs"],
        resolve: None,
        apply: apply_coverage_drop_contract,
    },
    Mutation {
        id: "css-direction-new-physical",
        gate: "css-direction",
        why: "a new physical (non-logical) direction utility enters the emitted css",
        files: &["dist/shadless.css"],
        resolve: None,
        apply: apply_css_direction_new_physical,
    },
    Mutation {
        id: "demo-parity-token-drift",
        gate: "demo-parity",
        why: "the --primary token in the shipped stylesheet no longer matches upstream's",
        files: &["dist/out.css"],
        resolve: None,
        apply: apply_demo_parity_token_drift,
    },
    Mutation {
        id: "demo-smoke-console-error",
        gate: "demo-smoke",
        why: "a shipped demo page throws on load",
        files: &["dist/components/accordion.html"],
        resolve: None,
        apply: apply_demo_smoke_console_error,
    },
    Mutation {
        id: "dist-complete-drop-component",
        gate: "dist-complete",
        why: "dist/out.css loses a component's slot rules (a partial-build out.css got committed)",
        files: &["dist/out.css"],
        resolve: None,
        apply: apply_dist_complete_drop_component,
    },
    Mutation {
        id: "docs-consistency-react-import",
        gate: "docs-consistency",
        why: "a built page teaches `@/components/ui` again after an upstream mdx reshape",
        files: &["docs/site/content/components/accordion.md"],
        resolve: None,
        apply: apply_docs_consistency_react_import,
    },
    Mutation {
        id: "docs-fidelity-drop-heading",
        gate: "docs-fidelity",
        why: "a built page silently loses a heading its mdx source has",
        files: &["docs/site/content/components/accordion.md"],
        resolve: None,
        apply: apply_docs_fidelity_drop_heading,
    },
    Mutation {
        id: "docs-smoke-broken-iframe",
        gate: "docs-smoke",
        why: "a preview iframe points at a page that does not exist",
        files: &["docs/site/public/components/accordion/index.html"],
        resolve: None,
        apply: apply_docs_smoke_broken_iframe,
    },
    Mutation {
        id: "pin-base-drift",
        gate: "pin",
        why: "pin.json names a different one of upstream's parallel registries than the graph converts from",
        files: &["src/registry/pin.json"],
        resolve: None,
        apply: apply_pin_base_drift,
    },
    Mutation {
        id: "rtl-dict-missing-dictionary",
        gate: "rtl-dict",
        why: "an upstream -rtl example loses its `translations` object — the extraction must fail instead of dropping that language set",
        files: &[".upstream/shadcn-ui/apps/v4/examples/aria/alert-rtl.tsx"],
        resolve: None,
        apply: apply_rtl_dict_missing_dictionary,
    },
    Mutation {
        id: "example-oracle-render-failure",
        gate: "example-oracle",
        why: "an upstream example stops rendering — the build must fail instead of dropping that page from the ownership manifest",
        files: &[],
        resolve: Some(resolve_owned_example),
        apply: apply_example_oracle_render_failure,
    },
    Mutation {
        id: "example-perturb-shipped",
        gate: "example-gate",
        why: "a shipped demo page drifts from the React oracle render of its example",
        files: &[],
        resolve: Some(|root| {
            Ok(vec![mut_find_file(
                root,
                "docs/demos",
                |s| s.contains(r#"data-slot="badge""#),
                &[],
            )?])
        }),
        apply: apply_example_perturb_shipped,
    },
    Mutation {
        id: "golden-perturb-oracle",
        gate: "golden-gate",
        why: "the local oracle render stops matching the recorded live-site snapshot",
        files: &["src/registry/upstream-snapshot/accordion.json"],
        resolve: None,
        apply: apply_golden_perturb_oracle,
    },
    Mutation {
        id: "interactivity-strip-script",
        gate: "interactivity-sweep",
        why: "an interactive example ships without its behavior — a dead button",
        files: &["docs/site/static/demos/dialog.html"],
        resolve: None,
        apply: apply_interactivity_strip_script,
    },
    Mutation {
        id: "ledger-budget-exceeded",
        gate: "ledger",
        why: "the count of golden-exempt demos grows past its recorded budget",
        files: &["src/registry/upstream-snapshot/exemptions.json"],
        resolve: None,
        apply: apply_ledger_budget_exceeded,
    },
    Mutation {
        id: "ledger-undocumented-exemption",
        gate: "ledger",
        why: "a golden exemption exists in the source with no ledger entry",
        files: &["src/registry/upstream-snapshot/exemptions.json"],
        resolve: None,
        apply: apply_ledger_undocumented_exemption,
    },
    Mutation {
        id: "overlay-orphaned-rule",
        gate: "overlay",
        why: "a conversion rule's anchor no longer exists (contract ignoreAttrs exempts a slot nothing emits)",
        files: &["tools/contracts/components/accordion.mjs"],
        resolve: None,
        apply: apply_overlay_orphaned_rule,
    },
    Mutation {
        id: "overlay-stale-authored",
        gate: "overlay",
        why: "the upstream input a kernel fixture was written against has changed",
        files: &["overlays/manifest.json"],
        resolve: None,
        apply: apply_overlay_stale_authored,
    },
    Mutation {
        id: "script-refs-dead-node-call",
        gate: "script-refs",
        why: "a package.json script calls `node` on a file that does not exist — the exact shape of the bug this gate exists to catch (a JS tool deleted during the Go port, an npm script left pointing at it)",
        files: &["package.json"],
        resolve: None,
        apply: apply_script_refs_dead_node_call,
    },
    Mutation {
        id: "pack-broken-export",
        gate: "pack",
        why: "package.json exports point at a file the tarball does not carry (a README-documented specifier that cannot resolve)",
        files: &["package.json"],
        resolve: None,
        apply: apply_pack_broken_export,
    },
    Mutation {
        id: "path-parity-drop-utility",
        gate: "path-parity",
        why: "badge's slot rule drops its padding — css-import consumers get an unpadded badge",
        files: &["dist/css/badge.css"],
        resolve: None,
        apply: apply_path_parity_drop_utility,
    },
    Mutation {
        id: "pin-commit-drift",
        gate: "pin",
        why: "pin.json records a commit the .upstream checkout is not sitting at",
        files: &["src/registry/pin.json"],
        resolve: None,
        apply: apply_pin_commit_drift,
    },
    Mutation {
        id: "product-drop-slot-rule",
        gate: "product-verify",
        why: "the product build loses a component's slot rules",
        files: &["dist/shadless.full.css"],
        resolve: None,
        apply: apply_product_drop_slot_rule,
    },
    Mutation {
        id: "reproducible-hand-edit",
        gate: "reproducible",
        why: "a shipped file differs from what the pipeline produces",
        files: &["dist/components/badge.html"],
        resolve: None,
        apply: apply_reproducible_hand_edit,
    },
    Mutation {
        id: "style-parity-perturb-padding",
        gate: "style-parity",
        why: "a shipped fixture's computed padding diverges from the oracle on one slot",
        files: &["tools/contracts/out/dialog/shadless.html"],
        resolve: None,
        apply: apply_style_parity_perturb_padding,
    },
    Mutation {
        id: "style-parity-recorded-value-drift",
        gate: "style-parity",
        why: "a cell that is already on the baseline starts differing by a different amount",
        files: &["gates/style-parity-baseline.json"],
        resolve: None,
        apply: apply_style_parity_recorded_value_drift,
    },
    Mutation {
        id: "unit-break-pure-fn",
        gate: "unit",
        why: "a pure helper (splitMarkers) stops separating marker classes from utilities",
        files: &["src/emitter/css.mjs"],
        resolve: None,
        apply: apply_unit_break_pure_fn,
    },
    Mutation {
        id: "typecheck-break-ir-contract",
        gate: "typecheck",
        why: "the IR contract types drift from what the code passes (a hint stops being a tag map)",
        files: &["src/tags.mjs"],
        resolve: None,
        apply: apply_typecheck_break_ir_contract,
    },
    Mutation {
        id: "variant-merge-defaults",
        gate: "path-parity",
        why: "default-variant utilities cascade into non-default variants",
        files: &["dist/css/button.css"],
        resolve: None,
        apply: apply_variant_merge_defaults,
    },
];

pub fn mutation_by_id(id: &str) -> Option<&'static Mutation> {
    MUTATIONS.iter().find(|m| m.id == id)
}

// ------------------------------------------------------------- snapshot

/// The saved state of the files a mutation is about to touch. It records the
/// REQUESTED paths, not just the ones that existed: restore has to handle
/// both directions.
pub struct Snapshot {
    root: std::path::PathBuf,
    request: Vec<String>,
    content: HashMap<String, Vec<u8>>,
}

use std::collections::{HashMap, HashSet};

pub fn take_snapshot(root: &Path, paths: &[String]) -> Result<Snapshot, String> {
    let mut content = HashMap::new();
    for p in paths {
        match std::fs::read(root.join(p)) {
            Ok(b) => {
                content.insert(p.clone(), b);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("snapshotting {}: {}", p, e)),
        }
    }
    Ok(Snapshot {
        root: root.to_path_buf(),
        request: paths.to_vec(),
        content,
    })
}

impl Snapshot {
    /// Puts the tree back. It reports errors rather than swallowing them: a
    /// failed restore leaves a mutated file in the working tree, which is far
    /// worse than a failed mutation.
    pub fn restore(&self) -> Result<(), String> {
        let mut errs: Vec<String> = Vec::new();
        for p in &self.request {
            let full = self.root.join(p);
            if let Some(b) = self.content.get(p) {
                if let Err(e) = std::fs::write(&full, b) {
                    errs.push(format!("{}: {}", p, e));
                }
                continue;
            }
            if let Err(e) = std::fs::remove_file(&full) {
                if e.kind() != std::io::ErrorKind::NotFound {
                    errs.push(format!("{}: {}", p, e));
                }
            }
        }
        if !errs.is_empty() {
            return Err(format!(
                "RESTORE FAILED (the working tree is still mutated):\n  {}",
                errs.join("\n  ")
            ));
        }
        Ok(())
    }
}

// activeRestore: the restore hook for the mutation currently applied, so an
// interrupt between apply and restore can still put the tree back.
static ACTIVE_RESTORE: Mutex<Option<Box<dyn FnOnce() -> Result<(), String> + Send>>> =
    Mutex::new(None);

pub fn set_active_restore(f: impl FnOnce() -> Result<(), String> + Send + 'static) {
    *ACTIVE_RESTORE.lock().unwrap() = Some(Box::new(f));
}

/// Undoes whatever mutation is applied right now, if any. Safe to call twice.
pub fn restore_active_mutation() -> Result<(), String> {
    let f = ACTIVE_RESTORE.lock().unwrap().take();
    match f {
        None => Ok(()),
        Some(f) => f(),
    }
}

// ------------------------------------------------------------- harness

/// Executes a gate's commands in order and reports whether it went red.
pub fn run_gate(root: &Path, n: &crate::nodes::Node) -> (bool, String) {
    let mut buf = String::new();
    for argv in &n.run {
        let exe = match crate::engine::resolve_argv0(&argv[0]) {
            Ok(p) => p,
            Err(e) => {
                buf.push_str(&format!("fork/exec {}: {}", argv[0], e));
                return (true, buf);
            }
        };
        let out = std::process::Command::new(exe)
            .args(&argv[1..])
            .current_dir(root)
            .output();
        match out {
            Ok(o) => {
                buf.push_str(&String::from_utf8_lossy(&o.stdout));
                buf.push_str(&String::from_utf8_lossy(&o.stderr));
                if !o.status.success() {
                    return (true, buf);
                }
            }
            Err(e) => {
                buf.push_str(&e.to_string());
                return (true, buf);
            }
        }
    }
    (false, buf)
}

/// One row of the meta report.
pub struct MutationResult {
    pub id: &'static str,
    pub gate: &'static str,
    pub caught: bool,
    pub note: String,
}

/// Applies one mutation, runs its gate, and restores the tree — including
/// when the mutation itself errors. The restore is the important part: it
/// must happen on every path out of this function.
pub fn run_mutation(
    root: &Path,
    g: &crate::graph::Graph,
    m: &Mutation,
) -> Result<(MutationResult, Option<String>), String> {
    let mut res = MutationResult {
        id: m.id,
        gate: m.gate,
        caught: false,
        note: String::new(),
    };
    let Some(n) = g.node(m.gate) else {
        res.note = format!("unknown gate {:?}", m.gate);
        return Ok((res, None));
    };
    // Resolved before the snapshot on purpose: a mutation that locates its
    // target inside a build artifact must fail loudly on an unbuilt tree.
    let files = match m.targets(root) {
        Ok(f) => f,
        Err(e) => {
            res.note = format!("could not resolve target: {}", e);
            return Ok((res, None));
        }
    };
    let snap = match take_snapshot(root, &files) {
        Ok(s) => s,
        Err(e) => {
            res.note = format!("could not snapshot: {}", e);
            return Ok((res, None));
        }
    };
    let root_c = root.to_path_buf();
    set_active_restore(move || snap.restore_at(&root_c));
    let out = (| | -> Result<(MutationResult, Option<String>), String> {
        if let Err(e) = (m.apply)(root, &files) {
            res.note = format!("mutation itself errored: {}", first_line(&e));
            return Ok((res, None));
        }
        let (red, _) = run_gate(root, n);
        res.caught = red;
        Ok((res, None))
    })();
    let restore_err = match restore_active_mutation() {
        Ok(()) => None,
        Err(e) => Some(e),
    };
    match out {
        Ok((res, _)) => Ok((res, restore_err)),
        Err(e) => Err(e),
    }
}

impl Snapshot {
    fn restore_at(&self, _root: &Path) -> Result<(), String> {
        self.restore()
    }
}

fn first_line(s: &str) -> String {
    match s.find('\n') {
        Some(i) => s[..i].to_string(),
        None => s.to_string(),
    }
}

fn tier_rank(t: &str) -> usize {
    // graph.rs TIERS: ["fast", "full"]; unknown sorts last
    match t {
        "fast" => 0,
        "full" => 1,
        _ => 2,
    }
}

/// Narrows the set the way gates/meta.mjs did: by explicit id list, or by the
/// effective tier of each mutation's gate.
pub fn select_mutations(
    g: &crate::graph::Graph,
    muts: &[&'static Mutation],
    only: &str,
    tier: &str,
) -> Result<Vec<&'static Mutation>, String> {
    let out: Vec<&'static Mutation> = if !only.is_empty() {
        let mut want: std::collections::HashMap<String, bool> = only
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .map(|s| (s, true))
            .collect();
        let mut out = Vec::new();
        for m in muts {
            if want.remove(m.id).unwrap_or(false) {
                out.push(*m);
            }
        }
        if !want.is_empty() {
            let mut missing: Vec<&String> = want.keys().collect();
            missing.sort();
            return Err(format!(
                "unknown mutation(s): {}",
                missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
        out
    } else if !tier.is_empty() {
        let max = tier_rank(tier);
        if max >= 2 {
            return Err(format!("unknown tier: {} (fast|full)", tier));
        }
        muts
            .iter()
            .filter(|m| g.node(m.gate).is_some())
            .filter(|m| tier_rank(&g.effective_tier(m.gate)) <= max)
            .copied()
            .collect()
    } else {
        muts.to_vec()
    };
    if out.is_empty() {
        return Err("no mutations selected".to_string());
    }
    Ok(out)
}

// ------------------------------------------------------------- wiring

/// Cross-checks the graph against the mutation set in both directions and
/// returns every problem found, so one run reports all of them.
pub fn meta_wiring(g: &crate::graph::Graph, muts: &[&'static Mutation]) -> Vec<String> {
    let mut by_id: HashMap<&str, &Mutation> = HashMap::new();
    for m in muts {
        if by_id.contains_key(m.id) {
            // a duplicate id would shadow one definition and silently halve
            // the proven set
            continue;
        }
        by_id.insert(m.id, m);
    }
    let mut problems: Vec<String> = Vec::new();
    for id in g.ids() {
        let Some(n) = g.node(id) else { continue };
        if n.kind != "gate" {
            continue;
        }
        if n.mutations.is_empty() {
            problems.push(format!("gate {}: declares no mutation — unproven", id));
        }
        for mid in &n.mutations {
            if !by_id.contains_key(mid.as_str()) {
                problems.push(format!(
                    "gate {}: declares mutation {:?} but no such mutation is defined in mutations.rs",
                    id, mid
                ));
            }
        }
        if n.why.is_empty() {
            problems.push(format!(
                "gate {}: no Why — a gate nobody can explain cannot be reviewed",
                id
            ));
        }
    }
    let mut seen: HashSet<&str> = HashSet::new();
    for m in muts {
        if !seen.insert(m.id) {
            problems.push(format!("mutation {}: defined twice", m.id));
        }
        match g.node(m.gate) {
            None => problems.push(format!(
                "mutation {}: targets unknown gate {:?}",
                m.id, m.gate
            )),
            Some(n) => {
                if !n.mutations.iter().any(|x| x == m.id) {
                    problems.push(format!(
                        "mutation {}: gate {:?} does not list it in nodes.rs",
                        m.id, m.gate
                    ));
                }
            }
        }
        if m.why.is_empty() {
            problems.push(format!(
                "mutation {}: no Why — say which real bug class this proves",
                m.id
            ));
        }
        if m.files.is_empty() == m.resolve.is_none() {
            problems.push(format!(
                "mutation {}: set exactly one of Files or Resolve",
                m.id
            ));
        }
    }
    problems.sort();
    problems
}

/// Lists build nodes with no gate anywhere downstream: artifacts that ship
/// without anything asserting they are correct. Reported, not fatal.
pub fn ungated_builds(g: &crate::graph::Graph) -> Vec<String> {
    let mut gated: HashSet<String> = HashSet::new();
    for id in g.ids() {
        let Some(n) = g.node(id) else { continue };
        if n.kind != "gate" {
            continue;
        }
        if let Ok(closure) = g.plan(&[id.clone()]) {
            for d in closure {
                gated.insert(d.id.clone());
            }
        }
    }
    let mut out: Vec<String> = Vec::new();
    for id in g.ids() {
        if let Some(n) = g.node(id) {
            if n.kind == "build" && !gated.contains(id) {
                out.push(id.clone());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tree(files: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "shadless-mut-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        for (p, c) in files {
            let full = root.join(p);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, c).unwrap();
        }
        root
    }

    fn read(root: &Path, p: &str) -> String {
        std::fs::read_to_string(root.join(p)).unwrap()
    }

    fn all_muts() -> Vec<&'static Mutation> {
        MUTATIONS.iter().collect()
    }

    /// Go TestUnitMutEditRejectsNoOp.
    #[test]
    fn unit_mut_edit_rejects_no_op() {
        let root = tree(&[("a.txt", "original")]);
        let err = mut_edit(&root, "a.txt", |s| Ok(s.to_string())).unwrap_err();
        assert!(err.contains("no-op"), "{}", err);
    }

    /// Go TestUnitMutEditRejectsMissingFile.
    #[test]
    fn unit_mut_edit_rejects_missing_file() {
        let root = tree(&[]);
        let err = mut_edit(&root, "a.txt", |s| Ok(s.to_string())).unwrap_err();
        assert!(err.contains("target missing"), "{}", err);
    }

    /// Go TestUnitMutReplaceOnce / ReplaceAll / ReplaceReFirstMatchOnly.
    #[test]
    fn unit_mut_replace_once_and_all() {
        let root = tree(&[("a.txt", "x y x")]);
        mut_replace_once(&root, "a.txt", "x", "z").unwrap();
        assert_eq!(read(&root, "a.txt"), "z y x");

        mut_replace_all(&root, "a.txt", "y", "w").unwrap();
        assert_eq!(read(&root, "a.txt"), "z w x");
        mut_replace_all(&root, "a.txt", "x", "q").unwrap();
        assert_eq!(read(&root, "a.txt"), "z w q");
        mut_replace_all(&root, "a.txt", "gone", "?").unwrap_err(); // anchor missing
    }

    #[test]
    fn unit_mut_replace_re_first_match_only() {
        let root = tree(&[("a.txt", "v1 v2")]);
        mut_replace_re(&root, "a.txt", &Regex::new(r"v\d").unwrap(), "N").unwrap();
        assert_eq!(read(&root, "a.txt"), "N v2");
    }

    /// Go TestUnitMutReplaceReExpandsGroups.
    #[test]
    fn unit_mut_replace_re_expands_groups() {
        let root = tree(&[(
            "e.tsx",
            "export default function Demo() {\n  return null\n}\n",
        )]);
        mut_replace_re(
            &root,
            "e.tsx",
            re_export_default_fn(),
            "$1\n  throw new Error(\"x\")",
        )
        .unwrap();
        let got = read(&root, "e.tsx");
        assert!(
            got.contains("export default function Demo() {\n  throw new Error(\"x\")"),
            "group expansion did not keep the signature: {:?}",
            got
        );
    }

    /// Go TestUnitMutReplaceReRejectsMissingPattern.
    #[test]
    fn unit_mut_replace_re_rejects_missing_pattern() {
        let root = tree(&[("a.json", r#"{"commit": "nope"}"#)]);
        assert!(mut_replace_re(&root, "a.json", re_commit(), "x").is_err());
    }

    /// Go TestUnitMutFindFileIsDeterministic.
    #[test]
    fn unit_mut_find_file_is_deterministic() {
        let root = tree(&[
            ("d/b.html", "<i data-slot=\"badge\">"),
            ("d/a.html", "<i data-slot=\"badge\">"),
            ("d/c.html", "nothing"),
        ]);
        let want = "d/a.html"; // sorted, so the choice cannot drift with readdir order
        for _ in 0..3 {
            let got = mut_find_file(&root, "d", |s| s.contains("data-slot=\"badge\""), &[])
                .unwrap();
            assert_eq!(got, want);
        }
        assert!(mut_find_file(&root, "d", |_| false, &[]).is_err());
    }

    /// Go TestUnitInsertAfter.
    #[test]
    fn unit_insert_after() {
        let (got, ok) = insert_after(
            "{\"examples\": {\n  \"a\": {}",
            "\"examples\": {",
            "\n  \"new\": {},",
        );
        assert!(ok);
        assert!(
            got.starts_with("{\"examples\": {\n  \"new\": {},"),
            "insert landed in the wrong place: {:?}",
            got
        );
        let (_, ok) = insert_after("abc", "zzz", "x");
        assert!(!ok, "a missing anchor reported success");
    }

    /// Go TestUnitJSONMutationsAreMinimal: the JSON-shaped mutations must
    /// change exactly one thing.
    #[test]
    fn unit_json_mutations_are_minimal() {
        const EXEMPTIONS: &str = r#"{
 "examples": {
  "attachment-demo": {
   "reason": "token drift vs live"
  }
 }
}
"#;
        for id in ["ledger-undocumented-exemption", "ledger-budget-exceeded"] {
            let m = mutation_by_id(id).unwrap();
            let root = tree(&[(m.files[0], EXEMPTIONS)]);
            (m.apply)(&root, &[m.files[0].to_string()]).unwrap();
            let got = read(&root, m.files[0]);
            assert!(
                got.contains("\"attachment-demo\"") && got.contains("\"reason\": \"token drift vs live\""),
                "{} reformatted or dropped the existing entry:\n{}",
                id,
                got
            );
            assert!(got.contains("__mutation"), "{} did not add its entry:\n{}", id, got);
        }
        // budget-exceeded must REUSE an existing reason, or it is just the
        // undocumented-exemption mutation under another name
        let m = mutation_by_id("ledger-budget-exceeded").unwrap();
        let root = tree(&[(m.files[0], EXEMPTIONS)]);
        (m.apply)(&root, &[m.files[0].to_string()]).unwrap();
        let n = read(&root, m.files[0])
            .matches("\"reason\": \"token drift vs live\"")
            .count();
        assert_eq!(n, 2, "budget mutation should reuse the existing reason (want 2 copies, got {})", n);
    }

    /// Go TestUnitOverlayStaleAuthoredZeroesOnlyItsUnit.
    #[test]
    fn unit_overlay_stale_authored_zeroes_only_its_unit() {
        let m = mutation_by_id("overlay-stale-authored").unwrap();
        let manifest = r#"{
  "units": {
    "behavior:dialog": {
      "hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    },
    "behavior:sheet": {
      "hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    }
  }
}
"#;
        let root = tree(&[(m.files[0], manifest)]);
        (m.apply)(&root, &[m.files[0].to_string()]).unwrap();
        let got = read(&root, m.files[0]);
        assert!(got.contains(&"0".repeat(64)), "the sheet hash was not zeroed");
        assert!(
            got.contains(&"a".repeat(64)),
            "it also changed behavior:dialog — the mutation must touch one unit"
        );
    }

    /// Go TestUnitStyleParityValueDriftNeedsRecordedValues.
    #[test]
    fn unit_style_parity_value_drift_needs_recorded_values() {
        let m = mutation_by_id("style-parity-recorded-value-drift").unwrap();
        // a baseline that still records bare ids proves nothing about whether
        // the gate reads values, so the mutation must refuse rather than no-op
        let root = tree(&[(m.files[0], r#"{"cells": ["some-id"]}"#)]);
        assert!((m.apply)(&root, &[m.files[0].to_string()]).is_err());
        let root = tree(&[(
            m.files[0],
            r#"{"cells": [{"id": "x", "shadless": "4px", "oracle": "8px"}]}"#,
        )]);
        (m.apply)(&root, &[m.files[0].to_string()]).unwrap();
        let got = read(&root, m.files[0]);
        assert!(got.contains("999px"), "the recorded value was not perturbed: {}", got);
        assert!(got.contains("\"oracle\": \"8px\""), "it changed more than the one cell value: {}", got);
    }

    /// Go TestUnitSnapshotRestoresContent / RestoresDeletedFile /
    /// RemovesCreatedFile / ActiveRestoreIsIdempotent.
    #[test]
    fn unit_snapshot_semantics() {
        let root = tree(&[("a.txt", "original")]);
        let snap = take_snapshot(&root, &["a.txt".to_string()]).unwrap();
        std::fs::write(root.join("a.txt"), b"mutated").unwrap();
        snap.restore().unwrap();
        assert_eq!(read(&root, "a.txt"), "original");

        // coverage-drop-contract DELETES its target, so restore recreates it
        std::fs::remove_file(root.join("a.txt")).unwrap();
        snap.restore().unwrap();
        assert_eq!(read(&root, "a.txt"), "original");

        // a file that did NOT exist must be removed again
        let root2 = tree(&[]);
        let snap2 = take_snapshot(&root2, &["new.txt".to_string()]).unwrap();
        std::fs::write(root2.join("new.txt"), b"x").unwrap();
        snap2.restore().unwrap();
        assert!(!root2.join("new.txt").exists(), "restore left behind a file the snapshot never had");
    }

    #[test]
    fn unit_active_restore_is_idempotent() {
        let root = tree(&[("a.txt", "original")]);
        let snap = take_snapshot(&root, &["a.txt".to_string()]).unwrap();
        let root_c = root.clone();
        set_active_restore(move || snap.restore_at(&root_c));
        std::fs::write(root.join("a.txt"), b"mutated").unwrap();
        restore_active_mutation().unwrap();
        // the second call must be a no-op rather than restoring a stale
        // snapshot over later work
        std::fs::write(root.join("a.txt"), b"later").unwrap();
        restore_active_mutation().unwrap();
        assert_eq!(read(&root, "a.txt"), "later", "a second restore clobbered later work");
    }

    /// Go TestUnitSelectMutations.
    #[test]
    fn unit_select_mutations() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => PathBuf::from(r),
            Err(_) => {
                let m = Path::new(env!("CARGO_MANIFEST_DIR")).join("../shadless");
                m.canonicalize().unwrap_or(m)
            }
        };
        let _ = &root; // tier selection reads the authored graph, not the tree
        let g = match crate::graph::Graph::new(crate::nodes::all()) {
            Ok(g) => g,
            Err(e) => panic!("authored graph: {}", e),
        };
        let all = select_mutations(&g, &all_muts(), "", "").unwrap();
        assert_eq!(all.len(), MUTATIONS.len());
        let one = select_mutations(&g, &all_muts(), "pin-commit-drift", "").unwrap();
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].id, "pin-commit-drift");
        assert!(select_mutations(&g, &all_muts(), "no-such-mutation", "").is_err());
        let fast = select_mutations(&g, &all_muts(), "", "fast").unwrap();
        assert!(!fast.is_empty() && fast.len() < MUTATIONS.len(),
            "tier=fast selected {} of {} — expected a proper subset", fast.len(), MUTATIONS.len());
        for m in &fast {
            assert!(
                tier_rank(&g.effective_tier(m.gate)) <= tier_rank("fast"),
                "{} targets {}, whose effective tier is {}",
                m.id, m.gate, g.effective_tier(m.gate)
            );
        }
        assert!(select_mutations(&g, &all_muts(), "", "nonsense").is_err());
    }

    /// Go TestUnitRunGateDetectsRedAndGreen.
    #[test]
    fn unit_run_gate_detects_red_and_green() {
        let root = std::env::temp_dir();
        let node = |cmds: &[&[&str]]| crate::nodes::Node {
            id: "t".into(),
            kind: "gate".into(),
            tier: "fast".into(),
            needs: vec![],
            run: cmds.iter().map(|c| c.iter().map(|s| s.to_string()).collect()).collect(),
            inputs: None,
            produces: None,
            why: String::new(),
            mutations: vec![],
        };
        let (red, _) = run_gate(&root, &node(&[&["false"]]));
        assert!(red, "a failing command was not reported as red");
        let (red, _) = run_gate(&root, &node(&[&["true"]]));
        assert!(!red, "a passing command was reported as red");
        // commands run in order and the first failure stops the gate
        let (red, _) = run_gate(&root, &node(&[&["true"], &["false"], &["true"]]));
        assert!(red, "a failure in a later command was missed");
    }

    /// Go TestUnitMutationRegexesCompile.
    #[test]
    fn unit_mutation_regexes_compile() {
        let cases: Vec<(&str, &Regex, String)> = vec![
            ("commit", re_commit(), format!("\"commit\": \"{}\"", "a".repeat(40))),
            ("primary", re_primary_token(), "--primary: oklch(0.2 0 0);".into()),
            ("padding", re_padding(), "flex px-2.5 items-center".into()),
            ("dialog script", re_dialog_script(), "<script src=\"../js/dialog.js\"></script>".into()),
            ("reason", re_first_reason(), "\"reason\": \"because\"".into()),
            ("shadless cell", re_first_shadless_cell(), "\"shadless\": \"4px\"".into()),
        ];
        for (name, re, input) in cases {
            assert!(re.is_match(&input), "{}: {} does not match {:?}", name, re.as_str(), input);
        }
    }

    /// Go TestUnitMetaWiring — every gate is proven by at least one mutation
    /// that exists, every mutation targets a gate that lists it, and both
    /// carry a Why. Pure — reads the graph and the mutation set, executes
    /// nothing.
    #[test]
    fn unit_meta_wiring() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => PathBuf::from(r),
            Err(_) => {
                let m = Path::new(env!("CARGO_MANIFEST_DIR")).join("../shadless");
                m.canonicalize().unwrap_or(m)
            }
        };
        let g = match crate::graph::Graph::new(crate::nodes::all()) {
            Ok(g) => g,
            Err(e) => panic!("authored graph: {}", e),
        };
        let problems = meta_wiring(&g, &all_muts());
        assert!(
            problems.is_empty(),
            "FAIL  meta (graph/mutation wiring)\n  {}",
            problems.join("\n  ")
        );
        let gates = g.ids().iter().filter(|id| g.node(id).map(|n| n.kind == "gate").unwrap_or(false)).count();
        eprintln!("PASS  meta-wiring ({} gates, {} mutations, every gate proven)", gates, MUTATIONS.len());
    }
}
