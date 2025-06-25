use rx_bevy_observable::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::TryCaptureOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionTryCapture: Operator + Sized {
	fn lift_result(self) -> CompositeOperator<Self, TryCaptureOperator<Self::Out, Self::OutError>> {
		CompositeOperator::new(self, TryCaptureOperator::default())
	}
}

impl<Op> CompositeOperatorExtensionTryCapture for Op where Op: Operator {}
