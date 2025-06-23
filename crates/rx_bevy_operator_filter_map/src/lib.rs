mod filter_map_operator;

pub use filter_map_operator::*;

#[cfg(feature = "compose")]
pub mod filter_map_extension_compose;

#[cfg(feature = "pipe")]
pub mod filter_map_extension_pipe;

pub mod prelude {
	pub use crate::filter_map_operator::*;

	#[cfg(feature = "compose")]
	pub use crate::filter_map_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use crate::filter_map_extension_pipe::*;
}
