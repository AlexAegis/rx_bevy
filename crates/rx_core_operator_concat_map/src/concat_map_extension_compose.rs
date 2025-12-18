use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, Signal};

use crate::operator::ConcatMapOperator;

pub trait OperatorComposeExtensionConcatMap: ComposableOperator + Sized {
	#[inline]
	fn concat_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		ConcatMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		self.compose_with(ConcatMapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionConcatMap for Op where Op: ComposableOperator {}
