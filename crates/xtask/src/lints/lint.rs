use clap::Subcommand;
use thiserror::Error;

use crate::lints::{CodecovArgs, CodecovLintError, DocsLintError, lint_codecov, lint_docs};

#[derive(Subcommand, Debug)]
pub enum LintCommand {
	/// Verify coverage component configuration.
	Codecov(CodecovArgs),
	/// Verify crate documentation coverage.
	Docs,
}

impl LintCommand {
	pub fn run(self) -> Result<(), LintError> {
		match self {
			LintCommand::Codecov(args) => {
				lint_codecov(&args)?;
				Ok(())
			}
			LintCommand::Docs => {
				lint_docs()?;
				Ok(())
			}
		}
	}
}

#[derive(Debug, Error)]
pub enum LintError {
	#[error(transparent)]
	Codecov(#[from] CodecovLintError),
	#[error(transparent)]
	Docs(#[from] DocsLintError),
}
