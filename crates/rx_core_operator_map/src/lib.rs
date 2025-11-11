mod map_operator;
mod map_subscriber;

pub use map_subscriber::*;

pub mod operator {
	pub use super::map_operator::*;
}

#[cfg(feature = "compose")]
mod map_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::map_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod map_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::map_extension_pipe::*;
}

#[cfg(feature = "operator_fn")]
mod map_fn;

#[cfg(feature = "operator_fn")]
pub mod operator_fn {
	pub use super::map_fn::*;
}
