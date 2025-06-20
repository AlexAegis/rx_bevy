mod tap_operator;
mod tap_subscriber;

pub use tap_operator::*;
pub use tap_subscriber::*;

#[cfg(feature = "pipe")]
pub mod tap_extension;

pub mod prelude {
	pub use crate::tap_operator::*;

	#[cfg(feature = "pipe")]
	pub use crate::tap_extension::*;
}
