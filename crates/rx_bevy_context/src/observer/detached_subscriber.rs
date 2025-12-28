use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, SubscriptionClosedFlag, SubscriptionData, SubscriptionLike, Teardown,
	TeardownCollection,
};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	#[destination]
	destination: Destination,
	closed_flag: SubscriptionClosedFlag,
	teardown: Option<SubscriptionData>,
}

impl<Destination> DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	pub(crate) fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed_flag: false.into(),
			teardown: None,
		}
	}
}

impl<Destination> Observer for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
			self.unsubscribe();
		}
	}
}

impl<Destination> SubscriptionLike for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed_flag.close();
			if let Some(mut teardown) = self.teardown.take() {
				teardown.unsubscribe();
			}
		}
	}
}

impl<Destination> TeardownCollection for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.teardown.get_or_insert_default().add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl<Destination> Drop for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	/// When you make a subscription in rx_bevy, the Subscribe event stores
	/// the destination you want to subscribe to, this way you're not limited
	/// to make only subscriptions that send events to another entity, you
	/// can use ad-hoc pipelines just for that subscription, etc.
	/// But that means that the simple destination has to be pre-upgraded to
	/// a subscriber, and if the subscription "misses", aka the output types
	/// of the event doesn't match up with any observables on the target entity
	/// the event will just drop without being used.
	fn drop(&mut self) {
		// This would be closed to not panic just because of a "missed" subscription.
		self.closed_flag.close();

		if self.teardown.is_some() {
			self.unsubscribe();
		}
	}
}
