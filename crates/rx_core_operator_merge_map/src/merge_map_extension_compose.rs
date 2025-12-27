use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, Signal};

use crate::operator::MergeMapOperator;

pub trait OperatorComposeExtensionMergeMap: ComposableOperator + Sized {
	#[inline]
	fn merge_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
		ErrorMapper: 'static + Fn(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		concurrency_limit: usize,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<
		Self,
		MergeMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		self.compose_with(MergeMapOperator::new(
			mapper,
			error_mapper,
			concurrency_limit,
		))
	}
}

impl<Op> OperatorComposeExtensionMergeMap for Op where Op: ComposableOperator {}
