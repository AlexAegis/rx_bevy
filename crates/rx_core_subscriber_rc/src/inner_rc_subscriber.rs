use std::ops::{Deref, DerefMut};

use disqualified::ShortName;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber, SubscriptionClosedFlag, SubscriptionLike};

/// Internal to [RcSubscriber]
#[doc(hidden)]
#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	/// Starts from 1
	pub(crate) ref_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	pub(crate) completion_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	pub(crate) unsubscribe_count: usize,

	closed_flag: SubscriptionClosedFlag,
	completed_flag: SubscriptionClosedFlag,
}

impl<Destination> InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		let is_already_closed = destination.is_closed();
		Self {
			destination,
			ref_count: 1,
			completion_count: is_already_closed.into(),
			unsubscribe_count: is_already_closed.into(),
			closed_flag: is_already_closed.into(),
			completed_flag: is_already_closed.into(),
		}
	}

	pub fn unsubscribe_if_can(&mut self) {
		if self.unsubscribe_count == self.ref_count && !self.closed_flag.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe();
		}
	}

	pub fn complete_if_can(&mut self) {
		if self.completion_count == self.ref_count
			&& !self.closed_flag.is_closed()
			&& !self.completed_flag.is_closed()
		{
			self.destination.complete();
			self.completed_flag.close();
		}
	}
}

impl<Destination> Deref for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Target = Destination;

	fn deref(&self) -> &Self::Target {
		&self.destination
	}
}

impl<Destination> DerefMut for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.destination
	}
}

impl<Destination> Observer for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.closed_flag.is_closed() {
			self.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.closed_flag.is_closed() {
			self.destination.error(error);
			// An error immediately unsubscribes.
			self.closed_flag.close();
			self.ref_count = 0;
			self.completion_count = 0;
			self.unsubscribe_count = 0;
			self.destination.unsubscribe();
		}
	}

	fn complete(&mut self) {
		self.complete_if_can();
	}
}

impl<Destination> SubscriptionLike for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag || self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.unsubscribe_if_can();
	}
}

impl<Destination> Drop for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	/// This should only happen when all counters reach 0.
	fn drop(&mut self) {
		self.completed_flag.close();

		if !self.is_closed() {
			self.destination.unsubscribe();
		}

		debug_assert_eq!(
			self.unsubscribe_count,
			0,
			"The unsubscribe_count did not reach 0 of {} on drop.",
			ShortName::of::<Self>()
		);

		debug_assert_eq!(
			self.ref_count,
			0,
			"The ref_count did not reach 0 of {} on drop.",
			ShortName::of::<Self>()
		);

		debug_assert_eq!(
			self.completion_count,
			0,
			"The completion_count did not reach 0 of {} on drop.",
			ShortName::of::<Self>()
		);
	}
}
