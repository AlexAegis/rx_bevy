// The implementation is in a separate file so it's easier to search for it
mod map;
pub use map::*;

#[cfg(feature = "pipe")]
pub mod map_extension;

pub mod prelude {
	pub use crate::map::*;

	#[cfg(feature = "pipe")]
	pub use crate::map_extension::*;
}
