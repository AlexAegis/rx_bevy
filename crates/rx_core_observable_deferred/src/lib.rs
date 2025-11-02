#![no_std]

mod deferred_observable;

pub mod observable {
	pub use super::deferred_observable::*;
}

#[cfg(feature = "observable_fn")]
mod deferred_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::deferred_observable_fn::*;
}
