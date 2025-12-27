use core::convert::Infallible;

/// Never cannot be constructed as it is an enum with no variants.
/// So it's perfect to denote a signal that will never be emitted, like
/// Observables that never error, or emit. Or Operators who catch errors.
///
/// Never is really just a type alias for the built in [Infallible] type, but
/// the name `Infallible` limits it's meaning to errors for the `Result` type,
/// while Never is about events/signals of any kinds that can never happen.
pub type Never = Infallible;

pub trait WithErrorMapper {
	fn error_mapper<E>() -> impl Fn(Never) -> E + Clone + Send + Sync;
}

impl WithErrorMapper for Never {
	/// A error mapper for the never type into anything.
	/// Since Never cannot be created, nothing has to be actually converted.
	///
	/// The implementation is just: `|_| unreachable!()`.
	#[inline]
	fn error_mapper<E>() -> impl Fn(Never) -> E + Clone + Send + Sync {
		|_| unreachable!("Never cannot be created!")
	}
}
