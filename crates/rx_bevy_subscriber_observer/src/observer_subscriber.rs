use rx_bevy_core::{
	DropContext, InnerDropSubscription, ObservableOutput, Observer, ObserverInput, Operation,
	SignalContext, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

/// A simple wrapper for a plain [Observer] to make it "closeable"
pub struct ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub destination: Destination,
	pub closed: bool,
	pub teardown: InnerDropSubscription<Destination::Context>,
}

impl<Destination> ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
			teardown: InnerDropSubscription::new_empty(),
		}
	}
}

impl<Destination> Observer for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> ObserverInput for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> ObservableOutput for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type Out = Destination::In;
	type OutError = Destination::InError;
}

impl<Destination> SignalContext for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type Context = Destination::Context;
}

impl<Destination> SubscriptionLike for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.closed = true;
		self.teardown.unsubscribe(context);
	}
}

impl<Destination> SubscriptionCollection for ObserverSubscriber<Destination>
where
	Destination: Observer,
	Destination: SubscriptionCollection,
{
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		self.teardown.add_finalizer(subscription, context);
	}
}

impl<Destination> Operation for ObserverSubscriber<Destination>
where
	Destination: Observer,
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

impl<Destination> From<Destination> for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn from(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
			teardown: InnerDropSubscription::new_empty(),
		}
	}
}
