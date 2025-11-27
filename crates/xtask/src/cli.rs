use clap::{Parser, Subcommand};

use crate::lints::LintCommand;

#[derive(Parser)]
#[command(name = "xtask", version, about = "Repository maintenance tasks", long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
	/// Run repository lints.
	#[command(subcommand)]
	Lint(LintCommand),
}
