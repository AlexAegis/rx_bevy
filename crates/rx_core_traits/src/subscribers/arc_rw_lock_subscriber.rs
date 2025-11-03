use std::sync::{Arc, RwLock};

use disqualified::ShortName;

use crate::{
	Observer, ObserverInput, Subscriber, SubscriptionLike, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::SharedDestination},
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

fn poisoned_destination_message<T>() -> String {
	format!("Poisoned destination lock: {}", ShortName::of::<T>())
}

impl<Destination> SharedDestination<Destination> for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn clone_with_context(
		&self,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		self.clone()
	}

	fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(&Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		if let Ok(destination) = self.read() {
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
		if let Ok(mut destination) = self.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<Destination> Observer for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			let mut destination = self
				.write()
				.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));
			destination.next(next, context);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			let mut destination = self
				.write()
				.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));
			destination.error(error, context);
			destination.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			let mut destination = self
				.write()
				.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));
			destination.complete(context);
		}
	}
}

impl<Destination> Tickable for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn tick(
		&mut self,
		tick: crate::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		let mut destination = self
			.write()
			.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));
		destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for Arc<RwLock<Destination>>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn is_closed(&self) -> bool {
		let destination = self
			.read()
			.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));
		destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			let mut destination = self
				.write()
				.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));
			destination.unsubscribe(context);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: crate::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			let mut destination = self
				.write()
				.unwrap_or_else(|_| panic!("{}", poisoned_destination_message::<Self>()));

			destination.add_teardown(teardown, context);
		}
	}
}
