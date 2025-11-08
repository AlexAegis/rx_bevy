use std::sync::{Arc, RwLock};

use disqualified::ShortName;

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, SignalBound,
	Subscriber, SubscriptionLike, TeardownCollection, Tickable, WithPrimaryCategory,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::ErasedSharedDestination},
};

pub struct SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	destination:
		Arc<RwLock<dyn Subscriber<In = In, InError = InError, Context = Context> + Send + Sync>>,
}

impl<In, InError, Context> ErasedSharedDestination
	for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
}

impl<In, InError, Context> WithPrimaryCategory for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError, Context> ObserverUpgradesToSelf
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
			destination: Arc::new(RwLock::new(destination)),
		}
	}

	pub fn read<F>(&self, reader: F)
	where
		F: Fn(&dyn Subscriber<Context = Context, In = In, InError = InError>),
	{
		if let Ok(lock) = self.destination.read() {
			reader(&*lock);
		} else {
			println!("Poisoned destination lock: {}", ShortName::of::<Self>());
		}
	}

	pub fn write<F>(&self, mut writer: F)
	where
		F: FnMut(&mut dyn Subscriber<Context = Context, In = In, InError = InError>),
	{
		if let Ok(mut lock) = self.destination.write() {
			writer(&mut *lock);
		} else {
			println!("Poisoned destination lock: {}", ShortName::of::<Self>());
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
			destination: self.destination.clone(),
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
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
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
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
			}
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.complete(context);
			} else {
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
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
			println!("Poisoned destination lock: {}", ShortName::of::<Self>());
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
		if let Ok(lock) = self.destination.read() {
			lock.is_closed()
		} else {
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
			}
		}
	}
}

impl<In, InError, Context> TeardownCollection for SharedHeapSubscriberErased<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn add_teardown(
		&mut self,
		teardown: crate::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.write() {
				lock.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
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
