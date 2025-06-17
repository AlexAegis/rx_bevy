pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
}

pub trait Observer: ObserverInput {
	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::InError);
	fn complete(&mut self);
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
