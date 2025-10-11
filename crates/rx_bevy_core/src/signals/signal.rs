/// A [Signal] is what can be used as the output of an
/// [Observable][crate::Observable] or as the input of an
/// [Observer][crate::Observer]
pub trait SignalBound: 'static + Send + Sync {}

impl<T> SignalBound for T where T: 'static + Send + Sync {}
