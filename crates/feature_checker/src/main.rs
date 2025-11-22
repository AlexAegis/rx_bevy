use cargo_metadata::MetadataCommand;
use std::collections::{HashMap, HashSet};

fn main() {
	let metadata = MetadataCommand::new()
		.exec()
		.expect("Failed to get cargo metadata");

	let packages: HashMap<_, _> = metadata
		.packages
		.iter()
		.map(|p| (p.id.clone(), p))
		.collect();

	let mut had_warning = 0;

	// Collect workspace member names to filter out third-party dependencies
	let workspace_names: HashSet<String> = metadata
		.workspace_packages()
		.iter()
		.map(|p| p.name.as_str().to_string())
		.collect();

	for package in metadata.workspace_packages() {
		println!("Checking: {}", package.name);
		// Keep a copy of existing feature enables so we can preserve them in suggestions
		let mut existing_for_feature: HashMap<String, Vec<String>> = HashMap::new();
		// Track missing propagations per feature; capture dep name and whether it's optional
		let mut missing_by_feature: HashMap<String, Vec<(String, bool)>> = HashMap::new();
		for (feature, enables) in &package.features {
			// Record current enables for this feature so we don't drop them in the suggestion
			existing_for_feature.insert(feature.clone(), enables.clone());
			// Skip special-case default feature
			if feature == "default" {
				continue;
			}

			// Skip features that should not propagate
			if feature == "example" {
				continue;
			}

			// Skip aggregate features for non aggregator crates
			if (feature == "compose" || feature == "pipe" || feature == "operator_fn")
				&& (package.name != "rx_core" || package.name != "rx_bevy")
			{
				continue;
			}

			for dep in &package.dependencies {
				// Skip aggregator crates
				if dep.name == "rx_core" || dep.name == "rx_bevy" {
					continue;
				}

				// Skip non-workspace dependencies
				if !workspace_names.contains(dep.name.as_str()) {
					continue;
				}
				// Find the actual dependency package
				if let Some(dep_pkg) = packages
					.values()
					.find(|p| p.name.as_str() == dep.name.as_str())
				{
					// If dependency has a feature with the same name
					if dep_pkg.features.contains_key(feature) {
						let dep_feature_plain = format!("{}/{}", dep.name, feature);
						let dep_feature_optional = format!("{}?/{}", dep.name, feature);
						// Check if this package’s feature propagates it (either plain or optional form)
						if !enables.contains(&dep_feature_plain)
							&& !enables.contains(&dep_feature_optional)
						{
							missing_by_feature
								.entry(feature.clone())
								.or_default()
								.push((dep.name.clone(), dep.optional));
						}
					}
				}
			}
		}

		// If there are missing propagations, report grouped and print a Toml snippet
		if !missing_by_feature.is_empty() {
			let mut feature_dep_pairs: Vec<(String, Vec<(String, bool)>)> =
				missing_by_feature.into_iter().collect();
			feature_dep_pairs.sort_by(|a, b| a.0.cmp(&b.0));

			// Grouped warnings per feature
			for (feature, deps) in feature_dep_pairs.iter().cloned() {
				let mut names: Vec<String> = deps.into_iter().map(|(n, _)| n).collect();
				names.sort();
				names.dedup();
				println!(
					"⚠️  feature `{}` does not propagate to dependencies: {}",
					feature,
					names.join(", ")
				);
			}
			had_warning += feature_dep_pairs.len();

			// Ready-to-paste Cargo.toml features block
			println!(
				"⚠️  Missing feature propagations!\nAdd the following to {}/Cargo.toml\n\n[features]",
				package.name
			);
			for (feature, deps) in feature_dep_pairs {
				// Start with existing items for this feature
				let mut merged: Vec<String> = existing_for_feature
					.get(&feature)
					.cloned()
					.unwrap_or_default();
				// Append missing dep/feature items if not already present
				for (dep_name, optional) in deps {
					let plain = format!("{}/{}", dep_name, feature);
					let opt = format!("{}?/{}", dep_name, feature);
					let item = if optional { opt.clone() } else { plain.clone() };
					// If either variant already exists, don't add another
					if !merged.iter().any(|e| e == &plain || e == &opt) {
						merged.push(item);
					}
				}
				// Sort and dedup for stable output
				merged.sort();
				merged.dedup();
				// Print with quotes
				let items: Vec<String> = merged.into_iter().map(|s| format!("\"{}\"", s)).collect();
				println!("{} = [{}]", feature, items.join(", "));
			}
			println!();
		}
	}

	// Exit with a non-zero status if any warnings were found
	if had_warning > 0 {
		std::process::exit(1);
	}
}
