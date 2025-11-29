mod merge_map_operator;
mod merge_map_subscriber;

pub use merge_map_subscriber::*;

pub mod operator {
	pub use super::merge_map_operator::*;
}

#[cfg(feature = "compose")]
mod merge_map_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::merge_map_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod merge_map_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::merge_map_extension_pipe::*;
}
