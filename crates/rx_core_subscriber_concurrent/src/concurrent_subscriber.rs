use core::num::NonZero;
use std::{
	marker::PhantomData,
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, Ordering},
	},
};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, Signal, Subscriber, SubscriptionData,
	SubscriptionHandle, SubscriptionLike, Teardown, TeardownCollection,
	TeardownCollectionExtension,
};
use slab::Slab;

use crate::internal::{ConcurrentSubscriberInner, ConcurrentSubscriberState};

#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
pub struct ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	outer_teardown: SubscriptionHandle,
	shared_destination: Arc<Mutex<Destination>>,
	upstream_completed: Arc<AtomicBool>,
	state: Arc<Mutex<ConcurrentSubscriberState<InnerObservable>>>,
	inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub fn new(destination: Destination, concurrency_limit: NonZero<usize>) -> Self {
		let shared_destination = Arc::new(Mutex::new(destination));
		let upstream_completed = Arc::new(AtomicBool::new(false));
		let state = Arc::new(Mutex::new(ConcurrentSubscriberState::new(
			concurrency_limit,
		)));
		let inner_subscriptions = Arc::new(Mutex::new(Slab::new()));

		Self {
			outer_teardown: SubscriptionHandle::default(),
			upstream_completed,
			shared_destination,
			state,
			inner_subscriptions,
			_phantom_data: PhantomData,
		}
	}
}

pub(crate) fn create_inner_subscription<InnerObservable, Destination>(
	mut next_observable: InnerObservable,
	state: Arc<Mutex<ConcurrentSubscriberState<InnerObservable>>>,
	inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
	shared_destination: Arc<Mutex<Destination>>,
	outer_teardown: SubscriptionHandle,
) where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	let key = {
		let mut slab = inner_subscriptions.lock_ignore_poison();
		let entry = slab.vacant_entry();
		let key = entry.key();
		entry.insert(SubscriptionData::default());
		key
	};

	let next_subscription =
		next_observable.subscribe(ConcurrentSubscriberInner::<InnerObservable, _>::new(
			key,
			shared_destination.clone(),
			state,
			inner_subscriptions.clone(),
			outer_teardown,
		));

	if !next_subscription.is_closed() {
		let mut slab = inner_subscriptions.lock_ignore_poison();
		slab.get_mut(key).unwrap().add(next_subscription);
	}
}

impl<InnerObservable, Destination> Observer for ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let mut state = self.state.lock_ignore_poison();

			if state.non_completed_subscriptions < state.concurrency_limit.into() {
				state.non_completed_subscriptions += 1;
				state.non_unsubscribed_subscriptions += 1;
				drop(state);

				create_inner_subscription(
					next,
					self.state.clone(),
					self.inner_subscriptions.clone(),
					self.shared_destination.clone(),
					self.outer_teardown.clone(),
				);
			} else {
				state.queue.push_back(next);
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			{
				let mut state = self.state.lock_ignore_poison();
				state.upstream_unsubscribed = true;
			}
			self.shared_destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.upstream_completed.store(true, Ordering::Relaxed);

			let mut state = self.state.lock_ignore_poison();
			state.upstream_completed = true;
			state.upstream_unsubscribed = true;
			if state.can_downstream_complete() {
				drop(state);
				self.shared_destination.complete();
				self.unsubscribe();
			}
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.outer_teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			let mut state = self.state.lock_ignore_poison();
			state.upstream_unsubscribed = true;
			if state.can_downstream_unsubscribe() {
				drop(state);

				self.outer_teardown.unsubscribe();

				let inner_subscriptions = self
					.inner_subscriptions
					.lock_ignore_poison()
					.drain()
					.collect::<Vec<_>>();

				for mut inner_subscription in inner_subscriptions {
					inner_subscription.unsubscribe();
				}

				self.shared_destination.unsubscribe();
			}
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		self.outer_teardown.add(teardown);
	}
}
