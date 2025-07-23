use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

// TODO: Fix
pub struct FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
{
	destination: Destination,
	/// It's in an option so it can be removed when used, allowing the use of an FnOnce
	callback: Option<Callback>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback, Destination> FinalizeSubscriber<In, InError, Callback, Destination>
where
	In: 'static,
	InError: 'static,
	Callback: 'static + FnOnce(),
	Destination: Subscriber,
{
	pub fn new(destination: Destination, callback: Callback) -> Self {
		Self {
			destination,
			callback: Some(callback),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Destination> Observer
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
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
}

impl<In, InError, Callback, Destination> SubscriptionLike
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if let Some(finalize) = self.callback.take() {
			(finalize)();
		}
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Callback, Destination> ObservableOutput
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Operation
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	type Destination = Destination;

	#[inline]
	fn get_destination(&self) -> &Self::Destination {
		&self.destination
	}

	#[inline]
	fn get_destination_mut(&mut self) -> &mut Self::Destination {
		&mut self.destination
	}
}
