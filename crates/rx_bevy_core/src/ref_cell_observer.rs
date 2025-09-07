use std::cell::RefCell;

use crate::{Observer, ObserverInput, SignalContext};

impl<T> ObserverInput for RefCell<T>
where
	T: ObserverInput,
{
	type In = T::In;
	type InError = T::InError;
}

impl<T> SignalContext for RefCell<T>
where
	T: Observer,
{
	type Context<'c> = T::Context<'c>;
}

impl<T> Observer for RefCell<T>
where
	T: Observer,
{
	#[inline]
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		self.borrow_mut().next(next, context);
	}

	#[inline]
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.borrow_mut().error(error, context);
	}

	#[inline]
	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.borrow_mut().complete(context);
	}

	#[inline]
	fn tick<'c>(&mut self, tick: crate::Tick, context: &mut Self::Context<'c>) {
		self.borrow_mut().tick(tick, context);
	}
}
