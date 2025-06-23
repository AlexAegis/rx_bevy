mod filter_map_operator;

pub use filter_map_operator::*;

#[cfg(feature = "pipe")]
pub mod filter_map_extension;

pub mod prelude {
	pub use crate::filter_map_operator::*;

	#[cfg(feature = "pipe")]
	pub use crate::filter_map_extension::*;
}
