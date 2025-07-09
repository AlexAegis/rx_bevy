mod deferred_observable;
mod detached_observable;

pub use deferred_observable::*;
pub use detached_observable::*;

pub mod prelude {
	pub use super::deferred_observable::*;
	pub use super::detached_observable::*;
}
