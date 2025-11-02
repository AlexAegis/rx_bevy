#![cfg_attr(not(test), no_std)]

mod combine_latest_observable;
mod combine_latest_subscriber;

pub use combine_latest_subscriber::*;

pub mod observable {
	pub use super::combine_latest_observable::*;
}

#[cfg(feature = "observable_fn")]
mod combine_latest_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::combine_latest_fn::*;
}
