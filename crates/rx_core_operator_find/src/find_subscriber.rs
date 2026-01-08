use derive_where::derive_where;

use rx_core_common::{RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::operator::FindOperatorError;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct FindSubscriber<InError, P, Destination>
where
	InError: Signal,
	P: Fn(&Destination::In) -> bool,
	Destination: Subscriber<InError = FindOperatorError<InError>>,
{
	#[destination]
	destination: Destination,
	predicate: P,
	match_observed: bool,
	next_observed: bool,
}

impl<InError, P, Destination> FindSubscriber<InError, P, Destination>
where
	InError: Signal,
	P: Fn(&Destination::In) -> bool,
	Destination: Subscriber<InError = FindOperatorError<InError>>,
{
	pub fn new(destination: Destination, predicate: P) -> Self {
		Self {
			destination,
			predicate,
			match_observed: false,
			next_observed: false,
		}
	}
}

impl<InError, P, Destination> RxObserver for FindSubscriber<InError, P, Destination>
where
	InError: Signal,
	P: Fn(&Destination::In) -> bool,
	Destination: Subscriber<InError = FindOperatorError<InError>>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.next_observed = true;
		if !self.match_observed && (self.predicate)(&next) {
			self.match_observed = true;
			self.destination.next(next);
			self.destination.complete();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(FindOperatorError::Upstream(error));
	}

	#[inline]
	fn complete(&mut self) {
		if !self.next_observed {
			self.destination
				.error(FindOperatorError::NoNextObservedBeforeComplete);
		} else if !self.match_observed {
			self.destination.error(FindOperatorError::NoMatchObserved);
		}
	}
}
