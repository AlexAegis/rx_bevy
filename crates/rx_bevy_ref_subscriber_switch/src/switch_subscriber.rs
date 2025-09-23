use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, Observer, ObserverInput, ShareableSubscriber, SharedSubscriber, SignalContext,
	Subscriber, SubscriptionCollection, SubscriptionLike, Tick,
};

use rx_bevy_subscriber_detached::DetachedSubscriber;

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
{
	destination: SharedSubscriber<Destination, Sharer>,
	inner_subscription: Option<InnerObservable::Subscription>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination, Sharer> SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: SharedSubscriber::new(destination),
			inner_subscription: None,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
impl<InnerObservable, Destination, Sharer> ObserverInput
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination, Sharer> SignalContext
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
{
	type Context = Destination::Context;
}

impl<InnerObservable, Destination, Sharer> Observer
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
{
	fn next(&mut self, mut next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe(context);
			}

			let subscription =
				next.subscribe(DetachedSubscriber::new(self.destination.clone()), context);
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
		if !self.is_closed() {
			if self.inner_subscription.is_none() {
				self.destination.complete(context);
			}
			self.closed = true;
		}
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>
		+ SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		let mut context = self.destination.get_unsubscribe_context();
		self.unsubscribe(&mut context);
	}
}
