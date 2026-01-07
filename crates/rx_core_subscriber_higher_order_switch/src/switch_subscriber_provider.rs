use core::num::NonZero;

use rx_core_common::{Observable, Signal, Subscriber};
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};

use crate::SwitchSubscriber;

pub struct SwitchSubscriberProvider;

impl HigherOrderSubscriberProvider for SwitchSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>
		= SwitchSubscriber<InnerObservable, Destination>
	where
		InnerObservable: Observable + Signal,
		Destination:
			'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>;
}

impl<InnerObservable, Destination> HigherOrderSubscriberFactory<Destination>
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn new_from_destination(destination: Destination, _concurrency_limit: NonZero<usize>) -> Self {
		Self::new(destination)
	}
}
