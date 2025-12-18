use rx_core_operator_composite::{
	extension_compose::OperatorComposeExtension, operator::CompositeOperator,
};
use rx_core_traits::{ComposableOperator, Signal};

use crate::operator::MapOperator;

pub trait OperatorComposeExtensionMap: ComposableOperator + Sized {
	#[inline]
	fn map<NextOut: Signal, Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut>> {
		self.compose_with(MapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionMap for Op where Op: ComposableOperator {}
