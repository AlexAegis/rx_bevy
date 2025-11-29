mod finalize_operator;

pub mod operator {
	pub use super::finalize_operator::*;
}

#[cfg(feature = "compose")]
mod finalize_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::finalize_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod finalize_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::finalize_extension_pipe::*;
}
