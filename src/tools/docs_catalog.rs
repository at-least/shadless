//! Port of pipeline/docs_catalog.go — enumerate every ComponentPreview +
//! ComponentSource name referenced by the docs mirror set and write
//! docs/catalog.json.
//!
//! Status rules (recorded in PLAN §Wave F):
//!
//!   preview "X-demo" + dist/components/X.html exists -> existing-dist (primary
//!   demos reuse dist/, per scope decision v2); every other name -> to-author
//!   (docs/demos/<name>.html, authored in FT7 waves).
//!   ComponentSource name=X -> dist/components/X.html (existing-dist | no-dist).
//!   Previews whose component is tombstoned (no implementation in shadless) get
//!   status="tombstoned" instead of "to-author" — they can never be authored.

use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

use super::docs_transforms::{
    attr_of, attr_or_absent, attr_or_null, grey_components, strip_fences, OptStr,
};
use crate::jsonorder::{marshal_js, Json, JsonObj};

const RADIX_DIR: &str = ".upstream/shadcn-ui/apps/v4/content/docs/components/radix";
const DIST_COMPS: &str = "dist/components";
const CATALOG_OUT: &str = "docs/catalog.json";

fn re_docs_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<(ComponentPreview|ComponentSource)\b([^>]*)>").unwrap())
}
fn re_preview_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<ComponentPreview\b([^>]*)>").unwrap())
}
fn re_primary() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^([a-z0-9-]+)-demo$").unwrap())
}
fn re_hooks() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(
            r"\b(useState|useEffect|useRef|useContext|useMemo|useCallback|useReducer|useLayoutEffect|useImperativeHandle|useId|useTransition|useDeferredValue|useSyncExternalStore|useInsertionEffect)\b",
        )
        .unwrap()
    })
}

#[derive(Clone, Default)]
struct PreviewRec {
    name: String,
    component: String,
    style_name: OptStr,
    description: OptStr,
    host_pages: Vec<String>,
    status: String,
    demo_path: String, // "" with demo_null means an explicit null
    demo_null: bool,
    quality: String,
}

#[derive(Clone, Default)]
struct SourceRec {
    name: String,
    component: String,
    host_pages: Vec<String>,
    status: String,
    demo_path: String,
    demo_null: bool,
}

#[derive(Clone, Default)]
struct FlagRec {
    file: String,
    kind: String,
    reason: String,
    src: OptStr,
}

#[derive(Default)]
struct ScanResult {
    key: String,
    dir: String,
    files: usize,
    preview_tags: usize,
    multiline_tag: usize,
    previews: Vec<PreviewRec>,
    sources: Vec<SourceRec>,
    flags: Vec<FlagRec>,
}

fn scan_set(root: &Path, key: &str, dir: &str) -> Result<ScanResult, String> {
    let mut r = ScanResult {
        key: key.to_string(),
        dir: dir.to_string(),
        ..ScanResult::default()
    };
    let ents = std::fs::read_dir(root.join(dir)).map_err(|e| e.to_string())?;
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if !n.ends_with(".mdx") {
            continue;
        }
        r.files += 1;
        let component = n.trim_end_matches(".mdx").to_string();
        let b = std::fs::read_to_string(root.join(dir).join(&n)).map_err(|e| e.to_string())?;
        let text = strip_fences(&b);
        for caps in re_docs_tag().captures_iter(&text) {
            let (tag, attrs) = (&caps[1], &caps[2]);
            if tag == "ComponentPreview" {
                r.preview_tags += 1;
                if attrs.contains('\n') {
                    r.multiline_tag += 1;
                }
                let (name, ok) = attr_of(attrs, "name");
                if !ok || name.is_empty() {
                    r.flags.push(FlagRec {
                        file: n.clone(),
                        kind: tag.to_string(),
                        reason: "no name= attr".to_string(),
                        ..FlagRec::default()
                    });
                    continue;
                }
                r.previews.push(PreviewRec {
                    name,
                    component: component.clone(),
                    style_name: attr_or_null(attrs, "styleName"),
                    description: attr_or_null(attrs, "description"),
                    ..PreviewRec::default()
                });
                continue;
            }
            let (name, ok) = attr_of(attrs, "name");
            if !ok || name.is_empty() {
                r.flags.push(FlagRec {
                    file: n.clone(),
                    kind: tag.to_string(),
                    reason: "no name= attr".to_string(),
                    src: attr_or_null(attrs, "src"),
                });
                continue;
            }
            r.sources.push(SourceRec {
                name,
                component: component.clone(),
                ..SourceRec::default()
            });
        }
    }
    Ok(r)
}

