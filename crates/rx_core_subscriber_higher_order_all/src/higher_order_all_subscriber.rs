use core::{marker::PhantomData, num::NonZero};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};
use rx_core_traits::{Observable, Observer, Signal, Subscriber};

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct HigherOrderAllSubscriber<In, InError, HigherOrderSubscriber, ErrorMapper, Destination>
where
	In: Signal + Observable,
	InError: Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Send + Sync,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	#[destination]
	destination: HigherOrderSubscriber::HigherOrderSubscriber<In, Destination>,
	error_mapper: Option<ErrorMapper>,
	_phantom_data: PhantomData<InError>,
}

impl<In, InError, HigherOrderSubscriber, ErrorMapper, Destination>
	HigherOrderAllSubscriber<In, InError, HigherOrderSubscriber, ErrorMapper, Destination>
where
	In: Signal + Observable,
	InError: Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Send + Sync,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	pub fn new(
		destination: Destination,
		error_mapper: ErrorMapper,
		concurrency_limit: NonZero<usize>,
	) -> Self {
		Self {
			destination: HigherOrderSubscriber::HigherOrderSubscriber::<In, Destination>::new_from_destination(
				destination,
				concurrency_limit
			),
			error_mapper: Some(error_mapper),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, HigherOrderSubscriber, ErrorMapper, Destination> Observer
	for HigherOrderAllSubscriber<In, InError, HigherOrderSubscriber, ErrorMapper, Destination>
where
	In: Signal + Observable,
	InError: Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Send + Sync,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
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
