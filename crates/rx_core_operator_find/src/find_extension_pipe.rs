use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::FindOperator;

pub trait ObservablePipeExtensionFind: Observable + Sized {
	fn find<P>(self, predicate: P) -> Pipe<Self, FindOperator<Self::Out, Self::OutError, P>>
	where
		P: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		Pipe::new(self, FindOperator::new(predicate))
	}
}

impl<O> ObservablePipeExtensionFind for O where O: Observable {}
