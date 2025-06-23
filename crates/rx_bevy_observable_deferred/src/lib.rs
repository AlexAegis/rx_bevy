mod deferred_observable;
mod detached_observable;

pub use deferred_observable::*;
pub use detached_observable::*;

pub mod prelude {
	pub use crate::detached_observable::*;

	pub use crate::deferred_observable::*;
}
