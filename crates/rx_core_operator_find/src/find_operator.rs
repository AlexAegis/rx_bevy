use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Signal, Subscriber};

use crate::{FindOperatorError, FindSubscriber};

#[derive_where(Debug, Clone)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(FindOperatorError<InError>)]
pub struct FindOperator<In, InError, P>
where
	P: 'static + Fn(&In) -> bool + Clone + Send + Sync,
	In: Signal,
	InError: Signal,
{
	predicate: P,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, P> FindOperator<In, InError, P>
where
	P: 'static + Fn(&In) -> bool + Clone + Send + Sync,
	In: Signal,
	InError: Signal,
{
	pub fn new(predicate: P) -> Self {
		Self {
			predicate,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, P> ComposableOperator for FindOperator<In, InError, P>
where
	P: 'static + Fn(&In) -> bool + Clone + Send + Sync,
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= FindSubscriber<InError, P, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		FindSubscriber::new(destination, self.predicate.clone())
	}
}
