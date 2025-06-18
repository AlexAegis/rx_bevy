mod composite_operator;
mod identity_operator;
mod intermediate_observer;
mod option_operator;

pub use composite_operator::*;
pub use identity_operator::*;
pub use intermediate_observer::*;
pub use option_operator::*;

pub mod prelude {
	pub use crate::operators::composite_operator::*;
	pub use crate::operators::identity_operator::*;
	pub use crate::operators::intermediate_observer::*;
	pub use crate::operators::option_operator::*;
}
