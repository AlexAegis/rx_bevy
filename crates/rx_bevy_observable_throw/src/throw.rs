use rx_bevy_observable::{Observable, ObservableOutput, Observer};

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
	type Subscription = ();

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: Observer<InError = Error>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		observer.error(self.error.clone());
	}
}

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
	use rx_bevy_testing::MockObserver;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mut mock_observer = MockObserver::new_shared();

		observable.subscribe(mock_observer.clone());

		mock_observer.read(|d| {
			assert_eq!(d.errors, vec![error]);
		});
	}
}
