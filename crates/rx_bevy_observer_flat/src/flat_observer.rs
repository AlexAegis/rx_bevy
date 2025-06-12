use std::marker::PhantomData;

use rx_bevy_observable::{Observable, Observer, Subscription};
use rx_bevy_observable_flat::ForwardFlattener;
use rx_bevy_observer_shared::SharedObserver;

pub struct SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
{
	closed: bool,
	inner_subscriber: Option<InObservable::Subscription>,
	_phantom_data: PhantomData<(InObservable, InError)>,
}

impl<InObservable, InError> SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
{
	pub fn new() -> Self {
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

impl<InObservable, InError> ForwardFlattener for SwitchFlattener<InObservable, InError>
where
	InObservable: Observable,
	InError: Into<InObservable::Error>,
{
	type InObservable = InObservable;
	type InError = InError;

	fn flatten_next<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as Observable>::Out,
				Error = <Self::InObservable as Observable>::Error,
			>,
	>(
		&mut self,
		mut next: Self::InObservable,
		destination: &mut SharedObserver<Destination>,
	) {
		if !self.closed {
			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}

			let d = destination.clone();
			let subscription = next.subscribe(d);
			self.inner_subscriber = Some(subscription);
		}
	}

	fn error_forward<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as Observable>::Out,
				Error = <Self::InObservable as Observable>::Error,
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
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as Observable>::Out,
				Error = <Self::InObservable as Observable>::Error,
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
	Destination: Observer<In = InnerObservable::Out, Error = InnerObservable::Error>,
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
	Destination: Observer<In = InnerObservable::Out, Error = InnerObservable::Error>,
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

impl<InnerObservable, InnerSubscription, Destination> Observer
	for FlatObserver<InnerObservable, InnerSubscription, Destination>
where
	InnerObservable: Observable<Subscription = InnerSubscription>,
	InnerSubscription: Subscription,
	InnerObservable::Out: 'static,
	InnerObservable::Error: 'static,
	Destination: 'static + Observer<In = InnerObservable::Out, Error = InnerObservable::Error>,
{
	type In = InnerObservable;
	type Error = InnerObservable::Error;

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

	fn error(&mut self, error: Self::Error) {
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
