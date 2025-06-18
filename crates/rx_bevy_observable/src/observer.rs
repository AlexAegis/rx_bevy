pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
}

pub trait Observer: ObserverInput {
	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::InError);
	fn complete(&mut self);
}
