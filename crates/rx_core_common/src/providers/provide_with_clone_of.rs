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

#[cfg(test)]
mod tests {
	use super::*;

	mod provider {
		use super::*;

		#[test]
		fn it_should_provide_by_clone() {
			let provider = ProvideWithCloneOf(42usize);
			assert_eq!(provider.provide(), 42);
		}
	}

	mod provider_mut {
		use super::*;

		#[test]
		fn it_should_provide_by_clone() {
			let mut provider = ProvideWithCloneOf(String::from("hello"));
			assert_eq!(ProviderMut::provide(&mut provider), "hello");
		}
	}
}
