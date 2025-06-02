// The implementation is in a separate file so it's easier to search for it
mod throw;
pub use throw::*;

pub mod prelude {
	pub use crate::throw::*;
}
