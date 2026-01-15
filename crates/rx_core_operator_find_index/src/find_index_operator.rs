use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::{FindIndexSubscriber, operator::FindIndexOperatorError};

#[derive_where(Debug, Clone)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(usize)]
#[rx_out_error(FindIndexOperatorError<InError>)]
pub struct FindIndexOperator<In, InError, P>
where
	P: 'static + Fn(&In) -> bool + Clone + Send + Sync,
	In: Signal,
	InError: Signal,
{
	predicate: P,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, P> FindIndexOperator<In, InError, P>
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

impl<In, InError, P> ComposableOperator for FindIndexOperator<In, InError, P>
where
	P: 'static + Fn(&In) -> bool + Clone + Send + Sync,
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= FindIndexSubscriber<In, InError, P, Destination>
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
		FindIndexSubscriber::new(destination, self.predicate.clone())
	}
}
