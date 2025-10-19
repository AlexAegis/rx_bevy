use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, SignalBound};

use crate::operator::MapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMap: Operator + Sized {
	fn map<
		NextOut: SignalBound,
		Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		MapOperator<Self::Out, Self::OutError, Mapper, NextOut, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, MapOperator::new(mapper))
	}
}

impl<T> CompositeOperatorExtensionMap for T where T: Operator {}
