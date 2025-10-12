use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	Observer, ObserverInput, Subscriber, SubscriptionLike, Tickable,
	context::{WithSubscriptionContext, allocator::SharedDestination},
};

impl<Destination> WithSubscriptionContext for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type Context = Destination::Context;
}

impl<Destination> ObserverInput for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SharedDestination<Destination> for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		if let Ok(destination) = self.read() {
			accessor(&*destination)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Ok(mut destination) = self.write() {
			accessor(&mut *destination)
		}
	}

	fn access_with_context<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Destination, &mut Self::Context),
	{
		if let Ok(destination) = self.read() {
			accessor(&*destination, context)
		}
	}

	fn access_with_context_mut<F>(&mut self, mut accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Destination, &mut Self::Context),
	{
		if let Ok(mut destination) = self.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<Destination> Observer for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut destination) = self.write() {
				destination.next(next, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut destination) = self.write() {
				destination.error(error, context);
				destination.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut destination) = self.write() {
				destination.complete(context);
				destination.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<Destination> Tickable for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		if let Ok(mut destination) = self.write() {
			destination.tick(tick, context);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Destination> SubscriptionLike for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn is_closed(&self) -> bool {
		if let Ok(destination) = self.read() {
			destination.is_closed()
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut destination) = self.write() {
				destination.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn add_teardown(
		&mut self,
		teardown: crate::Teardown<Self::Context>,
		context: &mut Self::Context,
	) {
		if !self.is_closed() {
			if let Ok(mut destination) = self.write() {
				destination.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Ok(mut destination) = self.write() {
			destination.get_context_to_unsubscribe_on_drop()
		} else {
			panic!(
				"Context can't be acquired in a {} as the destination RwLock is poisoned!",
				short_type_name::<Self>(),
			)
		}
	}
}
