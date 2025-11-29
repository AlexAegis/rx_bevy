use clap::Subcommand;

use crate::{
	RxWorkspaceError, lint_readme,
	lints::{lint_aggregator_package_features, lint_codecov, lint_docs},
};

#[derive(Subcommand, Debug)]
pub enum LintCommand {
	Codecov,
	Docs,
	Aggregators,
	Readme,
}

impl LintCommand {
	pub fn run(self) -> Result<(), RxWorkspaceError> {
		match self {
			LintCommand::Codecov => lint_codecov(),
			LintCommand::Docs => lint_docs(),
			LintCommand::Aggregators => lint_aggregator_package_features(),
			LintCommand::Readme => lint_readme(),
		}
	}
}
