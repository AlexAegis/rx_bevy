mod adsr;
mod adsr_operator;
mod adsr_operator_options;
mod adsr_subscriber;
mod model;

pub use adsr::*;
pub use adsr_subscriber::*;
pub use model::*;

pub mod operator {
	pub use super::adsr::*;
	pub use super::adsr_operator::*;
	pub use super::adsr_operator_options::*;
	pub use super::model::*;
}

#[cfg(feature = "compose")]
mod adsr_extension_compose;

#[cfg(feature = "compose")]
pub mod extension_compose {
	pub use super::adsr_extension_compose::*;
}

#[cfg(feature = "pipe")]
mod adsr_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::adsr_extension_pipe::*;
}
