//! shadless-rs — Rust port of shadless/pipeline (the graph runner + converters).
//! Byte-identity with the Go binary is the acceptance bar; see PLAN.md.

pub mod convert;
pub mod engine;
pub mod fanout;
pub mod glob;
pub mod emit;
pub mod gates;
pub mod graph;
pub mod jsbuild;
pub mod jsonorder;
pub mod key;
pub mod nodes;
pub mod oracle;
pub mod produces;
pub mod runner;
pub mod stamps;
pub mod tools;
pub mod tsx;
pub mod twmerge;
pub mod verify;
