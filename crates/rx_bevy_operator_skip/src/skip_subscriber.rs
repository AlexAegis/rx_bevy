use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct SkipSubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	count: usize,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> SkipSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, count: usize) -> Self {
		Self {
			destination,
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> Observer for SkipSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if self.count == 0 {
			self.destination.next(next);
		} else {
			self.count -= 1;
		}
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

impl<In, InError, Destination> SubscriptionLike for SkipSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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

impl<In, InError, Destination> ObservableOutput for SkipSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Destination> ObserverInput for SkipSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> Operation for SkipSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;
}
