use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::{ObserverInput, SignalBound, SignalContext, Subscriber, WithContext};

impl<'a, In, InError, Context> WithContext
	for RwLockReadGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
}

impl<'a, In, InError, Context> ObserverInput
	for RwLockReadGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type In = In;
	type InError = InError;
}

impl<'a, In, InError, Context> WithContext
	for RwLockWriteGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
}

impl<'a, In, InError, Context> ObserverInput
	for RwLockWriteGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type In = In;
	type InError = InError;
}
