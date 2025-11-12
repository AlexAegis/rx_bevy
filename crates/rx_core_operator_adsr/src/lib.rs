mod adsr;
mod adsr_operator;
mod adsr_operator_options;
mod adsr_subscriber;
mod model;

pub use adsr::*;
pub use adsr_subscriber::*;
pub use model::*;

// TODO: Extension pipe once the operator is ready

pub mod operator {
	pub use super::adsr::*;
	pub use super::adsr_operator::*;
	pub use super::adsr_operator_options::*;
	pub use super::model::*;
}

#[cfg(feature = "compose")]
mod adsr_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use super::adsr_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod adsr_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::adsr_extension_pipe::*;
}

#[cfg(feature = "operator_fn")]
mod adsr_fn;

#[cfg(feature = "operator_fn")]
pub mod operator_fn {
	pub use super::adsr_fn::*;
}
