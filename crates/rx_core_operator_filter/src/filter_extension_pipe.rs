use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::FilterOperator;

pub trait ObservablePipeExtensionFilter: Observable + Sized {
	fn filter<Filter: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> Pipe<Self, FilterOperator<Self::Out, Self::OutError, Filter>> {
		Pipe::new(self, FilterOperator::new(filter))
	}
}

impl<O> ObservablePipeExtensionFilter for O where O: Observable {}
