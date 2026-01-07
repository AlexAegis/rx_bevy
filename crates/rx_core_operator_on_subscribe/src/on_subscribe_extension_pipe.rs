use rx_core_common::{Observable, Operator, Subscriber};

use crate::operator::OnSubscribeOperator;

pub trait ObservablePipeExtensionOnSubscribe<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn on_subscribe<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> <OnSubscribeOperator<OnSubscribe, Self::Out, Self::OutError> as Operator<'o>>::OutObservable<
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

impl<'o, O> ObservablePipeExtensionOnSubscribe<'o> for O where O: 'o + Observable + Send + Sync {}
