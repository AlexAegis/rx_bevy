use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ReduceOperator;

pub trait ObservablePipeExtensionReduce<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn reduce<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> <ReduceOperator<Self::Out, Self::OutError, Reducer, NextOut> as Operator<'o>>::OutObservable<
		Self,
	>{
		ReduceOperator::new(reducer, seed).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionReduce<'o> for O where O: 'o + Observable + Send + Sync {}