// FT8: guide previews (mode-toggle, card-rtl, …) live on guide pages, not in
// the radix subtree.
const GUIDE_SOURCES: [(&str, &str); 5] = [
    ("installation", "docs/content/installation.mdx"),
    ("dark-mode", "docs/content/dark-mode.mdx"),
    ("rtl", ".upstream/shadcn-ui/apps/v4/content/docs/rtl/index.mdx"),
    ("shimmer", ".upstream/shadcn-ui/apps/v4/content/docs/utils/shimmer.mdx"),
    ("scroll-fade", ".upstream/shadcn-ui/apps/v4/content/docs/utils/scroll-fade.mdx"),
];

fn scan_guides(root: &Path) -> Result<ScanResult, String> {
    let mut r = ScanResult {
        key: "guides".to_string(),
        dir: "docs/content/ + UP/...".to_string(),
        ..ScanResult::default()
    };
    for (slug, path) in GUIDE_SOURCES {
        let Ok(b) = std::fs::read_to_string(root.join(path)) else {
            continue;
        };
        r.files += 1;
        let text = strip_fences(&b);
        for caps in re_preview_tag().captures_iter(&text) {
            let attrs = &caps[1];
            r.preview_tags += 1;
            if attrs.contains('\n') {
                r.multiline_tag += 1;
            }
            let (name, ok) = attr_of(attrs, "name");
            if !ok || name.is_empty() {
                r.flags.push(FlagRec {
                    file: slug.to_string(),
                    kind: "ComponentPreview".to_string(),
                    reason: "no name= attr".to_string(),
                    ..FlagRec::default()
                });
                continue;
            }
            r.previews.push(PreviewRec {
                name,
                component: slug.to_string(),
                style_name: attr_or_absent(attrs, "styleName"),
                description: attr_or_absent(attrs, "description"),
                ..PreviewRec::default()
            });
        }
    }
    Ok(r)
}

/// dedupeByName merges records by name: a name may be referenced from several
/// pages, FT7 authors it once. hostPages keeps every referencing page (in
/// first-seen order, no duplicates); the defining record's own metadata wins.
fn dedupe_previews(records: &[PreviewRec]) -> Vec<PreviewRec> {
    let mut order: Vec<String> = Vec::new();
    let mut by_name: std::collections::HashMap<String, PreviewRec> =
        std::collections::HashMap::new();
    for r in records {
        let key = r.name.clone();
        let e = by_name.entry(key.clone()).or_insert_with(|| {
            order.push(key);
            PreviewRec {
                name: r.name.clone(),
                component: r.component.clone(),
                style_name: r.style_name.clone(),
                description: r.description.clone(),
                ..PreviewRec::default()
            }
        });
        if !e.host_pages.contains(&r.component) {
            e.host_pages.push(r.component.clone());
        }
    }
    order
        .iter()
        .map(|n| by_name[n].clone())
        .collect()
}

fn dedupe_sources(records: &[SourceRec]) -> Vec<SourceRec> {
    let mut order: Vec<String> = Vec::new();
    let mut by_name: std::collections::HashMap<String, SourceRec> =
        std::collections::HashMap::new();
    for r in records {
        let key = r.name.clone();
        let e = by_name.entry(key.clone()).or_insert_with(|| {
            order.push(key);
            SourceRec {
                name: r.name.clone(),
                component: r.component.clone(),
                ..SourceRec::default()
            }
        });
        if !e.host_pages.contains(&r.component) {
            e.host_pages.push(r.component.clone());
        }
    }
    order
        .iter()
        .map(|n| by_name[n].clone())
        .collect()
}

fn is_tombstone_name(name: &str) -> bool {
    for p in grey_components() {
        if name == *p || name.starts_with(&format!("{}-", p)) {
            return true;
        }
    }
    false
}

fn opt_json(o: &OptStr) -> Option<Json> {
    if !o.present {
        return None;
    }
    if o.null {
        return Some(Json::Null);
    }
    Some(Json::Str(o.val.clone()))
}

