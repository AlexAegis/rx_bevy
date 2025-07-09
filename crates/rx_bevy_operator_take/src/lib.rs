mod take_operator;
mod take_subscriber;

pub use take_operator::*;
pub use take_subscriber::*;

#[cfg(feature = "compose")]
pub mod take_extension_compose;

#[cfg(feature = "pipe")]
pub mod take_extension_pipe;

pub mod prelude {
	pub use super::take_operator::*;

	#[cfg(feature = "compose")]
	pub use super::take_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::take_extension_pipe::*;
}
