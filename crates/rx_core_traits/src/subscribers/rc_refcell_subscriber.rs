use std::{cell::RefCell, rc::Rc};

use crate::{
	Observer, ObserverInput, Subscriber, SubscriptionLike, Tickable,
	SubscriptionContext, WithSubscriptionContext,
};

impl<S> WithSubscriptionContext for Rc<RefCell<S>>
where
	S: Subscriber,
{
	type Context = S::Context;
}

impl<S> ObserverInput for Rc<RefCell<S>>
where
	S: Subscriber,
{
	type In = S::In;
	type InError = S::InError;
}

impl<S> Observer for Rc<RefCell<S>>
where
	S: Subscriber,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.borrow_mut().next(next, context);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.borrow_mut().error(error, context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.borrow_mut().complete(context);
		}
	}
}

impl<S> Tickable for Rc<RefCell<S>>
where
	S: Subscriber,
{
	fn tick(
		&mut self,
		tick: crate::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.borrow_mut().tick(tick, context);
	}
}

impl<S> SubscriptionLike for Rc<RefCell<S>>
where
	S: Subscriber,
{
	fn is_closed(&self) -> bool {
		self.borrow().is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.borrow_mut().unsubscribe(context);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: crate::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.borrow_mut().add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}
