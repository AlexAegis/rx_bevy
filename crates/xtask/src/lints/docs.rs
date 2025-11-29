use std::{collections::HashMap, fs::read_to_string};

use cargo_metadata::camino::Utf8PathBuf;
use lazy_static::lazy_static;
use thiserror::Error;

use crate::{RxCrateCategory, RxWorkspace, RxWorkspaceError, WorkspaceProblems};

lazy_static! {
	static ref RX_CRATE_CATEGORY_DOCS_DIRECTORY: HashMap<(String, RxCrateCategory), Utf8PathBuf> = {
		let mut hash_map = HashMap::<(String, RxCrateCategory), Utf8PathBuf>::new();
		hash_map.insert(
			("rx_core".to_string(), RxCrateCategory::Observable),
			"10_observables_core".into(),
		);
		hash_map.insert(
			("rx_bevy".to_string(), RxCrateCategory::Observable),
			"11_observables_bevy".into(),
		);
		hash_map.insert(
			("rx_core".to_string(), RxCrateCategory::Operator),
			"12_operators_core".into(),
		);
		hash_map.insert(
			("rx_bevy".to_string(), RxCrateCategory::Operator),
			"13_operators_bevy".into(),
		);
		hash_map
	};
}

impl RxCrateCategory {
	pub fn doc_directory(&self, aggregator_name: String) -> Option<&Utf8PathBuf> {
		RX_CRATE_CATEGORY_DOCS_DIRECTORY.get(&(aggregator_name, self.clone()))
	}
}

pub fn lint_docs() -> Result<(), RxWorkspaceError> {
	let workspace = RxWorkspace::parse_workspace()?;

	let workspace_root = workspace.metadata.workspace_root.clone();
	let summary_md_path = workspace_root.join("docs/SUMMARY.md");
	let summary_md = read_to_string(&summary_md_path)
		.map_err(|_| RxWorkspaceError::MissingFile(summary_md_path))?;

	let mut workspace_problems = WorkspaceProblems::default();

	for package in workspace.rx_packages.values() {
		let mut package_problems = workspace_problems.scope(&package.package.name);

		let short_name = package.get_short_name();

		if let Some(doc_directory_path) = package
			.category
			.doc_directory(package.aggregator.package.name.to_string())
		{
			let mdbook_entry_path = doc_directory_path.join(format!("{}.md", short_name));
			let mdbook_entry_path_abs = workspace_root
				.as_path()
				.join(format!("docs/{}", mdbook_entry_path));

			if !mdbook_entry_path_abs.is_file() {
				package_problems
					.add_problem(DocsLintProblem::MissingDocs(mdbook_entry_path.clone()));
			}

			let summary_entry = format!("- [{short_name}]({mdbook_entry_path})");

			if !summary_md.contains(&summary_entry) {
				package_problems
					.add_problem(DocsLintProblem::MissingSummaryEntry(summary_entry.clone()));
			}
		}
	}

	workspace_problems.try_into()
}

#[derive(Debug, Error)]
pub enum DocsLintProblem {
	#[error(
		"book md file not found at \"{0}\", it should just include the readme.md of the package."
	)]
	MissingDocs(Utf8PathBuf),
	#[error("entry \"{0}\" is missing from SUMMARY.md")]
	MissingSummaryEntry(String),
}
