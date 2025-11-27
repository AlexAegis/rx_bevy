use cargo_metadata::MetadataCommand;
use clap::Args;
use std::{
	collections::{BTreeSet, HashMap, HashSet},
	fs,
	path::PathBuf,
};
use thiserror::Error;
use yaml_rust2::{ScanError, Yaml, YamlLoader};

#[derive(Args, Debug, Default)]
pub struct CodecovArgs {
	/// Crate names to ignore when verifying codecov component coverage.
	#[arg(long = "ignore", value_name = "CRATE", value_delimiter = ',')]
	pub ignore: Vec<String>,
}

pub fn lint_codecov(args: &CodecovArgs) -> Result<(), CodecovLintError> {
	let metadata = MetadataCommand::new().exec()?;
	let packages: HashMap<_, _> = metadata
		.packages
		.iter()
		.map(|package| (package.id.clone(), package))
		.collect();

	let workspace_crates: BTreeSet<String> = metadata
		.workspace_members
		.iter()
		.filter_map(|id| packages.get(id))
		.map(|package| package.name.clone().to_string())
		.collect();

	let config_path = PathBuf::from(&metadata.workspace_root).join("codecov.yml");
	let raw_config =
		fs::read_to_string(&config_path).map_err(|source| CodecovLintError::ReadConfig {
			path: config_path.clone(),
			source,
		})?;
	let configured_crates = configured_components(&raw_config)?;
	let ignored: HashSet<&str> = args.ignore.iter().map(String::as_str).collect();

	let missing: Vec<String> = workspace_crates
		.difference(&configured_crates)
		.filter(|name| !ignored.contains(name.as_str()))
		.cloned()
		.collect();

	if missing.is_empty() {
		println!("codecov lint passed");
		Ok(())
	} else {
		eprintln!(
			"codecov lint failed: the following workspace crates are missing from codecov.yml:",
		);
		for crate_name in &missing {
			eprintln!("  - {crate_name}");
		}

		Err(CodecovLintError::MissingComponents { missing })
	}
}

fn configured_components(raw_config: &str) -> Result<BTreeSet<String>, CodecovLintError> {
	let documents = YamlLoader::load_from_str(raw_config).map_err(CodecovLintError::ParseConfig)?;
	let document = documents
		.into_iter()
		.find(|doc| !doc.is_badvalue() && !doc.is_null())
		.ok_or(CodecovLintError::EmptyConfig)?;

	extract_component_ids(&document)
}

fn extract_component_ids(doc: &Yaml) -> Result<BTreeSet<String>, CodecovLintError> {
	let components = match &doc["component_management"]["individual_components"] {
		Yaml::BadValue | Yaml::Null => return Ok(BTreeSet::new()),
		Yaml::Array(items) => items,
		_ => return Err(CodecovLintError::ComponentsNotArray),
	};

	let mut ids = BTreeSet::new();
	for (index, component) in components.iter().enumerate() {
		let Some(component_id) = component["component_id"].as_str() else {
			return Err(CodecovLintError::ComponentIdMissing { index });
		};
		ids.insert(component_id.to_owned());
	}

	Ok(ids)
}

#[derive(Debug, Error)]
pub enum CodecovLintError {
	#[error("workspace metadata query failed")]
	Metadata(#[from] cargo_metadata::Error),
	#[error("failed to read codecov.yml at {path}")]
	ReadConfig {
		path: PathBuf,
		#[source]
		source: std::io::Error,
	},
	#[error("failed to parse codecov.yml")]
	ParseConfig(#[source] ScanError),
	#[error("codecov.yml does not contain any YAML documents")]
	EmptyConfig,
	#[error("codecov.yml component #{index} is missing a `component_id` string")]
	ComponentIdMissing { index: usize },
	#[error("codecov.yml `component_management.individual_components` must be an array")]
	ComponentsNotArray,
	#[error(
		"codecov.yml is missing {missing_len} workspace crate entries",
		missing_len = "{missing.len()}"
	)]
	MissingComponents { missing: Vec<String> },
}
