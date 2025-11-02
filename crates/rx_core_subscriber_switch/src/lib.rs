mod externally_managed_subscriber;
mod switch_subscriber;

pub use externally_managed_subscriber::*;
pub use switch_subscriber::*;

pub mod prelude {
	pub use super::switch_subscriber::*;
}
