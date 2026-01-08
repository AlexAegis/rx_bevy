use core::{marker::PhantomData, num::NonZero};

use rx_core_common::{Observable, RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct HigherOrderMapSubscriber<
	In,
	InError,
	Mapper,
	InnerObservable,
	HigherOrderSubscriber,
	ErrorMapper,
	Destination,
> where
	In: Signal,
	InError: Signal,
	Mapper: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable::OutError + Send + Sync,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[destination]
	destination: HigherOrderSubscriber::HigherOrderSubscriber<InnerObservable, Destination>,
	mapper: Mapper,
	error_mapper: Option<ErrorMapper>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Mapper, InnerObservable, HigherOrderSubscriber, ErrorMapper, Destination>
	HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		HigherOrderSubscriber,
		ErrorMapper,
		Destination,
	>
where
	In: Signal,
	InError: Signal,
	Mapper: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable::OutError + Send + Sync,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(
		destination: Destination,
		mapper: Mapper,
		error_mapper: ErrorMapper,
		concurrency_limit: NonZero<usize>,
	) -> Self {
		Self {
			destination: HigherOrderSubscriber::HigherOrderSubscriber::<
				InnerObservable,
				Destination,
			>::new_from_destination(destination, concurrency_limit),
			mapper,
			error_mapper: Some(error_mapper),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, InnerObservable, HigherOrderSubscriber, ErrorMapper, Destination>
	RxObserver
	for HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		HigherOrderSubscriber,
		ErrorMapper,
		Destination,
	>
where
	In: Signal,
	InError: Signal,
	Mapper: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable::OutError + Send + Sync,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next((self.mapper)(next));
	}

	/// For upstream errors
	#[inline]
	fn error(&mut self, error: Self::InError) {
		if let Some(error_mapper) = self.error_mapper.take() {
			self.destination.error((error_mapper)(error));
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
