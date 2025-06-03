use rx_bevy_observable::{Observable, Observer};

/// Observable creator for [ThrowObservable]
pub fn throw<Error>(error: Error) -> ThrowObservable<Error>
where
	Error: Clone,
{
	ThrowObservable::new(error)
}

impl<Error> Observable for ThrowObservable<Error>
where
	Error: Clone,
{
	type Out = ();
	type Error = Error;

	type Subscription = ();

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: Observer<Error = Error>>(
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
	use rx_bevy_testing::{MockObserver, SharedForwardObserver};

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mock_observer = MockObserver::new_shared();

		let f = SharedForwardObserver::new(&mock_observer);

		observable.subscribe(f);

		assert_eq!(mock_observer.read().unwrap().errors, vec![error]);
	}
}
