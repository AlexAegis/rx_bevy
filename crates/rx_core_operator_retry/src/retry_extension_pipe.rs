use rx_core_traits::{Observable, Operator};

use crate::operator::RetryOperator;

pub trait ObservablePipeExtensionRetry<'o>: 'o + Observable + Sized + Send + Sync
where
	'o: 'static,
{
	#[inline]
	fn retry(
		self,
		max_retries: usize,
	) -> <RetryOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		RetryOperator::<Self::Out, Self::OutError>::new(max_retries).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionRetry<'o> for O
where
	O: 'o + Observable + Send + Sync,
	'o: 'static,
{
}
