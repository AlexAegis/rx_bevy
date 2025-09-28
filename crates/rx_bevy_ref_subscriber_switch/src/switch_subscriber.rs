use std::{marker::PhantomData, task::Context};

use rx_bevy_core::{
	Observable, Observer, ObserverInput, ShareableSubscriber, SharedSubscriber, SignalContext,
	Subscriber, SubscriptionCollection, SubscriptionLike, Tick,
};

use rx_bevy_subscriber_detached::DetachedSubscriber;

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	destination: SharedSubscriber<Destination, Sharer>,
	inner_subscription: Option<<InnerObservable as Observable>::Subscription>,
	closed: bool,
	is_complete: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination, Sharer> SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: SharedSubscriber::new(destination),
			inner_subscription: None,
			closed: false,
			is_complete: false,
			_phantom_data: PhantomData,
		}
	}

	fn complete_if_can(
		&mut self,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		if self.is_complete && self.inner_subscription.is_none() {
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}
}
impl<InnerObservable, Destination, Sharer> ObserverInput
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination, Sharer> SignalContext
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	type Context = Destination::Context;
}

impl<InnerObservable, Destination, Sharer> Observer
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	fn next(&mut self, mut next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe(context);
			}

			let mut subscription =
				next.subscribe(DetachedSubscriber::new(self.destination.clone()), context);
			// Whenever the inner subscription completes, the switch subscriber should also check if it can  forward this completion to the destination
			subscription.add_fn(
				|c| {
					self.complete_if_can(c);
				},
				context,
			);
			self.inner_subscription = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.complete_if_can(context);
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<InnerObservable, Destination, Sharer> SubscriptionLike
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.closed = true;
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<InnerObservable, Destination, Sharer> SubscriptionCollection
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<rx_bevy_core::Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<InnerObservable, Destination, Sharer> Drop
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.destination.get_unsubscribe_context();
			self.unsubscribe(&mut context);
		}
	}
}
