pub use rx_bevy_observable::*;

// Observables
#[cfg(feature = "observable_of")]
pub use rx_bevy_observable_of::*;
#[cfg(feature = "observable_throw")]
pub use rx_bevy_observable_throw::*;
// Pipes
#[cfg(feature = "pipe_flat")]
pub use rx_bevy_pipe_flat::*;
#[cfg(feature = "pipe_operator")]
pub use rx_bevy_pipe_operator::*;
// Observers
#[cfg(feature = "observer_flat")]
pub use rx_bevy_observer_flat::*;
#[cfg(feature = "observer_fn")]
pub use rx_bevy_observer_fn::*;
#[cfg(feature = "observer_noop")]
pub use rx_bevy_observer_noop::*;
#[cfg(feature = "observer_print")]
pub use rx_bevy_observer_print::*;
#[cfg(feature = "observer_shared")]
pub use rx_bevy_observer_shared::*;
// Operators
#[cfg(feature = "operator_finalize")]
pub use rx_bevy_operator_finalize::*;
#[cfg(feature = "operator_identity")]
pub use rx_bevy_operator_identity::*;
#[cfg(feature = "operator_map")]
pub use rx_bevy_operator_map::*;
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

	// Observables
	#[cfg(feature = "observable_of")]
	pub use rx_bevy_observable_of::prelude::*;
	#[cfg(feature = "observable_throw")]
	pub use rx_bevy_observable_throw::prelude::*;
	// Pipes
	#[cfg(feature = "pipe_flat")]
	pub use rx_bevy_pipe_flat::prelude::*;
	#[cfg(feature = "pipe_operator")]
	pub use rx_bevy_pipe_operator::prelude::*;
	// Observers
	#[cfg(feature = "observer_flat")]
	pub use rx_bevy_observer_flat::prelude::*;
	#[cfg(feature = "observer_fn")]
	pub use rx_bevy_observer_fn::prelude::*;
	#[cfg(feature = "observer_noop")]
	pub use rx_bevy_observer_noop::prelude::*;
	#[cfg(feature = "observer_print")]
	pub use rx_bevy_observer_print::prelude::*;
	#[cfg(feature = "observer_shared")]
	pub use rx_bevy_observer_shared::prelude::*;
	// Operators
	#[cfg(feature = "operator_finalize")]
	pub use rx_bevy_operator_finalize::prelude::*;
	#[cfg(feature = "operator_identity")]
	pub use rx_bevy_operator_identity::prelude::*;
	#[cfg(feature = "operator_map")]
	pub use rx_bevy_operator_map::prelude::*;
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
