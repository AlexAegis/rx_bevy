use rx_bevy_core::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::TapNextOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionTapNext: Operator + Sized {
	fn tap_next<
		OnNext: 'static + Clone + for<'a> Fn(&'a Self::Out, &'a mut <Self as Operator>::Context),
	>(
		self,
		callback: OnNext,
	) -> CompositeOperator<
		Self,
		TapNextOperator<Self::Out, Self::OutError, OnNext, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, TapNextOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionTapNext for T where T: Operator {}
