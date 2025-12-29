use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observer, Subscriber, SubscriptionClosedFlag, SubscriptionLike,
	Teardown, TeardownCollection,
};

use crate::{HigherOrderSubscriberState, HigherOrderSubscriberStateConditions};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct HigherOrderInnerSubscriber<State, OnComplete, OnUnsubscribe, Destination>
where
	State: HigherOrderSubscriberStateConditions,
	OnComplete: FnOnce(usize),
	OnUnsubscribe: FnOnce(usize),
	Destination: 'static + Subscriber,
{
	closed: SubscriptionClosedFlag,
	key: usize,
	state: Arc<Mutex<HigherOrderSubscriberState<State>>>,
	shared_destination: Arc<Mutex<Destination>>,
	on_complete: Option<OnComplete>,
	on_unsubscribe: Option<OnUnsubscribe>,
}

impl<State, OnComplete, OnUnsubscribe, Destination>
	HigherOrderInnerSubscriber<State, OnComplete, OnUnsubscribe, Destination>
where
	State: HigherOrderSubscriberStateConditions,
	OnComplete: FnOnce(usize),
	OnUnsubscribe: FnOnce(usize),
	Destination: 'static + Subscriber,
{
	pub fn new(
		key: usize,
		shared_destination: Arc<Mutex<Destination>>,
		state: Arc<Mutex<HigherOrderSubscriberState<State>>>,
		on_complete: OnComplete,
		on_unsubscribe: OnUnsubscribe,
	) -> Self {
		Self {
			closed: false.into(),
			key,
			shared_destination,
			state,
			on_complete: Some(on_complete),
			on_unsubscribe: Some(on_unsubscribe),
		}
	}
}

impl<State, OnComplete, OnUnsubscribe, Destination> Observer
	for HigherOrderInnerSubscriber<State, OnComplete, OnUnsubscribe, Destination>
where
	State: HigherOrderSubscriberStateConditions,
	OnComplete: FnOnce(usize),
	OnUnsubscribe: FnOnce(usize),
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.shared_destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.state.lock_ignore_poison().downstream_error();
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
			}

			if let Some(on_complete) = self.on_complete.take() {
				on_complete(self.key);
			}

			{
				let mut state = self.state.lock_ignore_poison();

				if state.can_downstream_complete() {
					state.downstream_subscriber_state.complete();
					drop(state);

					self.shared_destination.complete();
					self.shared_destination.unsubscribe();
				}
			}

			self.unsubscribe();
		}
	}
}

impl<State, OnComplete, OnUnsubscribe, Destination> TeardownCollection
	for HigherOrderInnerSubscriber<State, OnComplete, OnUnsubscribe, Destination>
where
	State: HigherOrderSubscriberStateConditions,
	OnComplete: FnOnce(usize),
	OnUnsubscribe: FnOnce(usize),
	Destination: 'static + Subscriber,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.shared_destination.add_teardown(teardown);
	}
}

impl<State, OnComplete, OnUnsubscribe, Destination> SubscriptionLike
	for HigherOrderInnerSubscriber<State, OnComplete, OnUnsubscribe, Destination>
where
	State: HigherOrderSubscriberStateConditions,
	OnComplete: FnOnce(usize),
	OnUnsubscribe: FnOnce(usize),
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

			if let Some(on_unsubscribe) = self.on_unsubscribe.take() {
				on_unsubscribe(self.key);
			}
		}
	}
}

impl<State, OnComplete, OnUnsubscribe, Destination> Drop
	for HigherOrderInnerSubscriber<State, OnComplete, OnUnsubscribe, Destination>
where
	State: HigherOrderSubscriberStateConditions,
	OnComplete: FnOnce(usize),
	OnUnsubscribe: FnOnce(usize),
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe();
		self.closed.close();
	}
}
