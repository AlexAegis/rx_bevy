use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct EndWithSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	end_with: Option<Destination::In>,
}

impl<Destination> EndWithSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination, end_with: Destination::In) -> Self {
		Self {
			destination,
			end_with: Some(end_with),
		}
	}
}

impl<Destination> Observer for EndWithSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		if let Some(last) = self.end_with.take() {
			self.destination.next(last);
		}
		self.destination.complete();
	}
}
