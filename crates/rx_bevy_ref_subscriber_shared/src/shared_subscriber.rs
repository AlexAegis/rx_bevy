use std::sync::{Arc, RwLock};

use rx_bevy_core::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

pub struct SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Arc<RwLock<Destination>>,
}

impl<Destination> From<Destination> for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(RwLock::new(destination)),
		}
	}

	pub fn new_from_shared(destination: impl Into<Arc<RwLock<Destination>>>) -> Self {
		Self {
			destination: destination.into(),
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&Destination),
	{
		reader(&self.destination.read().expect("poisoned"))
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut Destination),
	{
		reader(&mut self.destination.write().expect("poisoned"))
	}
}

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: Subscriber,
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

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick) {
		self.destination.tick(tick);
	}
}

impl<Destination> SubscriptionLike for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		self.destination.add(subscription);
	}
}

impl<Destination> Drop for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	/// Should not unsubscribe on drop as it's shared
	fn drop(&mut self) {}
}

impl<Destination> Operation for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Arc<RwLock<Destination>>;

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
