// The implementation is in a separate file so it's easier to search for it
mod switch_map;

pub use switch_map::*;

#[cfg(feature = "pipe")]
pub mod switch_map_extension;

pub mod prelude {
	pub use crate::switch_map::*;

	#[cfg(feature = "pipe")]
	pub use crate::switch_map_extension::*;
}
