use rx_bevy_core::{
	ErasedDestinationSharer, Observable, ObservableOutput, Observer, ObserverInput, SignalBound,
	Subscriber, SubscriptionContext, SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable,
	WithSubscriptionContext,
};
use smallvec::SmallVec;

use crate::MulticastSubscription;

/// A multicast subject that fan-outs every incoming signal to all subscribed destinations.
///
/// Unlike the previous implementation this version DOES NOT require the Context to be drop-safe
/// (ie. `DropSafety = DropSafeSubscriptionContext`). That means we never attempt to synthesize a
/// context value during `Drop`, so contexts that borrow (eg. `&mut World`) can be used.
///
/// Because we cannot obtain a context during `Drop`, the per-subscriber subscription returned
/// from `subscribe` will NOT automatically unsubscribe the inner subscriber when it's dropped.
/// Users must explicitly call `unsubscribe` with a valid context if eager cleanup is desired.
/// Closed subscribers are lazily cleaned up on the next `next` / `tick` emission.
pub struct Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	subscribers:
		SmallVec<[<Context::ErasedSharer<In, InError> as ErasedDestinationSharer>::Shared; 1]>,
	teardown: Option<SubscriptionData<Context>>,
}

impl<In, InError, Context> Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	/// Drops all closed subscribers
	fn clean(&mut self) {
		self.subscribers
			.retain(|subscriber| !subscriber.is_closed());
	}

	/// Closes the multicast and drains all its resources so the caller
	/// can perform an unsubscribe
	#[inline]
	pub(crate) fn close(
		&mut self,
	) -> Option<(
		Vec<<Context::ErasedSharer<In, InError> as ErasedDestinationSharer>::Shared>,
		Option<SubscriptionData<Context>>,
	)> {
		if self.is_closed() {
			None
		} else {
			let subscribers = self.subscribers.drain(..).collect::<Vec<_>>();
			let teardown = self.teardown.take();

			Some((subscribers, teardown))
		}
	}

	#[inline]
	pub fn is_closed(&self) -> bool {
		self.teardown.is_none()
	}

	#[inline]
	pub fn add_teardown(&mut self, teardown: Teardown<Context>, context: &mut Context) {
		if let Some(teardowns) = &mut self.teardown {
			teardowns.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<In, InError, Context> Observable for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let shared = Context::ErasedSharer::share(destination, context);
		self.subscribers.push(shared.clone());
		MulticastSubscription::new(shared)
	}
}

impl<In, InError, Context> Observer for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
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
}

impl<In, InError, Context> Tickable for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		for destination in self.subscribers.iter_mut() {
			destination.tick(tick.clone(), context);
		}
		self.clean();
	}
}

impl<In, InError, Context> ObserverInput for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Default for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			subscribers: SmallVec::new(),
			teardown: Some(SubscriptionData::default()),
		}
	}
}

impl<In, InError, Context> WithSubscriptionContext for Multicast<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}
