use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, Signal};

use crate::operator::SwitchMapOperator;

pub trait OperatorComposeExtensionSwitchMap: ComposableOperator + Sized {
	#[inline]
	fn switch_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		SwitchMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		self.compose_with(SwitchMapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionSwitchMap for Op where Op: ComposableOperator {}
