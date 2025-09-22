use rx_bevy_core::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::EnumerateOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionEnumerate: Operator + Sized {
	fn enumerate(
		self,
	) -> CompositeOperator<
		Self,
		EnumerateOperator<Self::Out, Self::OutError, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, EnumerateOperator::default())
	}
}

impl<Op> CompositeOperatorExtensionEnumerate for Op where Op: Operator {}
