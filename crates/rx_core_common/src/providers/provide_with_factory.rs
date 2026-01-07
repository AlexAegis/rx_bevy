use crate::{Provider, ProviderMut};

pub struct ProvideWithFactory<F>(pub F);

impl<F, T> From<F> for ProvideWithFactory<F>
where
	F: Fn() -> T,
{
	#[inline]
	fn from(value: F) -> Self {
		ProvideWithFactory(value)
	}
}

impl<F, T> Provider for ProvideWithFactory<F>
where
	F: Fn() -> T,
	T: 'static,
{
	type Provided = T;

	#[inline]
	fn provide(&self) -> T {
		(self.0)()
	}
}

impl<F, T> Provider for F
where
	F: Fn() -> T,
	T: 'static,
{
	type Provided = T;

	#[inline]
	fn provide(&self) -> T {
		(self)()
	}
}

impl<T, F> ProviderMut for ProvideWithFactory<F>
where
	F: FnMut() -> T,
{
	type Provided = T;

	#[inline]
	fn provide(&mut self) -> T {
		(self.0)()
	}
}

impl<F, T> ProviderMut for F
where
	F: FnMut() -> T,
{
	type Provided = T;

	#[inline]
	fn provide(&mut self) -> T {
		(self)()
	}
}
