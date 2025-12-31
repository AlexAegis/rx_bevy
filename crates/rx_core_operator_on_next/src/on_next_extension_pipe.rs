use rx_core_traits::{Observable, Operator, Subscriber};

use crate::operator::OnNextOperator;

pub trait ObservablePipeExtensionOnNext: Observable + Sized {
	#[inline]
	fn on_next<OnNext>(
		self,
		on_next: OnNext,
	) -> <OnNextOperator<OnNext, Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		OnNext: 'static
			+ FnMut(&Self::Out, &mut dyn Subscriber<In = Self::Out, InError = Self::OutError>)
			+ Send
			+ Sync
			+ Clone,
	{
		OnNextOperator::new(on_next).operate(self)
	}
}

impl<O> ObservablePipeExtensionOnNext for O where O: Observable {}
