use rx_bevy_core::Observable;
use rx_bevy_ref_pipe::Pipe;

use crate::FilterMapOperator;

/// Operator creator function
pub fn filter_map<In, InError, Mapper, Out>(
	mapper: Mapper,
) -> FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Clone + Fn(In) -> Option<Out>,
{
	FilterMapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFilterMap: Observable + Sized {
	fn filter_map<NextOut: 'static, Mapper: 'static + Clone + Fn(Self::Out) -> Option<NextOut>>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut>> {
		Pipe::new(self, FilterMapOperator::new(mapper))
	}
}

impl<T> ObservableExtensionFilterMap for T where T: Observable {}
