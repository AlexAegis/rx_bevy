#![cfg_attr(not(test), no_std)]

mod iterator_observable;
mod iterator_observable_extension;

pub mod observable {
	pub use super::iterator_observable::*;
	pub use super::iterator_observable_extension::*;
}
