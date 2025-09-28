mod tap_next_operator;
mod tap_next_subscriber;

pub use tap_next_operator::*;
pub use tap_next_subscriber::*;

#[cfg(feature = "compose")]
pub mod tap_next_extension_compose;

#[cfg(feature = "pipe")]
pub mod tap_next_extension_pipe;

pub mod prelude {
	pub use super::tap_next_operator::*;

	#[cfg(feature = "compose")]
	pub use super::tap_next_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::tap_next_extension_pipe::*;
}
