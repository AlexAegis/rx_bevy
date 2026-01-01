mod pairwise_operator;
mod pairwise_subscriber;

pub use pairwise_subscriber::*;

pub mod operator {
	pub use super::pairwise_operator::*;
}

#[cfg(feature = "compose")]
mod pairwise_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::pairwise_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod pairwise_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::pairwise_extension_pipe::*;
}
