mod identity_operator;
mod option_operator;

pub use identity_operator::*;
pub use option_operator::*;

pub mod prelude {
	pub use crate::operators::identity_operator::*;
	pub use crate::operators::option_operator::*;
}
