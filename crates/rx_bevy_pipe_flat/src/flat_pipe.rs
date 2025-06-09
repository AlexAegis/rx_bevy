use std::marker::PhantomData;

use rx_bevy_observable::{Observable, Observer};
use rx_bevy_observer_flat::FlatObserver;
use rx_bevy_observer_shared::SharedObserver;

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
