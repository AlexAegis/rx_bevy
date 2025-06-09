// The implementation is in a separate file so it's easier to search for it
mod flat_pipe;

pub use flat_pipe::*;

pub mod flat_pipe_extension;

pub mod prelude {
	pub use crate::flat_pipe::*;
	pub use crate::flat_pipe_extension::*;
}
