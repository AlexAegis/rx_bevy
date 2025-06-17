mod map_operator;
pub use map_operator::*;

#[cfg(feature = "pipe")]
pub mod map_extension;

pub mod prelude {
	pub use crate::map_operator::*;

	#[cfg(feature = "pipe")]
	pub use crate::map_extension::*;
}
