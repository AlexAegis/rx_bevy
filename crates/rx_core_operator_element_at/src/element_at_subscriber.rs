use std::marker::PhantomData;
use std::sync::Arc;

use derive_where::derive_where;

use rx_core_common::{
	PhantomInvariant, Provider, RxObserver, Signal, Subscriber, SubscriptionClosedFlag,
	SubscriptionLike,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::operator::ElementAtOperatorError;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct ElementAtSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = In, InError = ElementAtOperatorError<InError>>,
{
	#[destination]
	destination: Destination,
	index: usize,
	default_value: Option<Arc<dyn Provider<Provided = In> + Send + Sync>>,
	nexts_observed: usize,
	/// Closedness is tracked in case downstream doesn't immediately reflect it.
	closed: SubscriptionClosedFlag,
	_phantom_data: PhantomInvariant<In>,
}

impl<In, InError, Destination> ElementAtSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = In, InError = ElementAtOperatorError<InError>>,
{
	pub fn new(
		destination: Destination,
		index: usize,
		default_value: Option<Arc<dyn Provider<Provided = In> + Send + Sync>>,
	) -> Self {
		let closed: SubscriptionClosedFlag = destination.is_closed().into();
		Self {
			destination,
			index,
			default_value,
			nexts_observed: 0,
			closed,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> RxObserver for ElementAtSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = In, InError = ElementAtOperatorError<InError>>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if self.is_closed() {
			return;
		}

		if self.nexts_observed == self.index {
			self.destination.next(next);
			self.destination.complete();
			self.closed.close();
			return;
		}

		self.nexts_observed += 1;
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if self.is_closed() {
			return;
		}
		self.destination
			.error(ElementAtOperatorError::Upstream(error));
		self.closed.close();
	}

	#[inline]
	fn complete(&mut self) {
		if self.is_closed() {
			return;
		}
		if let Some(default_value) = self.default_value.as_ref() {
			self.destination.next(default_value.provide());
			self.destination.complete();
			self.closed.close();
		} else {
			self.destination
				.error(ElementAtOperatorError::IndexOutOfRange {
					requested_index: self.index,
					observed_nexts: self.nexts_observed,
				});
			self.closed.close();
		}
	}
}

impl<In, InError, Destination> SubscriptionLike for ElementAtSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = In, InError = ElementAtOperatorError<InError>>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed || self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.destination.unsubscribe();
		}
		self.closed.close();
	}
}

impl<In, InError, Destination> Drop for ElementAtSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = In, InError = ElementAtOperatorError<InError>>,
{
	#[inline]
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
