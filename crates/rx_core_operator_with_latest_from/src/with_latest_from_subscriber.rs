use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use rx_core_common::{
	LockWithPoisonBehavior, Observable, ObserverSubscriber, PhantomInvariant, RxObserver,
	SharedSubscriber, Signal, Subscriber, SubscriptionLike,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{WithLatestFromInnerDestination, WithLatestFromInnerDestinationState};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
pub struct WithLatestFromSubscriber<In, InnerObservable, Destination>
where
	In: Signal,
	InnerObservable: Observable,
	InnerObservable::Out: Clone,
	Destination:
		'static + Subscriber<In = (In, InnerObservable::Out), InError = InnerObservable::OutError>,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	inner_subscription: <InnerObservable as Observable>::Subscription<
		ObserverSubscriber<WithLatestFromInnerDestination<InnerObservable::Out, Destination>>,
	>,
	state: Arc<Mutex<WithLatestFromInnerDestinationState<InnerObservable::Out>>>,
	_phantom_data: PhantomInvariant<InnerObservable>,
}

impl<In, InnerObservable, Destination> WithLatestFromSubscriber<In, InnerObservable, Destination>
where
	In: Signal,
	InnerObservable: Observable,
	InnerObservable::Out: Clone,
	Destination: Subscriber<In = (In, InnerObservable::Out), InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination, inner_observable: &mut InnerObservable) -> Self {
		let shared_destination = SharedSubscriber::new(destination);
		let inner_destination =
			WithLatestFromInnerDestination::<InnerObservable::Out, Destination>::new(
				shared_destination.clone(),
			);
		let state = inner_destination.get_state();
		let inner_subscription = inner_observable.subscribe(inner_destination);
		Self {
			destination: shared_destination,
			inner_subscription,
			state,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InnerObservable, Destination> RxObserver
	for WithLatestFromSubscriber<In, InnerObservable, Destination>
where
	In: Signal,
	InnerObservable: Observable,
	InnerObservable::Out: Clone,
	Destination: Subscriber<In = (In, InnerObservable::Out), InError = InnerObservable::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let latest = self.state.lock_ignore_poison().get_latest_value().clone();

		if let Some(latest) = latest {
			self.destination.next((next, latest));
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InnerObservable, Destination> SubscriptionLike
	for WithLatestFromSubscriber<In, InnerObservable, Destination>
where
	In: Signal,
	InnerObservable: Observable,
	InnerObservable::Out: Clone,
	Destination: Subscriber<In = (In, InnerObservable::Out), InError = InnerObservable::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.inner_subscription.unsubscribe();
		self.destination.unsubscribe();
	}
}
