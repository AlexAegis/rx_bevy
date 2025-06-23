mod finalize_operator;
mod finalize_subscriber;

pub use finalize_operator::*;
pub use finalize_subscriber::*;

#[cfg(feature = "compose")]
pub mod finalize_extension_compose;

#[cfg(feature = "pipe")]
pub mod finalize_extension_pipe;

pub mod prelude {
	pub use crate::finalize_operator::*;

	#[cfg(feature = "compose")]
	pub use crate::finalize_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use crate::finalize_extension_pipe::*;
}
