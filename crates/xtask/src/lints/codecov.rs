use cargo_metadata::camino::Utf8PathBuf;
use std::fs;
use thiserror::Error;
use yaml_rust2::{ScanError, Yaml, YamlLoader};

use crate::{RxWorkspace, RxWorkspaceError, WorkspaceProblems};

pub fn lint_codecov() -> Result<(), RxWorkspaceError> {
	let rx_workspace = RxWorkspace::parse_workspace()?;

	let ignored_packages = [
		"xtask",
		"feature_checker",
		"examples_common",
		"bevy_mod_alternate_system_on_press",
	];

	let mut workspace_problems = WorkspaceProblems::default();

	let codecov_yml_path: Utf8PathBuf = "codecov.yml".into();
	let codecov_yml_path_abs = rx_workspace.metadata.workspace_root.join(&codecov_yml_path);
	let raw_config = fs::read_to_string(&codecov_yml_path_abs)
		.map_err(|_| RxWorkspaceError::MissingFile(codecov_yml_path))?;

	let codecov_yml = parse_yaml(&raw_config)?;

	for workspace_package in rx_workspace
		.workspace_packages_by_id
		.values()
		.filter(|p| !ignored_packages.contains(&p.name.as_str()))
	{
		let mut package_problems = workspace_problems.scope(&workspace_package.name);

		let individual_components = &codecov_yml["component_management"]["individual_components"];
		let package_component = match individual_components {
			Yaml::Array(arr) => arr.iter().find(|individual_component| {
				let component_id = &individual_component["component_id"];

				component_id.as_str().expect("value") == *workspace_package.name
			}),
			_ => return Err(CodecovLintParseError::IndividualComponentsIsNotAnArray.into()),
		};

		if let Some(package_component) = package_component {
			let paths_array = &package_component["paths"];
			match paths_array {
				Yaml::Array(paths) => {
					if let Some(path) = paths.first().and_then(|path| path.as_str()) {
						let should_be_path = format!("crates/{}/**", workspace_package.name);
						if path != should_be_path {
							package_problems.add_problem(CodecovLintProblems::IncorrectPath(
								path.to_string(),
								should_be_path,
							));
						}
					} else {
						package_problems.add_problem(CodecovLintProblems::MalformedPaths);
					}
				}
				_ => {
					package_problems.add_problem(CodecovLintProblems::MalformedPaths);
				}
			}
		} else {
			package_problems.add_problem(CodecovLintProblems::MissingComponent);
		}
	}

	workspace_problems.try_into()
}

fn parse_yaml(raw_config: &str) -> Result<Yaml, CodecovLintParseError> {
	let documents =
		YamlLoader::load_from_str(raw_config).map_err(CodecovLintParseError::ParseConfig)?;
	documents
		.into_iter()
		.find(|doc| !doc.is_badvalue() && !doc.is_null())
		.ok_or(CodecovLintParseError::EmptyConfig)
}

#[derive(Debug, Error)]
pub enum CodecovLintParseError {
	#[error("failed to parse codecov.yml")]
	ParseConfig(#[source] ScanError),
	#[error("codecov.yml does not contain any YAML documents")]
	EmptyConfig,
	#[error("codecov.yml does not have a `component_management.individual_components` array")]
	IndividualComponentsIsNotAnArray,
}

#[derive(Debug, Error)]
pub enum CodecovLintProblems {
	#[error("Missing entry from codecov.yml")]
	MissingComponent,
	#[error("Entry in codecov.yml has malformed `paths`")]
	MalformedPaths,
	#[error("Incorrect path in codecov.yml \"{0}\" should be: \"{1}\"")]
	IncorrectPath(String, String),
}
