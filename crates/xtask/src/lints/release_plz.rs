use std::fs::read_to_string;

use cargo_metadata::camino::Utf8PathBuf;
use thiserror::Error;

use crate::{RxWorkspace, RxWorkspaceError, WorkspaceProblems};

pub fn lint_release_plz() -> Result<(), RxWorkspaceError> {
	let workspace = RxWorkspace::parse_workspace()?;

	let release_plz_path: Utf8PathBuf = "release-plz.toml".into();
	let release_plz_path_abs = workspace.metadata.workspace_root.join(&release_plz_path);
	let release_plz_contents = read_to_string(&release_plz_path_abs)
		.map_err(|_| RxWorkspaceError::MissingFile(release_plz_path))?;
	let parsed_packages = parse_release_plz(&release_plz_contents);

	let mut workspace_problems = WorkspaceProblems::default();
	for package in workspace.workspace_packages_by_id.values() {
		if !(package.name.starts_with("rx_core") || package.name.starts_with("rx_bevy")) {
			continue;
		}

		let mut package_problems = workspace_problems.scope(&package.name);
		let expected_version_group = if package.name.starts_with("rx_bevy") {
			"rx_bevy"
		} else {
			"rx_core"
		};
		let expected_tag = if package.name.starts_with("rx_bevy") {
			"v{{ version }}"
		} else {
			"core-v{{ version }}"
		};

		match parsed_packages.iter().find(|p| package.name == p.name) {
			None => package_problems.add_problem(ReleasePlzLintProblem::MissingEntry {
				expected: format!("name = \"{}\"", package.name),
			}),
			Some(entry) if entry.version_group.as_deref() != Some(expected_version_group) => {
				package_problems.add_problem(ReleasePlzLintProblem::WrongVersionGroup {
					package: package.name.to_string(),
					expected: expected_version_group.to_string(),
					found: entry.version_group.clone(),
				});
			}
			Some(entry) if entry.git_tag_name.as_deref() != Some(expected_tag) => {
				package_problems.add_problem(ReleasePlzLintProblem::WrongGitTagName {
					package: package.name.to_string(),
					expected: expected_tag.to_string(),
					found: entry.git_tag_name.clone(),
				});
			}
			_ => {}
		};
	}

	workspace_problems.try_into()
}

#[derive(Debug)]
struct ReleasePlzPackage {
	name: String,
	version_group: Option<String>,
	git_tag_name: Option<String>,
}

fn parse_release_plz(contents: &str) -> Vec<ReleasePlzPackage> {
	contents
		.split("[[package]]")
		.skip(1)
		.filter_map(|block| {
			let mut name: Option<String> = None;
			let mut version_group: Option<String> = None;
			let mut git_tag_name: Option<String> = None;

			for line in block.lines() {
				let line = line.trim();
				if line.is_empty() || line.starts_with('#') {
					continue;
				}

				if line.starts_with("name")
					&& let Some(val) = line.split('=').nth(1)
				{
					name = Some(val.trim().trim_matches('"').to_string());
				}

				if line.starts_with("version_group")
					&& let Some(val) = line.split('=').nth(1)
				{
					version_group = Some(val.trim().trim_matches('"').to_string());
				}

				if line.starts_with("git_tag_name")
					&& let Some(val) = line.split('=').nth(1)
				{
					git_tag_name = Some(val.trim().trim_matches('"').to_string());
				}
			}

			name.map(|name| ReleasePlzPackage {
				name,
				version_group,
				git_tag_name,
			})
		})
		.collect()
}

#[derive(Debug, Error)]
pub enum ReleasePlzLintProblem {
	#[error("Entry missing from release-plz.toml. Please add: {expected}")]
	MissingEntry { expected: String },
	#[error("Wrong version_group for {package}. expected: {expected} found: {found:?}")]
	WrongVersionGroup {
		package: String,
		expected: String,
		found: Option<String>,
	},
	#[error("Wrong git_tag_name for {package}. expected: {expected} found: {found:?}")]
	WrongGitTagName {
		package: String,
		expected: String,
		found: Option<String>,
	},
}
