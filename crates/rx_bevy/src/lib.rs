pub use rx_core_traits::*;

pub mod observable {
	pub use rx_core::observable::*;

	#[cfg(feature = "observable_event")]
	pub use rx_bevy_observable_event::observable::*;
	#[cfg(feature = "observable_keyboard")]
	pub use rx_bevy_observable_keyboard::observable::*;
	#[cfg(feature = "observable_message")]
	pub use rx_bevy_observable_message::observable::*;
	#[cfg(feature = "observable_proxy")]
	pub use rx_bevy_observable_proxy::observable::*;
}

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	pub use rx_core::observable_fn::*;
}

pub mod operator {
	pub use rx_core::operator::*;
}

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use rx_core::extension_composite::*;
}

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use rx_core::extension_pipe::*;
}

#[cfg(feature = "operator_fn")]
pub mod operator_fn {
	pub use rx_core::operator_fn::*;
}

pub mod observer {
	pub use rx_core::observer::*;
}

pub mod subject {
	pub use rx_core::subject::*;
}

pub mod context {
	pub use rx_bevy_context::*;
}

pub mod prelude {
	pub use rx_core_traits::*;

	pub use super::context::*;
	pub use super::observable::*;
	pub use super::observer::*;
	pub use super::operator::*;
	pub use super::subject::*;

	#[cfg(feature = "pipe")]
	pub use super::extension_pipe::*;

	#[cfg(feature = "compose")]
	pub use super::extension_composite::*;

	#[cfg(feature = "operator_fn")]
	pub use super::operator_fn::*;
}
