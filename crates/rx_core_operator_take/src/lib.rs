mod take_operator;
mod take_subscriber;

pub use take_subscriber::*;

pub mod operator {
	pub use super::take_operator::*;
}

#[cfg(feature = "compose")]
mod take_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::take_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod take_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::take_extension_pipe::*;
}

#[cfg(feature = "operator_fn")]
mod take_fn;

#[cfg(feature = "operator_fn")]
pub mod operator_fn {
	pub use super::take_fn::*;
}
