mod closed_observable;

pub mod observable {
	pub use super::closed_observable::*;
}

#[cfg(feature = "observable_fn")]
mod closed_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::closed_observable_fn::*;
}
