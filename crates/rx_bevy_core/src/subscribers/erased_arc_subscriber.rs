use std::{
	marker::PhantomData,
	sync::{Arc, RwLock, RwLockReadGuard},
};

use short_type_name::short_type_name;

use crate::{
	DropContext, InnerSubscription, Observer, ObserverInput, ShareableSubscriber, SignalContext,
	Subscriber, SubscriptionCollection, SubscriptionLike,
};

// todo check if its even needed where it is currently, not having add is pretty bad, OR MAYBE put add on another trait and add a simpler fn on subscriber
pub struct ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	destination: Arc<RwLock<dyn Subscriber<In = In, InError = InError, Context = Context>>>,
	teardown: InnerSubscription<Context>,
	_ph: PhantomData<*mut Context>,
}

impl<In, InError, Context> ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	pub fn new<Destination>(destination: Destination) -> Self
	where
		Destination: 'static
			+ Subscriber<In = In, InError = InError, Context = Context>
			+ SubscriptionCollection,
	{
		Self {
			destination: Arc::new(RwLock::new(destination)),
			teardown: InnerSubscription::default(),
			_ph: PhantomData,
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
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
			teardown: InnerSubscription::default(), // New instance, new teardowns
			_ph: PhantomData,
		}
	}
}

impl<In, InError, Context> ObserverInput for ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> SignalContext for ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.next(next, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	#[inline]
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

	#[inline]
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

	#[inline]
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

impl<In, InError, Context> SubscriptionLike for ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
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

impl<In, InError, Context> ShareableSubscriber for ErasedArcSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Shared<Destination>
		= ErasedArcSubscriber<In, InError, Context>
	where
		Destination: 'static
			+ Subscriber<In = In, InError = InError, Context = Context>
			+ SubscriptionCollection;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination: 'static
			+ Subscriber<In = In, InError = InError, Context = Context>
			+ SubscriptionCollection,
	{
		ErasedArcSubscriber::new(destination)
	}
}

impl<In, InError, Context> SignalContext
	for RwLockReadGuard<'_, dyn Subscriber<Context = Context, In = In, InError = InError>>
where
	Context: DropContext,
{
	type Context = Context;
}

impl<'d, In, InError, Context> ObserverInput
	for RwLockReadGuard<'d, dyn Subscriber<Context = Context, In = In, InError = InError>>
where
	In: 'static + Clone,
	InError: 'static + Clone,
{
	type In = In;
	type InError = InError;
}
