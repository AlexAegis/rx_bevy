use rx_bevy_core::{Operator, SignalBound};
use rx_bevy_operator_composite::CompositeOperator;

use crate::MapIntoOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionInto: Operator + Sized {
	fn map_into<NextOut: SignalBound, NextOutError: SignalBound>(
		self,
	) -> CompositeOperator<
		Self,
		MapIntoOperator<
			Self::Out,
			Self::OutError,
			NextOut,
			NextOutError,
			<Self as Operator>::Context,
		>,
	>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		CompositeOperator::new(self, MapIntoOperator::default())
	}
}

impl<T> CompositeOperatorExtensionInto for T where T: Operator {}
