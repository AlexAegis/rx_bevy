use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::MergeAllOperator;

pub trait OperatorComposeExtensionMergeAll: Operator + Sized {
	fn merge_all(
		self,
		concurrency_limit: usize,
	) -> CompositeOperator<Self, MergeAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		CompositeOperator::new(self, MergeAllOperator::new(concurrency_limit))
	}
}

impl<Op> OperatorComposeExtensionMergeAll for Op where Op: Operator {}
