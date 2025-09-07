use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

#[derive(Debug)]
pub struct IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
}

impl<Destination> IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> ObservableOutput for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	type Out = Destination::In;
	type OutError = Destination::InError;
}

impl<Destination> ObserverInput for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick<'c>(&mut self, tick: Tick, context: &mut Self::Context<'c>) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.unsubscribe(context);
	}
}

impl<Destination> SubscriptionCollection for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	) {
		self.destination.add(subscription, context);
	}
}

impl<Destination> Operation for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
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
