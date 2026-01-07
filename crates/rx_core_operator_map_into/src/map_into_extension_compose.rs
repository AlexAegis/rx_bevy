use rx_core_common::{ComposableOperator, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::MapIntoOperator;

pub trait OperatorComposeExtensionInto: ComposableOperator + Sized {
	#[inline]
	fn map_into<NextOut: Signal, NextOutError: Signal>(
		self,
	) -> CompositeOperator<Self, MapIntoOperator<Self::Out, Self::OutError, NextOut, NextOutError>>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		self.compose_with(MapIntoOperator::default())
	}
}

impl<Op> OperatorComposeExtensionInto for Op where Op: ComposableOperator {}
