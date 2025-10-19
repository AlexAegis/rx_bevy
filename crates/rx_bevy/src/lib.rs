pub use rx_bevy_plugin::*;
pub use rx_core_traits::*;

pub mod observable {
	pub use rx_core::observable::*;

	#[cfg(feature = "observable_keyboard")]
	pub use rx_bevy_observable_keyboard::observable::*;
}

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	pub use rx_core::extension_pipe::*;
}

#[cfg(feature = "compose")]
pub mod extension_composite {
	pub use rx_core::extension_composite::*;
}

pub mod operator {
	pub use rx_core::operator::*;
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
	pub use rx_bevy_plugin::*;
	pub use rx_core_traits::*;

	pub use super::context::*;
	pub use super::observable::*;
	pub use super::observer::*;
	pub use super::operator::*;
	pub use super::subject::*;

	pub use super::extension_composite::*;
	pub use super::extension_pipe::*;
}
