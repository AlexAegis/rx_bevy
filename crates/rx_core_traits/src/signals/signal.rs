/// # [Signal]
///
/// A [Signal] is what can be used as the output of an
/// [Observable][crate::Observable] or as the input of an
/// [Observer][crate::Observer]
///
/// ## Trait Bounds
///
/// - `'static`: Signals must always outlive their subscribers
/// - `Send + Sync`: Must be able to cross thread boundaries
pub trait Signal: 'static + Send + Sync {}

impl<T> Signal for T where T: 'static + Send + Sync {}
