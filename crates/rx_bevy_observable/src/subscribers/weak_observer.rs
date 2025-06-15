use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::{Observer, ObserverInput};

pub struct WeakObserver<Destination>
where
	Destination: Observer,
{
	destination: Weak<RefCell<Destination>>,
}

impl<Destination> WeakObserver<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		let w = Rc::new(RefCell::new(destination));
		Self {
			destination: Rc::<RefCell<Destination>>::downgrade(&w),
		}
	}
}

impl<Destination> ObserverInput for WeakObserver<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for WeakObserver<Destination>
where
	Destination: Observer,
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
