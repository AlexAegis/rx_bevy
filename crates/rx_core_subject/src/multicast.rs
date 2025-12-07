use derive_where::derive_where;
use rx_core_traits::{
	Observable, ObservableOutput, Observer, ObserverInput, PrimaryCategorySubject, Signal,
	Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike, Tick, Tickable,
	UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
	allocator::ErasedDestinationAllocator,
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
#[derive_where(Debug)]
pub struct Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	#[derive_where(skip(Debug))]
	subscribers: SmallVec<
		[<Context::ErasedDestinationAllocator as ErasedDestinationAllocator>::Shared<In, InError>;
			1],
	>,
	closed_flag: SubscriptionClosedFlag,
}

impl<In, InError, Context> Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
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
	) -> Option<
		Vec<
			<Context::ErasedDestinationAllocator as ErasedDestinationAllocator>::Shared<
				In,
				InError,
			>,
		>,
	> {
		if self.is_closed() {
			None
		} else {
			let subscribers = self.subscribers.drain(..).collect::<Vec<_>>();

			Some(subscribers)
		}
	}
}

impl<In, InError, Context> Observable for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError, Context>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		if !self.is_closed() {
			let shared = Context::ErasedDestinationAllocator::share(destination.upgrade(), context);
			self.subscribers.push(shared.clone());
			MulticastSubscription::new(shared)
		} else {
			MulticastSubscription::new_closed()
		}
	}
}

impl<In, InError, Context> Observer for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		for destination in self.subscribers.iter_mut() {
			destination.next(next.clone(), context);
		}
		self.clean();
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		for mut destination in self.subscribers.drain(..) {
			destination.error(error.clone(), context);
			destination.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		for mut destination in self.subscribers.drain(..) {
			destination.complete(context);
			destination.unsubscribe(context);
		}
	}
}

impl<In, InError, Context> Tickable for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		for destination in self.subscribers.iter_mut() {
			destination.tick(tick, context);
		}
		self.clean();
	}
}

impl<In, InError, Context> SubscriptionLike for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed_flag.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();

			if let Some(subscribers) = self.close() {
				for mut destination in subscribers {
					destination.unsubscribe(context);
				}
			}
		}
	}
}

impl<In, InError, Context> ObserverInput for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> WithPrimaryCategory for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategorySubject;
}

impl<In, InError, Context> Default for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			subscribers: SmallVec::new(),
			closed_flag: false.into(),
		}
	}
}

impl<In, InError, Context> WithSubscriptionContext for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> Drop for Multicast<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		// Does not need to unsubscribe on drop as it's just a collection of
		// shared subscribers, the subscription given to the user is what must
		// be unsubscribed, not the multicast.

		// Close the flag regardless to avoid the safety check on drop.
		self.closed_flag.close();
	}
}
