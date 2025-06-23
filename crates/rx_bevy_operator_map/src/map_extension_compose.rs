use rx_bevy_observable::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::MapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMap: Operator + Sized {
	fn map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> NextOut>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut>> {
		CompositeOperator::new(self, MapOperator::new(mapper))
	}
}

impl<T> CompositeOperatorExtensionMap for T where T: Operator {}
