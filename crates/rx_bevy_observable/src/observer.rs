use crate::Subscription;

pub trait Observer<In> {
	// type In;

	fn on_push(&mut self, next: In);
}

pub struct DynObserver<T> {
	pub dyn_on_push: Box<dyn FnMut(T) -> ()>,
}

impl<T> Observer<T> for DynObserver<T> {
	fn on_push(&mut self, next: T) {
		(self.dyn_on_push)(next);
	}
}
