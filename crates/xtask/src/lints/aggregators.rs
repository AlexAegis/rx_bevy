use cargo_metadata::PackageId;
use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;

use crate::{
	AggregatorPackage, RxCrateCategory, RxPackage, RxWorkspace, RxWorkspaceError,
	WorkspaceProblemPackageScope, WorkspaceProblems,
};

pub struct TransitiveFeature {
	pub name: String,
	pub optional: bool,
}

lazy_static! {
	static ref RX_CRATE_CATEGORY_TRANSITIVE_FEATURES: HashMap<RxCrateCategory, Vec<TransitiveFeature>> = {
		let mut hash_map = HashMap::<RxCrateCategory, Vec<TransitiveFeature>>::new();
		hash_map.insert(RxCrateCategory::Observable, vec![]);
		hash_map.insert(
			RxCrateCategory::Operator,
			vec![
				TransitiveFeature {
					name: "pipe".to_string(),
					optional: false,
				},
				TransitiveFeature {
					name: "compose".to_string(),
					optional: true,
				},
			],
		);
		hash_map.insert(RxCrateCategory::Subscription, vec![]);
		hash_map.insert(RxCrateCategory::Subscriber, vec![]);
		hash_map.insert(RxCrateCategory::Scheduler, vec![]);
		hash_map.insert(RxCrateCategory::Subject, vec![]);
		hash_map.insert(RxCrateCategory::Observer, vec![]);
		hash_map.insert(RxCrateCategory::Macro, vec![]);
		hash_map
	};
}

impl RxCrateCategory {
	pub fn transitive_features(&self) -> &Vec<TransitiveFeature> {
		RX_CRATE_CATEGORY_TRANSITIVE_FEATURES.get(self).unwrap()
	}
}

struct RuleIgnores {
	does_not_have_transitive_features: Vec<String>,
	does_not_have_an_aggregator_feature: Vec<String>,
}

pub fn lint_aggregator_package_features() -> Result<(), RxWorkspaceError> {
	let rx_workspace = RxWorkspace::parse_workspace()?;

	let rule_ignores = RuleIgnores {
		does_not_have_transitive_features: vec![
			"rx_core_operator_composite".to_string(),
			"rx_core_operator_identity".to_string(),
		],
		does_not_have_an_aggregator_feature: vec![],
	};

	let mut workspace_problems = WorkspaceProblems::default();

	for rx_package in rx_workspace.rx_packages.values() {
		let mut problem_problems = workspace_problems.scope(&rx_package.package.name);
		lint_rx_package(
			rx_package,
			&rule_ignores,
			&rx_workspace.aggregator_packages,
			&mut problem_problems,
		);
	}

	workspace_problems.try_into()
}

fn lint_rx_package_is_a_dependency_in_aggregator(
	package: &RxPackage,
) -> Result<(), RxAggregatorLintProblem> {
	// These crates are not part of the public api
	if matches!(
		package.category,
		RxCrateCategory::Subscriber | RxCrateCategory::Subscription | RxCrateCategory::Macro
	) {
		return Ok(());
	}

	let valid = package
		.aggregator
		.package
		.dependencies
		.iter()
		.any(|d| d.name == *package.package.name);

	if valid {
		Ok(())
	} else {
		Err(RxAggregatorLintProblem::NotInAggregator(
			package.aggregator.package.name.to_string(),
		))
	}
}

fn lint_rx_package_has_feature_in_aggregator(
	package: &RxPackage,
	ignored: &[String],
) -> Result<(), RxAggregatorLintProblem> {
	// These crates are not part of the public api
	if matches!(
		package.category,
		RxCrateCategory::Subscriber | RxCrateCategory::Subscription | RxCrateCategory::Macro
	) {
		return Ok(());
	}

	if ignored.contains(&package.package.name) {
		return Ok(());
	}

	let valid = package
		.aggregator
		.package
		.features
		.contains_key(&package.as_feature_name());

	if valid {
		Ok(())
	} else {
		Err(RxAggregatorLintProblem::DoesNotHaveFeatureInAggregator(
			package.as_feature_name(),
			package.aggregator.package.name.to_string(),
		))
	}
}

