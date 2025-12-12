use cargo_metadata::{
	Metadata, MetadataCommand, Package, PackageId, PackageName, camino::Utf8PathBuf,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fmt::Display, fs::read_to_string};
use thiserror::Error;

use crate::{CodecovLintParseError, WorkspaceProblems};

lazy_static! {
	static ref RX_CRATE_REGEX: Regex =
		Regex::new("^(rx_core|rx_bevy)_([^_]+)_?(.+)?$").expect("regex compiled");
}

#[derive(Error, Debug)]
pub enum RxWorkspaceError {
	#[error("workspace metadata query failed")]
	Metadata(#[from] cargo_metadata::Error),
	#[error("Malformed name: '{0}' valid crates have 3 segments")]
	MalformedName(String),
	#[error("Missing file: \"{0}\"")]
	MissingFile(Utf8PathBuf),
	#[error("Not a valid aggregator crate: {0}")]
	AggregatorInvalid(String),
	#[error("Category in crate name is invalid '{0}'")]
	CategoryInvalid(String),
	#[error(transparent)]
	CodecovLintParseError(#[from] CodecovLintParseError),
	#[error(transparent)]
	WorkspaceProblems(#[from] WorkspaceProblems),
}

pub struct RxWorkspace {
	pub metadata: Metadata,
	pub workspace_packages_by_id: HashMap<PackageId, Package>,
	pub aggregator_packages: HashMap<PackageId, AggregatorPackage>,
	pub rx_packages: HashMap<PackageName, RxPackage>,
}

impl RxWorkspace {
	pub fn parse_workspace() -> Result<Self, RxWorkspaceError> {
		let metadata = MetadataCommand::new().exec()?;

		let packages_by_id: HashMap<PackageId, &Package> = metadata
			.packages
			.iter()
			.map(|package| (package.id.clone(), package))
			.collect();

		let workspace_packages_by_id: HashMap<PackageId, Package> = metadata
			.workspace_members
			.iter()
			.filter_map(|id| {
				packages_by_id
					.get(id)
					.map(|&package| (id.clone(), package.clone()))
			})
			.collect();

		let rx_core = AggregatorPackage::new(
			"rx_core".to_string(),
			Some("rx_bevy".to_string()),
			None,
			&workspace_packages_by_id,
		);

		let rx_bevy = AggregatorPackage::new(
			"rx_bevy".to_string(),
			None,
			Some("rx_core".to_string()),
			&workspace_packages_by_id,
		);

		let aggregator_packages = {
			let mut hash_map = HashMap::<PackageId, AggregatorPackage>::new();
			hash_map.insert(rx_core.id.clone(), rx_core);
			hash_map.insert(rx_bevy.id.clone(), rx_bevy);
			hash_map
		};

		let utility_rx_crates = [
			"rx_bevy_context",
			"rx_bevy_common",
			"rx_core_traits",
			"rx_core_testing",
			"rx_core_emission_variants",
			"rx_core_macro_common",
			"rx_core_macro_subscription_derive",
			"rx_core_macro_subject_derive",
			"rx_core_macro_subscriber_derive",
			"rx_core_macro_observer_derive",
			"rx_core_macro_operator_derive",
			"rx_core_macro_observable_derive",
		];

		let rx_packages = packages_by_id
			.iter()
			.filter(|workspace_package| {
				!aggregator_packages.contains_key(workspace_package.0)
					&& !utility_rx_crates.contains(&workspace_package.1.name.as_str())
					&& aggregator_packages.values().any(|aggregator_crate| {
						workspace_package
							.1
							.name
							.starts_with(aggregator_crate.package.name.as_str())
					})
			})
			.flat_map(|package| {
				RxPackage::parse((*package.1).clone(), &aggregator_packages)
					.map_err(|e| {
						eprintln!("error parsing package {}.. {}", package.1.name, e);
						e
					})
					.ok()
			})
			.map(|p| (p.package.name.clone(), p))
			.collect::<HashMap<PackageName, RxPackage>>();

		Ok(RxWorkspace {
			metadata,
			workspace_packages_by_id,
			aggregator_packages,
			rx_packages,
		})
	}
}

#[derive(Clone, Debug)]
pub struct AggregatorPackage {
	pub id: PackageId,
	pub package: Package,
	pub wrapped_in: Option<PackageId>,
	pub wrapping_from: Option<PackageId>,
	pub lib_rs: String,
}

impl AggregatorPackage {
	pub fn new(
		name: String,
		wrapped_in: Option<String>,
		wrapping_from: Option<String>,
		workspace_packages: &HashMap<PackageId, Package>,
	) -> Self {
		let workspace_package = workspace_packages
			.iter()
			.find(|(_id, package)| package.name == name)
			.expect("not found");

		let lib_rs_path = workspace_package
			.1
			.manifest_path
			.parent()
			.unwrap()
			.join("src/lib.rs")
			.clone();

		let lib_rs: String = read_to_string(lib_rs_path).expect("rx_core lib.rs to be readable");

		AggregatorPackage {
			id: workspace_package.0.clone(),
			package: workspace_package.1.clone(),
			wrapped_in: wrapped_in
				.and_then(|name| workspace_packages.iter().find(|(_, p)| p.name == name))
				.map(|w| w.0.clone()),
			wrapping_from: wrapping_from
				.and_then(|name| workspace_packages.iter().find(|(_, p)| p.name == name))
				.map(|w| w.0.clone()),
			lib_rs,
		}
	}
}

impl Display for AggregatorPackage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.package.name)
	}
}

