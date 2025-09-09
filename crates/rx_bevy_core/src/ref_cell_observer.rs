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
	type Context = T::Context;
}

impl<T> Observer for RefCell<T>
where
	T: Observer,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.borrow_mut().next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.borrow_mut().error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.borrow_mut().complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		self.borrow_mut().tick(tick, context);
	}
}
