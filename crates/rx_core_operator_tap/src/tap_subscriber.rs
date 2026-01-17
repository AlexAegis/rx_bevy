use rx_core_common::{ObserverSubscriber, RxObserver, SharedSubscriber, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct TapSubscriber<TapDestination, Destination>
where
	TapDestination:
		'static + RxObserver<In = Destination::In, InError = Destination::InError> + Send + Sync,
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	tap_destination: SharedSubscriber<ObserverSubscriber<TapDestination>>,
}

impl<TapDestination, Destination> TapSubscriber<TapDestination, Destination>
where
	TapDestination:
		'static + RxObserver<In = Destination::In, InError = Destination::InError> + Send + Sync,
	Destination: Subscriber,
{
	pub fn new(
		destination: Destination,
		tap_destination: SharedSubscriber<ObserverSubscriber<TapDestination>>,
	) -> Self {
		Self {
			destination,
			tap_destination,
		}
	}
}

impl<TapDestination, Destination> RxObserver for TapSubscriber<TapDestination, Destination>
where
	TapDestination:
		'static + RxObserver<In = Destination::In, InError = Destination::InError> + Send + Sync,
	Destination: Subscriber,
	Destination::In: Clone,
	Destination::InError: Clone,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.tap_destination.next(next.clone());
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.tap_destination.error(error.clone());
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.tap_destination.complete();
		self.destination.complete();
	}
}
