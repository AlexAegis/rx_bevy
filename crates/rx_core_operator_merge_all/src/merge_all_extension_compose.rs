use rx_core_common::{ComposableOperator, Observable, ObservableOutput};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::MergeAllOperator;

pub trait OperatorComposeExtensionMergeAll: ComposableOperator + Sized {
	#[inline]
	fn merge_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		concurrency_limit: usize,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<Self, MergeAllOperator<Self::Out, Self::OutError, ErrorMapper>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(MergeAllOperator::new(concurrency_limit, error_mapper))
	}
}

impl<Op> OperatorComposeExtensionMergeAll for Op where Op: ComposableOperator {}
