pub trait ObserverInput {
	type In;
	type InError;
}

pub trait Observer: ObserverInput {
	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::InError);
	fn complete(&mut self);
}

pub struct DynObserver<T, E> {
	pub dyn_next: Box<dyn FnMut(T) -> ()>,
	pub dyn_error: Box<dyn FnMut(E) -> ()>,
	pub dyn_complete: Box<dyn FnMut() -> ()>,
}

impl<T, E> ObserverInput for DynObserver<T, E> {
	type In = T;
	type InError = E;
}

impl<T, E> Observer for DynObserver<T, E> {
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

impl ObserverInput for () {
	type In = ();
	type InError = ();
}

impl Observer for () {
	fn next(&mut self, _next: Self::In) {}

	fn error(&mut self, _error: Self::InError) {}

	fn complete(&mut self) {}
}
