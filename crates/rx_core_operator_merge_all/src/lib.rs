mod merge_all_operator;

pub mod operator {
	pub use super::merge_all_operator::*;
}

#[cfg(feature = "compose")]
mod merge_all_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::merge_all_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod merge_all_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::merge_all_extension_pipe::*;
}
