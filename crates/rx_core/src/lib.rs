#![no_std]

pub use rx_core_traits::*;

pub mod observable {
	#[cfg(feature = "observable_combine_latest")]
	pub use rx_core_observable_combine_latest::observable::*;
	#[cfg(feature = "observable_connectable")]
	pub use rx_core_observable_connectable::observable::*;
	#[cfg(feature = "observable_deferred")]
	pub use rx_core_observable_deferred::observable::*;
	#[cfg(feature = "observable_interval")]
	pub use rx_core_observable_interval::observable::*;
	#[cfg(feature = "observable_iterator")]
	pub use rx_core_observable_iterator::observable::*;
	#[cfg(feature = "observable_iterator_on_tick")]
	pub use rx_core_observable_iterator_on_tick::observable::*;
	#[cfg(feature = "observable_merge")]
	pub use rx_core_observable_merge::observable::*;
	#[cfg(feature = "observable_of")]
	pub use rx_core_observable_of::observable::*;
	#[cfg(feature = "observable_pipe")]
	pub use rx_core_observable_pipe::observable::*;
	#[cfg(feature = "observable_throw")]
	pub use rx_core_observable_throw::observable::*;
	#[cfg(feature = "observable_zip")]
	pub use rx_core_observable_zip::observable::*;
}

#[cfg(feature = "observable_fn")]
pub mod observable_fn {
	#[cfg(feature = "observable_combine_latest")]
	pub use rx_core_observable_combine_latest::observable_fn::*;
	#[cfg(feature = "observable_deferred")]
	pub use rx_core_observable_deferred::observable_fn::*;
	#[cfg(feature = "observable_merge")]
	pub use rx_core_observable_merge::observable_fn::*;
	#[cfg(feature = "observable_of")]
	pub use rx_core_observable_of::observable_fn::*;
	#[cfg(feature = "observable_throw")]
	pub use rx_core_observable_throw::observable_fn::*;
	#[cfg(feature = "observable_zip")]
	pub use rx_core_observable_zip::observable_fn::*;
}

pub mod operator {
	#[cfg(feature = "operator_adsr")]
	pub use rx_core_operator_adsr::operator::*;
	#[cfg(feature = "operator_composite")]
	pub use rx_core_operator_composite::operator::*;
	#[cfg(feature = "operator_enumerate")]
	pub use rx_core_operator_enumerate::operator::*;
	#[cfg(feature = "operator_error_boundary")]
	pub use rx_core_operator_error_boundary::operator::*;
	#[cfg(feature = "operator_fallback_when_silent")]
	pub use rx_core_operator_fallback_when_silent::operator::*;
	#[cfg(feature = "operator_filter")]
	pub use rx_core_operator_filter::operator::*;
	#[cfg(feature = "operator_filter_map")]
	pub use rx_core_operator_filter_map::operator::*;
	#[cfg(feature = "operator_finalize")]
	pub use rx_core_operator_finalize::operator::*;
	#[cfg(feature = "operator_identity")]
	pub use rx_core_operator_identity::operator::*;
	#[cfg(feature = "operator_into_result")]
	pub use rx_core_operator_into_result::operator::*;
	#[cfg(feature = "operator_lift_option")]
	pub use rx_core_operator_lift_option::operator::*;
	#[cfg(feature = "operator_lift_result")]
	pub use rx_core_operator_lift_result::operator::*;
	#[cfg(feature = "operator_map")]
	pub use rx_core_operator_map::operator::*;
	#[cfg(feature = "operator_map_into")]
	pub use rx_core_operator_map_into::operator::*;
	#[cfg(feature = "operator_merge_all")]
	pub use rx_core_operator_merge_all::operator::*;
	#[cfg(feature = "operator_merge_map")]
	pub use rx_core_operator_merge_map::operator::*;
	#[cfg(feature = "operator_reduce")]
	pub use rx_core_operator_reduce::operator::*;
	#[cfg(feature = "operator_scan")]
	pub use rx_core_operator_scan::operator::*;
	#[cfg(feature = "operator_skip")]
	pub use rx_core_operator_skip::operator::*;
	#[cfg(feature = "operator_switch_all")]
	pub use rx_core_operator_switch_all::operator::*;
	#[cfg(feature = "operator_switch_map")]
	pub use rx_core_operator_switch_map::operator::*;
	#[cfg(feature = "operator_take")]
	pub use rx_core_operator_take::operator::*;
	#[cfg(feature = "operator_tap_next")]
	pub use rx_core_operator_tap_next::operator::*;
}

