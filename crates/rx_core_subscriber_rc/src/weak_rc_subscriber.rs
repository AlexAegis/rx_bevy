use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Subscriber, SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection,
	Tick, Tickable, allocator::DestinationSharedTypes,
};

use crate::InnerRcSubscriber;

/// Acquired by calling `downgrade` on `RcSubscriber`
#[derive(RxSubscriber)]
#[derive_where(Clone)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_context(Destination::Context)]
pub struct WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) shared_destination:
		<InnerRcSubscriber<Destination> as DestinationSharedTypes>::Shared,
}

impl<Destination> Observer for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.next(next, context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.shared_destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.shared_destination.complete(context);
	}
}

impl<Destination> Tickable for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.tick(tick, context);
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

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.shared_destination.unsubscribe(context);
		}
	}
}

impl<Destination> TeardownCollection for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.add_teardown(teardown, context);
	}
}
