use rx_bevy_core::Observable;
use rx_bevy_ref_pipe::Pipe;

use crate::MapOperator;

/// Operator creator function
pub fn map<In, InError, Mapper, Out>(mapper: Mapper) -> MapOperator<In, InError, Mapper, Out>
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
	) -> Pipe<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut>> {
		Pipe::new(self, MapOperator::new(mapper))
	}
}

impl<T> ObservableExtensionMap for T where T: Observable {}
