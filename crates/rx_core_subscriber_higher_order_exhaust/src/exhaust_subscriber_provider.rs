use std::num::NonZero;

use rx_core_common::{Observable, Signal, Subscriber};
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};

use crate::ExhaustSubscriber;

pub struct ExhaustSubscriberProvider;

impl HigherOrderSubscriberProvider for ExhaustSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>
		= ExhaustSubscriber<InnerObservable, Destination>
	where
		InnerObservable: Observable + Signal,
		Destination:
			'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>;
}

impl<InnerObservable, Destination> HigherOrderSubscriberFactory<Destination>
	for ExhaustSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn new_from_destination(destination: Destination, _concurrency_limit: NonZero<usize>) -> Self {
		Self::new(destination)
	}
}
