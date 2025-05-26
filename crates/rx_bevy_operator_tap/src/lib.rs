// The implementation is in a separate file so it's easier to search for it
mod tap;
pub use tap::*;

#[cfg(feature = "pipe")]
pub mod tap_extension;

pub mod prelude {
	pub use crate::tap::*;

	#[cfg(feature = "pipe")]
	pub use crate::tap_extension::*;
}