fn lint_rx_package_feature_is_part_of_the_feature_group(
	package: &RxPackage,
	ignored: &[String],
	aggregators: &HashMap<PackageId, AggregatorPackage>,
) -> Result<(), RxAggregatorLintProblem> {
	// These crates are not part of the public api
	if matches!(
		package.category,
		RxCrateCategory::Subscriber | RxCrateCategory::Subscription | RxCrateCategory::Macro
	) {
		return Ok(());
	}

	if ignored.contains(&package.package.name) {
		return Ok(());
	}

	let package_feature_group = package.as_feature_group(aggregators);

	let feature_group_features = package
		.aggregator
		.package
		.features
		.get(&package_feature_group)
		.ok_or(
			RxAggregatorLintProblem::DoesNotHaveCategoryFeatureGroupInAggregator(
				package_feature_group.clone(),
				package.aggregator.package.name.to_string(),
			),
		)?;

	let valid = feature_group_features.contains(&package.as_feature_name());

	if valid {
		Ok(())
	} else {
		Err(RxAggregatorLintProblem::NotPartOfFeatureGroup(
			package.as_feature_name(),
			package_feature_group,
			package.aggregator.package.name.to_string(),
		))
	}
}

fn lint_rx_package_operator_has_transitive_feature_in_group(
	package: &RxPackage,
	ignored: &[String],
	transitive_feature: &TransitiveFeature,
) -> Result<(), RxAggregatorLintProblem> {
	if ignored.contains(&package.package.name) {
		return Ok(());
	}

	let transitive_feature_group = package
		.aggregator
		.package
		.features
		.get(&transitive_feature.name)
		.ok_or(RxAggregatorLintProblem::TransitiveFeatureGroupMissing(
			transitive_feature.name.clone(),
			package.aggregator.package.name.to_string(),
		))?;

	let crate_has_feature = package
		.package
		.features
		.contains_key(&transitive_feature.name);

	if transitive_feature.optional && !crate_has_feature {
		return Ok(());
	};

	if !crate_has_feature {
		return Err(RxAggregatorLintProblem::DoesNotContainTransitiveFeature {
			feature_name: transitive_feature.name.clone(),
		});
	}

	let feature = format!("{}?/{}", package.package.name, &transitive_feature.name);

	if !transitive_feature_group.contains(&feature) {
		return Err(RxAggregatorLintProblem::TransitiveFeatureMissingFromGroup(
			feature,
			transitive_feature.name.clone(),
			package.aggregator.package.name.to_string(),
		));
	}

	Ok(())
}

fn lint_rx_package_has_lib_rs_entries(
	package: &RxPackage,
	ignored_transitive: &[String],
) -> Result<(), RxAggregatorLintProblem> {
	// These crates are not part of the public api
	if matches!(
		package.category,
		RxCrateCategory::Subscriber | RxCrateCategory::Subscription | RxCrateCategory::Macro
	) {
		return Ok(());
	}

	let aggregator_crate_name = package.aggregator.package.name.to_string();

	let category_entry = format!(
		"#[cfg(feature = \"{}\")]\n\tpub use {}::{}::*;",
		package.as_feature_name(),
		package.package.name,
		package.category
	);

	if !package.aggregator.lib_rs.contains(&category_entry) {
		return Err(RxAggregatorLintProblem::LibRsExportMissing(
			category_entry,
			aggregator_crate_name,
		));
	}

	if !ignored_transitive.contains(&package.package.name) {
		for transitive_feature in package.category.transitive_features() {
			let crate_has_feature = package
				.package
				.features
				.contains_key(&transitive_feature.name);

			if transitive_feature.optional && !crate_has_feature {
				return Ok(());
			};

			let transitive_feature_entry = format!(
				"#[cfg(feature = \"{}\")]\n\tpub use {}::extension_{}::*;",
				package.as_feature_name(),
				package.package.name,
				transitive_feature.name
			);

			if !package
				.aggregator
				.lib_rs
				.contains(&transitive_feature_entry)
			{
				return Err(RxAggregatorLintProblem::LibRsExportMissing(
					transitive_feature_entry,
					aggregator_crate_name,
				));
			}
		}
	}

	Ok(())
}

