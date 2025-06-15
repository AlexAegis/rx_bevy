use std::marker::PhantomData;

use crate::{
	Forwarder, Observable, ObservableOutput, Observer, ObserverInput, SharedObserver, Subscription,
};

pub struct SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
{
	closed: bool,
	inner_subscriber: Option<InObservable::Subscription>,
	_phantom_data: PhantomData<(InObservable, InError)>,
}

impl<InObservable, InError> Default for SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
{
	fn default() -> Self {
		Self {
			closed: false,
			inner_subscriber: None,
			_phantom_data: PhantomData,
		}
	}
}

impl<InObservable, InError> Clone for SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
	InObservable::Subscription: Clone,
{
	fn clone(&self) -> Self {
		Self {
			inner_subscriber: self.inner_subscriber.clone(),
			closed: self.closed,
			_phantom_data: PhantomData,
		}
	}
}

impl<InObservable, InError> ObserverInput for SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
{
	type In = InObservable;
	type InError = InError;
}

impl<InObservable, InError> ObservableOutput for SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
{
	type Out = <InObservable as ObservableOutput>::Out;
	type OutError = <InObservable as ObservableOutput>::OutError;
}

impl<InObservable, InError> Forwarder for SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
	InError: Into<InObservable::OutError>,
{
	fn next_forward<
		Destination: Observer<
				In = <Self::In as ObservableOutput>::Out,
				InError = <Self::In as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		mut next: Self::In,
		destination: &mut Destination,
		//destination: &mut SharedObserver<Destination>,
	) {
		if !self.closed {
			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}

			// let d = destination.clone();
			// let subscription = next.subscribe(d);
			// self.inner_subscriber = Some(subscription);
		}
	}

	fn error_forward<
		Destination: Observer<
				In = <Self::In as ObservableOutput>::Out,
				InError = <Self::In as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		if !self.closed {
			destination.error(error.into());

			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}
		}
	}
	fn complete_forward<
		Destination: Observer<
				In = <Self::In as ObservableOutput>::Out,
				InError = <Self::In as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: &mut Destination,
	) {
		if !self.closed {
			self.closed = true;
			destination.complete();
			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}
		}
	}
}

pub struct FlatObserver<InnerObservable, InnerSubscriber, Destination>
where
	InnerObservable: Observable,
	InnerSubscriber: Subscription,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	shared_observer: SharedObserver<Destination>,
	inner_subscriber: Option<InnerSubscriber>,
	closed: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, InnerSubscriber, Destination>
	FlatObserver<InnerObservable, InnerSubscriber, Destination>
where
	InnerObservable: Observable,
	InnerSubscriber: Subscription,
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
impl<InnerObservable, InnerSubscription, Destination> ObserverInput
	for FlatObserver<InnerObservable, InnerSubscription, Destination>
where
	InnerObservable: Observable<Subscription = InnerSubscription>,
	InnerSubscription: Subscription,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static + Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, InnerSubscription, Destination> Observer
	for FlatObserver<InnerObservable, InnerSubscription, Destination>
where
	InnerObservable: Observable<Subscription = InnerSubscription>,
	InnerSubscription: Subscription,
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
