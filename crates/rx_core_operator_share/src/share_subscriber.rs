use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber};

#[derive(RxSubscriber, Debug)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct ShareSubscriber<ShareDestination, Destination>
where
	ShareDestination: Observer<In = Destination::In, InError = Destination::InError>,
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	share_destination: ShareDestination,
}

impl<ShareDestination, Destination> ShareSubscriber<ShareDestination, Destination>
where
	ShareDestination: Observer<In = Destination::In, InError = Destination::InError>,
	Destination: Subscriber,
{
	pub fn new(destination: Destination, share_destination: ShareDestination) -> Self {
		Self {
			destination,
			share_destination,
		}
	}
}

impl<ShareDestination, Destination> Observer for ShareSubscriber<ShareDestination, Destination>
where
	ShareDestination: Observer<In = Destination::In, InError = Destination::InError>,
	Destination: Subscriber,
	Destination::In: Clone,
	Destination::InError: Clone,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.share_destination.next(next.clone());
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.share_destination.error(error.clone());
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.share_destination.complete();
		self.destination.complete();
	}
}
