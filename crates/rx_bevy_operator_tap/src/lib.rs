mod tap_operator;
mod tap_subscriber;

pub use tap_operator::*;
pub use tap_subscriber::*;

#[cfg(feature = "compose")]
pub mod tap_extension_compose;

#[cfg(feature = "pipe")]
pub mod tap_extension_pipe;

pub mod prelude {
	pub use super::tap_operator::*;

	#[cfg(feature = "compose")]
	pub use super::tap_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::tap_extension_pipe::*;
}
