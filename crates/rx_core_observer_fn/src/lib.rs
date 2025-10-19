mod dyn_fn_observer;
mod fn_observer;

pub mod observer {
	pub use super::dyn_fn_observer::*;
	pub use super::fn_observer::*;
}
