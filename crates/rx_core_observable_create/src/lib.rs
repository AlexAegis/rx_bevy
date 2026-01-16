mod create_observable;

pub mod observable {
	pub use super::create_observable::*;
}

#[cfg(feature = "observable_fn")]
mod create_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::create_observable_fn::*;
}
