// The implementation is in a separate file so it's easier to search for it
mod filter_map;
pub use filter_map::*;

#[cfg(feature = "pipe")]
pub mod filter_map_extension;

pub mod prelude {
	pub use crate::filter_map::*;

	#[cfg(feature = "pipe")]
	pub use crate::filter_map_extension::*;
}
