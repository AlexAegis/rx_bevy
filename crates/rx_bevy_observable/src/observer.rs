pub trait Observer {
	type In;
	type Error;

	fn on_push(&mut self, next: Self::In);
	fn on_error(&mut self, error: Self::Error);
	fn on_complete(&mut self);
}

pub struct DynObserver<T, E> {
	pub dyn_on_push: Box<dyn FnMut(T) -> ()>,
	pub dyn_on_error: Box<dyn FnMut(E) -> ()>,
	pub dyn_on_complete: Box<dyn FnMut() -> ()>,
}

impl<T, E> Observer for DynObserver<T, E> {
	type In = T;
	type Error = E;

	fn on_push(&mut self, next: T) {
		(self.dyn_on_push)(next);
	}

	fn on_error(&mut self, error: E) {
		(self.dyn_on_error)(error);
	}

	fn on_complete(&mut self) {
		(self.dyn_on_complete)();
	}
}
