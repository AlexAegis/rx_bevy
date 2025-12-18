use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::{observable::ShareObservable, operator::ShareOptions};

#[derive_where(Clone, Default)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct ShareOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	options: ShareOptions<In, InError>,
	_phantom_data: PhantomData<fn(In, InError) -> (In, InError)>,
}

impl<In, InError> ShareOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn new(options: ShareOptions<In, InError>) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for ShareOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type OutObservable<InObservable>
		= ShareObservable<InObservable>
	where
		InObservable: Observable<Out = Self::In, OutError = Self::InError>;

	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: Observable<Out = Self::In, OutError = Self::InError>,
	{
		ShareObservable::new(source, self.options)
	}
}
