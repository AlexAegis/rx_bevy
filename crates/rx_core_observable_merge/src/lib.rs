mod merge_observable;

pub mod observable {
	pub use super::merge_observable::*;
}

#[cfg(feature = "observable_fn")]
mod merge_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::merge_observable_fn::*;
}
