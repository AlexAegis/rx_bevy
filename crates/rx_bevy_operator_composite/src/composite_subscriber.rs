use std::marker::PhantomData;

use rx_bevy_core::{
	Observer, ObserverInput, Operation, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike,
};

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

impl<Inner, Destination> SignalContext for CompositeSubscriber<Inner, Destination>
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
}

impl<Inner, Destination> SubscriptionCollection for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
	Inner: SubscriptionCollection,
{
	#[inline]
	fn add<S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		self.subscriber.add(subscription, context);
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
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		self.subscriber.read_destination(|operation| {
			operation.read_destination(|destination| reader(destination))
		});
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		self.subscriber.write_destination(|operation| {
			operation.write_destination(|destination| writer(destination))
		});
	}
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
