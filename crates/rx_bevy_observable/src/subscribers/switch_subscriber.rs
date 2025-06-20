use std::marker::PhantomData;

use super::SharedSubscriber;
use crate::{Observable, Observer, ObserverInput, Subscriber, Subscription, SubscriptionLike};

/// TODO: Add a dedicated error mapper
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	shared_destination: SharedSubscriber<Destination>,
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
			inner_subscription: None,
			shared_destination: SharedSubscriber::new(destination),
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
			if let Some(mut inner_subscriber) = self.inner_subscription.take() {
				inner_subscriber.unsubscribe();
			}

			let destination = self.shared_destination.clone();
			let subscription = next.subscribe(destination);
			self.inner_subscription = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.shared_destination.error(error);

			if let Some(mut inner_subscriber) = self.inner_subscription.take() {
				inner_subscriber.unsubscribe();
			}
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.shared_destination.complete();
			self.unsubscribe();
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
		if let Some(mut inner_subscriber) = self.inner_subscription.take() {
			inner_subscriber.unsubscribe();
		}
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
