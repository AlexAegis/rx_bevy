use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, SubscriptionContext};

use crate::operator::TapNextOperator;

pub trait OperatorComposeExtensionTapNext: Operator + Sized {
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

impl<Op> OperatorComposeExtensionTapNext for Op where Op: Operator {}
