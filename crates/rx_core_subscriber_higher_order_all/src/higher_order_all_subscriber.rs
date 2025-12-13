use std::marker::PhantomData;

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
#[rx_delegate_teardown_collection_to_destination]
pub struct HigherOrderAllSubscriber<In, InError, HigherOrderSubscriber, Destination>
where
	In: Signal + Observable,
	InError: Signal + Into<In::OutError>,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	#[destination]
	destination: HigherOrderSubscriber::HigherOrderSubscriber<In, Destination>,
	_phantom_data: PhantomData<InError>,
}

impl<In, InError, HigherOrderSubscriber, Destination>
	HigherOrderAllSubscriber<In, InError, HigherOrderSubscriber, Destination>
where
	In: Signal + Observable,
	InError: Signal + Into<In::OutError>,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	pub fn new(destination: Destination, concurrency_limit: usize) -> Self {
		Self {
			destination:
				HigherOrderSubscriber::HigherOrderSubscriber::<In, Destination>::new_from_destination(
					destination,
					concurrency_limit.max(1)
				),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, HigherOrderSubscriber, Destination> Observer
	for HigherOrderAllSubscriber<In, InError, HigherOrderSubscriber, Destination>
where
	In: Signal + Observable,
	InError: Signal + Into<In::OutError>,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
