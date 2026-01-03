use core::{marker::PhantomData, num::NonZero};

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_subscriber_higher_order_concurrent::ConcurrentSubscriberProvider;
use rx_core_traits::{ComposableOperator, Observable, Signal, Subscriber};

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In::Out)]
#[rx_out_error(In::OutError)]
pub struct MergeAllOperator<In, InError, ErrorMapper>
where
	In: Observable + Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Clone + Send + Sync,
{
	concurrency_limit: NonZero<usize>,
	error_mapper: ErrorMapper,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, ErrorMapper> MergeAllOperator<In, InError, ErrorMapper>
where
	In: Observable + Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Clone + Send + Sync,
{
	pub fn new(concurrency_limit: usize, error_mapper: ErrorMapper) -> Self {
		Self {
			concurrency_limit: NonZero::new(concurrency_limit).unwrap_or(NonZero::<usize>::MIN),
			error_mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, ErrorMapper> ComposableOperator for MergeAllOperator<In, InError, ErrorMapper>
where
	In: Observable + Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Clone + Send + Sync,
{
	type Subscriber<Destination>
		= HigherOrderAllSubscriber<In, InError, ConcurrentSubscriberProvider, ErrorMapper, Destination>
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
		HigherOrderAllSubscriber::new(
			destination,
			self.error_mapper.clone(),
			self.concurrency_limit,
		)
	}
}
