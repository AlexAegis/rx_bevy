mod filter_map_operator;

pub use filter_map_operator::*;

#[cfg(feature = "compose")]
pub mod filter_map_extension_compose;

#[cfg(feature = "pipe")]
pub mod filter_map_extension_pipe;

pub mod prelude {
	pub use super::filter_map_operator::*;

	#[cfg(feature = "compose")]
	pub use super::filter_map_extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::filter_map_extension_pipe::*;
}
