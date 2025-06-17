use rx_bevy_observable::{CompositeOperator, Observable, Operator};
use rx_bevy_pipe_operator::Pipe;

use crate::FilterOperator;

/// Operator creator function
pub fn filter<Error, Filter, Out>(filter: Filter) -> FilterOperator<Out, Error, Filter>
where
	Filter: Clone + for<'a> Fn(&'a Out) -> bool,
{
	FilterOperator::new(filter)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFilter: Observable + Sized {
	fn filter<Filter: 'static + Clone + for<'a> Fn(&'a Self::Out) -> bool>(
		self,
		filter: Filter,
	) -> Pipe<Self, FilterOperator<Self::Out, Self::OutError, Filter>> {
		Pipe::new(self, FilterOperator::new(filter))
	}
}

impl<T> ObservableExtensionFilter for T where T: Observable {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFilter: Operator + Sized {
	fn filter<Filter: 'static + Clone + for<'a> Fn(&'a Self::Out) -> bool>(
		self,
		filter: Filter,
	) -> CompositeOperator<Self, FilterOperator<Self::Out, Self::OutError, Filter>> {
		CompositeOperator::new(self, FilterOperator::new(filter))
	}
}

impl<T> CompositeOperatorExtensionFilter for T where T: Operator {}
