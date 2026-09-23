//! Ports of the remaining pipeline tools: the pure-file builders and the
//! browser-driven gates. Each submodule is a direct port of its Go file;
//! byte-identity with the Go binary was the port's acceptance bar
//! (go-parity-final tag — the harness that proved it is retired;
//! tests/gen_golden.sh now self-records).

pub mod css_direction_update;
pub mod demo_parity;
pub mod docs_transforms;
pub mod demo_smoke;
pub mod docs_build;
pub mod docs_catalog;
pub mod docs_consistency;
pub mod docs_fidelity;
pub mod docs_smoke;
pub mod docs_upstream_mirror;
pub mod interactivity_sweep;
pub mod ir_diff;
pub mod oracle_css;
pub mod overlay;
pub mod parity_baseline;
pub mod path_parity;
pub mod resolve_skins;
pub mod rtl_dict;
pub mod style_parity;
pub mod upstream;
pub mod upstream_snapshot;

// The Oxc A/B experiment (PLAN.md「為什麼不是 Rolldown/Oxc」) is compiled
// only under `--features oxc`; the default build never sees this module.
#[cfg(feature = "oxc")]
pub mod oxc_probe;
