use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use rx_core_macro_subscriber_derive::RxSubscriber;

use rx_core_common::{
	LockWithPoisonBehavior, Observable, RxObserver, SharedSubscriber, SharedSubscription, Signal,
	Subscriber, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection,
};
use rx_core_subscriber_higher_order::{HigherOrderInnerSubscriber, HigherOrderSubscriberState};

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	shared_destination: SharedSubscriber<Destination>,
	state: Arc<Mutex<HigherOrderSubscriberState<()>>>,
	inner_subscription: Option<SubscriptionData>,
	outer_teardown: SharedSubscription,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			outer_teardown: SharedSubscription::default(),
			shared_destination: SharedSubscriber::new(destination),
			state: Arc::new(Mutex::new(HigherOrderSubscriberState::default())),
			inner_subscription: None,
			_phantom_data: PhantomData,
		}
	}
}

impl<InnerObservable, Destination> RxObserver for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			let mut state_lock = self.state.lock_ignore_poison();
			state_lock.non_completed_subscriptions += 1;
			state_lock.non_unsubscribed_subscriptions += 1;
			drop(state_lock);

			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe();
			}

			let state_on_unsubscribe_clone = self.state.clone();
			let mut outer_teardown_on_unsubscribe_clone = self.outer_teardown.clone();
			let mut shared_destination_on_unsubscribe_clone = self.shared_destination.clone();

			let subscription = next.subscribe(HigherOrderInnerSubscriber::new(
				0,
				self.shared_destination.clone(),
				self.state.clone(),
				move |_| {},
				move |_| {
					if state_on_unsubscribe_clone
						.lock_ignore_poison()
						.can_downstream_unsubscribe()
					{
						outer_teardown_on_unsubscribe_clone.unsubscribe();
						shared_destination_on_unsubscribe_clone.unsubscribe();
					}
				},
			));

			if !subscription.is_closed() {
				self.inner_subscription =
					Some(SubscriptionData::new_with_teardown(subscription.into()));
			}

			if self.shared_destination.is_closed() {
				self.unsubscribe();
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
				.upstream_completed_can_downstream()
		{
			self.shared_destination.complete();
			self.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.shared_destination.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.outer_teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.outer_teardown.unsubscribe();
			self.shared_destination.unsubscribe();
			if let Some(subscription_handle) = &mut self.inner_subscription.take() {
				subscription_handle.unsubscribe();
			};
		}
	}
}
