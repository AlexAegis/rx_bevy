mod switch_map_operator;
mod switch_map_subscriber;

pub use switch_map_subscriber::*;

pub mod operator {
	pub use super::switch_map_operator::*;
}

#[cfg(feature = "compose")]
mod switch_map_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::switch_map_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod switch_map_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::switch_map_extension_pipe::*;
}
