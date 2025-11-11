mod identity_operator;
mod identity_subscriber;

pub use identity_subscriber::*;

pub mod operator {
	pub use super::identity_operator::*;
}

#[cfg(feature = "operator_fn")]
mod identity_fn;

#[cfg(feature = "operator_fn")]
pub mod operator_fn {
	pub use super::identity_fn::*;
}
