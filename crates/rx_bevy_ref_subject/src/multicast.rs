use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropContext, DropSafeSignalContext, DynSubscriber, ErasedArcSubscriber, Observable,
	ObservableOutput, Observer, ObserverInput, SharedSubscriber, SignalContext, Subscriber,
	SubscriptionLike, Tick,
};

use rx_bevy_subscription_drop::DropSubscription;
use smallvec::SmallVec;

/// TODO: Drop the need for a DropSafeSignalContext once the method of sharing is configurable
pub struct Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	subscribers: SmallVec<[ErasedArcSubscriber<In, InError, Context>; 1]>,
	closed: bool,
}

impl<In, InError, Context> Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
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
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let shared = ErasedArcSubscriber::share(destination);
		self.subscribers.push(shared.clone());
		DropSubscription::new(shared)
	}
}

impl<In, InError, Context> Observer for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
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
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		for mut destination in self.subscribers.drain(..) {
			destination.unsubscribe(context);
		}
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Self::Context::get_context_for_drop()
	}
}

impl<In, InError, Context> ObserverInput for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Default for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self {
			subscribers: SmallVec::new(),
			closed: false,
		}
	}
}

impl<In, InError, Context> SignalContext for Multicast<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}
