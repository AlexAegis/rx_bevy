use std::marker::PhantomData;

use rx_core_common::{Observable, Operator, PhantomInvariant, Signal};
use rx_core_macro_operator_derive::RxOperator;

use crate::observable::RetryObservable;

#[derive(RxOperator, Clone, Default)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct RetryOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	max_retries: usize,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> RetryOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(max_retries: usize) -> Self {
		Self {
			max_retries,
			_phantom_data: PhantomData,
		}
	}
}

impl<'o, In, InError> Operator<'o> for RetryOperator<In, InError>
where
	In: Signal,
	InError: Signal,
	'o: 'static,
{
	type OutObservable<InObservable>
		= RetryObservable<'o, InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;

	#[inline]
	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync,
	{
		RetryObservable::new(source, self.max_retries)
	}
}
