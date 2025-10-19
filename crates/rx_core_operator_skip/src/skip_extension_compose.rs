use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::SkipOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionSkip: Operator + Sized {
	fn skip(
		self,
		count: usize,
	) -> CompositeOperator<Self, SkipOperator<Self::Out, Self::OutError, <Self as Operator>::Context>>
	{
		CompositeOperator::new(self, SkipOperator::new(count))
	}
}

impl<T> CompositeOperatorExtensionSkip for T where T: Operator {}
