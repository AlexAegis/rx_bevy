use std::{
	collections::VecDeque,
	sync::{Arc, Mutex},
};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Observer, Signal, Subscriber, SubscriptionClosedFlag, SubscriptionData,
	SubscriptionLike, Teardown, TeardownCollection, TeardownCollectionExtension,
};

pub struct ConcatSubscriberProvider;

impl HigherOrderSubscriberProvider for ConcatSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>
		= ConcatSubscriber<InnerObservable, Destination>
	where
		InnerObservable:
			Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
		Destination: 'static + Subscriber;
}

impl<InnerObservable, Destination> HigherOrderSubscriberFactory<Destination>
	for ConcatSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn new_from_destination(destination: Destination) -> Self {
		Self::new(destination, 1)
	}
}

struct ConcatSubscriberData<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	// TODO: This could be a regular sharedsub no rc
	destination: RcSubscriber<Destination>,
	observable_queue: VecDeque<InnerObservable>,
	upstream_completed: bool,
	active_subscriptions: usize,
	concurrency_limit: usize,
	shared_outer_teardown: Arc<Mutex<SubscriptionData>>,
}

impl<InnerObservable, Destination> ConcatSubscriberData<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub(crate) fn new(
		destination: RcSubscriber<Destination>,
		concurrency_limit: usize,
		shared_outer_teardown: Arc<Mutex<SubscriptionData>>,
	) -> Self {
		Self {
			destination,
			active_subscriptions: 0,
			concurrency_limit,
			observable_queue: VecDeque::new(),
			upstream_completed: false,
			shared_outer_teardown,
		}
	}

	pub(crate) fn try_complete(&mut self) {
		if self.active_subscriptions == 0
			&& self.observable_queue.is_empty()
			&& self.upstream_completed
		{
			self.destination.complete();
		}
	}
}

#[derive(RxSubscriber)]
#[rx_in(InnerObservable::Out)]
#[rx_in_error(InnerObservable::OutError)]
struct ConcatInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	is_finished: bool,
	data: Arc<Mutex<ConcatSubscriberData<InnerObservable, Destination>>>,
}

impl<InnerObservable, Destination> ConcatInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub fn new(data: Arc<Mutex<ConcatSubscriberData<InnerObservable, Destination>>>) -> Self {
		Self {
			data,
			is_finished: false,
		}
	}
}

impl<InnerObservable, Destination> Observer for ConcatInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			lock.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.is_finished = true;
			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			lock.destination.error(error);
		}
	}

	fn complete(&mut self) {
		self.unsubscribe();
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for ConcatInnerSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		let mut lock = self
			.data
			.lock()
			.unwrap_or_else(|poison_error| poison_error.into_inner());
		lock.destination.add_teardown(teardown);
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for ConcatInnerSubscriber<InnerObservable, Destination>
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
			.lock()
			.unwrap_or_else(|poison_error| poison_error.into_inner());
		lock.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.is_finished = true;
			let next_observable = {
				let mut lock = self
					.data
					.lock()
					.unwrap_or_else(|poison_error| poison_error.into_inner());

				lock.observable_queue.pop_front()
			};

			let next_subscription = if let Some(mut next_observable) = next_observable {
				let subscription =
					next_observable.subscribe(ConcatInnerSubscriber::new(self.data.clone()));
				Some(subscription)
			} else {
				None
			};

			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());

			if let Some(subscription) = next_subscription {
				lock.shared_outer_teardown.add(subscription);
			} else {
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
pub struct ConcatSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	shared_teardown: Arc<Mutex<SubscriptionData>>,
	data: Arc<Mutex<ConcatSubscriberData<InnerObservable, Destination>>>,
	closed_flag: SubscriptionClosedFlag,
}

impl<InnerObservable, Destination> ConcatSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub fn new(destination: Destination, concurrency_limit: usize) -> Self {
		let destination = RcSubscriber::new(destination);
		let shared_teardown = Arc::new(Mutex::new(SubscriptionData::default()));

		Self {
			data: Arc::new(Mutex::new(ConcatSubscriberData::new(
				destination,
				concurrency_limit,
				shared_teardown.clone(),
			))),
			shared_teardown,
			closed_flag: false.into(),
		}
	}
}

impl<InnerObservable, Destination> Observer for ConcatSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			if lock.active_subscriptions < lock.concurrency_limit {
				lock.active_subscriptions += 1;
				drop(lock);
				let subscription = next.subscribe(ConcatInnerSubscriber::new(self.data.clone()));
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
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			lock.destination.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			lock.upstream_completed = true;
			lock.try_complete();
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for ConcatSubscriber<InnerObservable, Destination>
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
			self.shared_teardown.unsubscribe();
			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			lock.active_subscriptions = 0;
			lock.observable_queue.clear();
			lock.destination.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for ConcatSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			let mut lock = self
				.data
				.lock()
				.unwrap_or_else(|poison_error| poison_error.into_inner());
			lock.destination.add_downstream_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}
