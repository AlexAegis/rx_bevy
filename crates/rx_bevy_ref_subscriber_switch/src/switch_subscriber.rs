use std::marker::PhantomData;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, Observable, Observer, ObserverInput, Operation, SignalContext,
	Subscriber, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_ref_subscriber_shared::SharedSubscriber;
use rx_bevy_subscriber_detached::DetachedSubscriber;

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
{
	destination: SharedSubscriber<Destination>,
	inner_subscription: Option<InnerObservable::Subscription>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
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
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> SignalContext for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
{
	type Context<'c> = Destination::Context<'c>;
}

impl<InnerObservable, Destination> Observer for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
{
	fn next<'c>(&mut self, mut next: Self::In, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe(context);
			}

			let subscription =
				next.subscribe(DetachedSubscriber::new(self.destination.clone()), context);
			self.inner_subscription = Some(subscription);
		}
	}

	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			if self.inner_subscription.is_none() {
				self.destination.complete(context);
			}
			self.closed = true;
		}
	}

	fn tick<'c>(&mut self, tick: Tick, context: &mut Self::Context<'c>) {
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
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe<'c>(&mut self, context: &mut Self::Context<'c>) {
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
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	) {
		self.destination.add(subscription, context);
	}
}

impl<InnerObservable, Destination> Drop for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
{
	#[inline]
	fn drop(&mut self) {
		// TODO: Check!
		// self.unsubscribe();
		self.assert_closed_when_dropped();
	}
}

impl<InnerObservable, Destination> Operation for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: for<'c> Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context<'c> = <InnerObservable::Subscription as SignalContext>::Context<'c>,
		> + Clone,
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
