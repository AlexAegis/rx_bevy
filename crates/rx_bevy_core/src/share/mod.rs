mod erased_shared_destination;
mod shared_destination;

pub use erased_shared_destination::*;
pub use shared_destination::*;

pub mod prelude {
	pub use super::erased_shared_destination::*;
	pub use super::shared_destination::*;
}
