mod share_observable;
mod share_operator;

pub mod observable {
	pub use super::share_observable::*;
}

pub mod operator {
	pub use super::share_operator::*;
	pub use rx_core_observable_connectable::observable::ConnectableOptions;
}

#[cfg(feature = "pipe")]
mod share_extension_pipe;

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use super::share_extension_pipe::*;
}
