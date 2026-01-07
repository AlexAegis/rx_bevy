use rx_core_common::{ComposableOperator, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::FilterMapOperator;

pub trait OperatorComposeExtensionFilterMap: ComposableOperator + Sized {
	#[inline]
	fn filter_map<
		NextOut: Signal,
		Mapper: 'static + Fn(Self::Out) -> Option<NextOut> + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<Self, FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut>> {
		self.compose_with(FilterMapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionFilterMap for Op where Op: ComposableOperator {}
