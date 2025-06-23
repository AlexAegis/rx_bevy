use rx_bevy_observable::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::FinalizeOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFinalize: Operator + Sized {
	fn finalize<Callback: 'static + Clone + FnOnce()>(
		self,
		callback: Callback,
	) -> CompositeOperator<Self, FinalizeOperator<Self::Out, Self::OutError, Callback>> {
		CompositeOperator::new(self, FinalizeOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionFinalize for T where T: Operator {}
