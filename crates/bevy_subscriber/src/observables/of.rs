use crate::observers::Observer;

use super::Observable;

pub struct OfObservable<T>
where
	T: Clone,
{
	value: T,
}

impl<T> OfObservable<T>
where
	T: Clone,
{
	pub fn new(value: T) -> Self {
		Self { value }
	}
}

impl<Destination, T> Observable<Destination> for OfObservable<T>
where
	T: Clone,
	Destination: Observer<In = T>,
{
	type Out = T;

	fn internal_subscribe(self, mut observer: Destination) {
		observer.on_push(self.value.clone());
	}
}
