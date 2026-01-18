use rx_core_common::{RxObserver, Subscriber, SubscriptionClosedFlag, SubscriptionLike};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	count: usize,
	/// Closedness is tracked in case downstream doesn't immediately reflect it.
	closed: SubscriptionClosedFlag,
}

impl<Destination> TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(mut destination: Destination, count: usize) -> Self {
		if count == 0 && !destination.is_closed() {
			destination.complete();
		}

		let closed: SubscriptionClosedFlag = destination.is_closed().into();

		Self {
			destination,
			count,
			closed,
		}
	}
}

impl<Destination> RxObserver for TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if self.count > 0 {
			self.count -= 1;
			self.destination.next(next);

			if self.count == 0 && !self.is_closed() {
				self.complete();
			}
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
		self.closed.close();
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
		self.closed.close();
	}
}

impl<Destination> SubscriptionLike for TakeSubscriber<Destination>
where
	Destination: Subscriber,
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

impl<Destination> Drop for TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
