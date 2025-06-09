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
	type Error = ();

	type Subscription = ();

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: Observer<In = Out>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		// TODO: Use an actual Subscriber
		observer.next(self.value.clone());
		observer.complete();
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
			assert_eq!(d.values, vec![value]);
		});
	}
}
