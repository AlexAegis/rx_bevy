use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct TapSubscriber<In, InError, Callback, Destination>
where
	Callback: for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	destination: Destination,
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback, Destination> TapSubscriber<In, InError, Callback, Destination>
where
	Callback: for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	pub fn new(destination: Destination, callback: Callback) -> Self {
		Self {
			destination,
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Destination> Observer
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		(self.callback)(&next);
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick) {
		self.destination.tick(tick);
	}
}

impl<In, InError, Callback, Destination> SubscriptionLike
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Subscriber<In = In, InError = InError>,
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
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Callback, Destination> ObservableOutput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Operation
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	type Destination = Destination;

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
