use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_ref_subscriber_shared::SharedSubscriber;
use rx_bevy_subscriber_detached::DetachedSubscriber;

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	destination: SharedSubscriber<Destination>,
	inner_subscription: Option<InnerObservable::Subscription>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
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
impl<InnerObservable, Destination> ObserverInput for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> SignalContext for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type Context = Destination::Context;
}

impl<InnerObservable, Destination> Observer for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
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

impl<InnerObservable, Destination> SubscriptionLike
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
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
}

impl<InnerObservable, Destination> SubscriptionCollection
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<InnerObservable, Destination> Drop for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	#[inline]
	fn drop(&mut self) {
		// self.unsubscribe();
	}
}

impl<InnerObservable, Destination> Operation for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type Destination = SharedSubscriber<Destination>;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}
