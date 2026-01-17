mod just_observable;

pub mod observable {
	pub use super::just_observable::*;
}

#[cfg(feature = "observable_fn")]
mod just_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::just_observable_fn::*;
}
