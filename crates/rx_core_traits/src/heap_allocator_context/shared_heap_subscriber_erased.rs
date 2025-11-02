use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriptionData, SubscriptionLike, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::ErasedSharedDestination},
};

pub struct SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	is_closed: bool,
	destination:
		Arc<RwLock<dyn Subscriber<In = In, InError = InError, Context = Context> + Send + Sync>>,
	teardown: SubscriptionData<Context>,
}

impl<In, InError, Context> ErasedSharedDestination
	for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
}

impl<In, InError, Context> SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	pub fn new<Destination>(destination: Destination) -> Self
	where
		Destination:
			'static + Subscriber<In = In, InError = InError, Context = Context> + Send + Sync,
	{
		Self {
			is_closed: destination.is_closed(),
			destination: Arc::new(RwLock::new(destination)),
			teardown: SubscriptionData::default(),
		}
	}

	pub fn read<F>(&self, reader: F)
	where
		F: Fn(&dyn Subscriber<Context = Context, In = In, InError = InError>),
	{
		if let Ok(lock) = self.destination.read() {
			reader(&*lock);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}

	pub fn write<F>(&self, mut writer: F)
	where
		F: FnMut(&mut dyn Subscriber<Context = Context, In = In, InError = InError>),
	{
		if let Ok(mut lock) = self.destination.write() {
			writer(&mut *lock);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<In, InError, Context> Clone for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			is_closed: self.is_closed,
			destination: self.destination.clone(),
			teardown: SubscriptionData::default(), // New instance, new teardowns
		}
	}
}

impl<In, InError, Context> ObserverInput for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithSubscriptionContext
	for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
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
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.complete(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<In, InError, Context> Tickable for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
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

impl<In, InError, Context> SubscriptionLike for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.is_closed = true;
			if let Ok(mut lock) = self.destination.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
		// Must always run
		self.teardown.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: crate::Teardown<Self::Context>,
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

impl<In, InError, Context> Drop for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		// Should not do anything on drop as it's shared!
	}
}
