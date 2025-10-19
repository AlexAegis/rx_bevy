mod of_observable;

pub mod observable {
	pub use super::of_observable::*;
}

#[cfg(feature = "observable_fn")]
mod of_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::of_observable_fn::*;
}
