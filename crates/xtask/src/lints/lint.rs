use clap::Subcommand;

use crate::{
	RxWorkspaceError, lint_readme,
	lints::{lint_aggregator_package_features, lint_codecov, lint_docs, lint_release_plz},
};

#[derive(Subcommand, Debug)]
pub enum LintCommand {
	Codecov,
	Docs,
	Aggregators,
	ReleasePlz,
	Readme,
}

impl LintCommand {
	pub fn run(self) -> Result<(), RxWorkspaceError> {
		match self {
			LintCommand::Codecov => lint_codecov(),
			LintCommand::Docs => lint_docs(),
			LintCommand::Aggregators => lint_aggregator_package_features(),
			LintCommand::ReleasePlz => lint_release_plz(),
			LintCommand::Readme => lint_readme(),
		}
	}
}
