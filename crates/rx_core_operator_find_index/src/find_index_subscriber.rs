use std::marker::PhantomData;

use derive_where::derive_where;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FindIndexOperatorError<InError>
where
	InError: Signal,
{
	#[error("FindIndexOperatorError::NoNextObservedBeforeComplete")]
	NoNextObservedBeforeComplete,
	#[error("FindIndexOperatorError::NoMatchObserved")]
	NoMatchObserved,
	#[error(transparent)]
	Upstream(InError),
}

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct FindIndexSubscriber<In, InError, P, Destination>
where
	In: Signal,
	InError: Signal,
	P: Fn(&In) -> bool,
	Destination: Subscriber<In = usize, InError = FindIndexOperatorError<InError>>,
{
	#[destination]
	destination: Destination,
	predicate: P,
	match_observed: bool,
	nexts_observed: usize,
	_phantom_data: PhantomData<In>,
}

impl<In, InError, P, Destination> FindIndexSubscriber<In, InError, P, Destination>
where
	In: Signal,
	InError: Signal,
	P: Fn(&In) -> bool,
	Destination: Subscriber<In = usize, InError = FindIndexOperatorError<InError>>,
{
	pub fn new(destination: Destination, predicate: P) -> Self {
		Self {
			destination,
			predicate,
			match_observed: false,
			nexts_observed: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, P, Destination> Observer for FindIndexSubscriber<In, InError, P, Destination>
where
	In: Signal,
	InError: Signal,
	P: Fn(&In) -> bool,
	Destination: Subscriber<In = usize, InError = FindIndexOperatorError<InError>>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		// Since indexing starts from 0, the result is pre-increment, but
		// the completetion error expects to know the already incremented number
		let nexts_observed_so_far = self.nexts_observed;
		self.nexts_observed += 1;
		if !self.match_observed && (self.predicate)(&next) {
			self.match_observed = true;
			self.destination.next(nexts_observed_so_far);
			self.destination.complete();
			self.destination.unsubscribe();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination
			.error(FindIndexOperatorError::Upstream(error));
	}

	#[inline]
	fn complete(&mut self) {
		if self.nexts_observed == 0 {
			self.destination
				.error(FindIndexOperatorError::NoNextObservedBeforeComplete);
		} else if !self.match_observed {
			self.destination
				.error(FindIndexOperatorError::NoMatchObserved);
		}
	}
}
