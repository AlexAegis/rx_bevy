use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::{Observer, ObserverInput, Subscriber, SubscriptionLike};

pub struct WeakSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Weak<RefCell<Destination>>,
}

impl<Destination> WeakSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		let w = Rc::new(RefCell::new(destination));
		Self {
			destination: Rc::<RefCell<Destination>>::downgrade(&w),
		}
	}
}

impl<Destination> ObserverInput for WeakSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for WeakSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if let Some(destination) = self.destination.upgrade() {
			destination.borrow_mut().next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if let Some(destination) = self.destination.upgrade() {
			destination.borrow_mut().error(error);
		}
	}

	fn complete(&mut self) {
		if let Some(destination) = self.destination.upgrade() {
			destination.borrow_mut().complete();
		}
	}
}

impl<Destination> SubscriptionLike for WeakSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		if let Some(destination) = self.destination.upgrade() {
			destination.borrow().is_closed()
		} else {
			true
		}
	}

	fn unsubscribe(&mut self) {
		if let Some(destination) = self.destination.upgrade() {
			destination.borrow_mut().unsubscribe();
		}
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		if let Some(destination) = self.destination.upgrade() {
			destination.borrow_mut().add(subscription);
		}
	}
}

impl<Destination> Drop for WeakSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe()
	}
}
