use core::{marker::PhantomData, num::NonZero};

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Observable, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_higher_order_concurrent::ConcurrentSubscriberProvider;
use rx_core_subscriber_higher_order_map::HigherOrderMapSubscriber;

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
pub struct MergeMapOperator<In, InError, Mapper, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable::OutError + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	mapper: Mapper,
	error_mapper: ErrorMapper,
	concurrency_limit: NonZero<usize>,
	_phantom_data: PhantomInvariant<(In, InError, InnerObservable)>,
}

impl<In, InError, Mapper, ErrorMapper, InnerObservable>
	MergeMapOperator<In, InError, Mapper, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable::OutError + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	pub fn new(mapper: Mapper, error_mapper: ErrorMapper, concurrency_limit: usize) -> Self {
		Self {
			mapper,
			error_mapper,
			concurrency_limit: NonZero::new(concurrency_limit).unwrap_or(NonZero::<usize>::MIN),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, ErrorMapper, InnerObservable> ComposableOperator
	for MergeMapOperator<In, InError, Mapper, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable::OutError + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	type Subscriber<Destination>
		= HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		ConcurrentSubscriberProvider,
		ErrorMapper,
		Destination,
	>
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
		HigherOrderMapSubscriber::new(
			destination,
			self.mapper.clone(),
			self.error_mapper.clone(),
			self.concurrency_limit,
		)
	}
}
