use crate::Signal;

pub trait ObserverInput {
	type In: Signal;
	type InError: Signal;
}

pub trait Observer: ObserverInput {
	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::InError);
	fn complete(&mut self);
}
