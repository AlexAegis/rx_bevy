use crate::{
	InnerSubscription, ObservableOutput, Observer, ObserverInput, Operation, SubscriptionLike,
	Teardown,
};

/// A simple wrapper for a plain [Observer] to make it "closeable"
pub struct ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub destination: Destination,
	pub closed: bool,
	pub teardown: InnerSubscription,
}

impl<Destination> ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
			teardown: InnerSubscription::new_empty(),
		}
	}
}

impl<Destination> Observer for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
			self.unsubscribe();
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

impl<Destination> SubscriptionLike for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
		self.teardown.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.teardown.add(Teardown::Sub(subscription));
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
			teardown: InnerSubscription::new_empty(),
		}
	}
}

impl<Destination> Drop for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
