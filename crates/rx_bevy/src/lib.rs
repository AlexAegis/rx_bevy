pub use rx_bevy_core::*;

// Observables
#[cfg(feature = "observable_deferred")]
pub use rx_bevy_observable_deferred::*;
#[cfg(feature = "observable_iterator")]
pub use rx_bevy_observable_iterator::*;
#[cfg(feature = "observable_of")]
pub use rx_bevy_observable_of::*;
#[cfg(feature = "observable_throw")]
pub use rx_bevy_observable_throw::*;
#[cfg(feature = "ref_observable_combine_latest")]
pub use rx_bevy_ref_observable_combine_latest::*;
#[cfg(feature = "ref_observable_connectable")]
pub use rx_bevy_ref_observable_connectable::*;
#[cfg(feature = "ref_observable_merge")]
pub use rx_bevy_ref_observable_merge::*;
#[cfg(feature = "ref_observable_zip")]
pub use rx_bevy_ref_observable_zip::*;
// Pipe
#[cfg(feature = "pipe")]
pub use rx_bevy_ref_pipe::*;
// Observers
#[cfg(feature = "observer_fn")]
pub use rx_bevy_observer_fn::*;
#[cfg(feature = "observer_noop")]
pub use rx_bevy_observer_noop::*;
#[cfg(feature = "observer_print")]
pub use rx_bevy_observer_print::*;
// Operators
#[cfg(feature = "operator_adsr")]
pub use rx_bevy_operator_adsr::*;
#[cfg(feature = "operator_composite")]
pub use rx_bevy_operator_composite::*;
#[cfg(feature = "operator_enumerate")]
pub use rx_bevy_operator_enumerate::*;
#[cfg(feature = "operator_filter")]
pub use rx_bevy_operator_filter::*;
#[cfg(feature = "operator_filter_map")]
pub use rx_bevy_operator_filter_map::*;
#[cfg(feature = "operator_finalize")]
pub use rx_bevy_operator_finalize::*;
#[cfg(feature = "operator_identity")]
pub use rx_bevy_operator_identity::*;
#[cfg(feature = "operator_lift_option")]
pub use rx_bevy_operator_lift_option::*;
#[cfg(feature = "operator_lift_result")]
pub use rx_bevy_operator_lift_result::*;
#[cfg(feature = "operator_map")]
pub use rx_bevy_operator_map::*;
#[cfg(feature = "operator_map_into")]
pub use rx_bevy_operator_map_into::*;
#[cfg(feature = "operator_skip")]
pub use rx_bevy_operator_skip::*;
#[cfg(feature = "operator_switch_map")]
pub use rx_bevy_operator_switch_map::*;
#[cfg(feature = "operator_take")]
pub use rx_bevy_operator_take::*;
#[cfg(feature = "operator_tap")]
pub use rx_bevy_operator_tap_next::*;
#[cfg(feature = "operator_try_capture")]
pub use rx_bevy_operator_try_capture::*;
// Subjects
#[cfg(feature = "ref_subject")]
pub use rx_bevy_ref_subject::*;
#[cfg(feature = "ref_subject_behavior")]
pub use rx_bevy_ref_subject_behavior::*;
#[cfg(feature = "ref_subject_replay")]
pub use rx_bevy_ref_subject_replay::*;

pub mod prelude {
	pub use rx_bevy_core::prelude::*;

	// Observables
	#[cfg(feature = "observable_deferred")]
	pub use rx_bevy_observable_deferred::prelude::*;
	#[cfg(feature = "observable_iterator")]
	pub use rx_bevy_observable_iterator::prelude::*;
	#[cfg(feature = "observable_of")]
	pub use rx_bevy_observable_of::prelude::*;
	#[cfg(feature = "observable_throw")]
	pub use rx_bevy_observable_throw::prelude::*;
	#[cfg(feature = "ref_observable_combine_latest")]
	pub use rx_bevy_ref_observable_combine_latest::prelude::*;
	#[cfg(feature = "ref_observable_connectable")]
	pub use rx_bevy_ref_observable_connectable::prelude::*;
	#[cfg(feature = "ref_observable_merge")]
	pub use rx_bevy_ref_observable_merge::prelude::*;
	#[cfg(feature = "ref_observable_zip")]
	pub use rx_bevy_ref_observable_zip::prelude::*;
	// Pipe
	#[cfg(feature = "pipe")]
	pub use rx_bevy_ref_pipe::prelude::*;
	// Observers
	#[cfg(feature = "observer_fn")]
	pub use rx_bevy_observer_fn::prelude::*;
	#[cfg(feature = "observer_noop")]
	pub use rx_bevy_observer_noop::prelude::*;
	#[cfg(feature = "observer_print")]
	pub use rx_bevy_observer_print::prelude::*;
	// Operators
	#[cfg(feature = "operator_adsr")]
	pub use rx_bevy_operator_adsr::prelude::*;
	#[cfg(feature = "operator_composite")]
	pub use rx_bevy_operator_composite::prelude::*;
	#[cfg(feature = "operator_enumerate")]
	pub use rx_bevy_operator_enumerate::prelude::*;
	#[cfg(feature = "operator_filter")]
	pub use rx_bevy_operator_filter::prelude::*;
	#[cfg(feature = "operator_filter_map")]
	pub use rx_bevy_operator_filter_map::prelude::*;
	#[cfg(feature = "operator_finalize")]
	pub use rx_bevy_operator_finalize::prelude::*;
	#[cfg(feature = "operator_identity")]
	pub use rx_bevy_operator_identity::prelude::*;
	#[cfg(feature = "operator_lift_option")]
	pub use rx_bevy_operator_lift_option::prelude::*;
	#[cfg(feature = "operator_lift_result")]
	pub use rx_bevy_operator_lift_result::prelude::*;
	#[cfg(feature = "operator_map")]
	pub use rx_bevy_operator_map::prelude::*;
	#[cfg(feature = "operator_map_into")]
	pub use rx_bevy_operator_map_into::prelude::*;
	#[cfg(feature = "operator_skip")]
	pub use rx_bevy_operator_skip::prelude::*;
	#[cfg(feature = "operator_switch_map")]
	pub use rx_bevy_operator_switch_map::prelude::*;
	#[cfg(feature = "operator_take")]
	pub use rx_bevy_operator_take::prelude::*;
	#[cfg(feature = "operator_tap")]
	pub use rx_bevy_operator_tap_next::prelude::*;
	#[cfg(feature = "operator_try_capture")]
	pub use rx_bevy_operator_try_capture::prelude::*;

	// Subjects
	#[cfg(feature = "ref_subject")]
	pub use rx_bevy_ref_subject::prelude::*;
	#[cfg(feature = "ref_subject_behavior")]
	pub use rx_bevy_ref_subject_behavior::prelude::*;
	#[cfg(feature = "ref_subject_replay")]
	pub use rx_bevy_ref_subject_replay::prelude::*;
}