#[derive(Clone, Debug)]
pub struct RxPackage {
	pub package: Package,
	pub aggregator: AggregatorPackage,
	pub category: RxCrateCategory,
	name_only: String,
	pub is_viewed_as_wrapped: bool,
}

impl RxPackage {
	pub fn get_short_name(&self) -> String {
		if matches!(self.category, RxCrateCategory::Subject) && self.name_only.is_empty() {
			self.category.to_string()
		} else {
			self.name_only.to_string()
		}
	}

	pub fn as_feature_name(&self) -> String {
		if matches!(self.category, RxCrateCategory::Subject) && self.name_only.is_empty() {
			format!("{}", self.category)
		} else {
			format!("{}_{}", self.category, self.name_only)
		}
	}

	pub fn as_feature_group(&self, aggregators: &HashMap<PackageId, AggregatorPackage>) -> String {
		if self.is_viewed_as_wrapped
			&& let Some(wrapping_from) = self
				.aggregator
				.wrapping_from
				.as_ref()
				.and_then(|package_id| aggregators.get(package_id))
		{
			let wrapping_from_aggregator_name =
				wrapping_from.package.name.strip_prefix("rx_").unwrap();

			format!("all_{}_{}s", wrapping_from_aggregator_name, self.category)
		} else {
			format!("all_{}s", self.category)
		}
	}

	pub fn as_in_wrapper(
		&self,
		aggregators: &HashMap<PackageId, AggregatorPackage>,
	) -> Option<RxPackage> {
		self.aggregator
			.wrapped_in
			.as_ref()
			.and_then(|package_id| aggregators.get(package_id))
			.map(|wrapper_aggregator| RxPackage {
				package: self.package.clone(),
				aggregator: wrapper_aggregator.clone(),
				category: self.category.clone(),
				name_only: self.name_only.clone(),
				is_viewed_as_wrapped: true,
			})
	}

	pub fn parse(
		package: Package,
		aggregators: &HashMap<PackageId, AggregatorPackage>,
	) -> Result<Self, RxWorkspaceError> {
		let captures = RX_CRATE_REGEX
			.captures(&package.name)
			.ok_or(RxWorkspaceError::MalformedName(package.name.to_string()))?;

		let aggregator_name = captures
			.get(1)
			.ok_or(RxWorkspaceError::MalformedName(package.name.to_string()))?
			.as_str()
			.to_string();

		let aggregator = aggregators
			.values()
			.find(|a| a.package.name == aggregator_name)
			.ok_or(RxWorkspaceError::AggregatorInvalid(aggregator_name))?
			.clone();

		let category: RxCrateCategory = captures
			.get(2)
			.ok_or(RxWorkspaceError::MalformedName(package.name.to_string()))?
			.as_str()
			.to_string()
			.try_into()?;

		if matches!(category, RxCrateCategory::Subject) && captures.get(3).is_none() {
			// Special case
			return Ok(RxPackage {
				package,
				aggregator,
				category,
				name_only: "".to_string(),
				is_viewed_as_wrapped: false,
			});
		}

		let name = captures
			.get(3)
			.ok_or(RxWorkspaceError::MalformedName(package.name.to_string()))?
			.as_str()
			.to_string();

		Ok(RxPackage {
			package,
			aggregator,
			category,
			name_only: name,
			is_viewed_as_wrapped: false,
		})
	}
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RxCrateCategory {
	Observable,
	Operator,
	Subscription,
	Subscriber,
	Subject,
	Observer,
	Scheduler,
	Macro,
}

impl Display for RxCrateCategory {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let display_str = match self {
			RxCrateCategory::Observable => "observable",
			RxCrateCategory::Observer => "observer",
			RxCrateCategory::Operator => "operator",
			RxCrateCategory::Subscription => "subscription",
			RxCrateCategory::Subscriber => "subscriber",
			RxCrateCategory::Subject => "subject",
			RxCrateCategory::Scheduler => "scheduler",
			RxCrateCategory::Macro => "macro",
		};

		f.write_str(display_str)
	}
}

impl TryFrom<String> for RxCrateCategory {
	type Error = RxWorkspaceError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		if value == "observable" {
			Ok(RxCrateCategory::Observable)
		} else if value == "operator" {
			Ok(RxCrateCategory::Operator)
		} else if value == "subscription" {
			Ok(RxCrateCategory::Subscription)
		} else if value == "subscriber" {
			Ok(RxCrateCategory::Subscriber)
		} else if value == "observer" {
			Ok(RxCrateCategory::Observer)
		} else if value == "scheduler" {
			Ok(RxCrateCategory::Scheduler)
		} else if value == "subject" {
			Ok(RxCrateCategory::Subject)
		} else if value == "macro" {
			Ok(RxCrateCategory::Macro)
		} else {
			Err(RxWorkspaceError::CategoryInvalid(value))
		}
	}
}
