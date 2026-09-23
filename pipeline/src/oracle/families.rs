//! Port of pipeline/docs_families.go — the kernel-tier behavior protocol
//! table (only what example-fixture consumes; the MDX doc builders are
//! Rust — pipeline/src/tools/docs_build.rs).

#[derive(Clone, Debug, Default)]
pub struct FamilyEnt {
    pub kind: &'static str, // dialog | portal | menu | select | nav | inline | none
    pub open: &'static str, // click | hover | contextmenu | ""
    pub attr: &'static str,
    pub js: &'static str,
}

pub fn family(comp: &str) -> Option<FamilyEnt> {
    Some(match comp {
        "alert-dialog" => FamilyEnt { kind: "dialog", js: "alert-dialog", ..Default::default() },
        "dialog" => FamilyEnt { kind: "dialog", js: "dialog", ..Default::default() },
        "sheet" => FamilyEnt { kind: "dialog", js: "sheet", ..Default::default() },
        "popover" => FamilyEnt { kind: "portal", open: "click", js: "popover", ..Default::default() },
        "tooltip" => FamilyEnt { kind: "portal", open: "hover", js: "tooltip", ..Default::default() },
        "hover-card" => FamilyEnt { kind: "portal", open: "hover", js: "hover-card", ..Default::default() },
        "tabs" => FamilyEnt { kind: "inline", js: "tabs", ..Default::default() },
        "slider" => FamilyEnt { kind: "none", js: "slider", ..Default::default() },
        "scroll-area" => FamilyEnt { kind: "none", js: "scroll-area", ..Default::default() },
        "dropdown-menu" => FamilyEnt { kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger", js: "dropdown-menu" },
        "context-menu" => FamilyEnt { kind: "menu", open: "contextmenu", attr: "data-radixuigo-context-trigger", js: "context-menu" },
        "menubar" => FamilyEnt { kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger", js: "menubar" },
        "select" => FamilyEnt { kind: "select", js: "select", ..Default::default() },
        "carousel" => FamilyEnt { kind: "none", js: "carousel", ..Default::default() },
        "navigation-menu" => FamilyEnt { kind: "nav", js: "navigation-menu", ..Default::default() },
        _ => return None,
    })
}
