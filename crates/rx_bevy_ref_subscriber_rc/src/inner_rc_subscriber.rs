use std::ops::{Deref, DerefMut};

use rx_bevy_core::{
	Observer, ObserverInput, SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike,
	Teardown, Tick,
};
use short_type_name::short_type_name;

/// Internal to [RcSubscriber]
#[doc(hidden)]
pub struct InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	/// Starts from 1
	pub(crate) ref_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	pub(crate) completion_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	pub(crate) unsubscribe_count: usize,

	closed: bool,
}

impl<Destination> InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		let is_already_closed = destination.is_closed();
		Self {
			destination,
			ref_count: 1,
			completion_count: is_already_closed.into(),
			unsubscribe_count: is_already_closed.into(),
			closed: is_already_closed,
		}
	}

	pub fn unsubscribe_if_can(&mut self, context: &mut <Self as SignalContext>::Context) {
		if self.unsubscribe_count == self.ref_count && !self.closed {
			self.closed = true;
			self.destination.unsubscribe(context);
		}
	}

	pub fn complete_if_can(&mut self, context: &mut <Self as SignalContext>::Context) {
		if self.completion_count == self.ref_count && !self.closed {
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}
}

impl<Destination> Deref for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Target = Destination;

	fn deref(&self) -> &Self::Target {
		&self.destination
	}
}

impl<Destination> DerefMut for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.destination
	}
}

impl<Destination> ObserverInput for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.closed {
			self.destination.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.closed {
			self.destination.error(error, context);
			// An error immediately unsubscribes.
			self.closed = true;
			self.ref_count = 0;
			self.completion_count = 0;
			self.unsubscribe_count = 0;
			self.destination.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.complete_if_can(context);
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.closed {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.unsubscribe_if_can(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination> SubscriptionCollection for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<Destination> Drop for InnerRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	/// This should only happen when all counters reach 0.
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.destination.get_unsubscribe_context();
			self.destination.unsubscribe(&mut context);
		}

		debug_assert_eq!(
			self.ref_count,
			0,
			"The ref count of {} did not reach 0 on drop.",
			short_type_name::<Self>()
		);
		debug_assert_eq!(
			self.completion_count,
			0,
			"The completion count of {} did not reach 0 on drop.",
			short_type_name::<Self>()
		);
		debug_assert_eq!(
			self.unsubscribe_count,
			0,
			"The unsubscribe count of {} did not reach 0 on drop.",
			short_type_name::<Self>()
		);
	}
}
