use core::num::NonZero;

use rx_core_traits::{Observable, Signal, Subscriber};

pub trait HigherOrderSubscriberFactory<Destination> {
	fn new_from_destination(destination: Destination, concurrency_limit: NonZero<usize>) -> Self;
}

pub trait HigherOrderSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>: Subscriber<In = InnerObservable, InError = InnerObservable::OutError>
		+ HigherOrderSubscriberFactory<Destination>
	where
		InnerObservable: Observable + Signal,
		Destination:
			'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>;
}
