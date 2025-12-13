use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};
use rx_core_traits::{Observable, Signal, Subscriber};

use crate::ConcurrentSubscriber;

pub struct ConcurrentSubscriberProvider;

impl HigherOrderSubscriberProvider for ConcurrentSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>
		= ConcurrentSubscriber<InnerObservable, Destination>
	where
		InnerObservable:
			Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
		Destination: 'static + Subscriber;
}

impl<InnerObservable, Destination> HigherOrderSubscriberFactory<Destination>
	for ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn new_from_destination(destination: Destination, concurrency_limit: usize) -> Self {
		Self::new(destination, concurrency_limit)
	}
}
