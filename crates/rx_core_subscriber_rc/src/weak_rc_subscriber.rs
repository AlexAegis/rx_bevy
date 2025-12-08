use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber, SubscriptionLike, Teardown, TeardownCollection};

use crate::InnerRcSubscriber;

/// Acquired by calling `downgrade` on `RcSubscriber`
#[derive(RxSubscriber)]
#[derive_where(Clone)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
pub struct WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) shared_destination: Arc<Mutex<InnerRcSubscriber<Destination>>>,
}

impl<Destination> Observer for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		self.shared_destination.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.shared_destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		self.shared_destination.complete();
	}
}

impl<Destination> SubscriptionLike for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.shared_destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.shared_destination.unsubscribe();
		}
	}
}

impl<Destination> TeardownCollection for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.shared_destination.add_teardown(teardown);
	}
}
