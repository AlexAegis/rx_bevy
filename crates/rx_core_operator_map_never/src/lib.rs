mod map_never_both_operator;
mod map_never_both_subscriber;
mod map_never_error_operator;
mod map_never_error_subscriber;
mod map_never_next_operator;
mod map_never_next_subscriber;

pub use map_never_both_subscriber::*;
pub use map_never_error_subscriber::*;
pub use map_never_next_subscriber::*;

pub mod operator {
	pub use super::map_never_both_operator::*;
	pub use super::map_never_error_operator::*;
	pub use super::map_never_next_operator::*;
}

#[cfg(feature = "compose")]
mod map_never_next_extension_compose;

#[cfg(feature = "compose")]
mod map_never_error_extension_compose;

#[cfg(feature = "compose")]
mod map_never_both_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::map_never_both_extension_compose::*;
	pub use super::map_never_error_extension_compose::*;
	pub use super::map_never_next_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod map_never_next_extension_pipe;

#[cfg(feature = "pipe")]
mod map_never_error_extension_pipe;

#[cfg(feature = "pipe")]
mod map_never_both_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::map_never_both_extension_pipe::*;
	pub use super::map_never_error_extension_pipe::*;
	pub use super::map_never_next_extension_pipe::*;
}