pub fn run_docs_catalog(root: &Path) -> i32 {
    let fail = |e: String| -> i32 {
        eprintln!("docs-catalog: {}", e);
        1
    };
    let exists = |p: &str| root.join(p).exists();

    let pin_raw = match std::fs::read_to_string(root.join("src/registry/pin.json")) {
        Ok(b) => b,
        Err(e) => return fail(e.to_string()),
    };
    #[derive(Deserialize)]
    struct Pin {
        #[serde(rename = "shadcn_ui", default)]
        shadcn_ui: PinShadcnUi,
    }
    #[derive(Deserialize, Default)]
    struct PinShadcnUi {
        #[serde(default)]
        repo: String,
        #[serde(default)]
        tag: String,
        #[serde(default)]
        commit: String,
    }
    let pin: Pin = match serde_json::from_str(&pin_raw) {
        Ok(p) => p,
        Err(e) => return fail(e.to_string()),
    };
    let tiers_raw = match std::fs::read_to_string(root.join("src/registry/tiers.json")) {
        Ok(b) => b,
        Err(e) => return fail(e.to_string()),
    };
    #[derive(Deserialize, Default)]
    struct TierEntry {
        #[serde(default)]
        tier: String,
    }
    let tiers: std::collections::HashMap<String, TierEntry> =
        match serde_json::from_str(&tiers_raw) {
            Ok(t) => t,
            Err(e) => return fail(e.to_string()),
        };

    let radix = match scan_set(root, "components/radix", RADIX_DIR) {
        Ok(r) => r,
        Err(e) => return fail(e),
    };
    let guides = match scan_guides(root) {
        Ok(g) => g,
        Err(e) => return fail(e),
    };
    let sets = [radix, guides];

    let mut dist_files: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(entries) = std::fs::read_dir(root.join(DIST_COMPS)) {
        for e in entries.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            dist_files.insert(n.trim_end_matches(".html").to_string());
        }
    }
    let dist_path = |name: &str| -> String {
        if dist_files.contains(name) {
            format!("{}/{}.html", DIST_COMPS, name)
        } else {
            String::new()
        }
    };

    // Authored status derives from the FILE SYSTEM, not the previous catalog
    // (Wave H D5): docs/demos/<name>.html existing ⇒ authored.
    let mut prev_authored: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(b) = std::fs::read_to_string(root.join(CATALOG_OUT)) {
        #[derive(Deserialize)]
        struct Prev {
            #[serde(default)]
            previews: Vec<PrevPreview>,
        }
        #[derive(Deserialize)]
        struct PrevPreview {
            #[serde(default)]
            name: String,
            #[serde(default)]
            status: String,
        }
        if let Ok(prev) = serde_json::from_str::<Prev>(&b) {
            for p in prev.previews {
                if p.status == "authored" {
                    prev_authored.insert(p.name);
                }
            }
        }
    }

    // A -demo is kernel iff the underlying component is.
    let is_kernel = |name: &str| -> bool {
        let key = re_primary()
            .captures(name)
            .map(|m| m[1].to_string())
            .unwrap_or_else(|| name.to_string());
        tiers.get(&key).map(|t| t.tier == "kernel").unwrap_or(false)
    };

    // Per-set dedupe (so each set's uniquePreviewNames count stays honest) and
    // a global dedupe across sets.
    let radix_unique = dedupe_previews(&sets[0].previews);
    let guides_unique = dedupe_previews(&sets[1].previews);
    let mut all: Vec<PreviewRec> = sets[0].previews.clone();
    all.extend(sets[1].previews.clone());
    let unique_previews = dedupe_previews(&all);

    let (mut p_existing, mut p_author, mut p_to_author, mut p_tomb, mut unavail) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut previews: Vec<PreviewRec> = Vec::new();
    for mut p in unique_previews {
        let mut d = String::new();
        if let Some(m) = re_primary().captures(&p.name) {
            d = dist_path(&m[1]);
        }
        let direct = dist_path(&p.name);
        let mut is_guide_only = true;
        for h in &p.host_pages {
            if exists(&format!("{}/{}.mdx", RADIX_DIR, h)) {
                is_guide_only = false;
            }
        }
        let is_base_style = p.style_name.present
            && !p.style_name.null
            && p.style_name.val.starts_with("base-");
        let authored_file = format!("docs/demos/{}.html", p.name);
        let has_authored_file = exists(&authored_file);

        if has_authored_file && !is_kernel(&p.name) {
            p.status = "authored".to_string();
            p.demo_path = authored_file;
            p_author += 1;
        } else if has_authored_file {
            // kernel-tier -demo: the authored file exists (the oracle ran), but
            // the iframe must point at the dist fixture, not the oracle static
            // DOM (which cannot open popups). Strip the oracle artifact so the
            // next oracle run sees the dist fixture as canonical.
            if let Err(e) = std::fs::remove_file(root.join(&authored_file)) {
                return fail(e.to_string());
            }
            if !direct.is_empty() {
                p.status = "existing-dist".to_string();
                p.demo_path = direct;
                p_existing += 1;
            } else if !d.is_empty() {
                p.status = "existing-dist".to_string();
                p.demo_path = d;
                p_existing += 1;
            } else {
                p.status = "to-author".to_string();
                p.demo_path = authored_file;
                p_to_author += 1;
            }
        } else if prev_authored.contains(&p.name) {
            // inconsistency: previously authored but the file is gone — loud,
            // not silent (a deleted demo must be a decision, not an accident)
            eprintln!(
                "FAIL catalog: {} was authored but {} is missing",
                p.name, authored_file
            );
            return 1;
        } else if !direct.is_empty() {
            p.status = "existing-dist".to_string();
            p.demo_path = direct;
            p_existing += 1;
        } else if !d.is_empty() {
            p.status = "existing-dist".to_string();
            p.demo_path = d;
            p_existing += 1;
        } else if is_tombstone_name(&p.name) {
            p.status = "tombstoned".to_string();
            p.demo_null = true;
            p_tomb += 1;
        } else if is_guide_only && is_base_style {
            // FT8: guide-only preview with a base-* styleName → unavailable
            // (base-line demo; shadless implements the radix line only)
            p.status = "unavailable".to_string();
            p.demo_null = true;
            unavail += 1;
        } else {
            p.status = "to-author".to_string();
            p.demo_path = authored_file;
            p_to_author += 1;
        }
        previews.push(p);
    }

    let mut sources: Vec<SourceRec> = Vec::new();
    let mut flags: Vec<FlagRec> = Vec::new();
    let mut set_stats = JsonObj::new();
    for s in &sets {
        for mut src in dedupe_sources(&s.sources) {
            let d = dist_path(&src.name);
            if !d.is_empty() {
                src.status = "existing-dist".to_string();
                src.demo_path = d;
            } else {
                src.status = "no-dist".to_string();
                src.demo_null = true;
            }
            sources.push(src);
        }
        flags.extend(s.flags.clone());
        let unique = if s.key == "components/radix" {
            radix_unique.len()
        } else {
            guides_unique.len()
        };
        let (mut source_flags, mut preview_flags) = (0usize, 0usize);
        for f in &s.flags {
            if f.kind == "ComponentSource" {
                source_flags += 1;
            } else {
                preview_flags += 1;
            }
        }
        set_stats = set_stats.add(
            &s.key,
            Json::from_obj(
                JsonObj::new()
                    .add("dir", Json::Str(s.dir.clone()))
                    .add("mdxFiles", Json::Int(s.files as i64))
                    .add("previewTags", Json::Int(s.preview_tags as i64))
                    .add("multilinePreviewTags", Json::Int(s.multiline_tag as i64))
                    .add("uniquePreviewNames", Json::Int(unique as i64))
                    .add(
                        "sourceTags",
                        Json::Int((s.sources.len() + source_flags) as i64),
                    )
                    .add("namedSourceTags", Json::Int(s.sources.len() as i64)),
            ),
        );
        println!(
            "{}: {} mdx, {} preview tags ({} multiline), {} unique preview names, {} source tags ({} named, {} flagged)",
            s.key,
            s.files,
            s.preview_tags,
            s.multiline_tag,
            unique,
            s.sources.len() + s.flags.len() - preview_flags,
            s.sources.len(),
            source_flags
        );
    }
    print!(
        "total previews: {} existing-dist, {} authored, {} to-author, {} tombstoned",
        p_existing, p_author, p_to_author, p_tomb
    );
    if unavail > 0 {
        print!(" + {} unavailable (base-style)", unavail);
    }
    println!();

    // FT8: mark demos whose upstream mdx file uses React hooks as
    // "informational" — they were authored from React example-registry code in
    // FT7 batches, so their interactive semantics aren't validated against the
    // radix oracle the way primary demos are.
    let mut file_has_hooks: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    if let Ok(entries) = std::fs::read_dir(root.join(RADIX_DIR)) {
        for e in entries.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.ends_with(".mdx") {
                continue;
            }
            let Ok(b) = std::fs::read_to_string(root.join(RADIX_DIR).join(&n)) else {
                continue;
            };
            if re_hooks().is_match(&strip_fences(&b)) {
                file_has_hooks.insert(n.trim_end_matches(".mdx").to_string());
            }
        }
    }
    let mut marked = 0usize;
    for p in previews.iter_mut() {
        for h in &p.host_pages {
            if file_has_hooks.contains(h) {
                p.quality = "informational".to_string();
                marked += 1;
                break;
            }
        }
    }
    println!(
        "quality: {} informational (host radix page uses React hooks; not contract-tested)",
        marked
    );

    let mut preview_json: Vec<Json> = Vec::new();
    for p in &previews {
        let mut o = JsonObj::new().add("name", Json::Str(p.name.clone()));
        if let Some(v) = opt_json(&p.style_name) {
            o = o.add("styleName", v);
        }
        if let Some(v) = opt_json(&p.description) {
            o = o.add("description", v);
        }
        o = o
            .add(
                "hostPages",
                Json::Arr(p.host_pages.iter().map(|h| Json::Str(h.clone())).collect()),
            )
            .add("status", Json::Str(p.status.clone()));
        if p.demo_null {
            o = o.add("demoPath", Json::Null);
        } else {
            o = o.add("demoPath", Json::Str(p.demo_path.clone()));
        }
        if !p.quality.is_empty() {
            o = o.add("quality", Json::Str(p.quality.clone()));
        }
        preview_json.push(Json::from_obj(o));
    }
    let mut source_json: Vec<Json> = Vec::new();
    for s in &sources {
        let mut o = JsonObj::new()
            .add("name", Json::Str(s.name.clone()))
            .add(
                "hostPages",
                Json::Arr(s.host_pages.iter().map(|h| Json::Str(h.clone())).collect()),
            )
            .add("status", Json::Str(s.status.clone()));
        if s.demo_null {
            o = o.add("demoPath", Json::Null);
        } else {
            o = o.add("demoPath", Json::Str(s.demo_path.clone()));
        }
        source_json.push(Json::from_obj(o));
    }
    let mut flag_json: Vec<Json> = Vec::new();
    for f in &flags {
        let mut o = JsonObj::new()
            .add("file", Json::Str(f.file.clone()))
            .add("kind", Json::Str(f.kind.clone()))
            .add("reason", Json::Str(f.reason.clone()));
        if let Some(v) = opt_json(&f.src) {
            o = o.add("src", v);
        }
        flag_json.push(Json::from_obj(o));
    }

    let catalog = JsonObj::new()
        .add("version", Json::Int(1))
        .add(
            "generatedFrom",
            Json::from_obj(
                JsonObj::new()
                    .add("repo", Json::Str(pin.shadcn_ui.repo.clone()))
                    .add("tag", Json::Str(pin.shadcn_ui.tag.clone()))
                    .add("commit", Json::Str(pin.shadcn_ui.commit.clone())),
            ),
        )
        .add("sets", Json::from_obj(set_stats))
        .add("previews", Json::Arr(preview_json))
        .add("sources", Json::Arr(source_json))
        .add("flags", Json::Arr(flag_json));

    if let Err(e) = std::fs::create_dir_all(root.join("docs")) {
        return fail(e.to_string());
    }
    if let Err(e) = std::fs::write(
        root.join(CATALOG_OUT),
        format!("{}\n", marshal_js(&Json::from_obj(catalog))),
    ) {
        return fail(e.to_string());
    }
    // radixStats := setStats[0].V.(jsonObj); radixStats[4].V — the
    // uniquePreviewNames value of the first set
    let radix_unique_names = radix_unique.len();
    println!("radix unique preview names: {}", radix_unique_names);
    println!(
        "catalog: {} ({} previews, {} sources, {} flags)",
        CATALOG_OUT,
        previews.len(),
        sources.len(),
        flags.len()
    );
    0
}
