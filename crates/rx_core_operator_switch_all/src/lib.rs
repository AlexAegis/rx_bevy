mod switch_all_operator;
mod switch_all_subscriber;

pub use switch_all_subscriber::*;

pub mod operator {
	pub use super::switch_all_operator::*;
}

#[cfg(feature = "compose")]
mod switch_all_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::switch_all_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod switch_all_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::switch_all_extension_pipe::*;
}
