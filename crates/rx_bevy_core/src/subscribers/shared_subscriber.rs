use std::marker::PhantomData;

use crate::{
	DestinationSharer, Observer, ObserverInput, SharedDestination, Subscriber, SubscriptionLike,
	Teardown, Tick, Tickable, WithContext,
};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
// TODO: Maybe this and RcSubscriber should be joined together
pub struct SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	destination: Sharer::Shared<Destination>,
	_phantom_data: PhantomData<Destination>,
}

impl<Destination, Sharer> SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	pub fn new(destination: Destination, context: &mut Sharer::Context) -> Self {
		Self {
			destination: Sharer::share(destination, context),
			_phantom_data: PhantomData,
		}
	}

	pub fn access<F>(&mut self, accessor: F, context: &mut Sharer::Context)
	where
		F: Fn(
			&<Sharer::Shared<Destination> as SharedDestination<Destination>>::Access,
			&mut Sharer::Context,
		),
	{
		self.destination.access(accessor, context);
	}

	pub fn access_mut<F>(&mut self, accessor: F, context: &mut Sharer::Context)
	where
		F: FnMut(
			&mut <Sharer::Shared<Destination> as SharedDestination<Destination>>::Access,
			&mut Sharer::Context,
		),
	{
		self.destination.access_mut(accessor, context);
	}
}

impl<Destination, Sharer> Clone for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, Sharer> ObserverInput for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	type In = Sharer::In;
	type InError = Sharer::InError;
}

impl<Destination, Sharer> WithContext for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	type Context = Sharer::Context;
}

impl<Destination, Sharer> Observer for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}
}

impl<Destination, Sharer> Tickable for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.access_mut(
			move |destination, inner_context| destination.tick(tick.clone(), inner_context),
			context,
		);
	}
}

impl<Destination, Sharer> SubscriptionLike for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Destination, Sharer> Drop for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber + Send + Sync,
	Sharer: DestinationSharer<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		>,
{
	fn drop(&mut self) {
		// Should not unsubscribe on drop as it's shared!
	}
}
