use std::ops::{Deref, DerefMut};

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, SignalBound,
	Subscriber, SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection, Tick,
	Tickable, WithPrimaryCategory, WithSubscriptionContext,
};

impl<In, InError, Context> Observer
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.deref_mut().next(next, context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.deref_mut().error(error, context);
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.deref_mut().complete(context);
	}
}

impl<In, InError, Context> WithSubscriptionContext
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> ObserverInput
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithPrimaryCategory
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError, Context> ObserverUpgradesToSelf
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
}

impl<In, InError, Context> Tickable
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.deref_mut().tick(tick, context);
	}
}

impl<In, InError, Context> SubscriptionLike
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.deref().is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.deref_mut().unsubscribe(context);
	}
}

impl<In, InError, Context> TeardownCollection
	for Box<dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.deref_mut().add_teardown(teardown, context);
	}
}
