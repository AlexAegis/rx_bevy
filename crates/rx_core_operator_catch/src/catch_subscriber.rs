use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use rx_core_common::{
	LockWithPoisonBehavior, Observable, PhantomInvariant, RxObserver, SharedSubscriber,
	SharedSubscription, Signal, Subscriber, SubscriptionData, SubscriptionLike, Teardown,
	TeardownCollection,
};
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{HigherOrderInnerSubscriber, HigherOrderSubscriberState};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
pub struct CatchSubscriber<In, InError, InnerObservable, ErrorMapper, Destination>
where
	In: Signal,
	InError: Signal,
	InnerObservable: Observable<Out = In> + Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Send + Sync,
	Destination: 'static + Subscriber<In = In, InError = InnerObservable::OutError>,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
	state: Arc<Mutex<HigherOrderSubscriberState<()>>>,
	error_mapper: Option<ErrorMapper>,
	inner_subscription: Option<SubscriptionData>,
	outer_teardown: SharedSubscription,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, InnerObservable, ErrorMapper, Destination>
	CatchSubscriber<In, InError, InnerObservable, ErrorMapper, Destination>
where
	In: Signal,
	InError: Signal,
	InnerObservable: Observable<Out = In> + Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Send + Sync,
	Destination: 'static + Subscriber<In = In, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination, error_mapper: ErrorMapper) -> Self {
		Self {
			shared_destination: SharedSubscriber::new(destination),
			error_mapper: Some(error_mapper),
			inner_subscription: None,
			outer_teardown: SharedSubscription::default(),
			state: Arc::new(Mutex::new(HigherOrderSubscriberState::default())),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, InnerObservable, ErrorMapper, Destination> RxObserver
	for CatchSubscriber<In, InError, InnerObservable, ErrorMapper, Destination>
where
	In: Signal,
	InError: Signal,
	InnerObservable: Observable<Out = In> + Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Send + Sync,
	Destination: 'static + Subscriber<In = In, InError = InnerObservable::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.shared_destination.next(next);

		if self.shared_destination.is_closed() {
			self.unsubscribe();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if !self.is_closed()
			&& let Some(error_mapper) = self.error_mapper.take()
		{
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

			let mut next_observable = (error_mapper)(error);

			let mut higher_order_subscriber = HigherOrderInnerSubscriber::new(
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
			);

			{
				// Upstream is closed, no more notifications are expected.
				let mut higher_order_state = higher_order_subscriber.get_state_mut();
				higher_order_state.upstream_subscriber_state.complete();
				higher_order_state.upstream_subscriber_state.unsubscribe();
			}

			let subscription = next_observable.subscribe(higher_order_subscriber);

			if !subscription.is_closed() {
				self.inner_subscription =
					Some(SubscriptionData::new_with_teardown(subscription.into()));
			}

			if self.shared_destination.is_closed() {
				self.unsubscribe();
			}
		}
	}

	#[inline]
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

impl<In, InError, InnerObservable, ErrorMapper, Destination> TeardownCollection
	for CatchSubscriber<In, InError, InnerObservable, ErrorMapper, Destination>
where
	In: Signal,
	InError: Signal,
	InnerObservable: Observable<Out = In> + Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Send + Sync,
	Destination: 'static + Subscriber<In = In, InError = InnerObservable::OutError>,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.shared_destination.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl<In, InError, InnerObservable, ErrorMapper, Destination> SubscriptionLike
	for CatchSubscriber<In, InError, InnerObservable, ErrorMapper, Destination>
where
	In: Signal,
	InError: Signal,
	InnerObservable: Observable<Out = In> + Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Send + Sync,
	Destination: 'static + Subscriber<In = In, InError = InnerObservable::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.outer_teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			if let Some(subscription_handle) = &mut self.inner_subscription.take() {
				subscription_handle.unsubscribe();
			};

			self.outer_teardown.unsubscribe();
			self.shared_destination.unsubscribe();
		}
	}
}
