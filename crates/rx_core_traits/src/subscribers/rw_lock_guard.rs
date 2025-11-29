use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::{ObserverInput, Signal, Subscriber, SubscriptionContext, WithSubscriptionContext};

impl<'a, In, InError, Context> WithSubscriptionContext
	for RwLockReadGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<'a, In, InError, Context> ObserverInput
	for RwLockReadGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<'a, In, InError, Context> WithSubscriptionContext
	for RwLockWriteGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<'a, In, InError, Context> ObserverInput
	for RwLockWriteGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}
