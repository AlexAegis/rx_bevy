use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::MapIntoOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionInto: Operator + Sized {
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

impl<T> CompositeOperatorExtensionInto for T where T: Operator {}
