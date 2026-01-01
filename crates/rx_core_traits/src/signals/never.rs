use core::convert::Infallible;

use crate::Signal;

/// Never cannot be constructed as it is an enum with no variants.
/// So it's perfect to denote a signal that will never be emitted, like
/// Observables that never error, or emit. Or Operators who catch errors.
///
/// Never is really just a type alias for the built in [Infallible] type, but
/// the name `Infallible` limits it's meaning to errors for the `Result` type,
/// while Never is about events/signals of any kinds that can never happen.
pub type Never = Infallible;

pub trait NeverMapIntoExtension {
	fn map_into<T>() -> impl Fn(Never) -> T + Signal + Clone;
}

impl NeverMapIntoExtension for Never {
	/// A mapper for the never type into anything.
	/// Since Never cannot be created, nothing has to be actually converted.
	///
	/// The implementation is just: `|_| unreachable!()`.
	#[inline]
	fn map_into<T>() -> impl Fn(Never) -> T + Signal + Clone {
		|_| unreachable!("Never cannot be created!")
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_return_a_function_that_cant_be_called_but_its_return_type_can_be_anything() {
		let _impossible = Never::map_into::<usize>();
	}
}
