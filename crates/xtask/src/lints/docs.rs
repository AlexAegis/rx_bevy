use cargo_metadata::MetadataCommand;
use std::collections::HashMap;
use thiserror::Error;

pub fn lint_docs() -> Result<(), DocsLintError> {
	let metadata = MetadataCommand::new().exec()?;
	let workspace_root = metadata.workspace_root.clone();

	let packages: HashMap<_, _> = metadata
		.packages
		.iter()
		.map(|package| (package.id.clone(), package))
		.collect();

	let mut missing = Vec::new();

	for member in &metadata.workspace_members {
		let Some(package) = packages.get(member) else {
			continue;
		};
		let crate_name = package.name.as_str();

		if let Some(rule) = DocRule::for_crate(crate_name) {
			let suffix = &crate_name[rule.prefix.len()..];
			if suffix.is_empty() {
				continue;
			}
			let expected = workspace_root
				.join(rule.docs_subdir)
				.join(format!("{suffix}.md"));

			if !expected.is_file() {
				let relative_path = expected
					.strip_prefix(&workspace_root)
					.map(|path| path.to_string())
					.unwrap_or_else(|_| expected.to_string());
				missing.push(MissingDoc {
					crate_name: crate_name.to_owned(),
					expected_path: relative_path,
				});
			}
		}
	}

	if missing.is_empty() {
		println!("docs lint passed");
		Ok(())
	} else {
		eprintln!("docs lint failed: the following crates are missing documentation entries:");
		for entry in &missing {
			eprintln!("  - {} -> {}", entry.crate_name, entry.expected_path);
		}
		Err(DocsLintError::MissingDocs { missing })
	}
}

struct DocRule {
	prefix: &'static str,
	docs_subdir: &'static str,
}

impl DocRule {
	const RULES: &'static [DocRule] = &[
		DocRule {
			prefix: "rx_core_observable_",
			docs_subdir: "docs/10_observables_core",
		},
		DocRule {
			prefix: "rx_bevy_observable_",
			docs_subdir: "docs/11_observables_bevy",
		},
		DocRule {
			prefix: "rx_core_operator_",
			docs_subdir: "docs/12_operators_core",
		},
		DocRule {
			prefix: "rx_bevy_operator_",
			docs_subdir: "docs/13_operators_bevy",
		},
	];

	fn for_crate(crate_name: &str) -> Option<&'static DocRule> {
		Self::RULES
			.iter()
			.find(|rule| crate_name.starts_with(rule.prefix))
	}
}

#[derive(Debug)]
pub struct MissingDoc {
	pub crate_name: String,
	pub expected_path: String,
}

#[derive(Debug, Error)]
pub enum DocsLintError {
	#[error("workspace metadata query failed")]
	Metadata(#[from] cargo_metadata::Error),
	#[error(
		"documentation lint failed: {missing_len} crate(s) are missing docs",
		missing_len = "{missing.len()}"
	)]
	MissingDocs { missing: Vec<MissingDoc> },
}
