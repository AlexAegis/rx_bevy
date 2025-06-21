mod take_operator;
mod take_subscriber;

pub use take_operator::*;
pub use take_subscriber::*;

#[cfg(feature = "pipe")]
pub mod take_extension;

pub mod prelude {
	pub use crate::take_operator::*;

	#[cfg(feature = "pipe")]
	pub use crate::take_extension::*;
}
