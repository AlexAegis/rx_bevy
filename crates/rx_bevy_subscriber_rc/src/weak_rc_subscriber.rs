use rx_bevy_core::{
	Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick, Tickable,
	context::{
		WithSubscriptionContext,
		allocator::{DestinationSharedTypes, SharedDestination},
	},
	prelude::SubscriptionContext,
};

use crate::InnerRcSubscriber;

/// Acquired by calling `downgrade` on `RcSubscriber`
pub struct WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) shared_destination:
		<InnerRcSubscriber<Destination> as DestinationSharedTypes>::Shared,
	pub(crate) closed: bool,
}

impl<Destination> WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub fn access_destination<F>(&mut self, accessor: F)
	where
		F: Fn(&InnerRcSubscriber<Destination>),
	{
		self.shared_destination.access(accessor);
	}

	pub fn access_destination_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut InnerRcSubscriber<Destination>),
	{
		self.shared_destination.access_mut(accessor);
	}
}

impl<Destination> Clone for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn clone(&self) -> Self {
		Self {
			closed: self.closed,
			shared_destination: self.shared_destination.clone(),
		}
	}
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
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.shared_destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed = true;
			self.shared_destination.unsubscribe(context);
		}
	}

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
			// TODO: Figure out why Access can't be resolved into its actual type: : &mut InnerRcSubscriber<Destination>
			self.access_destination_mut(|destination| {
				let mut context = Destination::Context::create_context_to_unsubscribe_on_drop();
				destination.complete_if_can(&mut context);
				destination.unsubscribe_if_can(&mut context);
			});
		}
	}
}
