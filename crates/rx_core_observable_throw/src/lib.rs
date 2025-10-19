mod throw_observable;

pub mod observable {
	pub use super::throw_observable::*;
}

#[cfg(feature = "observable_fn")]
mod throw_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::throw_observable_fn::*;
}
