use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ScanOperator;

pub trait ObservablePipeExtensionScan: Observable + Sized {
	#[inline]
	fn scan<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> <ScanOperator<Self::Out, Self::OutError, Reducer, NextOut> as Operator>::OutObservable<Self>
	{
		ScanOperator::new(reducer, seed).operate(self)
	}
}

impl<O> ObservablePipeExtensionScan for O where O: Observable {}
