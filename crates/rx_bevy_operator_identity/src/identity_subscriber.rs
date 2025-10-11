use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick,
	Tickable, WithSubscriptionContext,
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

impl<Destination> WithSubscriptionContext for IdentitySubscriber<Destination>
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
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}
}

impl<Destination> Tickable for IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
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
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}
