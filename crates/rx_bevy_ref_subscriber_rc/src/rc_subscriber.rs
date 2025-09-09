use std::{
	ops::{Deref, DerefMut},
	sync::{Arc, RwLock},
};

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
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
	fn is_closed(&self) -> bool {
		self.closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.unsubscribe_count += 1;
			self.unsubscribe_if_can(context);
		}
	}
}

impl<Destination> SubscriptionCollection for RcDestination<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<Destination> Drop for RcDestination<Destination>
where
	Destination: Subscriber,
{
	/// When dropped, the reference counts don't matter, the destination has to be unsubscribed.
	/// This should only happen when all counters reach 0.
	fn drop(&mut self) {
		debug_assert_eq!(self.completion_count, 0);
		debug_assert_eq!(self.unsubscribe_count, 0);
		debug_assert_eq!(self.ref_count, 0);

		if !self.closed {
			panic!("Dropped without unsubscribing!");
		}
		// self.destination.unsubscribe();
	}
}

pub struct RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	// TODO Instead of an Arc, all this should guarantee that the destination is cloneable and it still points to the same thing. This is true for entities aswell
	destination: Arc<RwLock<RcDestination<Destination>>>,
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
			destination: Arc::new(RwLock::new(RcDestination::new(destination))),
			completed: false,
			unsubscribed: false,
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&RcDestination<Destination>),
	{
		reader(&self.destination.read().expect("poisoned"))
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn write<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut RcDestination<Destination>),
	{
		writer(&mut self.destination.write().expect("poisoned"))
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
		let mut destination = self.destination.write().expect("lock is poisoned!");
		destination.ref_count += 1;

		if self.completed {
			destination.completion_count += 1;
		}

		if self.unsubscribed {
			destination.unsubscribe_count += 1;
		}

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
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.completed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.complete(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.completed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		let is_this_clone_closed = self.completed || self.unsubscribed;

		is_this_clone_closed || {
			let lock = self.destination.read().expect("lock is poisoned");
			lock.is_closed()
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.unsubscribed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");

			lock.unsubscribe(context);
		}
	}
}

impl<Destination> SubscriptionCollection for RcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		let mut lock = self.destination.write().expect("lock is poisoned!");
		lock.add(subscription, context);
	}
}

impl<Destination> Drop for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		let mut lock = self.destination.write().expect("lock is poisoned!");

		lock.ref_count -= 1;

		if self.completed {
			lock.completion_count -= 1;
		}

		if self.unsubscribed {
			lock.unsubscribe_count -= 1;
		}

		self.assert_closed_when_dropped();

		//  lock.complete_if_can();
		//  lock.unsubscribe_if_can();
	}
}

impl<Destination> Operation for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Arc<RwLock<RcDestination<Destination>>>;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}

/// Acquired by calling `downgrade` on `RcSubscriber`
pub struct WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	// TODO: Since in bevy this won't be a pointer just an Entity, maybe we'd need a enum or trait here
	destination: Arc<RwLock<RcDestination<Destination>>>,
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
		if let Ok(destination) = self.destination.try_read() {
			reader(&destination)
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut RcDestination<Destination>),
	{
		if let Ok(mut destination) = self.destination.try_write() {
			reader(&mut destination)
		}
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
		if !self.is_closed()
			&& let Ok(mut lock) = self.destination.try_write()
		{
			lock.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.try_write() {
				lock.error(error, context);
			}
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.destination.try_write() {
				lock.complete(context);
			}
			self.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut lock) = self.destination.try_write()
		{
			lock.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.closed = true;
			if let Ok(mut lock) = self.destination.try_write() {
				lock.unsubscribe(context);
			}
		}
	}
}

impl<Destination> SubscriptionCollection for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		if let Ok(mut lock) = self.destination.try_write() {
			lock.add(subscription, context);
		}
	}
}

impl<Destination> Drop for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		//if let Ok(mut lock) = self.destination.try_write() {
		//	lock.complete_if_can();
		//	lock.unsubscribe_if_can();
		//}
	}
}

impl<Destination> Operation for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Arc<RwLock<RcDestination<Destination>>>;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}
