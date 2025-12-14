use std::ops::{Deref, DerefMut};

use derive_where::derive_where;
use rx_core_traits::{Observable, Signal};
use variadics_please::all_tuples;

use crate::observable::ErasedObservable;

#[derive_where(Clone)]
pub struct ErasedObservables<Out, OutError, const SIZE: usize>
where
	Out: Signal,
	OutError: Signal,
{
	observables: [ErasedObservable<Out, OutError>; SIZE],
}

impl<Out, OutError, const SIZE: usize> Deref for ErasedObservables<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	type Target = [ErasedObservable<Out, OutError>; SIZE];

	fn deref(&self) -> &Self::Target {
		&self.observables
	}
}

impl<Out, OutError, const SIZE: usize> DerefMut for ErasedObservables<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.observables
	}
}

impl<Out, OutError, const SIZE: usize, O> From<[O; SIZE]> for ErasedObservables<Out, OutError, SIZE>
where
	Out: 'static + Signal,
	OutError: 'static + Signal,
	O: 'static + Observable<Out = Out, OutError = OutError> + Send + Sync,
{
	fn from(value: [O; SIZE]) -> Self {
		Self {
			observables: value.map(|observable| ErasedObservable::new(observable)),
		}
	}
}

macro_rules! tuple_len {
	() => { 0usize };
	($($name:ident),+) => {
		0usize $(+ tuple_len!(@one $name))*
	};
	(@one $name:ident) => { 1usize };
}

macro_rules! impl_tuple_erased_observable {
	($(#[$meta:meta])* $($name:ident),*) => {
		$(#[$meta])*
		impl<
			Out: 'static + Signal,
			OutError: 'static + Signal,
			$($name: 'static + Observable<Out = Out, OutError = OutError> + Send + Sync),*
		> From<($($name,)*)> for ErasedObservables<Out, OutError, { tuple_len!($($name),*) }>
		{
			#[allow(non_snake_case)]
			fn from(value: ($($name,)*)) -> Self {
				#[allow(non_snake_case)]
				let ($($name,)*) = value;

				Self {
					observables: [
						$(ErasedObservable::new($name),)*
					],
				}
			}
		}
	};
}

all_tuples!(impl_tuple_erased_observable, 1, 15, O);
