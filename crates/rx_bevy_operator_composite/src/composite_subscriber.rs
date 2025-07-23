use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

#[derive(Debug)]
pub struct CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	subscriber: Inner,
	_phantom_data: PhantomData<Destination>,
}

impl<Inner, Destination> CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber + Operation,
	Destination: Observer,
{
	pub fn new(subscriber: Inner) -> Self {
		Self {
			subscriber,
			_phantom_data: PhantomData,
		}
	}
}

impl<Inner, Destination> Observer for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.subscriber.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subscriber.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subscriber.complete();
	}
}

impl<Inner, Destination> SubscriptionLike for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.subscriber.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.subscriber.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.subscriber.add(subscription);
	}
}

impl<Inner, Destination> ObserverInput for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	type In = Inner::In;
	type InError = Inner::InError;
}

impl<Inner, Destination> Operation for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber + Operation,
	<Inner as Operation>::Destination: Operation<Destination = Destination>,
	Destination: Observer,
{
	type Destination = Destination;

	#[inline]
	fn get_destination(&self) -> &Self::Destination {
		self.subscriber.get_destination().get_destination()
	}

	#[inline]
	fn get_destination_mut(&mut self) -> &mut Self::Destination {
		self.subscriber.get_destination_mut().get_destination_mut()
	}
}

impl<Inner, Destination> Drop for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
