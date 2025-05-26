use crate::observers::Observer;

use super::{Observable, ObservableExtensionPipe};

/// Observable creator for [OfObservable]
pub fn of<T>(value: T) -> OfObservable<T>
where
	T: Clone,
{
	OfObservable::new(value)
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

impl<Out> Observable for OfObservable<Out>
where
	Out: Clone,
{
	type Out = Out;

	fn subscribe<Destination: Observer<In = Out>>(self, mut observer: Destination) {
		observer.on_push(self.value.clone());
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::testing::{FwObserver, MockObserver};

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let observable = OfObservable::new(value);
		let mock_observer = MockObserver::new_shared();

		let f = FwObserver::new(&mock_observer);

		observable.subscribe(f);

		assert_eq!(mock_observer.read().unwrap().values, vec![4]);
	}
}
