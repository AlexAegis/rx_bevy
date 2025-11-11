mod tap_next_operator;
mod tap_next_subscriber;

pub use tap_next_subscriber::*;

pub mod operator {
	pub use super::tap_next_operator::*;
}

#[cfg(feature = "compose")]
mod tap_next_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::tap_next_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod tap_next_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::tap_next_extension_pipe::*;
}

#[cfg(feature = "operator_fn")]
mod tap_next_fn;

#[cfg(feature = "operator_fn")]
pub mod operator_fn {
	pub use super::tap_next_fn::*;
}
