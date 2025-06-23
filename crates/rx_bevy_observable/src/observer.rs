use crate::Subscriber;

pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
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