fn lint_rx_package(
	package: &RxPackage,
	rule_ignores: &RuleIgnores,
	aggregators: &HashMap<PackageId, AggregatorPackage>,
	package_problems: &mut WorkspaceProblemPackageScope,
) {
	// Package Level lints
	for transitive_feature in package.category.transitive_features() {
		if let Err(err) = lint_rx_package_operator_has_transitive_feature_in_group(
			package,
			&rule_ignores.does_not_have_transitive_features,
			transitive_feature,
		) {
			package_problems.add_problem(err);
		};
	}

	// Aggregator level lints

	if let Err(err) = lint_rx_package_is_a_dependency_in_aggregator(package) {
		package_problems.add_problem(err);
	};

	if let Err(err) = lint_rx_package_has_feature_in_aggregator(
		package,
		&rule_ignores.does_not_have_an_aggregator_feature,
	) {
		package_problems.add_problem(err);
	};

	if let Err(err) = lint_rx_package_feature_is_part_of_the_feature_group(
		package,
		&rule_ignores.does_not_have_an_aggregator_feature,
		aggregators,
	) {
		package_problems.add_problem(err);
	};

	if let Err(err) =
		lint_rx_package_has_lib_rs_entries(package, &rule_ignores.does_not_have_transitive_features)
	{
		package_problems.add_problem(err);
	};

	// Schedulers are not (necessarily) wrapped, as they probably wouldn't work in the new environment
	if !matches!(package.category, RxCrateCategory::Scheduler)
		&& let Some(wrapped_package) = package.as_in_wrapper(aggregators)
	{
		if let Err(err) = lint_rx_package_has_feature_in_aggregator(
			&wrapped_package,
			&rule_ignores.does_not_have_an_aggregator_feature,
		) {
			package_problems.add_problem(err);
		};

		if let Err(err) = lint_rx_package_feature_is_part_of_the_feature_group(
			&wrapped_package,
			&rule_ignores.does_not_have_an_aggregator_feature,
			aggregators,
		) {
			package_problems.add_problem(err);
		};
	}
}

#[derive(Error, Debug)]
pub enum RxAggregatorLintProblem {
	#[error("Does not contain the transitive feature \"{feature_name}\"")]
	DoesNotContainTransitiveFeature { feature_name: String },
	#[error("Package not present as a dependency in the aggregator crate \"{0}\"")]
	NotInAggregator(String),
	#[error("Package does not have its feature ({0}) in the aggregator crate \"{1}\"")]
	DoesNotHaveFeatureInAggregator(String, String),
	#[error(
		"Package does not have its corresponding feature group ({0}) in the aggregator crate \"{1}\""
	)]
	DoesNotHaveCategoryFeatureGroupInAggregator(String, String),
	#[error(
		"Package does not have its feature ({0}) in its categories feature group ({1}) in the aggregator crate \"{2}\""
	)]
	NotPartOfFeatureGroup(String, String, String),
	#[error("Transitive feature group ({0}) missing in aggregator crate ({1})")]
	TransitiveFeatureGroupMissing(String, String),
	#[error(
		"Transitive feature (\"{0}\") is missing from from its group ({1}) in aggregator crate \"{2}\""
	)]
	TransitiveFeatureMissingFromGroup(String, String, String),
	#[error("Export (\"{0}\") is missing in the aggregator crate \"{1}\"")]
	LibRsExportMissing(String, String),
}
