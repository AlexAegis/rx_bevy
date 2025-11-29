use std::{collections::HashMap, error::Error};

use cargo_metadata::PackageName;
use thiserror::Error;

use crate::RxWorkspaceError;

#[derive(Error, Debug, Default)]
pub struct WorkspaceProblems {
	packages_with_errors: HashMap<PackageName, Vec<Box<dyn Error>>>,
}

impl WorkspaceProblems {
	pub fn is_empty(&self) -> bool {
		self.packages_with_errors.is_empty()
	}

	pub fn add_problem_for_package(
		&mut self,
		package_name: &PackageName,
		error: impl 'static + Error,
	) {
		self.packages_with_errors
			.entry(package_name.clone())
			.or_default()
			.push(Box::new(error));
	}

	pub fn scope<'a>(
		&'a mut self,
		package_name: &'a PackageName,
	) -> WorkspaceProblemPackageScope<'a> {
		WorkspaceProblemPackageScope {
			package_name,
			workspace_problems: self,
		}
	}
}

impl TryFrom<WorkspaceProblems> for () {
	type Error = RxWorkspaceError;

	fn try_from(value: WorkspaceProblems) -> Result<Self, Self::Error> {
		if value.is_empty() {
			Ok(())
		} else {
			Err(value.into())
		}
	}
}

impl std::fmt::Display for WorkspaceProblems {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (package_name, package_errors) in self.packages_with_errors.iter() {
			write!(f, "\n- {}", package_name)?;
			for package_error in package_errors.iter() {
				write!(f, "\n\t- {}", package_error)?;
			}
		}

		Ok(())
	}
}

pub struct WorkspaceProblemPackageScope<'a> {
	workspace_problems: &'a mut WorkspaceProblems,
	package_name: &'a PackageName,
}

impl<'a> WorkspaceProblemPackageScope<'a> {
	pub fn add_problem(&mut self, error: impl 'static + Error) {
		self.workspace_problems
			.add_problem_for_package(self.package_name, error);
	}
}
