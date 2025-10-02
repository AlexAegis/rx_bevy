use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::{DropContext, ObserverInput, SignalContext, Subscriber};

impl<'a, In, InError, Context> SignalContext
	for RwLockReadGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
}

impl<'a, In, InError, Context> ObserverInput
	for RwLockReadGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<'a, In, InError, Context> SignalContext
	for RwLockWriteGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
}

impl<'a, In, InError, Context> ObserverInput
	for RwLockWriteGuard<'a, dyn Subscriber<In = In, InError = InError, Context = Context>>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}
