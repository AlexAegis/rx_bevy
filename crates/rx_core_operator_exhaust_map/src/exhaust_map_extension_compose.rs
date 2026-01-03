use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, Signal};

use crate::operator::ExhaustMapOperator;

pub trait OperatorComposeExtensionExhaustMap: ComposableOperator + Sized {
	#[inline]
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<
		Self,
		ExhaustMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		self.compose_with(ExhaustMapOperator::new(mapper, error_mapper))
	}
}

impl<Op> OperatorComposeExtensionExhaustMap for Op where Op: ComposableOperator {}
