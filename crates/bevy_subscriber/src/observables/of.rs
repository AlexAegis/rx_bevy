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

impl<Destination, Out> Observable<Destination> for OfObservable<Out>
where
	Out: Clone,
	Destination: Observer<In = Out>,
{
	type Out = Out;

	fn subscribe(self, mut observer: Destination) {
		observer.on_push(self.value.clone());
	}
}

/// TODO: Could be part of a possible observable macro
impl<Out, Destination> ObservableWithOperators<Destination, Out> for OfObservable<Out>
where
	Destination: Observer<In = Out>,
	Out: Clone,
{
}
