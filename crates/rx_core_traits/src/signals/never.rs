use core::convert::Infallible;

/// Never cannot be constructed as it is an enum with no variants.
/// So it's perfect to denote a signal that will never be emitted, like
/// Observables that never error, or emit. Or Operators who catch errors.
///
/// Never is really just a type alias for the built in [Infallible] type, but
/// the name `Infallible` limits it's meaning to errors for the `Result` type,
/// while Never is about events/signals of any kinds that can never happen.
pub type Never = Infallible;
