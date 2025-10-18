mod noop_observer;
pub use noop_observer::*;

pub mod prelude {
	pub use super::noop_observer::*;
}
