use rx_bevy_core::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::FilterMapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFilterMap: Operator + Sized {
	fn filter_map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> Option<NextOut>>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, FilterMapOperator::new(mapper))
	}
}

impl<T> CompositeOperatorExtensionFilterMap for T where T: Operator {}
