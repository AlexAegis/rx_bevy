// The implementation is in a separate file so it's easier to search for it
mod of;
pub use of::*;

pub mod prelude {
	pub use crate::of::*;
}
