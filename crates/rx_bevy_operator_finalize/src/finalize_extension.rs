use rx_bevy_observable::{CompositeOperator, Observable, ObservableOutput, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::FinalizeOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFinalize<Out>: Observable<Out = Out> + Sized {
	fn finalize<Callback: Clone + FnOnce()>(
		self,
		callback: Callback,
	) -> Pipe<Self, FinalizeOperator<Out, Self::OutError, Callback>> {
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionFinalize<Out> for T where T: Observable<Out = Out> {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFinalize: Operator + Sized {
	fn finalize<Callback: Clone + FnOnce()>(
		self,
		callback: Callback,
	) -> CompositeOperator<
		Self,
		FinalizeOperator<
			<Self::Fw as ObservableOutput>::Out,
			<Self::Fw as ObservableOutput>::OutError,
			Callback,
		>,
	> {
		CompositeOperator::new(self, FinalizeOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionFinalize for T where T: Operator {}
