use rx_core_common::{Observable, Operator, Subscriber};

use crate::operator::OnNextOperator;

pub trait ObservablePipeExtensionOnNext<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn on_next<OnNext>(
		self,
		on_next: OnNext,
	) -> <OnNextOperator<OnNext, Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		OnNext: 'static
			+ FnMut(&Self::Out, &mut dyn Subscriber<In = Self::Out, InError = Self::OutError>) -> bool
			+ Send
			+ Sync
			+ Clone,
	{
		OnNextOperator::new(on_next).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionOnNext<'o> for O where O: 'o + Observable + Send + Sync {}
