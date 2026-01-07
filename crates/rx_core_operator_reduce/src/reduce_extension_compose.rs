use rx_core_common::{ComposableOperator, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::ReduceOperator;

pub trait OperatorComposeExtensionReduce: ComposableOperator + Sized {
	#[inline]
	fn reduce<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> CompositeOperator<Self, ReduceOperator<Self::Out, Self::OutError, Reducer, NextOut>> {
		self.compose_with(ReduceOperator::new(reducer, seed))
	}
}

impl<Op> OperatorComposeExtensionReduce for Op where Op: ComposableOperator {}
