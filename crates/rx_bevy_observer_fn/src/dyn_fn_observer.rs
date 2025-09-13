use std::marker::PhantomData;

use rx_bevy_core::{
	InnerSubscription, Observer, ObserverInput, SignalContext, SubscriptionCollection,
	SubscriptionLike, Tick,
};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error, Context> {
	on_next: Option<Box<dyn FnMut(In)>>,
	on_error: Option<Box<dyn FnMut(Error)>>,
	on_complete: Option<Box<dyn FnOnce()>>,
	on_tick: Option<Box<dyn FnMut(Tick)>>,

	on_unsubscribe: Option<Box<dyn FnOnce()>>,

	inner_subscription: InnerSubscription<Context>,

	_phantom_data: PhantomData<Context>,
}

impl<In, InError, Context> ObserverInput for DynFnObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> SignalContext for DynFnObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for DynFnObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	fn next(&mut self, next: In, _context: &mut Self::Context) {
		if !self.is_closed()
			&& let Some(on_next) = &mut self.on_next
		{
			(on_next)(next);
		}
	}

	fn error(&mut self, error: InError, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(on_error) = &mut self.on_error {
				(on_error)(error);
			} else {
				panic!("DynFnObserver without an error observer encountered an error!");
			}

			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(on_complete) = self.on_complete.take() {
				(on_complete)();
			}

			self.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: rx_bevy_core::Tick, _context: &mut Self::Context) {
		if !self.is_closed()
			&& let Some(on_tick) = &mut self.on_tick
		{
			(on_tick)(tick);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for DynFnObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	fn is_closed(&self) -> bool {
		self.inner_subscription.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if let Some(on_unsubscribe) = self.on_unsubscribe.take() {
			(on_unsubscribe)();
		}
		self.inner_subscription.unsubscribe(context);
	}
}

impl<In, InError, Context> SubscriptionCollection for DynFnObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	fn add<S: 'static + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: impl Into<S>,
		context: &mut Context,
	) {
		self.inner_subscription.add(subscription, context);
	}
}

impl<In, InError, Context> Default for DynFnObserver<In, InError, Context> {
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
			on_tick: None,
			on_unsubscribe: None,
			inner_subscription: InnerSubscription::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> DynFnObserver<In, InError, Context> {
	pub fn with_next<OnPush: 'static + FnMut(In)>(self, next: OnPush) -> Self {
		Self {
			on_next: Some(Box::new(next)),
			..self
		}
	}

	pub fn with_error<OnError: 'static + FnMut(InError)>(self, error: OnError) -> Self {
		Self {
			on_error: Some(Box::new(error)),
			..self
		}
	}

	pub fn with_complete<OnComplete: 'static + FnOnce()>(self, complete: OnComplete) -> Self {
		Self {
			on_complete: Some(Box::new(complete)),
			..self
		}
	}

	pub fn with_unsubscribe<OnUnsubscribe: 'static + FnOnce()>(
		self,
		on_unsubscribe: OnUnsubscribe,
	) -> Self {
		Self {
			on_unsubscribe: Some(Box::new(on_unsubscribe)),
			..self
		}
	}
}
