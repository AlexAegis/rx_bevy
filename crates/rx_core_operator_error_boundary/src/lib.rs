mod error_boundary_operator;
mod error_boundary_subscriber;

pub use error_boundary_subscriber::*;

pub mod operator {
	pub use super::error_boundary_operator::*;
}

#[cfg(feature = "compose")]
mod error_boundary_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::error_boundary_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod error_boundary_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::error_boundary_extension_pipe::*;
}
