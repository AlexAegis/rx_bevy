use rx_bevy_observable::{CompositeOperator, Observable, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::FilterMapOperator;

/// Operator creator function
pub fn filter_map<Mapper, Out, NextOut, Error>(
	mapper: Mapper,
) -> FilterMapOperator<Mapper, Out, NextOut, Error>
where
	Mapper: Clone + Fn(Out) -> Option<NextOut>,
{
	FilterMapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMap: Observable + Sized {
	fn filter_map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> Option<NextOut>>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, FilterMapOperator<Mapper, Self::Out, NextOut, Self::OutError>> {
		Pipe::new(self, FilterMapOperator::new(mapper))
	}
}

impl<T> ObservableExtensionMap for T where T: Observable {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMap: Operator + Sized {
	fn filter_map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> Option<NextOut>>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<Self, FilterMapOperator<Mapper, Self::Out, NextOut, Self::OutError>> {
		CompositeOperator::new(self, FilterMapOperator::new(mapper))
	}
}

impl<T> CompositeOperatorExtensionMap for T where T: Operator {}
