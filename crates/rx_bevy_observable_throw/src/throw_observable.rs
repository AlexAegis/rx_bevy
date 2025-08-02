use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, Subscription, Teardown, UpgradeableObserver,
};

/// Observable creator for [ThrowObservable]
pub fn throw<Error>(error: Error) -> ThrowObservable<Error>
where
	Error: Clone,
{
	ThrowObservable::new(error)
}

impl<Error> ObservableOutput for ThrowObservable<Error>
where
	Error: 'static + Clone,
{
	type Out = ();
	type OutError = Error;
}

impl<Error> Observable for ThrowObservable<Error>
where
	Error: 'static + Clone,
{
	fn subscribe<Destination: 'static + UpgradeableObserver<In = (), InError = Error>>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let mut subscriber = destination.upgrade();
		subscriber.error(self.error.clone());
		Subscription::new(Teardown::new_from_subscription(subscriber))
	}
}

#[derive(Clone)]
pub struct ThrowObservable<Error>
where
	Error: Clone,
{
	error: Error,
}

impl<Error> ThrowObservable<Error>
where
	Error: Clone,
{
	pub fn new(error: Error) -> Self {
		Self { error }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rx_bevy_testing::prelude::*;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mut mock_observer = MockObserver::new_shared();

		let _s = observable.subscribe(mock_observer.clone());

		mock_observer.read(|d| {
			assert_eq!(d.destination.errors, vec![error]);
		});
	}
}
