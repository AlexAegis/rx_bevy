use rx_core_common::{Never, Observer, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber, Debug)]
#[rx_in(Destination::In)]
#[rx_in_error(Never)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[destination]
	destination: Destination,
}

impl<Destination> ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[inline]
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> Observer for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	fn error(&mut self, _error: Self::InError) {
		unreachable!("InError is Never")
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
