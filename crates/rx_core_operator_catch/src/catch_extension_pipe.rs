use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::CatchOperator;

pub trait ObservablePipeExtensionCatch<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn catch<
		NextInnerObservable: Observable<Out = Self::Out> + Signal,
		OnError: 'static + FnOnce(Self::OutError) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		on_error: OnError,
	) -> <CatchOperator<Self::Out, Self::OutError, OnError, NextInnerObservable> as Operator<
		'o,
	>>::OutObservable<Self>{
		CatchOperator::new(on_error).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionCatch<'o> for O where O: 'o + Observable + Send + Sync {}
