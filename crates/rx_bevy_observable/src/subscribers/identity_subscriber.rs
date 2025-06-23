use std::marker::PhantomData;

use crate::{ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

#[derive(Debug)]
pub struct IdentitySubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> IdentitySubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> ObservableOutput for IdentitySubscriber<In, InError, Destination>
where
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Destination> ObserverInput for IdentitySubscriber<In, InError, Destination>
where
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> Observer for IdentitySubscriber<In, InError, Destination>
where
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

impl<In, InError, Destination> SubscriptionLike for IdentitySubscriber<In, InError, Destination>
where
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<In, InError, Destination> Operation for IdentitySubscriber<In, InError, Destination>
where
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type Destination = Destination;
}
