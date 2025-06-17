mod iterator_observable;
pub use iterator_observable::*;

mod iterator_observable_extension;
pub use iterator_observable_extension::*;

pub mod prelude {
	pub use crate::iterator_observable::*;

	pub use crate::iterator_observable_extension::*;
}
