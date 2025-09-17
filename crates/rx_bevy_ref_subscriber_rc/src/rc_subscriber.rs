use std::ops::{Deref, DerefMut};

use rx_bevy_core::{
	ArcSubscriber, AssertSubscriptionClosedOnDrop, Observer, ObserverInput, Operation,
	SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

/// Internal to [RcSubscriber]
#[doc(hidden)]
pub struct RcDestination<Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	/// Starts from 1
	ref_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	completion_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	unsubscribe_count: usize,

	closed: bool,
}

impl<Destination> RcDestination<Destination>
where
	Destination: Subscriber,
{
	fn new(destination: Destination) -> Self {
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
			self.closed = true;
			self.destination.complete(context);
		}
	}
}

impl<Destination> Deref for RcDestination<Destination>
where
	Destination: Subscriber,
{
	type Target = Destination;

	fn deref(&self) -> &Self::Target {
		&self.destination
	}
}

impl<Destination> DerefMut for RcDestination<Destination>
where
	Destination: Subscriber,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.destination
	}
}

impl<Destination> ObserverInput for RcDestination<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for RcDestination<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for RcDestination<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.completion_count += 1;
			self.complete_if_can(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for RcDestination<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.unsubscribe_count += 1;
			self.unsubscribe_if_can(context);
		}
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination> SubscriptionCollection for RcDestination<Destination>
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

impl<Destination> Drop for RcDestination<Destination>
where
	Destination: Subscriber,
{
	/// This should only happen when all counters reach 0.
	fn drop(&mut self) {
		debug_assert_eq!(self.completion_count, 0);
		debug_assert_eq!(self.unsubscribe_count, 0);
		debug_assert_eq!(self.ref_count, 0);

		if !self.is_closed() {
			let mut context = self.destination.get_unsubscribe_context();
			self.destination.unsubscribe(&mut context);
		}
	}
}

pub struct RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	// TODO Instead of an Arc, all this should guarantee that the destination is cloneable and it still points to the same thing. This is true for entities aswell
	destination: ArcSubscriber<RcDestination<Destination>>,
	completed: bool,
	unsubscribed: bool,
}

impl<Destination> From<Destination> for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination> RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: ArcSubscriber::new(RcDestination::new(destination)),
			completed: false,
			unsubscribed: false,
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&RcDestination<Destination>),
	{
		self.destination.read(reader);
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn write<F>(&mut self, writer: F)
	where
		F: FnMut(&mut RcDestination<Destination>),
	{
		self.destination.write(writer);
	}

	/// Acquire a clone to the same reference which will not interact with
	/// the reference counts, and only attempts to complete or unsubscribe it
	/// when it too completes or unsubscribes. And can still be used as a
	/// subscriber
	pub fn downgrade(&self) -> WeakRcSubscriber<Destination> {
		WeakRcSubscriber {
			destination: self.destination.clone(),
			closed: self.completed || self.unsubscribed,
		}
	}
}

impl<Destination> Clone for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		self.destination.write(|destination| {
			destination.ref_count += 1;

			if self.completed {
				destination.completion_count += 1;
			}

			if self.unsubscribed {
				destination.unsubscribe_count += 1;
			}
		});

		Self {
			completed: self.completed,
			unsubscribed: self.completed,
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.completed = true;
			self.complete(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		let is_this_clone_closed = self.completed || self.unsubscribed;

		is_this_clone_closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.unsubscribed = true;
			self.destination.unsubscribe(context);
		}
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination> SubscriptionCollection for RcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<Destination> Drop for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		self.destination.write(|destination| {
			destination.ref_count -= 1;

			if self.completed {
				destination.completion_count -= 1;
			}

			if self.unsubscribed {
				destination.unsubscribe_count -= 1;
			}
		});

		self.assert_closed_when_dropped();

		//  lock.complete_if_can();
		//  lock.unsubscribe_if_can();
	}
}

impl<Destination> Operation for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = ArcSubscriber<RcDestination<Destination>>;
}

/// Acquired by calling `downgrade` on `RcSubscriber`
pub struct WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	// TODO: Since in bevy this won't be a pointer just an Entity, maybe we'd need a enum or trait here
	destination: ArcSubscriber<RcDestination<Destination>>,
	closed: bool,
}

impl<Destination> WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&RcDestination<Destination>),
	{
		self.destination.read(reader);
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn write<F>(&mut self, writer: F)
	where
		F: FnMut(&mut RcDestination<Destination>),
	{
		self.destination.write(writer);
	}
}

impl<Destination> Clone for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		Self {
			closed: self.closed,
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.closed = true;
			self.destination.unsubscribe(context);
		}
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination> SubscriptionCollection for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<Destination> Drop for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.destination.write(|destination| {
				let mut context = destination.get_unsubscribe_context();
				destination.complete_if_can(&mut context);
				destination.unsubscribe_if_can(&mut context);
			});
		}
	}
}

impl<Destination> Operation for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Destination;
}
