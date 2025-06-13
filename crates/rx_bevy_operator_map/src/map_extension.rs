use rx_bevy_observable::{CompositeOperator, Observable, ObservableOutput, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::MapOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMap<Out>: Observable<Out = Out> + Sized {
	fn map<NextOut, Mapper: Clone + Fn(Out) -> NextOut>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, MapOperator<Mapper, Out, NextOut, Self::OutError>> {
		Pipe::new(self, MapOperator::new(mapper))
	}
}

impl<T, Out> ObservableExtensionMap<Out> for T where T: Observable<Out = Out> {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMap: Operator + Sized {
	fn map<NextOut, Mapper: Clone + Fn(<Self::Fw as ObservableOutput>::Out) -> NextOut>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		MapOperator<
			Mapper,
			<Self::Fw as ObservableOutput>::Out,
			NextOut,
			<Self::Fw as ObservableOutput>::OutError,
		>,
	> {
		CompositeOperator::new(self, MapOperator::new(mapper))
	}
}

impl<T> CompositeOperatorExtensionMap for T where T: Operator {}
