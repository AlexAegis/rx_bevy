use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{HigherOrderInnerSubscriber, HigherOrderSubscriberState};
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, SharedSubscriber, SharedSubscription, Signal,
	Subscriber, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection,
};

/// A subscriber that only subscribes to an incoming observable, if there are
/// no active inner subscriptions already. If there is one, the incoming
/// observable is simply dropped, and won't be subscribed to.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
pub struct ExhaustSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	outer_teardown: SharedSubscription,
	shared_destination: SharedSubscriber<Destination>,
	state: Arc<Mutex<HigherOrderSubscriberState<()>>>,
	inner_subscription: Option<SubscriptionData>,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> ExhaustSubscriber<InnerObservable, Destination>
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

impl<InnerObservable, Destination> Observer for ExhaustSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In) {
		let mut state_lock = self.state.lock_ignore_poison();

		if !self.is_closed() && state_lock.non_unsubscribed_subscriptions == 0 {
			state_lock.non_completed_subscriptions += 1;
			state_lock.non_unsubscribed_subscriptions += 1;
			drop(state_lock);

			if let Some(mut inner_subscription) = self.inner_subscription.take() {
				inner_subscription.unsubscribe();
			}

			let state_on_unsubscribe_clone = self.state.clone();
			let mut outer_teardown_on_unsubscribe_clone = self.outer_teardown.clone();

			let mut shared_destination_on_unsubscribe = self.shared_destination.clone();

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
						shared_destination_on_unsubscribe.unsubscribe();
					}
				},
			));

			if !subscription.is_closed() {
				self.inner_subscription =
					Some(SubscriptionData::new_with_teardown(subscription.into()));
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

impl<InnerObservable, Destination> SubscriptionLike
	for ExhaustSubscriber<InnerObservable, Destination>
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
		if !self.is_closed()
			&& self
				.state
				.lock_ignore_poison()
				.upstream_unsubscribe_can_downstream()
		{
			if let Some(subscription_handle) = &mut self.inner_subscription {
				subscription_handle.unsubscribe();
			};

			self.outer_teardown.unsubscribe();
			self.shared_destination.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for ExhaustSubscriber<InnerObservable, Destination>
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
