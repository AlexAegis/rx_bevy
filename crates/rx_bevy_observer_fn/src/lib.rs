mod dyn_fn_observer;
mod fn_observer;

pub use dyn_fn_observer::*;
pub use fn_observer::*;

pub mod prelude {
	pub use super::dyn_fn_observer::*;
	pub use super::fn_observer::*;
}
