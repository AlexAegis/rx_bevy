use crate::observers::Observer;

use super::{Observable, ObservableWithOperators};

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

/// TODO: Could be part of a possible observable macro
impl<Out> ObservableWithOperators<Out> for OfObservable<Out> where Out: Clone {}
