mod finalize_operator;

pub use finalize_operator::*;

#[cfg(feature = "compose")]
pub mod finalize_extension_compose;

#[cfg(feature = "pipe")]
pub mod finalize_extension_pipe;

pub mod prelude {
	pub use super::finalize_operator::*;

	#[cfg(feature = "compose")]
	pub use super::finalize_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::finalize_extension_pipe::*;
}
