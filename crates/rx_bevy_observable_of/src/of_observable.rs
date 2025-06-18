use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, Subscription, prelude::ObserverSubscriber,
};

/// Observable creator for [OfObservable]
pub fn of<T>(value: T) -> OfObservable<T>
where
	T: Clone,
{
	OfObservable::new(value)
}

impl<Out> ObservableOutput for OfObservable<Out>
where
	Out: 'static + Clone,
{
	type Out = Out;
	type OutError = ();
}

impl<Out> Observable for OfObservable<Out>
where
	Out: 'static + Clone,
{
	type Subscriber<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>> =
		ObserverSubscriber<Destination>;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription<Self::Subscriber<Destination>> {
		let mut subscriber = ObserverSubscriber::new(destination);
		subscriber.next(self.value.clone());
		subscriber.complete();
		Subscription::new(subscriber)
	}
}

/// Emits a single value then immediately completes
#[derive(Clone)]
pub struct OfObservable<Out>
where
	Out: Clone,
{
	value: Out,
}

impl<Out> OfObservable<Out>
where
	Out: Clone,
{
	pub fn new(value: Out) -> Self {
		Self { value }
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use rx_bevy_testing::MockObserver;

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let mut observable = OfObservable::new(value);
		let mut mock_observer = MockObserver::new_shared();

		observable.subscribe(mock_observer.clone());

		mock_observer.read(|d| {
			assert_eq!(d.destination.values, vec![value]);
		});
	}
}
