use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, ObservableOutput};

use crate::operator::MergeAllOperator;

pub trait OperatorComposeExtensionMergeAll: ComposableOperator + Sized {
	#[inline]
	fn merge_all(
		self,
		concurrency_limit: usize,
	) -> CompositeOperator<Self, MergeAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(MergeAllOperator::new(concurrency_limit))
	}
}

impl<Op> OperatorComposeExtensionMergeAll for Op where Op: ComposableOperator {}
