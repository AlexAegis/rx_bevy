use rx_core_traits::{Observable, Operator, Subscriber};

use crate::operator::OnSubscribeOperator;

pub trait ObservablePipeExtensionOnSubscribe: Observable + Sized {
	#[inline]
	fn on_subscribe<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> <OnSubscribeOperator<OnSubscribe, Self::Out, Self::OutError> as Operator>::OutObservable<
		Self,
	>
	where
		OnSubscribe: 'static
			+ FnMut(&mut dyn Subscriber<In = Self::Out, InError = Self::OutError>)
			+ Send
			+ Sync,
	{
		OnSubscribeOperator::new(on_subscribe).operate(self)
	}
}

impl<O> ObservablePipeExtensionOnSubscribe for O where O: Observable {}
