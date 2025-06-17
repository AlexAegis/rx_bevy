use std::marker::PhantomData;

use super::SharedSubscriber;
use crate::{Observable, Observer, ObserverInput, Subscription};

/// TODO: Add a dedicated error mapper
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	shared_destination: SharedSubscriber<Destination>,
	inner_subscriber: Option<InnerObservable::Subscription>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			inner_subscriber: None,
			shared_destination: SharedSubscriber::new(destination),
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
impl<InnerObservable, Destination> ObserverInput for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> Observer for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static + Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}

			let subscription = next.subscribe(self.shared_destination.clone());
			self.inner_subscriber = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.shared_destination.error(error);

			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
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

impl<InnerObservable, Destination> Subscription for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static + Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
		if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
			inner_subscriber.unsubscribe();
		}
	}
}
