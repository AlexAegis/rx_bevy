mod identity_operator;
mod option_operator;

pub use identity_operator::*;
pub use option_operator::*;

pub mod prelude {
	pub use super::identity_operator::*;
	pub use super::option_operator::*;
}
