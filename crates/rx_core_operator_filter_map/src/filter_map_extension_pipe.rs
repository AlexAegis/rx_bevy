use rx_core_traits::{Observable, SignalBound};
use rx_core_observable_pipe::Pipe;

use crate::FilterMapOperator;

/// Operator creator function
pub fn filter_map<In, InError, Mapper, Out, Context>(
	mapper: Mapper,
) -> FilterMapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Fn(In) -> Option<Out> + Clone + Send + Sync,
{
	FilterMapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFilterMap: Observable + Sized {
	fn filter_map<
		NextOut: SignalBound,
		Mapper: 'static + Fn(Self::Out) -> Option<NextOut> + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut, Self::Context>> {
		Pipe::new(self, FilterMapOperator::new(mapper))
	}
}

impl<T> ObservableExtensionFilterMap for T where T: Observable {}
