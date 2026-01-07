use std::ops::{Deref, DerefMut};

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Signal, Subscriber,
	SubscriptionLike, Teardown, TeardownCollection, WithPrimaryCategory,
};

pub type BoxedSubscriber<In, InError> =
	Box<dyn 'static + Subscriber<In = In, InError = InError> + Send + Sync>;

impl<In, InError, S> Observer for Box<S>
where
	In: Signal,
	InError: Signal,
	S: ?Sized + Subscriber<In = In, InError = InError> + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.deref_mut().next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.deref_mut().error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.deref_mut().complete();
	}
}

impl<In, InError, S> ObserverInput for Box<S>
where
	In: Signal,
	InError: Signal,
	S: ?Sized + Subscriber<In = In, InError = InError> + Send + Sync,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, S> WithPrimaryCategory for Box<S>
where
	In: Signal,
	InError: Signal,
	S: ?Sized + Subscriber<In = In, InError = InError> + Send + Sync,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError, S> ObserverUpgradesToSelf for Box<S>
where
	In: Signal,
	InError: Signal,
	S: ?Sized + Subscriber<In = In, InError = InError> + Send + Sync,
{
}

impl<In, InError, S> SubscriptionLike for Box<S>
where
	In: Signal,
	InError: Signal,
	S: ?Sized + Subscriber<In = In, InError = InError> + Send + Sync,
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

impl<In, InError, S> TeardownCollection for Box<S>
where
	In: Signal,
	InError: Signal,
	S: ?Sized + Subscriber<In = In, InError = InError> + Send + Sync,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.deref_mut().add_teardown(teardown);
	}
}
