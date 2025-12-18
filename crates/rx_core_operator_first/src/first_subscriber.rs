use derive_where::derive_where;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FirstOperatorError<InError>
where
	InError: Signal,
{
	#[error("FirstOperatorError::NoNextObservedBeforeComplete")]
	NoNextObservedBeforeComplete,
	#[error(transparent)]
	Upstream(InError),
}

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct FirstSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<InError = FirstOperatorError<InError>>,
{
	#[destination]
	destination: Destination,
	first_observed: bool,
}

impl<InError, Destination> FirstSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<InError = FirstOperatorError<InError>>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			first_observed: false,
		}
	}
}

impl<InError, Destination> Observer for FirstSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<InError = FirstOperatorError<InError>>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.first_observed {
			self.first_observed = true;
			self.destination.next(next);
			self.destination.complete();
			self.destination.unsubscribe();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(FirstOperatorError::Upstream(error));
	}

	#[inline]
	fn complete(&mut self) {
		if !self.first_observed {
			self.destination
				.error(FirstOperatorError::NoNextObservedBeforeComplete);
		}
	}
}
