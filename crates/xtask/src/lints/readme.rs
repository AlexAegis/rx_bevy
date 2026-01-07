use std::fs::read_to_string;
use thiserror::Error;

use crate::{RxWorkspace, RxWorkspaceError, WorkspaceProblems};

pub fn lint_readme() -> Result<(), RxWorkspaceError> {
	let workspace = RxWorkspace::parse_workspace()?;

	let mut workspace_problems = WorkspaceProblems::default();

	for package in workspace.workspace_packages_by_id.values() {
		if !(package.name.starts_with("rx_core") || package.name.starts_with("rx_bevy"))
			|| package.name == "rx_bevy"
		{
			continue;
		}

		let mut package_problems = workspace_problems.scope(&package.name);

		let readme_md_path = package
			.manifest_path
			.parent()
			.unwrap()
			.join("readme.md")
			.clone();

		let Some(readme_md) = read_to_string(readme_md_path).ok() else {
			package_problems.add_problem(ReadmeLintProblem::NoReadmeFile);
			continue;
		};

		let stripped_package_name = if (package.name.starts_with("rx_core_")
			|| package.name.starts_with("rx_bevy_"))
			&& package.name != "rx_core_common"
			&& package.name != "rx_bevy_common"
		{
			package.name.split_at(8).1
		} else {
			&package.name
		};

		let title = format!(
			"# [{stripped_package_name}](https://github.com/AlexAegis/rx_bevy/tree/master/crates/{package_name})",
			package_name = package.name
		);

		if !readme_md.contains(&title) {
			package_problems.add_problem(ReadmeLintProblem::MissingTitle { title });
		}

		let crates_io_badge = format!(
			"[![crates.io](https://img.shields.io/crates/v/{package_name}.svg)](https://crates.io/crates/{package_name})",
			package_name = package.name
		);

		if !readme_md.contains(&crates_io_badge) {
			package_problems.add_problem(ReadmeLintProblem::MissingBadge {
				badge_name: "crates_io".to_string(),
				badge: crates_io_badge,
			});
		}

		let ci_badge = "[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)";

		if !readme_md.contains(ci_badge) {
			package_problems.add_problem(ReadmeLintProblem::MissingBadge {
				badge_name: "ci".to_string(),
				badge: ci_badge.to_string(),
			});
		}

		let codecov_component_badge = format!(
			"[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component={package_name})](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D={package_name})",
			package_name = package.name
		);

		if !readme_md.contains(&codecov_component_badge) {
			package_problems.add_problem(ReadmeLintProblem::MissingBadge {
				badge_name: "codecov".to_string(),
				badge: codecov_component_badge,
			});
		}
	}

	workspace_problems.try_into()
}

#[derive(Debug, Error)]
pub enum ReadmeLintProblem {
	#[error("package has no readme file")]
	NoReadmeFile,
	#[error("Missing title! Please add:\n{title}")]
	MissingTitle { title: String },
	#[error("Badge \"{badge_name}\" is missing from the readme. Please add:\n{badge}")]
	MissingBadge { badge_name: String, badge: String },
}
