use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::SharedDestination},
};

pub struct HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	destination: Arc<RwLock<Destination>>,
}

impl<Destination> SharedDestination<Destination> for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		if let Ok(destination) = self.destination.read() {
			accessor(&*destination)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Ok(mut destination) = self.destination.write() {
			accessor(&mut *destination)
		}
	}

	fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(&Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		if let Ok(destination) = self.destination.read() {
			accessor(&*destination, context)
		}
	}

	fn access_with_context_mut<F>(
		&mut self,
		mut accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(&mut Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		if let Ok(mut destination) = self.destination.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<Destination> HeapSubscriber<Destination>
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

impl<Destination> Clone for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.next(next, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.error(error, context);
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
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

impl<Destination> Tickable for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn tick(
		&mut self,
		tick: crate::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Ok(mut lock) = self.destination.write() {
			lock.tick(tick, context);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Destination> SubscriptionLike for HeapSubscriber<Destination>
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

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<Destination> Drop for HeapSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn drop(&mut self) {
		// Should not do anything on drop as it can be shared. Once the Arc
		// itself releases the Destination, the drop of that will ensure cleanup.
	}
}
