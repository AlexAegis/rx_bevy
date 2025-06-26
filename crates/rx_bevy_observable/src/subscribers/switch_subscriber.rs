use std::marker::PhantomData;

use crate::{
	DetachedSubscriber, Observable, Observer, ObserverInput, Operation, SharedSubscriber,
	Subscriber, Subscription, SubscriptionLike,
};

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
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe();
			}

			let subscription = next.subscribe(DetachedSubscriber::new(self.destination.clone()));
			self.inner_subscription = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			// If there's no active inner subscription, complete immediately
			// Otherwise, completion will be handled when the inner subscription finishes
			if self.inner_subscription.is_none() {
				self.destination.complete();
				self.unsubscribe();
			}
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
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe();
		}
		self.destination.unsubscribe();
	}
}

impl<InnerObservable, Destination> Drop for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn drop(&mut self) {
		self.unsubscribe();
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
	type Destination = Destination;
}
