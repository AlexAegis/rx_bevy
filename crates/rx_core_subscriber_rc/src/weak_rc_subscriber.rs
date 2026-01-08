use derive_where::derive_where;
use rx_core_common::{RxObserver, SharedSubscriber, Subscriber, SubscriptionLike};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::InnerRcSubscriber;

/// Acquired by calling `downgrade` on `RcSubscriber`
#[derive_where(Clone)]
#[derive(RxSubscriber)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
pub struct WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[destination]
	pub(crate) shared_destination: SharedSubscriber<InnerRcSubscriber<Destination>>,
}

impl<Destination> RxObserver for WeakRcSubscriber<Destination>
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
