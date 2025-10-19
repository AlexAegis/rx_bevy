use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound};

use crate::operator::LiftOptionOperator;

/// Operator creator function
pub fn lift_option<In, InError>() -> LiftOptionOperator<In, InError> {
	LiftOptionOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionLiftOption<T>: Observable<Out = Option<T>> + Sized
where
	T: SignalBound,
{
	fn lift_option(self) -> Pipe<Self, LiftOptionOperator<T, Self::OutError, Self::Context>> {
		Pipe::new(self, LiftOptionOperator::default())
	}
}

impl<Obs, T> ObservableExtensionLiftOption<T> for Obs
where
	Obs: Observable<Out = Option<T>>,
	T: SignalBound,
{
}
