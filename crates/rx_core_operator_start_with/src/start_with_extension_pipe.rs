use rx_core_traits::{Observable, Operator};

use crate::operator::StartWithOperator;

pub trait ObservablePipeExtensionStartWith: Observable + Sized {
	#[inline]
	fn start_with<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> <StartWithOperator<OnSubscribe, Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		OnSubscribe: 'static + FnMut() -> Self::Out + Send + Sync,
	{
		StartWithOperator::new(on_subscribe).operate(self)
	}
}

impl<O> ObservablePipeExtensionStartWith for O where O: Observable {}
