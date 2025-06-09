pub trait Observer {
	type In;
	type Error;

	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::Error);
	fn complete(&mut self);
}

pub struct DynObserver<T, E> {
	pub dyn_next: Box<dyn FnMut(T) -> ()>,
	pub dyn_error: Box<dyn FnMut(E) -> ()>,
	pub dyn_complete: Box<dyn FnMut() -> ()>,
}

impl<T, E> Observer for DynObserver<T, E> {
	type In = T;
	type Error = E;

	fn next(&mut self, next: T) {
		(self.dyn_next)(next);
	}

	fn error(&mut self, error: E) {
		(self.dyn_error)(error);
	}

	fn complete(&mut self) {
		(self.dyn_complete)();
	}
}

impl Observer for () {
	type In = ();
	type Error = ();

	fn next(&mut self, _next: ()) {}

	fn error(&mut self, _error: Self::Error) {}

	fn complete(&mut self) {}
}
