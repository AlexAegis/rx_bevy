mod switch_map_operator;

pub use switch_map_operator::*;

#[cfg(feature = "pipe")]
pub mod switch_map_extension;

pub mod prelude {
	pub use crate::switch_map_operator::*;

	// #[cfg(feature = "pipe")]
	// pub use crate::switch_map_extension::*;
}
