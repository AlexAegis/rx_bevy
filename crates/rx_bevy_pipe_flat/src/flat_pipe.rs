use std::marker::PhantomData;

use rx_bevy_observable::{Observable, Observer, Subscription};

use crate::SharedObserver;

pub struct FlatPipe<Source, InnerObservable>
where
	Source: Observable<Out = InnerObservable>,
	InnerObservable: Observable,
{
	pub(crate) source_observable: Source,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<Source, InnerObservable> Clone for FlatPipe<Source, InnerObservable>
where
	Source: Observable<Out = InnerObservable> + Clone,
	InnerObservable: Observable + Clone,
{
	fn clone(&self) -> Self {
		Self {
			source_observable: self.source_observable.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Source, InnerObservable> FlatPipe<Source, InnerObservable>
where
	Source: Observable<Out = InnerObservable>,
	InnerObservable: Observable,
{
	pub fn new(source_observable: Source) -> Self {
		Self {
			source_observable,
			_phantom_data: PhantomData,
		}
	}
}

impl<Source, InnerObservable> Observable for FlatPipe<Source, InnerObservable>
where
	Source: Observable<Out = InnerObservable, Error = InnerObservable::Error>,
	InnerObservable: Observable + 'static,
	InnerObservable::Out: 'static,
	InnerObservable::Error: 'static,
{
	type Out = InnerObservable::Out;
	type Error = InnerObservable::Error;
	type Subscription = Source::Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let shared_observer = SharedObserver::new(destination);
		self.source_observable
			.subscribe(FlatObserver::new(shared_observer))
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
			println!("LOL");
			self.shared_observer.complete();

			if let Some(mut inner_subscriber) = self.inner_subscriber.take() {
				inner_subscriber.unsubscribe();
			}
		}
	}
}
