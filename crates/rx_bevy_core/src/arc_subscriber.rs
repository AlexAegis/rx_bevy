use std::sync::{Arc, RwLock};

use crate::{Observer, ObserverInput, Subscriber, SubscriptionLike};

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
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.complete();
		}
	}

	#[cfg(feature = "tick")]
	fn tick(&mut self, tick: crate::Tick) {
		if !self.is_closed() {
			let mut lock = self.write().expect("lock is poisoned!");
			lock.tick(tick);
		}
	}
}

impl<Destination> SubscriptionLike for Arc<RwLock<Destination>>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		let lock = self.read().expect("lock is poisoned!");
		lock.is_closed()
	}

	fn unsubscribe(&mut self) {
		let mut lock = self.write().expect("lock is poisoned!");
		lock.unsubscribe();
	}

	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		let mut lock = self.write().expect("lock is poisoned!");
		lock.add(subscription);
	}
}
