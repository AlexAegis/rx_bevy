use std::cell::RefCell;

use crate::Subscriber;

pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
}

impl ObserverInput for () {
	type In = ();
	type InError = ();
}

pub trait Observer: ObserverInput {
	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::InError);
	fn complete(&mut self);
}

pub trait UpgradeableObserver: Observer {
	type Subscriber: Subscriber<In = Self::In, InError = Self::InError>;
	fn upgrade(self) -> Self::Subscriber;
}

impl<T> UpgradeableObserver for T
where
	T: Subscriber,
{
	type Subscriber = Self;

	#[inline]
	fn upgrade(self) -> Self::Subscriber {
		self
	}
}

impl<T> ObserverInput for RefCell<T>
where
	T: ObserverInput,
{
	type In = T::In;
	type InError = T::InError;
}

impl<T> Observer for RefCell<T>
where
	T: Observer,
{
	fn next(&mut self, next: Self::In) {
		self.borrow_mut().next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.borrow_mut().error(error);
	}

	fn complete(&mut self) {
		self.borrow_mut().complete();
	}
}
