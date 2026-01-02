use derive_where::derive_where;

use crate::providers::{Provider, ProviderMut};

#[derive_where(Clone)]
#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvideWithCloneOf<T>(pub T)
where
	T: Clone;

impl<T> Provider for ProvideWithCloneOf<T>
where
	T: 'static + Clone,
{
	type Provided = T;

	#[inline]
	fn provide(&self) -> T {
		self.0.clone()
	}
}

impl<T: Clone> ProviderMut for ProvideWithCloneOf<T> {
	type Provided = T;

	#[inline]
	fn provide(&mut self) -> T {
		self.0.clone()
	}
}
