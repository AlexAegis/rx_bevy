use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	Observer, ObserverInput, Operation, ShareableSubscriber, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown,
};

pub struct ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Arc<RwLock<Destination>>,
}

impl<Destination> ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(RwLock::new(destination)),
		}
	}

	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&Destination),
	{
		if let Ok(lock) = self.destination.read() {
			reader(&lock);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}

	pub fn write<F>(&self, mut writer: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Ok(mut lock) = self.destination.write() {
			writer(&mut lock);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Destination> Clone for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.next(next, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.error(error, context);
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.complete(context);
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.tick(tick, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<Destination> SignalContext for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> SubscriptionLike for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.destination.read() {
			lock.is_closed()
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Destination::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		if let Ok(mut lock) = self.destination.write() {
			lock.get_unsubscribe_context()
		} else {
			panic!(
				"Context can't be acquired in a {} as the destination RwLock is poisoned!",
				short_type_name::<Self>()
			)
		}
	}
}

impl<D> ShareableSubscriber for ArcSubscriber<D>
where
	D: 'static + Subscriber,
{
	type Shared<Destination>
		= ArcSubscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>,
	{
		ArcSubscriber::new(destination)
	}
}

impl<Destination> SubscriptionCollection for ArcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.add(subscription, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<Destination> Operation for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Destination;
}

impl<Destination> Drop for ArcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		// Should not do anything on drop as it can be shared. Once the Arc
		// itself releases the Destination, the drop of that will ensure cleanup.
	}
}
