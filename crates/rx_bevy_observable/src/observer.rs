pub trait Observer {
	type In;

	fn on_push(&mut self, next: Self::In);
}

pub struct DynObserver<T> {
	pub dyn_on_push: Box<dyn FnMut(T) -> ()>,
}

impl<T> Observer for DynObserver<T> {
	type In = T;

	fn on_push(&mut self, next: T) {
		(self.dyn_on_push)(next);
	}
}
