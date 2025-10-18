mod switch_map_operator;
mod switch_map_subscriber;

pub use switch_map_operator::*;
pub use switch_map_subscriber::*;

#[cfg(feature = "compose")]
pub mod switch_map_extension_compose;

#[cfg(feature = "pipe")]
pub mod switch_map_extension_pipe;

pub mod prelude {
	pub use super::switch_map_operator::*;

	#[cfg(feature = "compose")]
	pub use super::switch_map_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::switch_map_extension_pipe::*;
}
