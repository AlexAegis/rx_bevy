mod higher_order_pipe;
mod operator;

pub use higher_order_pipe::*;
pub use operator::*;

pub mod prelude {
	pub use crate::higher_order_pipe::*;
	pub use crate::operator::*;
}
