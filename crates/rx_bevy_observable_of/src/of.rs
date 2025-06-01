use rx_bevy_observable::{Observable, Observer};

/// Observable creator for [OfObservable]
pub fn of<T>(value: T) -> OfObservable<T>
where
	T: Clone,
{
	OfObservable::new(value)
}

impl<Out> Observable for OfObservable<Out>
where
	Out: Clone,
{
	type Out = Out;

	type Subscription = ();

	fn subscribe<Destination: Observer<In = Out>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		observer.on_push(self.value.clone());
	}
}

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
	use rx_bevy_testing::{MockObserver, SharedForwardObserver};

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let mut observable = OfObservable::new(value);
		let mock_observer = MockObserver::new_shared();

		let f = SharedForwardObserver::new(&mock_observer);

		observable.subscribe(f);

		assert_eq!(mock_observer.read().unwrap().values, vec![4]);
	}
}
