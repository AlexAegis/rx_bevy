use rx_bevy_observable::{CompositeOperator, Observable, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::MapOperator;

/// Operator creator function
pub fn map<Mapper, In, InError, Out>(mapper: Mapper) -> MapOperator<Mapper, In, InError, Out>
where
	Mapper: Clone + Fn(In) -> Out,
{
	MapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMap: Observable + Sized {
	fn map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> NextOut>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, MapOperator<Mapper, Self::Out, Self::OutError, NextOut>> {
		Pipe::new(self, MapOperator::new(mapper))
	}
}

impl<T> ObservableExtensionMap for T where T: Observable {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMap: Operator + Sized {
	fn map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> NextOut>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<Self, MapOperator<Mapper, Self::Out, Self::OutError, NextOut>> {
		CompositeOperator::new(self, MapOperator::new(mapper))
	}
}

impl<T> CompositeOperatorExtensionMap for T where T: Operator {}
