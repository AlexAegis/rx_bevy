use rx_core_common::{Observable, Operator, Signal};

use crate::operator::ScanOperator;

pub trait ObservablePipeExtensionScan<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn scan<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> <ScanOperator<Self::Out, Self::OutError, Reducer, NextOut> as Operator<'o>>::OutObservable<
		Self,
	> {
		ScanOperator::new(reducer, seed).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionScan<'o> for O where O: 'o + Observable + Send + Sync {}
