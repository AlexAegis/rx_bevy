use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::MapIntoOperator;

pub trait OperatorComposeExtensionInto: Operator + Sized {
	fn map_into<NextOut: Signal, NextOutError: Signal>(
		self,
	) -> CompositeOperator<
		Self,
		MapIntoOperator<Self::Out, Self::OutError, NextOut, NextOutError, Self::Context>,
	>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		CompositeOperator::new(self, MapIntoOperator::default())
	}
}

impl<Op> OperatorComposeExtensionInto for Op where Op: Operator {}
