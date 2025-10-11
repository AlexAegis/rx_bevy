use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	ErasedDestinationSharer, ErasedSharedDestination, Observer, ObserverInput, SignalBound,
	SignalContext, Subscriber, SubscriptionData, SubscriptionLike, Tickable, WithContext,
};

pub struct ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	destination:
		Arc<RwLock<dyn Subscriber<In = In, InError = InError, Context = Context> + Send + Sync>>,
	teardown: SubscriptionData<Context>,
}

impl<In, InError, Context> ErasedDestinationSharer for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type Shared = ErasedArcSubscriber<In, InError, Context>;

	fn share<Destination>(destination: Destination, _context: &mut Self::Context) -> Self::Shared
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		ErasedArcSubscriber::new(destination)
	}
}

impl<In, InError, Context> ErasedSharedDestination for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
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

	fn access_with_context<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Self::Access, &mut Self::Context),
	{
		if let Ok(destination) = self.destination.read() {
			accessor(&*destination, context)
		}
	}

	fn access_with_context_mut<F>(&mut self, mut accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Self::Access, &mut Self::Context),
	{
		if let Ok(mut destination) = self.destination.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<In, InError, Context> ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
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

impl<In, InError, Context> Clone for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
			teardown: SubscriptionData::default(), // New instance, new teardowns
		}
	}
}

impl<In, InError, Context> ObserverInput for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithContext for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
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
		// Must always run
		self.teardown.unsubscribe(context);
	}
}

impl<In, InError, Context> Tickable for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		if let Ok(mut lock) = self.destination.write() {
			lock.tick(tick, context);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<In, InError, Context> SubscriptionLike for ErasedArcSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.destination.read() {
			lock.is_closed()
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
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
		context: &mut Self::Context,
	) {
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
