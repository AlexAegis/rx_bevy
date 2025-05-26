// The implementation is in a separate file so it's easier to search for it
mod tap;
pub use tap::*;
pub mod tap_extension;

pub mod prelude {
	pub use crate::tap::*;
	pub use crate::tap_extension::*;
}
