mod never_observable;

pub mod observable {
	pub use super::never_observable::*;
}

#[cfg(feature = "observable_fn")]
mod never_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::never_observable_fn::*;
}