#[cfg(feature = "compose")]
pub mod extension_compose {
	#[cfg(feature = "operator_adsr")]
	pub use rx_core_operator_adsr::extension_compose::*;
	#[cfg(feature = "operator_composite")]
	pub use rx_core_operator_composite::extension_compose::*;
	#[cfg(feature = "operator_enumerate")]
	pub use rx_core_operator_enumerate::extension_compose::*;
	#[cfg(feature = "operator_error_boundary")]
	pub use rx_core_operator_error_boundary::extension_compose::*;
	#[cfg(feature = "operator_fallback_when_silent")]
	pub use rx_core_operator_fallback_when_silent::extension_compose::*;
	#[cfg(feature = "operator_filter")]
	pub use rx_core_operator_filter::extension_compose::*;
	#[cfg(feature = "operator_filter_map")]
	pub use rx_core_operator_filter_map::extension_compose::*;
	#[cfg(feature = "operator_finalize")]
	pub use rx_core_operator_finalize::extension_compose::*;
	#[cfg(feature = "operator_into_result")]
	pub use rx_core_operator_into_result::extension_compose::*;
	#[cfg(feature = "operator_lift_option")]
	pub use rx_core_operator_lift_option::extension_compose::*;
	#[cfg(feature = "operator_lift_result")]
	pub use rx_core_operator_lift_result::extension_compose::*;
	#[cfg(feature = "operator_map")]
	pub use rx_core_operator_map::extension_compose::*;
	#[cfg(feature = "operator_map_into")]
	pub use rx_core_operator_map_into::extension_compose::*;
	#[cfg(feature = "operator_merge_all")]
	pub use rx_core_operator_merge_all::extension_compose::*;
	#[cfg(feature = "operator_merge_map")]
	pub use rx_core_operator_merge_map::extension_compose::*;
	#[cfg(feature = "operator_reduce")]
	pub use rx_core_operator_reduce::extension_compose::*;
	#[cfg(feature = "operator_scan")]
	pub use rx_core_operator_scan::extension_compose::*;
	#[cfg(feature = "operator_skip")]
	pub use rx_core_operator_skip::extension_compose::*;
	#[cfg(feature = "operator_switch_all")]
	pub use rx_core_operator_switch_all::extension_compose::*;
	#[cfg(feature = "operator_switch_map")]
	pub use rx_core_operator_switch_map::extension_compose::*;
	#[cfg(feature = "operator_take")]
	pub use rx_core_operator_take::extension_compose::*;
	#[cfg(feature = "operator_tap_next")]
	pub use rx_core_operator_tap_next::extension_compose::*;
}

#[cfg(feature = "pipe")]
pub mod extension_pipe {
	#[cfg(feature = "observable_pipe")]
	pub use rx_core_observable_pipe::extension_pipe::*;
	#[cfg(feature = "operator_adsr")]
	pub use rx_core_operator_adsr::extension_pipe::*;
	#[cfg(feature = "operator_enumerate")]
	pub use rx_core_operator_enumerate::extension_pipe::*;
	#[cfg(feature = "operator_error_boundary")]
	pub use rx_core_operator_error_boundary::extension_pipe::*;
	#[cfg(feature = "operator_fallback_when_silent")]
	pub use rx_core_operator_fallback_when_silent::extension_pipe::*;
	#[cfg(feature = "operator_filter")]
	pub use rx_core_operator_filter::extension_pipe::*;
	#[cfg(feature = "operator_filter_map")]
	pub use rx_core_operator_filter_map::extension_pipe::*;
	#[cfg(feature = "operator_finalize")]
	pub use rx_core_operator_finalize::extension_pipe::*;
	#[cfg(feature = "operator_into_result")]
	pub use rx_core_operator_into_result::extension_pipe::*;
	#[cfg(feature = "operator_lift_option")]
	pub use rx_core_operator_lift_option::extension_pipe::*;
	#[cfg(feature = "operator_lift_result")]
	pub use rx_core_operator_lift_result::extension_pipe::*;
	#[cfg(feature = "operator_map")]
	pub use rx_core_operator_map::extension_pipe::*;
	#[cfg(feature = "operator_map_into")]
	pub use rx_core_operator_map_into::extension_pipe::*;
	#[cfg(feature = "operator_merge_all")]
	pub use rx_core_operator_merge_all::extension_pipe::*;
	#[cfg(feature = "operator_merge_map")]
	pub use rx_core_operator_merge_map::extension_pipe::*;
	#[cfg(feature = "operator_reduce")]
	pub use rx_core_operator_reduce::extension_pipe::*;
	#[cfg(feature = "operator_scan")]
	pub use rx_core_operator_scan::extension_pipe::*;
	#[cfg(feature = "operator_skip")]
	pub use rx_core_operator_skip::extension_pipe::*;
	#[cfg(feature = "operator_switch_all")]
	pub use rx_core_operator_switch_all::extension_pipe::*;
	#[cfg(feature = "operator_switch_map")]
	pub use rx_core_operator_switch_map::extension_pipe::*;
	#[cfg(feature = "operator_take")]
	pub use rx_core_operator_take::extension_pipe::*;
	#[cfg(feature = "operator_tap_next")]
	pub use rx_core_operator_tap_next::extension_pipe::*;
}

pub mod observer {
	#[cfg(feature = "observer_fn")]
	pub use rx_core_observer_fn::observer::*;
	#[cfg(feature = "observer_noop")]
	pub use rx_core_observer_noop::observer::*;
	#[cfg(feature = "observer_print")]
	pub use rx_core_observer_print::observer::*;
}

pub mod subject {
	#[cfg(feature = "subject")]
	pub use rx_core_subject::subject::*;
	#[cfg(feature = "subject_async")]
	pub use rx_core_subject_async::subject::*;
	#[cfg(feature = "subject_behavior")]
	pub use rx_core_subject_behavior::subject::*;
	#[cfg(feature = "subject_replay")]
	pub use rx_core_subject_replay::subject::*;
}

pub mod prelude {
	pub use rx_core_traits::*;

	pub use super::observable::*;
	pub use super::observer::*;
	pub use super::operator::*;
	pub use super::subject::*;

	#[cfg(feature = "observable_fn")]
	pub use super::observable_fn::*;

	#[cfg(feature = "compose")]
	pub use super::extension_compose::*;

	#[cfg(feature = "pipe")]
	pub use super::extension_pipe::*;
}
