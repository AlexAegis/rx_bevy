use rx_bevy_core::{Observable, SignalContext};
use rx_bevy_ref_pipe::Pipe;

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
	fn lift_option(
		self,
	) -> Pipe<
		Self,
		LiftOptionOperator<T, Self::OutError, <Self::Subscription as SignalContext>::Context>,
	> {
		Pipe::new(self, LiftOptionOperator::default())
	}
}

impl<Obs, T> ObservableExtensionLiftOption<T> for Obs
where
	Obs: Observable<Out = Option<T>>,
	T: 'static,
{
}
