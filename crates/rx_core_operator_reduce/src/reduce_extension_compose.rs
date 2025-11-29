use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::ReduceOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionReduce: Operator + Sized {
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

impl<T> CompositeOperatorExtensionReduce for T where T: Operator {}
