use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::SwitchAllOperator;

pub trait OperatorComposeExtensionSwitchAll: Operator + Sized {
	fn switch_all(self) -> CompositeOperator<Self, SwitchAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		CompositeOperator::new(self, SwitchAllOperator::default())
	}
}

impl<Op> OperatorComposeExtensionSwitchAll for Op where Op: Operator {}
