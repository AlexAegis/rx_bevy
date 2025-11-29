mod reduce_operator;
mod reduce_subscriber;

pub use reduce_subscriber::*;

pub mod operator {
	pub use super::reduce_operator::*;
}

#[cfg(feature = "compose")]
mod reduce_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::reduce_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod reduce_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::reduce_extension_pipe::*;
}
