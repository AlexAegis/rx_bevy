mod operator_chain;
mod pipe;

pub use operator_chain::*;
pub use pipe::*;

pub mod pipe_extension;

pub mod prelude {
	pub use crate::operator_chain::*;
	pub use crate::pipe::*;
	pub use crate::pipe_extension::*;
}
