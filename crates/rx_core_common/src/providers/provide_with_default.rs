use derive_where::derive_where;

use crate::{PhantomInvariant, Provider, ProviderMut};

#[derive_where(Clone, Copy)]
#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvideWithDefault<T> {
	_phantom_data: PhantomInvariant<T>,
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

#[cfg(test)]
mod test {
	use crate::{
		ProvideWithDefault,
		providers::{Provider, ProviderMut},
	};

	#[derive(Debug, PartialEq, Eq)]
	struct Foo(&'static str);

	impl Default for Foo {
		fn default() -> Self {
			Self("foo")
		}
	}

	mod provider {
		use super::*;

		#[test]
		fn it_should_provide_with_default() {
			let provider = ProvideWithDefault::<usize>::default();
			assert_eq!(provider.provide(), 0);
		}
	}

	mod provider_mut {
		use super::*;

		#[test]
		fn it_should_provide_with_default() {
			let mut provider = ProvideWithDefault::<Foo>::default();
			assert_eq!(ProviderMut::provide(&mut provider), Foo("foo"));
		}
	}
}
