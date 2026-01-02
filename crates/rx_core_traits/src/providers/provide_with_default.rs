use std::marker::PhantomData;

use derive_where::derive_where;

use crate::{Provider, ProviderMut};

#[derive_where(Clone, Copy)]
#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvideWithDefault<T> {
	_phantom_data: PhantomData<T>,
}

impl<T> Provider for ProvideWithDefault<T>
where
	T: 'static + Default,
{
	type Provided = T;

	#[inline]
	fn provide(&self) -> T {
		T::default()
	}
}

impl<T> ProviderMut for ProvideWithDefault<T>
where
	T: Default,
{
	type Provided = T;

	#[inline]
	fn provide(&mut self) -> T {
		T::default()
	}
}
