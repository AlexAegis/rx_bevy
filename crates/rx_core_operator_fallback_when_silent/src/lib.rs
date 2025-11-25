mod fallback_when_silent_operator;
mod fallback_when_silent_subscriber;

pub use fallback_when_silent_subscriber::*;

pub mod operator {
	pub use super::fallback_when_silent_operator::*;
}

#[cfg(feature = "compose")]
mod fallback_when_silent_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::fallback_when_silent_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod fallback_when_silent_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::fallback_when_silent_extension_pipe::*;
}
