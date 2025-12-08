use std::ops::{Deref, DerefMut};

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Signal, Subscriber,
	SubscriptionLike, Teardown, TeardownCollection, WithPrimaryCategory,
};

impl<In, InError> Observer for Box<dyn Subscriber<In = In, InError = InError>>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, next: Self::In) {
		self.deref_mut().next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.deref_mut().error(error);
	}

	fn complete(&mut self) {
		self.deref_mut().complete();
	}
}

impl<In, InError> ObserverInput for Box<dyn Subscriber<In = In, InError = InError>>
where
	In: Signal,
	InError: Signal,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> WithPrimaryCategory for Box<dyn Subscriber<In = In, InError = InError>>
where
	In: Signal,
	InError: Signal,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError> ObserverUpgradesToSelf for Box<dyn Subscriber<In = In, InError = InError>>
where
	In: Signal,
	InError: Signal,
{
}

impl<In, InError> SubscriptionLike for Box<dyn Subscriber<In = In, InError = InError>>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.deref().is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.deref_mut().unsubscribe();
	}
}

impl<In, InError> TeardownCollection for Box<dyn Subscriber<In = In, InError = InError>>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.deref_mut().add_teardown(teardown);
	}
}
