use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, ObservableOutput};

use crate::operator::SwitchAllOperator;

pub trait OperatorComposeExtensionSwitchAll: ComposableOperator + Sized {
	#[inline]
	fn switch_all(self) -> CompositeOperator<Self, SwitchAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(SwitchAllOperator::default())
	}
}

impl<Op> OperatorComposeExtensionSwitchAll for Op where Op: ComposableOperator {}
