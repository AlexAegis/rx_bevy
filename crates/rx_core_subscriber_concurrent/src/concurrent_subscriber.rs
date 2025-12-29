use core::num::NonZero;
use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{HigherOrderInnerSubscriber, HigherOrderSubscriberState};
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, Signal, Subscriber, SubscriptionData,
	SubscriptionHandle, SubscriptionLike, Teardown, TeardownCollection,
	TeardownCollectionExtension,
};
use slab::Slab;

use crate::concurrent_subscriber_queue::ConcurrentSubscriberQueue;

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
	state: Arc<Mutex<HigherOrderSubscriberState<ConcurrentSubscriberQueue<InnerObservable>>>>,
	inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
	concurrency_limit: NonZero<usize>,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub fn new(destination: Destination, concurrency_limit: NonZero<usize>) -> Self {
		let shared_destination = Arc::new(Mutex::new(destination));

		let state = Arc::new(Mutex::new(HigherOrderSubscriberState::default()));
		let inner_subscriptions = Arc::new(Mutex::new(Slab::new()));

		Self {
			outer_teardown: SubscriptionHandle::default(),
			shared_destination,
			state,
			inner_subscriptions,
			concurrency_limit,
			_phantom_data: PhantomData,
		}
	}
}

pub(crate) fn subscribe_to_next_in_queue<InnerObservable, Destination>(
	state: Arc<Mutex<HigherOrderSubscriberState<ConcurrentSubscriberQueue<InnerObservable>>>>,
	inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
	shared_destination: Arc<Mutex<Destination>>,
	outer_teardown: SubscriptionHandle,
	concurrency_limit: NonZero<usize>,
) where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	let mut state_lock = state.lock_ignore_poison();

	if let Some(next) = state_lock.state.queue.pop_front() {
		state_lock.non_completed_subscriptions += 1;
		state_lock.non_unsubscribed_subscriptions += 1;
		drop(state_lock);

		create_inner_subscription(
			next,
			state.clone(),
			inner_subscriptions,
			shared_destination,
			outer_teardown,
			concurrency_limit,
		);
	}
}

pub(crate) fn create_inner_subscription<InnerObservable, Destination>(
	mut next_observable: InnerObservable,
	state: Arc<Mutex<HigherOrderSubscriberState<ConcurrentSubscriberQueue<InnerObservable>>>>,
	inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
	mut shared_destination: Arc<Mutex<Destination>>,
	mut outer_teardown: SubscriptionHandle,
	concurrency_limit: NonZero<usize>,
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

	let state_on_complete_clone = state.clone();
	let shared_destination_on_complete_clone = shared_destination.clone();
	let inner_subscriptions_on_complete_clone = inner_subscriptions.clone();
	let outer_teardown_on_complete_clone = outer_teardown.clone();

	let inner_subscriptions_clone = inner_subscriptions.clone();

	let concurrency_limit_minus_one: usize = usize::from(concurrency_limit) - 1;

	let next_subscription = next_observable.subscribe(HigherOrderInnerSubscriber::new(
		key,
		shared_destination.clone(),
		state.clone(),
		move |_key| {
			subscribe_to_next_in_queue(
				state_on_complete_clone,
				inner_subscriptions_on_complete_clone,
				shared_destination_on_complete_clone,
				outer_teardown_on_complete_clone,
				concurrency_limit,
			);
		},
		move |key| {
			let no_more_subscribers_besides_this = {
				let mut inner_subscriptions = inner_subscriptions.lock_ignore_poison();
				inner_subscriptions.retain(|_k, s| !s.is_closed());
				inner_subscriptions
					.iter()
					.filter(|(k, _)| *k != key)
					.count() == concurrency_limit_minus_one
			};

			if no_more_subscribers_besides_this {
				subscribe_to_next_in_queue(
					state.clone(),
					inner_subscriptions.clone(),
					shared_destination.clone(),
					outer_teardown.clone(),
					concurrency_limit,
				);
			}

			// TODO: UNSURE
			if state
				.lock_ignore_poison()
				.inner_unsubscribe_can_downstream()
			{
				for (other_key, inner_subscription) in
					inner_subscriptions.lock_ignore_poison().iter_mut()
				{
					// Must not unsubscribe itself, as we're in the middle of
					// unsubscribing! It would lock up!
					if other_key != key && !inner_subscription.is_closed() {
						inner_subscription.unsubscribe();
					}
				}

				// Close downstream
				shared_destination.unsubscribe();
				outer_teardown.unsubscribe();
			}
		},
	));

	if !next_subscription.is_closed() {
		let mut slab = inner_subscriptions_clone.lock_ignore_poison();
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

			if state.non_completed_subscriptions < self.concurrency_limit.into() {
				state.non_completed_subscriptions += 1;
				state.non_unsubscribed_subscriptions += 1;
				drop(state);

				create_inner_subscription(
					next,
					self.state.clone(),
					self.inner_subscriptions.clone(),
					self.shared_destination.clone(),
					self.outer_teardown.clone(),
					self.concurrency_limit,
				);
			} else {
				state.state.queue.push_back(next);
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.state.lock_ignore_poison().upstream_error();

			self.shared_destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed()
			&& self
				.state
				.lock_ignore_poison()
				.upstream_complete_can_downstream()
		{
			self.shared_destination.complete();
			self.unsubscribe();
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
		if !self.is_closed()
			&& self
				.state
				.lock_ignore_poison()
				.upstream_unsubscribe_can_downstream()
		{
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
