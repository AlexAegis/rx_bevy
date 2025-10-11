use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	DestinationSharer, Observer, ObserverInput, SharedDestination, Subscriber, SubscriptionLike,
	Teardown, Tickable, WithContext,
};

pub struct ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	destination: Arc<RwLock<Destination>>,
}

impl<D> DestinationSharer for ArcSubscriber<D>
where
	D: 'static + Subscriber + Send + Sync,
{
	type Shared<Destination>
		= ArcSubscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Send
			+ Sync;

	fn share<Destination>(
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Shared<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		ArcSubscriber::new(destination)
	}
}

impl<Destination> SharedDestination<Destination> for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type Access = Destination;

	fn access<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Self::Access, &mut Self::Context),
	{
		if let Ok(destination) = self.destination.read() {
			accessor(&*destination, context)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Self::Access, &mut Self::Context),
	{
		if let Ok(mut destination) = self.destination.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<Destination> ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
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
			reader(&*lock);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}

	pub fn write<F>(&self, mut writer: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Ok(mut lock) = self.destination.write() {
			writer(&mut *lock);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Destination> Clone for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithContext for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
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
}

impl<Destination> Tickable for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		if let Ok(mut lock) = self.destination.write() {
			lock.tick(tick, context);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Destination> SubscriptionLike for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
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

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Ok(mut lock) = self.destination.write() {
			lock.get_context_to_unsubscribe_on_drop()
		} else {
			panic!(
				"Context can't be acquired in a {} as the destination RwLock is poisoned!",
				short_type_name::<Self>()
			)
		}
	}
}

impl<Destination> Drop for ArcSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn drop(&mut self) {
		// Should not do anything on drop as it can be shared. Once the Arc
		// itself releases the Destination, the drop of that will ensure cleanup.
	}
}
