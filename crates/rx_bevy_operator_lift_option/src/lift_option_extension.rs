use rx_bevy_observable::{CompositeOperator, Observable, Operator};
use rx_bevy_pipe::Pipe;

use crate::LiftOptionOperator;

/// Operator creator function
pub fn lift_option<In, InError>() -> LiftOptionOperator<In, InError> {
	LiftOptionOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionLiftOption<T>: Observable<Out = Option<T>> + Sized
where
	T: 'static,
{
	fn lift_option(self) -> Pipe<Self, LiftOptionOperator<T, Self::OutError>> {
		Pipe::new(self, LiftOptionOperator::default())
	}
}

impl<Obs, T> ObservableExtensionLiftOption<T> for Obs
where
	Obs: Observable<Out = Option<T>>,
	T: 'static,
{
}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionLiftOption<T>: Operator<Out = Option<T>> + Sized
where
	T: 'static,
{
	fn lift_option(self) -> CompositeOperator<Self, LiftOptionOperator<T, Self::OutError>> {
		CompositeOperator::new(self, LiftOptionOperator::default())
	}
}

impl<Op, T> CompositeOperatorExtensionLiftOption<T> for Op
where
	Op: Operator<Out = Option<T>>,
	T: 'static,
{
}
