mod timer_observable;
mod timer_subscription;

pub use timer_subscription::*;

pub mod observable {
	pub use super::timer_observable::*;
}

#[cfg(feature = "observable_fn")]
mod timer_observable_fn;

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use super::timer_observable_fn::*;
}

#[cfg(test)]
mod timer_observable_test;
