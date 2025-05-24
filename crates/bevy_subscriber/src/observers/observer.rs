use std::sync::{Arc, RwLock};

pub trait Observer<T> {
	fn on_push(&mut self, value: T);
}

pub struct ObserverContainer<T> {
	observer: Arc<RwLock<dyn Observer<T>>>,
}

impl<T> ObserverContainer<T> {
	pub fn from_observer(observer: impl Observer<T> + 'static) -> Self {
		ObserverContainer {
			observer: Arc::new(RwLock::new(observer)),
		}
	}
}

impl<T> Observer<T> for ObserverContainer<T> {
	fn on_push(&mut self, value: T) {
		let mut lock = self.observer.write().expect("TO BE WRITABLE");
		lock.on_push(value);
	}
}
