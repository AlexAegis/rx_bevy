use std::{
	cell::RefCell,
	marker::PhantomData,
	rc::{Rc, Weak},
};

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operator, Subscriber, Subscription,
	SubscriptionLike, forwarders::DynForwarder, subscribers::ObserverSubscriber,
};

use crate::{MulticastOperator, MulticastOuterSubscriber};

/// A Subject is a shared multicast observer, can be used for broadcasting
/// a clone of it still has the same set of subscribers, and is needed if you
/// want to make multiple pipes out of the same subject
pub struct Subject<In, InError = ()>
where
	In: 'static,
	InError: 'static,
{
	destinations: MulticastOperator<In, InError>,
}

impl<T, Error> Clone for Subject<T, Error> {
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			destinations: self.destinations.clone(),
		}
	}
}

impl<T, Error> Default for Subject<T, Error> {
	fn default() -> Self {
		Self {
			destinations: MulticastOperator::new(),
		}
	}
}

impl<T, Error> ObservableOutput for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<T, Error> Observable for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Subscriber<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>> =
		MulticastOuterSubscriber<ObserverSubscriber<Destination>>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription<Self::Subscriber<Destination>> {
		let multicast_subscriber = self
			.destinations
			.operator_subscribe(ObserverSubscriber::new(destination));

		Subscription::new(multicast_subscriber)
	}
}

impl<T, Error> ObserverInput for Subject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	type In = T;
	type InError = Error;
}

impl<T, Error> Observer for Subject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	fn next(&mut self, next: Self::In) {
		self.destinations.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.destinations.error(error);
	}

	fn complete(&mut self) {
		// TODO: Check what a subject actually does on complete
		self.destinations.complete();
	}
}
