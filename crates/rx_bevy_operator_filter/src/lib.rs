// The implementation is in a separate file so it's easier to search for it
mod filter;
pub use filter::*;

#[cfg(feature = "pipe")]
pub mod filter_extension;

pub mod prelude {
	pub use crate::filter::*;

	#[cfg(feature = "pipe")]
	pub use crate::filter_extension::*;
}
