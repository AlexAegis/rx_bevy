use crate::observers::{Observer, ObserverContainer};

pub trait Observable<T> {
	fn subscribe_container(&mut self, observer: ObserverContainer<T>);

	fn subscribe(&mut self, observer: impl Observer<T> + 'static) {
		self.subscribe_container(ObserverContainer::from_observer(observer));
	}
}
