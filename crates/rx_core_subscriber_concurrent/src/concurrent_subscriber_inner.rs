use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, Signal, Subscriber, SubscriptionClosedFlag,
	SubscriptionData, SubscriptionHandle, SubscriptionLike, Teardown, TeardownCollection,
};
use slab::Slab;

use crate::{create_inner_subscription, internal::ConcurrentSubscriberState};

#[derive(RxSubscriber)]
#[rx_in(InnerObservable::Out)]
#[rx_in_error(InnerObservable::OutError)]
#[rx_skip_unsubscribe_on_drop_impl]
pub(crate) struct ConcurrentSubscriberInner<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	closed: SubscriptionClosedFlag,
	errored: bool,
	key: usize,
	state: Arc<Mutex<ConcurrentSubscriberState<InnerObservable>>>,
	inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
	shared_destination: Arc<Mutex<Destination>>,
	outer_teardown: SubscriptionHandle,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> ConcurrentSubscriberInner<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	pub(crate) fn new(
		key: usize,
		shared_destination: Arc<Mutex<Destination>>,
		state: Arc<Mutex<ConcurrentSubscriberState<InnerObservable>>>,
		inner_subscriptions: Arc<Mutex<Slab<SubscriptionData>>>,
		outer_teardown: SubscriptionHandle,
	) -> Self {
		Self {
			closed: false.into(),
			errored: false,
			key,
			shared_destination,
			outer_teardown,
			state,
			inner_subscriptions,
			_phantom_data: PhantomData,
		}
	}
}

impl<InnerObservable, Destination> Observer
	for ConcurrentSubscriberInner<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.shared_destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.errored = true;
			self.state.lock_ignore_poison().error();
			self.shared_destination.error(error);
			self.shared_destination.unsubscribe();

			self.unsubscribe();
			self.closed.close();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			{
				let mut state = self.state.lock_ignore_poison();
				state.non_completed_subscriptions -= 1;

				if let Some(next) = state.queue.pop_front() {
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
				}
			}

			{
				let mut state = self.state.lock_ignore_poison();

				if state.can_downstream_complete() {
					state.downstream_completed = true;
					drop(state);
					self.shared_destination.complete();
				}
			}

			self.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for ConcurrentSubscriberInner<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.shared_destination.add_teardown(teardown);
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for ConcurrentSubscriberInner<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed || self.shared_destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !*self.closed {
			self.closed.close();

			{
				let mut state = self.state.lock_ignore_poison();
				state.non_unsubscribed_subscriptions -= 1;
			}

			{
				let no_more_subscribers_besides_this = {
					let mut inner_subscriptions = self.inner_subscriptions.lock_ignore_poison();
					inner_subscriptions.retain(|_k, s| !s.is_closed());
					inner_subscriptions
						.iter()
						.filter(|(k, _)| *k != self.key)
						.count() == 0
				};

				if no_more_subscribers_besides_this {
					let mut state = self.state.lock_ignore_poison();

					if let Some(next) = state.queue.pop_front() {
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
					}
				}
			}

			let mut state = self.state.lock_ignore_poison();

			if state.can_downstream_unsubscribe() || self.errored {
				state.downstream_unsubscribed = true;

				drop(state);
				for (key, inner_subscription) in
					self.inner_subscriptions.lock_ignore_poison().iter_mut()
				{
					// Must not unsubscribe itself, as we're in the middle of
					// unsubscribing! It would lock up!
					if key != self.key && !inner_subscription.is_closed() {
						inner_subscription.unsubscribe();
					}
				}

				// Close the subscriber, signaling upstream that we're closed
				self.outer_teardown.unsubscribe();
				// Close downstream
				self.shared_destination.unsubscribe();
			}
		}
	}
}

impl<InnerObservable, Destination> Drop for ConcurrentSubscriberInner<InnerObservable, Destination>
where
	InnerObservable: Observable<Out = Destination::In, OutError = Destination::InError> + Signal,
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe();
		self.closed.close();
	}
}
