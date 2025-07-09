mod map_into_operator;
mod map_into_subscriber;

pub use map_into_operator::*;
pub use map_into_subscriber::*;

#[cfg(feature = "compose")]
pub mod map_into_extension_compose;

#[cfg(feature = "pipe")]
pub mod map_into_extension_pipe;

pub mod prelude {
	pub use super::map_into_operator::*;

	#[cfg(feature = "compose")]
	pub use super::map_into_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::map_into_extension_pipe::*;
}
