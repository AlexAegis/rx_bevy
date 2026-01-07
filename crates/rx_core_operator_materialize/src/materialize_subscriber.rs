use rx_core_common::{Never, Observer, ObserverNotification, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct MaterializeSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = ObserverNotification<In, InError>, InError = Never>,
{
	#[destination]
	destination: Destination,
}

impl<In, InError, Destination> MaterializeSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = ObserverNotification<In, InError>, InError = Never>,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<In, InError, Destination> Observer for MaterializeSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = ObserverNotification<In, InError>, InError = Never>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(ObserverNotification::Next(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.next(ObserverNotification::Error(error));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(ObserverNotification::Complete);
	}
}
