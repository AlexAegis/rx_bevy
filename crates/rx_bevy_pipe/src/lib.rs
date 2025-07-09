mod pipe;

pub use pipe::*;

pub mod pipe_extension;

pub mod prelude {
	pub use super::pipe::*;

	pub use super::pipe_extension::*;
}
