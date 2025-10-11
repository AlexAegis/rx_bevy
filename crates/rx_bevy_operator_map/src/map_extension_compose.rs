use rx_bevy_core::{Operator, SignalBound};
use rx_bevy_operator_composite::CompositeOperator;

use crate::MapOperator;

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
