use disqualified::ShortName;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Never, Observer, Subscriber};

#[derive(RxSubscriber, Debug)]
#[rx_in(Destination::In)]
#[rx_in_error(Never)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
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

	#[inline]
	fn error(&mut self, _error: Self::InError) {
		// The operator only compiles if the upstream error is of type `Never`,
		// which is impossible to construct as it's an enum with no variants.

		// It'd be an interesting miracle if this panic was ever triggered.
		unreachable!(
			"An error was observed by {}, but it shouldn't have as `Never` is not a constructable type.",
			ShortName::of::<Self>()
		)
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
