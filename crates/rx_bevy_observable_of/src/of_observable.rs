use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, Subscription, UpgradeableObserver,
};

/// Observable creator for [OfObservable]
pub fn of<T>(value: T) -> OfObservable<T>
where
	T: Clone,
{
	OfObservable::new(value)
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

impl<Out> Observable for OfObservable<Out>
where
	Out: 'static + Clone,
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let mut subscriber = destination.upgrade();
		subscriber.next(self.value.clone());
		subscriber.complete();
		Subscription::new(subscriber)
	}
}

impl<Out> ObservableOutput for OfObservable<Out>
where
	Out: 'static + Clone,
{
	type Out = Out;
	type OutError = ();
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

		let _s = observable.subscribe(mock_observer.clone());

		mock_observer.read(|d| {
			assert_eq!(d.destination.values, vec![value]);
		});
	}
}
