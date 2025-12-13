use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MergeMapOperator;

pub trait OperatorComposeExtensionMergeMap: Operator + Sized {
	fn merge_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		concurrency_limit: usize,
	) -> CompositeOperator<
		Self,
		MergeMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, MergeMapOperator::new(mapper, concurrency_limit))
	}
}

impl<Op> OperatorComposeExtensionMergeMap for Op where Op: Operator {}
