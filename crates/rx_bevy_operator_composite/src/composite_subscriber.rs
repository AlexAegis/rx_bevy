use std::marker::PhantomData;

#[cfg(feature = "channel_context")]
#[cfg(feature = "tick")]
use rx_bevy_core::ChannelContext;
use rx_bevy_core::{
	SubscriptionCollection, Observer, ObserverInput, Operation, SignalContext, Subscriber,
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
	type Context = Destination::Context;
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

impl<Inner, Destination> SubscriptionLike<<Destination as Observer>::Context>
	for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.subscriber.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Destination as Observer>::Context) {
		self.subscriber.unsubscribe(context);
	}
}

impl<Inner, Destination> SubscriptionCollection<<Destination as Observer>::Context>
	for CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	#[inline]
	fn add(
		&mut self,
		subscription: impl Into<Teardown<<Destination as Observer>::Context>>,
		context: &mut <Destination as Observer>::Context,
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
		self.unsubscribe(());
	}
}
