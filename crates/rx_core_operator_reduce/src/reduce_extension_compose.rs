use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::ReduceOperator;

pub trait OperatorComposeExtensionReduce: Operator + Sized {
	fn reduce<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> CompositeOperator<
		Self,
		ReduceOperator<Self::Out, Self::OutError, Reducer, NextOut, Self::Context>,
	> {
		CompositeOperator::new(self, ReduceOperator::new(reducer, seed))
	}
}

impl<Op> OperatorComposeExtensionReduce for Op where Op: Operator {}
