use rx_bevy_core::{
	DropContext, ErasedArcSubscriber, InnerSubscription, Observable, ObservableOutput, Observer,
	ObserverInput, SharedDestination, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};
use smallvec::SmallVec;

use crate::MulticastSubscription;

/// A multicast subject that fan-outs every incoming signal to all subscribed destinations.
///
/// Unlike the previous implementation this version DOES NOT require the Context to be drop-safe
/// (ie. `DropSafety = DropSafeSignalContext`). That means we never attempt to synthesize a
/// context value during `Drop`, so contexts that borrow (eg. `&mut World`) can be used.
///
/// Because we cannot obtain a context during `Drop`, the per-subscriber subscription returned
/// from `subscribe` will NOT automatically unsubscribe the inner subscriber when it's dropped.
/// Users must explicitly call `unsubscribe` with a valid context if eager cleanup is desired.
/// Closed subscribers are lazily cleaned up on the next `next` / `tick` emission.
pub struct Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	subscribers: SmallVec<[ErasedArcSubscriber<In, InError, Context>; 1]>,
	closed: bool,
	teardown: InnerSubscription<Context>,
}

impl<In, InError, Context> Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	/// Drops all closed subscribers
	fn clean(&mut self) {
		self.subscribers
			.retain(|subscriber| !subscriber.is_closed());
	}
}

impl<In, InError, Context> Observable for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>
			+ SubscriptionCollection,
	{
		let shared = ErasedArcSubscriber::share(destination);
		self.subscribers.push(shared.clone());
		MulticastSubscription::new(shared)
	}
}

impl<In, InError, Context> Observer for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		for destination in self.subscribers.iter_mut() {
			destination.next(next.clone(), context);
		}
		self.clean();
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		for mut destination in self.subscribers.drain(..) {
			destination.error(error.clone(), context);
			destination.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		for mut destination in self.subscribers.drain(..) {
			destination.complete(context);
			destination.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		for destination in self.subscribers.iter_mut() {
			destination.tick(tick.clone(), context);
		}
		self.clean();
	}
}

impl<In, InError, Context> SubscriptionLike for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.closed = true;
			for mut destination in self.subscribers.drain(..) {
				destination.unsubscribe(context);
			}
			self.teardown.unsubscribe(context);
		}
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.teardown.add_teardown(teardown, context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Self::Context::get_context_for_drop()
	}
}

impl<In, InError, Context> ObserverInput for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Default for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn default() -> Self {
		Self {
			subscribers: SmallVec::new(),
			teardown: InnerSubscription::default(),
			closed: false,
		}
	}
}

impl<In, InError, Context> SignalContext for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}
