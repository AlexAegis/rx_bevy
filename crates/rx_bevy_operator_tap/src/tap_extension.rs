use rx_bevy_observable::{CompositeOperator, Observable, ObservableOutput, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::TapOperator;

/// Operator creator function
pub fn tap<In, InError, Callback>(callback: Callback) -> TapOperator<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	TapOperator::new(callback)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTapNext<Out>: Observable<Out = Out> + Sized {
	fn tap_next<Callback: Clone + for<'a> Fn(&'a Out)>(
		self,
		callback: Callback,
	) -> Pipe<Self, TapOperator<Out, Self::OutError, Callback>> {
		Pipe::new(self, TapOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionTapNext<Out> for T where T: Observable<Out = Out> {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionTapNext: Operator + Sized {
	fn tap_next<Callback: Clone + for<'a> Fn(&'a <Self::Fw as ObservableOutput>::Out)>(
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

impl<T> CompositeOperatorExtensionTapNext for T where T: Operator {}
