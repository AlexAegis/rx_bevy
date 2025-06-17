use std::marker::PhantomData;

use crate::{
	Forwarder, Observable, ObservableOutput, Observer, ObserverInput, SharedObserver,
	SubscriberForwarder, Subscription, subscribers::shared_observer,
};

pub struct SwitchFlattener<InObservable, InError, Destination>
where
	InObservable: Observable,
	Destination: Observer,
{
	closed: bool,
	destination: SharedObserver<Destination>,
	inner_subscriber: Option<InObservable::Subscription>,
	_phantom_data: PhantomData<(InObservable, InError)>,
}
/*
impl<InObservable, InError, Destination> Default
	for SwitchFlattener<InObservable, InError, Destination>
where
	InObservable: Observable,
	Destination: Observer,
{
	fn default() -> Self {
		Self {
			closed: false,
			destination: SharedObserver::new(destination)
			inner_subscriber: None,
			_phantom_data: PhantomData,
		}
	}
}
*/

impl<InObservable, InError, Destination> Clone
	for SwitchFlattener<InObservable, InError, Destination>
where
	InObservable: Observable,
	InObservable::Subscription: Clone,
	Destination: Observer,
{
	fn clone(&self) -> Self {
		Self {
			inner_subscriber: self.inner_subscriber.clone(),
			destination: self.destination.clone(),
			closed: self.closed,
			_phantom_data: PhantomData,
		}
	}
}

impl<InObservable, InError, Destination> ObserverInput
	for SwitchFlattener<InObservable, InError, Destination>
where
	InObservable: 'static + Observable,
	InError: 'static,
	Destination: 'static + Observer,
{
	type In = InObservable;
	type InError = InError;
}

impl<InObservable, InError, Destination> ObservableOutput
	for SwitchFlattener<InObservable, InError, Destination>
where
	InObservable: Observable,
	Destination: Observer,
{
	type Out = InObservable::Out;
	type OutError = InObservable::OutError;
}

impl<InObservable, InError, Destination> SubscriberForwarder
	for SwitchFlattener<InObservable, InError, Destination>
where
	InObservable: 'static + Observable,
	InObservable::Out: 'static,
	InObservable::OutError: 'static,
	InError: 'static + Into<InObservable::OutError>,
	Destination: 'static + Observer<In = InObservable::Out, InError = InObservable::OutError>,
{
	type Destination = Destination;

	fn next_forward(&mut self, mut next: Self::In, destination: &mut Self::Destination) {
		// destination.next(next);
		if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
			inner_subscriber.unsubscribe();
		}

		let subscription = next.subscribe(self.destination.clone());
		self.inner_subscriber = Some(subscription);
	}

	fn error_forward(&mut self, error: Self::InError, destination: &mut Self::Destination) {
		destination.error(error.into());
	}
	fn complete_forward(&mut self, destination: &mut Self::Destination) {
		destination.complete();
	}
}

pub struct FlatObserver<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	shared_observer: SharedObserver<Destination>,
	inner_subscriber: Option<InnerObservable::Subscription>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> FlatObserver<InnerObservable, Destination>
where
	InnerObservable: Observable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(shared_observer: SharedObserver<Destination>) -> Self {
		Self {
			inner_subscriber: None,
			shared_observer,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
impl<InnerObservable, Destination> ObserverInput for FlatObserver<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> Observer for FlatObserver<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static + Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In) {
		// TODO: This is a switching mechanic, so maybe it should a SwitchingFlatObserver?
		if !self.closed {
			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}

			let subscription = next.subscribe(self.shared_observer.clone());
			self.inner_subscriber = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.closed {
			self.shared_observer.error(error);

			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}
		}
	}

	fn complete(&mut self) {
		if !self.closed {
			self.closed = true;
			self.shared_observer.complete();
			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}
		}
	}
}
