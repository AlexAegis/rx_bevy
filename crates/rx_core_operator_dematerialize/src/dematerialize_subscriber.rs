use rx_core_common::{Never, Observer, ObserverNotification, Subscriber, SubscriptionLike};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(ObserverNotification<Destination::In, Destination::InError>)]
#[rx_in_error(Never)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct DematerializeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
}

impl<Destination> DematerializeSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> Observer for DematerializeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, notification: Self::In) {
		match notification {
			ObserverNotification::Next(next) => {
				if !self.is_closed() {
					self.destination.next(next)
				}
			}
			ObserverNotification::Error(error) => {
				if !self.is_closed() {
					self.destination.error(error);
				}
			}
			ObserverNotification::Complete => self.complete(),
		}
	}

	fn error(&mut self, _error: Self::InError) {
		unreachable!("The input error type is locked to never!")
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
		}
	}
}
