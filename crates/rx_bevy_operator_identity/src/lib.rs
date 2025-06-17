mod identity;
pub use identity::*;

#[cfg(feature = "pipe")]
pub mod identity_extension;

pub mod prelude {
	pub use crate::identity::*;

	#[cfg(feature = "pipe")]
	pub use crate::identity_extension::*;
}
