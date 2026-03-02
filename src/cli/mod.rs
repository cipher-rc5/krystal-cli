// file: src/cli/mod.rs
// description: CLI module re-exports; groups app, commands, and output sub-modules
// docs_reference: https://doc.rust-lang.org/reference/items/modules.html

pub mod app;
pub mod commands;
pub mod output;

pub use app::{Cli, run_cli};
pub use commands::*;
pub use output::*;
