use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::SwitchAllOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionSwitchAll: Operator + Sized {
	fn switch_all(self) -> CompositeOperator<Self, SwitchAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable<Context = Self::Context>,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		CompositeOperator::new(self, SwitchAllOperator::default())
	}
}

impl<T> CompositeOperatorExtensionSwitchAll for T where T: Operator {}
