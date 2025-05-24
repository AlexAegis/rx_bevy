use crate::observers::{Observer, ObserverContainer};

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

impl<T> Observable<T> for OfObservable<T>
where
	T: Clone,
{
	fn subscribe_container(&mut self, mut observer: ObserverContainer<T>) {
		observer.on_push(self.value.clone());
	}
}
