use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
	prelude::SwitchSubscriber,
};

pub struct SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	destination: SwitchSubscriber<InnerObservable, Destination>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, InnerObservable, Destination>
	SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Clone + Fn(In) -> InnerObservable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination, switcher: Switcher) -> Self {
		Self {
			destination: SwitchSubscriber::new(destination),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
{
	fn next(&mut self, next: Self::In) {
		self.destination.next((self.switcher)(next));
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> SubscriptionLike
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> ObserverInput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, InnerObservable, Destination> Operation
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type Destination = Destination;

	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		self.destination.read_destination(|shared_subscriber| {
			shared_subscriber.read_destination(|shared_destination| {
				let lock = shared_destination.read().expect("not be poisoned");
				reader(&lock);
			});
		});
	}

	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		self.destination.write_destination(|shared_subscriber| {
			shared_subscriber.write_destination(|shared_destination| {
				let mut lock = shared_destination.write().expect("not be poisoned");
				writer(&mut lock);
			});
		});
	}
}
