mod cli;
mod lints;

extern crate thiserror;

use crate::cli::{Cli, Command};
use clap::Parser;

fn main() {
	let cli = Cli::parse();

	let result = match cli.command {
		Command::Lint(command) => command.run(),
	};

	if let Err(err) = result {
		eprintln!("{err}");
		std::process::exit(1);
	}
}
