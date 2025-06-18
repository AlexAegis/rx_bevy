mod subject;

pub use subject::*;

// Crates extending a subject should have access to multicast
pub use rx_bevy_operator_multicast::*;

pub mod prelude {
	pub use crate::subject::*;
}
