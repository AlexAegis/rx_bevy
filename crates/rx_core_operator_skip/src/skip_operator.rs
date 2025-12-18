use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Never, Signal, Subscriber};

use crate::SkipSubscriber;

/// The [SkipOperator] is used to skip the first `n` emissions of an observable,
/// after which it does nothing.
#[derive(RxOperator)]
#[derive_where(Debug, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct SkipOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	count: usize,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> SkipOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ComposableOperator for SkipOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= SkipSubscriber<Destination>
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
		SkipSubscriber::new(destination, self.count)
	}
}
