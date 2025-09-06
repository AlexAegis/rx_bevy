use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, Observer, ObserverInput, Operation, Subscriber, Subscription, SubscriptionLike,
};
use rx_bevy_ref_subscriber_shared::SharedSubscriber;
use rx_bevy_subscriber_detached::DetachedSubscriber;

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	destination: SharedSubscriber<Destination>,
	inner_subscription: Option<Subscription>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
	Destination: Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> Observer for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In, context: &mut ChannelContext) {
		if !self.is_closed() {
			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe(context);
			}

			let subscription = next.subscribe(DetachedSubscriber::new(self.destination.clone()));
			self.inner_subscription = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut ChannelContext) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut ChannelContext) {
		if !self.is_closed() {
			if self.inner_subscription.is_none() {
				self.destination.complete(context);
			}
			self.closed = true;
		}
	}

	fn tick(&mut self, tick: rx_bevy_core::Tick, context: &mut ChannelContext) {
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
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut ChannelContext) {
		self.closed = true;
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add(&mut self, subscription: impl Into<Teardown>, context: &mut ChannelContext) {
		self.destination.add(subscription, context);
	}
}

impl<InnerObservable, Destination> Drop for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
