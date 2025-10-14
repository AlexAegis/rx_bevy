use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriptionData, SubscriptionLike, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::ErasedSharedDestination},
};

pub struct ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	destination:
		Arc<RwLock<dyn Subscriber<In = In, InError = InError, Context = Context> + Send + Sync>>,
	teardown: SubscriptionData<Context>,
}

impl<In, InError, Context> ErasedSharedDestination for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Access = dyn Subscriber<In = In, InError = InError, Context = Context>;

	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Self::Access),
	{
		if let Ok(destination) = self.destination.read() {
			accessor(&*destination)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F)
	where
		F: FnMut(&mut Self::Access),
	{
		if let Ok(mut destination) = self.destination.write() {
			accessor(&mut *destination)
		}
	}

	fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) where
		F: Fn(&Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_>),
	{
		if let Ok(destination) = self.destination.read() {
			accessor(&*destination, context)
		}
	}

	fn access_with_context_mut<F>(
		&mut self,
		mut accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) where
		F: FnMut(&mut Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_>),
	{
		if let Ok(mut destination) = self.destination.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<In, InError, Context> ErasedHeapSubscriber<In, InError, Context>
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

impl<In, InError, Context> Clone for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
			teardown: SubscriptionData::default(), // New instance, new teardowns
		}
	}
}

impl<In, InError, Context> ObserverInput for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithSubscriptionContext for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
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

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.complete(context);
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
		// Must always run
		self.teardown.unsubscribe(context);
	}
}

impl<In, InError, Context> Tickable for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: crate::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if let Ok(mut lock) = self.destination.write() {
			lock.tick(tick, context);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<In, InError, Context> SubscriptionLike for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.destination.read() {
			lock.is_closed()
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		if !self.is_closed() {
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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
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

impl<In, InError, Context> Drop for ErasedHeapSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
