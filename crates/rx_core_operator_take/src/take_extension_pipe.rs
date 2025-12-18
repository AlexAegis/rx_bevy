use rx_core_traits::{Observable, Operator};

use crate::operator::TakeOperator;

pub trait ObservablePipeExtensionTake: Observable + Sized {
	#[inline]
	fn take(
		self,
		count: usize,
	) -> <TakeOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self> {
		TakeOperator::new(count).operate(self)
	}
}

impl<O> ObservablePipeExtensionTake for O where O: Observable {}
