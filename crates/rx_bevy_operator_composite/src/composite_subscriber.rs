use std::marker::PhantomData;

use rx_bevy_core::{Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, WithContext};

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
	Inner: Subscriber,
	Destination: Observer,
{
	pub fn new(subscriber: Inner) -> Self {
		Self {
			subscriber,
			_phantom_data: PhantomData,
		}
	}
}

impl<Inner, Destination> WithContext for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	type Context = Inner::Context;
}

impl<Inner, Destination> Observer for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.subscriber.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.subscriber.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.subscriber.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick, context: &mut Self::Context) {
		self.subscriber.tick(tick, context);
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
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.subscriber.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.subscriber.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.subscriber.get_context_to_unsubscribe_on_drop()
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

impl<Inner, Destination> Drop for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			panic!(
				"Dropped {} without unsubscribing first!",
				short_type_name::short_type_name::<Self>()
			)
		}
	}
}
