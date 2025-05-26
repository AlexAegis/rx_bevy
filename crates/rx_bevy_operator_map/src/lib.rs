// The implementation is in a separate file so it's easier to search for it
mod map;
pub use map::*;
pub mod map_extension;

pub mod prelude {
	pub use crate::map::*;
	pub use crate::map_extension::*;
}
