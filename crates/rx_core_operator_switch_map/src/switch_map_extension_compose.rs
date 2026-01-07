use rx_core_common::{ComposableOperator, Observable, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::SwitchMapOperator;

pub trait OperatorComposeExtensionSwitchMap: ComposableOperator + Sized {
	#[inline]
	fn switch_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<
		Self,
		SwitchMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		self.compose_with(SwitchMapOperator::new(mapper, error_mapper))
	}
}

impl<Op> OperatorComposeExtensionSwitchMap for Op where Op: ComposableOperator {}
