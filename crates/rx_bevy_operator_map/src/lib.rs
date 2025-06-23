mod map_operator;
mod map_subscriber;

pub use map_operator::*;
pub use map_subscriber::*;

#[cfg(feature = "compose")]
pub mod map_extension_compose;

#[cfg(feature = "pipe")]
pub mod map_extension_pipe;

pub mod prelude {
	pub use crate::map_operator::*;

	#[cfg(feature = "compose")]
	pub use crate::map_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use crate::map_extension_pipe::*;
}
