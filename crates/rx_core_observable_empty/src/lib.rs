mod empty_observable;

pub mod observable {
	pub use super::empty_observable::*;
}

#[cfg(feature = "observable_fn")]
mod empty_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::empty_observable_fn::*;
}
