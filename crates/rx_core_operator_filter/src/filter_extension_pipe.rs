use rx_core_traits::{Observable, Operator};

use crate::operator::FilterOperator;

pub trait ObservablePipeExtensionFilter: Observable + Sized {
	#[inline]
	fn filter<Filter: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> <FilterOperator<Self::Out, Self::OutError, Filter> as Operator>::OutObservable<Self> {
		FilterOperator::new(filter).operate(self)
	}
}

impl<O> ObservablePipeExtensionFilter for O where O: Observable {}
