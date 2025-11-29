use clap::Parser;
use xtask::{Cli, Command};

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
