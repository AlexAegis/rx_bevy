mod into_result_operator;
mod into_result_subscriber;

pub use into_result_subscriber::*;

pub mod operator {
	pub use super::into_result_operator::*;
}

#[cfg(feature = "compose")]
mod into_result_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::into_result_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod into_result_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::into_result_extension_pipe::*;
}
