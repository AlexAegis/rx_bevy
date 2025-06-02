mod dyn_fn_observer;
mod fn_observer;

pub use dyn_fn_observer::*;
pub use fn_observer::*;

pub mod prelude {
	pub use crate::dyn_fn_observer::*;
	pub use crate::fn_observer::*;
}
