/// A [Signal] is what can be used as the output of an
/// [Observable][crate::Observable] or as the input of an
/// [Observer][crate::Observer]
///
/// Signals must always be owned values in order avoid non-intentional
/// side-effects.
pub trait SignalBound: 'static + Send + Sync {}

impl<T> SignalBound for T where T: 'static + Send + Sync {}
