use std::marker::PhantomData;

pub struct ProvideWithCloneOf<T>(pub T)
where
	T: Clone;

pub struct ProvideWithFactory<F>(pub F);

impl<F, T> From<F> for ProvideWithFactory<F>
where
	F: Fn() -> T,
{
	fn from(value: F) -> Self {
		ProvideWithFactory(value)
	}
}

#[derive(Default)]
pub struct ProvideWithDefault<T> {
	_phantom_data: PhantomData<T>,
}

pub trait Provider {
	type Provided;

	fn provide(&self) -> Self::Provided;
}

impl<T> Provider for ProvideWithCloneOf<T>
where
	T: Clone,
{
	type Provided = T;

	fn provide(&self) -> T {
		self.0.clone()
	}
}

impl<T> Provider for ProvideWithDefault<T>
where
	T: Default,
{
	type Provided = T;

	fn provide(&self) -> T {
		T::default()
	}
}

impl<F, T> Provider for ProvideWithFactory<F>
where
	F: Fn() -> T,
{
	type Provided = T;

	fn provide(&self) -> T {
		(self.0)()
	}
}

impl<F, T> Provider for F
where
	F: Fn() -> T,
{
	type Provided = T;

	fn provide(&self) -> T {
		(self)()
	}
}

pub trait ProviderMut {
	type Provided;

	fn provide(&mut self) -> Self::Provided;
}

impl<T: Clone> ProviderMut for ProvideWithCloneOf<T> {
	type Provided = T;

	fn provide(&mut self) -> T {
		self.0.clone()
	}
}

impl<T, F> ProviderMut for ProvideWithFactory<F>
where
	F: FnMut() -> T,
{
	type Provided = T;

	fn provide(&mut self) -> T {
		(self.0)()
	}
}

impl<F, T> ProviderMut for F
where
	F: FnMut() -> T,
{
	type Provided = T;

	fn provide(&mut self) -> T {
		(self)()
	}
}

/*
TODO: Finish idea + Provider enum over all types regular, mut, observable
pub trait ObservableProviderMut: Observable {
	type Provided;

	fn provide<OnNext>(&mut self, on_next: OnNext) -> Self::Subscription<Noop>
	where
		OnNext: Fn(Self::Provided);
}

impl<T> ObservableProviderMut for T
where
	T: Observable,
{
	type Provided = T::Out;

	fn provide<OnNext>(&mut self, on_next: OnNext)
	where
		OnNext: Fn(Self::Provided),
	{
		self.tap_next().subscribe(Noop)
	}
}
*/
