use rx_bevy_core::Observable;
use rx_bevy_ref_pipe::Pipe;

use crate::FilterOperator;

/// Operator creator function
pub fn filter<Error, Filter, Out>(filter: Filter) -> FilterOperator<Out, Error, Filter>
where
	Filter: for<'a> Fn(&'a Out) -> bool + Clone + Send + Sync,
{
	FilterOperator::new(filter)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFilter: Observable + Sized {
	fn filter<Filter: 'static + for<'a> Fn(&'a Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> Pipe<Self, FilterOperator<Self::Out, Self::OutError, Filter, Self::Context>> {
		Pipe::new(self, FilterOperator::new(filter))
	}
}

impl<T> ObservableExtensionFilter for T where T: Observable {}
