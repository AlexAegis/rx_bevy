mod identity_fn;
mod identity_operator;
mod identity_subscriber;

pub use identity_subscriber::*;

pub mod operator {
	pub use super::identity_fn::*;
	pub use super::identity_operator::*;
}
