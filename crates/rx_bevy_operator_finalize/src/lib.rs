// The implementation is in a separate file so it's easier to search for it
mod finalize;
pub use finalize::*;

#[cfg(feature = "pipe")]
pub mod finalize_extension;

pub mod prelude {
	pub use crate::finalize::*;

	#[cfg(feature = "pipe")]
	pub use crate::finalize_extension::*;
}
