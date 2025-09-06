use std::cell::RefCell;

use bevy_ecs::system::Commands;

use crate::{SignalContext, Subscriber};

pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
}

impl ObserverInput for () {
	type In = ();
	type InError = ();
}

// IDEA just pass the commands (in a wrapper so it's extensible) along the channels, so it's available.
/// To support non-owned references during next/error/complete/tick operations
/// TODO: How about not putting the contexts existence behind a feature flag, but only its content?
pub struct ChannelContext<'a, 'w, 's> {
	#[cfg(feature = "bevy")]
	pub commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> ChannelContext<'a, 'w, 's> {}

pub trait Observer: ObserverInput + SignalContext {
	fn next(&mut self, next: Self::In, context: &mut Self::Context);
	fn error(&mut self, error: Self::InError, context: &mut Self::Context);
	fn complete(&mut self, context: &mut Self::Context);

	/// Special fourth channel to process ticks issued by the schedulers.
	/// Some operators may produce other, new signals during a tick.
	/// None of the regular operators do anything on a tick but notify it's
	/// downstream of the tick.
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context);
}

/// TODO: CONSIDER turning, wherever this is needed this into simply a Into<Observer>
pub trait UpgradeableObserver: Observer {
	type Subscriber: Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
	fn upgrade(self) -> Self::Subscriber;
}

impl<T> UpgradeableObserver for T
where
	T: Subscriber,
{
	type Subscriber = Self;

	#[inline]
	fn upgrade(self) -> Self::Subscriber {
		self
	}
}

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
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.borrow_mut().next(next, context);
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.borrow_mut().error(error, context);
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.borrow_mut().complete(context);
	}

	#[cfg(feature = "tick")]
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		self.borrow_mut().tick(tick, context);
	}
}
