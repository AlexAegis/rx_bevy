use rx_bevy_core::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::TakeOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionTake: Operator + Sized {
	fn take(
		self,
		count: usize,
	) -> CompositeOperator<Self, TakeOperator<Self::Out, Self::OutError, <Self as Operator>::Context>>
	{
		CompositeOperator::new(self, TakeOperator::new(count))
	}
}

impl<T> CompositeOperatorExtensionTake for T where T: Operator {}
