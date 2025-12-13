use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ConcatAllOperator;

pub trait OperatorComposeExtensionConcatAll: Operator + Sized {
	fn concat_all(self) -> CompositeOperator<Self, ConcatAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		CompositeOperator::new(self, ConcatAllOperator::default())
	}
}

impl<Op> OperatorComposeExtensionConcatAll for Op where Op: Operator {}
