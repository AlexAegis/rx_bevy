use std::sync::{Arc, RwLock};

use crate::{
	ExpandableSubscriptionLike, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown,
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
	type Context = <Destination as Observer>::Context;

	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.complete(context);
		}
	}

	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike<<Destination as Observer>::Context> for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		let lock = self.read().expect("lock is poisoned!");
		lock.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Destination as Observer>::Context) {
		let mut lock = self.write().expect("lock is poisoned!");
		lock.unsubscribe(context);
	}
}

impl<Destination> ExpandableSubscriptionLike<<Destination as Observer>::Context>
	for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	fn add(
		&mut self,
		subscription: impl Into<Teardown<<Destination as Observer>::Context>>,
		context: &mut <Destination as Observer>::Context,
	) {
		let mut lock = self.write().expect("lock is poisoned!");
		lock.add(subscription, context);
	}
}
