use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> Observer for TryCaptureSubscriber<In, InError, Destination>
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
		self.destination.next(Ok(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.next(Err(error));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Destination> SubscriptionLike for TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Destination> ObserverInput for TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> ObservableOutput for TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type Out = Result<In, InError>;
	type OutError = ();
}

impl<In, InError, Destination> Operation for TryCaptureSubscriber<In, InError, Destination>
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
