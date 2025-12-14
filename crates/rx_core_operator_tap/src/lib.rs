mod tap_operator;
mod tap_subscriber;

pub use tap_subscriber::*;

pub mod operator {
	pub use super::tap_operator::*;
}

#[cfg(feature = "compose")]
mod tap_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::tap_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod tap_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::tap_extension_pipe::*;
}
