use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::FindIndexOperator;

pub trait ObservablePipeExtensionFindIndex: Observable + Sized {
	fn find_index<P>(
		self,
		predicate: P,
	) -> Pipe<Self, FindIndexOperator<Self::Out, Self::OutError, P>>
	where
		P: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		Pipe::new(self, FindIndexOperator::new(predicate))
	}
}

impl<O> ObservablePipeExtensionFindIndex for O where O: Observable {}
