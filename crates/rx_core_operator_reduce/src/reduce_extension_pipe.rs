use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ReduceOperator;

pub trait ObservablePipeExtensionReduce: Observable + Sized {
	#[inline]
	fn reduce<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> <ReduceOperator<Self::Out, Self::OutError, Reducer, NextOut> as Operator>::OutObservable<
		Self,
	> {
		ReduceOperator::new(reducer, seed).operate(self)
	}
}

impl<O> ObservablePipeExtensionReduce for O where O: Observable {}
