mod finalize_operator;
pub use finalize_operator::*;

#[cfg(feature = "pipe")]
pub mod finalize_extension;

pub mod prelude {
	pub use crate::finalize_operator::*;

	#[cfg(feature = "pipe")]
	pub use crate::finalize_extension::*;
}
