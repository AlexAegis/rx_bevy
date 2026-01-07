use rx_core_common::{Observable, Operator};

use crate::operator::FilterOperator;

pub trait ObservablePipeExtensionFilter<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn filter<Filter: 'static + Fn(&Self::Out, usize) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> <FilterOperator<Self::Out, Self::OutError, Filter> as Operator<'o>>::OutObservable<Self> {
		FilterOperator::new(filter).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionFilter<'o> for O where O: 'o + Observable + Send + Sync {}
