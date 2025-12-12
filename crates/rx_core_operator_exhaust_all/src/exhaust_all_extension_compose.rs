use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ExhaustAllOperator;

pub trait OperatorComposeExtensionExhaustAll: Operator + Sized {
	fn exhaust_all(self) -> CompositeOperator<Self, ExhaustAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		CompositeOperator::new(self, ExhaustAllOperator::default())
	}
}

impl<Op> OperatorComposeExtensionExhaustAll for Op where Op: Operator {}
