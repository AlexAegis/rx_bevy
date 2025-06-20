use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operator, Subscription,
	subscribers::ObserverSubscriber,
};

use rx_bevy_operator_multicast::MulticastOperator;

/// A Subject is a shared multicast observer, can be used for broadcasting
/// a clone of it still has the same set of subscribers, and is needed if you
/// want to make multiple pipes out of the same subject
pub struct Subject<In, InError = ()>
where
	In: 'static,
	InError: 'static,
{
	multicast: MulticastOperator<In, InError>,
}

impl<T, Error> Clone for Subject<T, Error> {
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<T, Error> Default for Subject<T, Error> {
	fn default() -> Self {
		Self {
			multicast: MulticastOperator::default(),
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
	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		Subscription::new(
			self.multicast
				.operator_subscribe(ObserverSubscriber::new(destination)),
		)
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
		self.multicast.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.multicast.error(error);
	}

	fn complete(&mut self) {
		self.multicast.complete();
	}
}
