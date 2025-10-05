use crate::{
	Observer, ObserverInput, SharedDestination, SignalContext, Subscriber, SubscriptionLike,
	Teardown, Tick,
};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
// TODO: Maybe this and RcSubscriber should be joined together
pub struct SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
{
	destination: Sharer::Shared<Destination>,
}

impl<Destination, Share> From<Destination> for SharedSubscriber<Destination, Share>
where
	Destination: 'static + Subscriber,
	Share: SharedDestination<Access = Destination>,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination, Sharer> SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Sharer::share(destination),
		}
	}

	fn access<F>(
		&mut self,
		accessor: F,
		context: &mut <<Sharer as SharedDestination>::Shared<Destination> as SignalContext>::Context,
	) where
		F: Fn(
			&<<Sharer as SharedDestination>::Shared<Destination> as SharedDestination>::Access,
			&mut <<Sharer as SharedDestination>::Shared<Destination> as SignalContext>::Context,
		),
	{
		self.destination.access(accessor, context);
	}

	fn access_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <<Sharer as SharedDestination>::Shared<Destination> as SignalContext>::Context,
	) where
		F: FnMut(
			&mut <<Sharer as SharedDestination>::Shared<Destination> as SharedDestination>::Access,
			&mut <<Sharer as SharedDestination>::Shared<Destination> as SignalContext>::Context,
		),
	{
		self.destination.access_mut(accessor, context);
	}
}

impl<Destination, Sharer> Clone for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination, Sharer> ObserverInput for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
{
	type In = <<
		<Sharer as SharedDestination>::Shared<Destination> as SharedDestination
	>::Access as ObserverInput>::In;
	type InError = <<
		<Sharer as SharedDestination>::Shared<Destination> as SharedDestination
	>::Access as ObserverInput>::InError;
}

impl<Destination, Sharer> SignalContext for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
{
	type Context = <<Sharer as SharedDestination>::Shared<Destination> as SignalContext>::Context;
}

impl<Destination, Sharer> Observer for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
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
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
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
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination, Sharer> Drop for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: SharedDestination<Access = Destination>,
{
	fn drop(&mut self) {
		// Should not unsubscribe on drop as it's shared!
	}
}
