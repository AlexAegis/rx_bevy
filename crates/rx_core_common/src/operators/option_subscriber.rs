use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{Observer, Subscriber, SubscriptionLike, Teardown, TeardownCollection};

#[derive(RxSubscriber)]
#[_rx_core_common_crate(crate)]
#[rx_in(InnerSubscriber::In)]
#[rx_in_error(InnerSubscriber::InError)]
pub enum OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<In = InnerSubscriber::In, InError = InnerSubscriber::InError>,
{
	Some(InnerSubscriber),
	None(Destination),
}

impl<InnerSubscriber, Destination> Observer for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<In = InnerSubscriber::In, InError = InnerSubscriber::InError>,
	InnerSubscriber::In: 'static,
	InnerSubscriber::InError: 'static,
{
	fn next(&mut self, next: Self::In) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.next(next),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.next(next),
		}
	}

	fn error(&mut self, error: Self::InError) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.error(error),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.error(error),
		}
	}

	fn complete(&mut self) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.complete(),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.complete(),
		}
	}
}

impl<InnerSubscriber, Destination> SubscriptionLike
	for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<In = InnerSubscriber::In, InError = InnerSubscriber::InError>,
	InnerSubscriber::In: 'static,
	InnerSubscriber::InError: 'static,
{
	fn is_closed(&self) -> bool {
		match self {
			OptionSubscriber::Some(internal_subscriber) => internal_subscriber.is_closed(),
			OptionSubscriber::None(fallback_subscriber) => fallback_subscriber.is_closed(),
		}
	}

	fn unsubscribe(&mut self) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => {
				internal_subscriber.unsubscribe();
			}
			OptionSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.unsubscribe();
			}
		}
	}
}

impl<InnerSubscriber, Destination> TeardownCollection
	for OptionSubscriber<InnerSubscriber, Destination>
where
	InnerSubscriber: Subscriber,
	Destination: Subscriber<In = InnerSubscriber::In, InError = InnerSubscriber::InError>,
	InnerSubscriber::In: 'static,
	InnerSubscriber::InError: 'static,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		match self {
			OptionSubscriber::Some(internal_subscriber) => {
				internal_subscriber.add_teardown(teardown);
			}
			OptionSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.add_teardown(teardown);
			}
		}
	}
}
