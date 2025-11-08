use rx_core_traits::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Subscriber,
	SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection,
	Tick, Tickable, WithPrimaryCategory, WithSubscriptionContext,
	allocator::{DestinationSharedTypes, SharedDestination},
};

use crate::InnerRcSubscriber;

/// Acquired by calling `downgrade` on `RcSubscriber`
pub struct WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) shared_destination:
		<InnerRcSubscriber<Destination> as DestinationSharedTypes>::Shared,
	pub(crate) closed_flag: SubscriptionClosedFlag,
}

impl<Destination> ObserverInput for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> WithPrimaryCategory for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for WeakRcSubscriber<Destination> where
	Destination: 'static + Subscriber
{
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
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
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

impl<Destination> Drop for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Destination::Context::create_context_to_unsubscribe_on_drop();

			self.shared_destination.access_with_context_mut(
				|destination, context| {
					destination.complete_if_can(context);
					destination.unsubscribe_if_can(context);
				},
				&mut context,
			);
		}
	}
}
