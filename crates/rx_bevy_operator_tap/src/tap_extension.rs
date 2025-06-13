use rx_bevy_observable::{CompositeOperator, Observable, ObservableOutput, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::TapOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTap<Out>: Observable<Out = Out> + Sized {
	fn tap<Callback: Clone + for<'a> Fn(&'a Out)>(
		self,
		callback: Callback,
	) -> Pipe<Self, TapOperator<Out, Self::OutError, Callback>> {
		Pipe::new(self, TapOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionTap<Out> for T where T: Observable<Out = Out> {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionTap: Operator + Sized {
	fn tap<Callback: Clone + for<'a> Fn(&'a <Self::Fw as ObservableOutput>::Out)>(
		self,
		callback: Callback,
	) -> CompositeOperator<
		Self,
		TapOperator<
			<Self::Fw as ObservableOutput>::Out,
			<Self::Fw as ObservableOutput>::OutError,
			Callback,
		>,
	> {
		CompositeOperator::new(self, TapOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionTap for T where T: Operator {}
