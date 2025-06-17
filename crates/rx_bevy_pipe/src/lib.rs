mod pipe;

pub use pipe::*;

pub mod pipe_extension;

pub mod prelude {
	pub use crate::pipe::*;
	pub use crate::pipe_extension::*;
}
