use rx_core_common::{ComposableOperator, Observable, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::ConcatMapOperator;

pub trait OperatorComposeExtensionConcatMap: ComposableOperator + Sized {
	#[inline]
	fn concat_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<
		Self,
		ConcatMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		self.compose_with(ConcatMapOperator::new(mapper, error_mapper))
	}
}

impl<Op> OperatorComposeExtensionConcatMap for Op where Op: ComposableOperator {}
