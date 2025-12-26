// #![cfg_attr(not(test), no_std)]

mod join_observable;
mod join_subscriber;

pub use join_subscriber::*;

pub mod observable {
	pub use super::join_observable::*;
}

#[cfg(feature = "observable_fn")]
mod join_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::join_fn::*;
}
