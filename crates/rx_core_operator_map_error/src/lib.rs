mod map_error_operator;
mod map_error_subscriber;

pub use map_error_subscriber::*;

pub mod operator {
	pub use super::map_error_operator::*;
}

#[cfg(feature = "compose")]
mod map_error_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::map_error_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod map_error_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::map_error_extension_pipe::*;
}
