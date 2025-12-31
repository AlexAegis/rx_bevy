use rx_core_traits::{Observable, Operator};

use crate::operator::TakeOperator;

pub trait ObservablePipeExtensionTake<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn take(
		self,
		count: usize,
	) -> <TakeOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		TakeOperator::new(count).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionTake<'o> for O where O: 'o + Observable + Send + Sync {}
