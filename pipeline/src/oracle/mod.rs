//! Port of the oracle chain: pipeline/{browser_shell,oracle_lib,
//! example_oracle,example_fixture,example_golden,contract}.go — React oracle
//! render + browser-driven behaviour gates.

pub mod browser_shell;
pub mod contract;
pub mod example_golden;
pub mod example_fixture;
pub mod example_oracle;
pub mod families;
pub mod fixture_families;
pub mod oracle_lib;

// rolldown-backed oracle bundle, gated by SHADLESS_ORACLE_BUNDLER=oxc at
// runtime; compiled only under --features oxc (see oxc_bundle.rs).
#[cfg(feature = "oxc")]
pub mod oxc_bundle;
