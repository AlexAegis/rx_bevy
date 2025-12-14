mod behavior_subject;

pub mod subject {
	pub use super::behavior_subject::*;
}

pub mod prelude {
	pub use super::subject::*;
}

#[cfg(test)]
mod behavior_subject_test;
