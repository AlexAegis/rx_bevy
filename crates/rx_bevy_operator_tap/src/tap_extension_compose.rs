use rx_bevy_observable::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::TapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionTapNext: Operator + Sized {
	fn tap_next<Callback: 'static + Clone + for<'a> Fn(&'a Self::Out)>(
		self,
		callback: Callback,
	) -> CompositeOperator<Self, TapOperator<Self::Out, Self::OutError, Callback>> {
		CompositeOperator::new(self, TapOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionTapNext for T where T: Operator {}
