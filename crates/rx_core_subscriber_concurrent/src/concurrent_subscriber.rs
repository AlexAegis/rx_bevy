use core::num::NonZero;
use std::{
	collections::VecDeque,
	sync::{Arc, Mutex},
};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, SharedSubscriber, Signal, Subscriber,
	SubscriptionClosedFlag, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection,
	TeardownCollectionExtension,
};

struct ConcurrentSubscriberData<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	destination: SharedSubscriber<Destination>,
	observable_queue: VecDeque<InnerObservable>,
	upstream_completed: bool,
	downstream_completed: bool,
	active_subscriptions: usize,
	concurrency_limit: NonZero<usize>,
	waits_for_completion: usize,
	shared_outer_teardown: Arc<Mutex<SubscriptionData>>,
}

impl<InnerObservable, Destination> ConcurrentSubscriberData<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub(crate) fn new(
		destination: SharedSubscriber<Destination>,
		concurrency_limit: NonZero<usize>,
		shared_outer_teardown: Arc<Mutex<SubscriptionData>>,
	) -> Self {
		Self {
			destination,
			waits_for_completion: 0,
			active_subscriptions: 0,
			concurrency_limit,
			observable_queue: VecDeque::new(),
			upstream_completed: false,
			downstream_completed: false,
			shared_outer_teardown,
		}
	}

	pub(crate) fn try_complete(&mut self) {
		if self.active_subscriptions == 0
			&& self.waits_for_completion == 0
			&& self.observable_queue.is_empty()
			&& self.upstream_completed
			&& !self.downstream_completed
		{
			self.downstream_completed = true;
			self.destination.complete();
			self.unsubscribe();
		}
	}

	pub(crate) fn unsubscribe(&mut self) {
		self.observable_queue.clear();
		self.destination.unsubscribe();
	}
}

#[derive(RxSubscriber)]
#[rx_in(InnerObservable::Out)]
#[rx_in_error(InnerObservable::OutError)]
struct ConcurrentInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	is_finished: bool,
	shared_upstream_unsubscribed_flag: Arc<Mutex<bool>>,
	data: Arc<Mutex<ConcurrentSubscriberData<InnerObservable, Destination>>>,
}

impl<InnerObservable, Destination> ConcurrentInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn new(
		data: Arc<Mutex<ConcurrentSubscriberData<InnerObservable, Destination>>>,
		shared_upstream_unsubscribed_flag: Arc<Mutex<bool>>,
	) -> Self {
		Self {
			data,
			shared_upstream_unsubscribed_flag,
			is_finished: false,
		}
	}
}

impl<InnerObservable, Destination> Observer
	for ConcurrentInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.destination.unsubscribe());
			lock.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.is_finished = true;
			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.destination.unsubscribe());
			lock.destination.error(error);
			lock.destination.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			{
				let mut lock = self
					.data
					.lock_with_poison_behavior(|inner| inner.destination.unsubscribe());
				lock.waits_for_completion -= 1;
			}
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for ConcurrentInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		let mut lock = self
			.data
			.lock_with_poison_behavior(|inner| inner.destination.unsubscribe());
		lock.destination.add_teardown(teardown);
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for ConcurrentInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		if self.is_finished {
			return true;
		}

		let lock = self
			.data
			.lock_with_poison_behavior(|inner| inner.unsubscribe());
		lock.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.is_finished = true;

			// When upstream unsubscribes, it holds a lock to `data` which would
			// deadlock this unsubscribe call. Luckily when upstream
			// unsubscribes, there's no need for locking `data` here, as
			// no new subscriptions should be made anyway.
			if *self.shared_upstream_unsubscribed_flag.lock_ignore_poison() {
				return;
			}

			let next_observable = {
				let mut lock = self
					.data
					.lock_with_poison_behavior(|inner| inner.unsubscribe());
				lock.waits_for_completion += 1; // Assume the next observable can complete immediately
				lock.observable_queue.pop_front()
			};

			let next_subscription = if let Some(mut next_observable) = next_observable {
				let subscription = next_observable.subscribe(ConcurrentInnerSubscriber::new(
					self.data.clone(),
					self.shared_upstream_unsubscribed_flag.clone(),
				));

				Some(subscription)
			} else {
				None
			};

			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.unsubscribe());

			if let Some(subscription) = next_subscription {
				lock.shared_outer_teardown.add(subscription);
			} else {
				lock.waits_for_completion -= 1;
				lock.active_subscriptions -= 1;
			}
			lock.try_complete();
		}
	}
}

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
pub struct ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	shared_teardown: Arc<Mutex<SubscriptionData>>,
	data: Arc<Mutex<ConcurrentSubscriberData<InnerObservable, Destination>>>,
	shared_upstream_unsubscribe_flag: Arc<Mutex<bool>>,
	closed_flag: SubscriptionClosedFlag,
}

impl<InnerObservable, Destination> ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub fn new(destination: Destination, concurrency_limit: NonZero<usize>) -> Self {
		let destination = SharedSubscriber::new(destination);
		let shared_teardown = Arc::new(Mutex::new(SubscriptionData::default()));
		let shared_upstream_unsubscribe_flag = Arc::new(Mutex::new(false));

		Self {
			data: Arc::new(Mutex::new(ConcurrentSubscriberData::new(
				destination,
				concurrency_limit,
				shared_teardown.clone(),
			))),
			shared_upstream_unsubscribe_flag,
			shared_teardown,
			closed_flag: false.into(),
		}
	}
}

impl<InnerObservable, Destination> Observer for ConcurrentSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.unsubscribe());
			if lock.active_subscriptions < lock.concurrency_limit.into() {
				lock.active_subscriptions += 1;
				lock.waits_for_completion += 1;
				drop(lock);
				let subscription = next.subscribe(ConcurrentInnerSubscriber::new(
					self.data.clone(),
					self.shared_upstream_unsubscribe_flag.clone(),
				));
				self.shared_teardown.add_teardown(subscription.into());
			} else {
				lock.observable_queue.push_back(next);
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.unsubscribe());
			lock.destination.error(error);
			lock.destination.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.unsubscribe());
			lock.upstream_completed = true;
			lock.try_complete();
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
		self.closed_flag.is_closed()
	}

	fn unsubscribe(&mut self) {
		// An upstream unsubscribe stops everything!
		if !self.is_closed() {
			self.closed_flag.close();

			*self.shared_upstream_unsubscribe_flag.lock_ignore_poison() = true;

			self.shared_teardown.unsubscribe();
			let mut lock = self.data.lock_ignore_poison();
			lock.try_complete();
			lock.unsubscribe();
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
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock_with_poison_behavior(|inner| inner.unsubscribe());
			lock.destination.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}
