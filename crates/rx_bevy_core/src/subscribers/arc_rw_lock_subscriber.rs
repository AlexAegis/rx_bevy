use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	DestinationSharer, Observer, ObserverInput, SharedDestination, SignalContext, Subscriber,
	SubscriptionLike,
};

impl<S> SignalContext for Arc<RwLock<S>>
where
	S: Subscriber,
{
	type Context = S::Context;
}

impl<S> ObserverInput for Arc<RwLock<S>>
where
	S: Subscriber,
{
	type In = S::In;
	type InError = S::InError;
}

impl<T> DestinationSharer for Arc<RwLock<T>>
where
	T: Subscriber + 'static,
{
	type Shared<Destination>
		= Arc<RwLock<Destination>>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>,
	{
		Arc::new(RwLock::new(destination))
	}
}

impl<T> SharedDestination<T> for Arc<RwLock<T>>
where
	T: Subscriber + 'static,
{
	type Access = T;

	fn access<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Self::Access, &mut Self::Context),
	{
		if let Ok(destination) = self.read() {
			accessor(&*destination, context)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Self::Access, &mut Self::Context),
	{
		if let Ok(mut destination) = self.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<S> Observer for Arc<RwLock<S>>
where
	S: Subscriber,
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

	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut destination) = self.write() {
				destination.tick(tick, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<S> SubscriptionLike for Arc<RwLock<S>>
where
	S: Subscriber,
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

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		if let Ok(mut destination) = self.write() {
			destination.get_unsubscribe_context()
		} else {
			panic!(
				"Context can't be acquired in a {} as the destination RwLock is poisoned!",
				short_type_name::<Self>(),
			)
		}
	}
}
