use std::sync::{Arc, RwLock};

use crate::{
	Observer, ObserverInput, SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike,
	Teardown,
};

impl<Destination> ObserverInput for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.next(next, context);
		}
	}

	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.error(error, context);
		}
	}

	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.complete(context);
		}
	}

	fn tick<'c>(&mut self, tick: crate::Tick, context: &mut Self::Context<'c>) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.tick(tick, context);
		}
	}
}

impl<Destination> SignalContext for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	type Context<'c> = Destination::Context<'c>;
}

impl<Destination> SubscriptionLike for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		let lock = self.read().expect("lock is poisoned!");
		lock.is_closed()
	}

	fn unsubscribe<'c>(&mut self, context: &mut Destination::Context<'c>) {
		let mut lock = self.write().expect("lock is poisoned!");
		lock.unsubscribe(context);
	}
}

impl<Destination> SubscriptionCollection for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Destination::Context<'c>>>,
		context: &mut Destination::Context<'c>,
	) {
		let mut lock = self.write().expect("lock is poisoned!");
		lock.add(subscription, context);
	}
}
