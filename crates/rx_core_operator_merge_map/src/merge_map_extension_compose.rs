use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator};

use crate::operator::MergeMapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMergeMap: Operator + Sized {
	fn switch_map<
		NextInnerObservable: 'static + Observable<Context = Self::Context> + Send + Sync,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
	) -> CompositeOperator<
		Self,
		MergeMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, MergeMapOperator::new(switcher))
	}
}

impl<T> CompositeOperatorExtensionMergeMap for T where T: Operator {}
