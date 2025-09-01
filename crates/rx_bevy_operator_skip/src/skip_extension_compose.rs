use rx_bevy_core::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::SkipOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionSkip: Operator + Sized {
	fn skip(
		self,
		count: usize,
	) -> CompositeOperator<Self, SkipOperator<Self::Out, Self::OutError>> {
		CompositeOperator::new(self, SkipOperator::new(count))
	}
}

impl<T> CompositeOperatorExtensionSkip for T where T: Operator {}
