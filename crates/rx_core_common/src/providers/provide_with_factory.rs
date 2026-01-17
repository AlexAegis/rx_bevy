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

#[cfg(test)]
mod tests {
	mod provide {
		use crate::{ProvideWithFactory, providers::Provider};

		#[test]
		fn it_should_provide_by_call_unit_struct() {
			let factory = ProvideWithFactory(|| 7);

			assert_eq!(factory.provide(), 7);
		}

		#[test]
		fn it_should_provide_by_call_into_unit_struct() {
			let factory = ProvideWithFactory::from(|| 7);

			assert_eq!(factory.provide(), 7);
		}

		#[test]
		fn it_should_provide_by_call_fn_impl() {
			let factory = || 5usize;
			assert_eq!(factory.provide(), 5);
		}
	}

	mod provide_mut {
		use crate::{ProvideWithFactory, providers::ProviderMut};

		#[test]
		fn it_should_provide_by_call_unit_struct() {
			let mut counter = 0usize;
			let mut factory = ProvideWithFactory(|| {
				counter += 1;
				counter * 2
			});

			assert_eq!(factory.provide(), 2);
			assert_eq!(factory.provide(), 4);
		}

		#[test]
		fn it_should_provide_by_call_fn_mut_impl() {
			let mut counter = 0usize;
			let mut factory = || {
				counter += 1;
				counter * 2
			};

			assert_eq!(factory.provide(), 2);
			assert_eq!(factory.provide(), 4);
		}
	}
}
