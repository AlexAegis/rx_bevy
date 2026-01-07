use rx_core_common::{ComposableOperator, Observable, ObservableOutput};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::SwitchAllOperator;

pub trait OperatorComposeExtensionSwitchAll: ComposableOperator + Sized {
	#[inline]
	fn switch_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<Self, SwitchAllOperator<Self::Out, Self::OutError, ErrorMapper>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(SwitchAllOperator::new(error_mapper))
	}
}

impl<Op> OperatorComposeExtensionSwitchAll for Op where Op: ComposableOperator {}
