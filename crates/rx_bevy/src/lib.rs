pub use rx_bevy_observable::*;
pub use rx_bevy_operator::*;

// Observables
#[cfg(feature = "observable_of")]
pub use rx_bevy_observable_of::*;
// Observers
#[cfg(feature = "observer_fn")]
pub use rx_bevy_observer_fn::*;
#[cfg(feature = "observer_print")]
pub use rx_bevy_observer_print::*;
// Operators
#[cfg(feature = "operator_finalize")]
pub use rx_bevy_operator_finalize::*;
#[cfg(feature = "operator_map")]
pub use rx_bevy_operator_map::*;
#[cfg(feature = "operator_pipe")]
pub use rx_bevy_operator_pipe::*;
#[cfg(feature = "operator_tap")]
pub use rx_bevy_operator_tap::*;
// Subjects
#[cfg(feature = "subject")]
pub use rx_bevy_subject::*;
#[cfg(feature = "subject_behavior")]
pub use rx_bevy_subject_behavior::*;
#[cfg(feature = "subject_replay")]
pub use rx_bevy_subject_replay::*;

pub mod prelude {
	pub use rx_bevy_observable::prelude::*;
	pub use rx_bevy_operator::prelude::*;

	// Observables
	#[cfg(feature = "observable_of")]
	pub use rx_bevy_observable_of::prelude::*;
	// Observers
	#[cfg(feature = "observer_fn")]
	pub use rx_bevy_observer_fn::prelude::*;
	#[cfg(feature = "observer_noop")]
	pub use rx_bevy_observer_noop::prelude::*;
	#[cfg(feature = "observer_print")]
	pub use rx_bevy_observer_print::prelude::*;
	// Operators
	#[cfg(feature = "operator_finalize")]
	pub use rx_bevy_operator_finalize::prelude::*;
	#[cfg(feature = "operator_map")]
	pub use rx_bevy_operator_map::prelude::*;
	#[cfg(feature = "operator_pipe")]
	pub use rx_bevy_operator_pipe::prelude::*;
	#[cfg(feature = "operator_tap")]
	pub use rx_bevy_operator_tap::prelude::*;

	// Subjects
	#[cfg(feature = "subject")]
	pub use rx_bevy_subject::prelude::*;
	#[cfg(feature = "subject_behavior")]
	pub use rx_bevy_subject_behavior::prelude::*;
	#[cfg(feature = "subject_replay")]
	pub use rx_bevy_subject_replay::prelude::*;
}
