use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, SubscriptionContext};

use crate::operator::TapNextOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionTapNext: Operator + Sized {
	fn tap_next<
		OnNext: 'static
			+ Fn(&Self::Out, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		callback: OnNext,
	) -> CompositeOperator<Self, TapNextOperator<Self::Out, Self::OutError, OnNext, Self::Context>>
	{
		CompositeOperator::new(self, TapNextOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionTapNext for T where T: Operator {}
