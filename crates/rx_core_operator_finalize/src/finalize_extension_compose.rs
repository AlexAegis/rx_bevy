use rx_core_traits::{Operator, prelude::SubscriptionContext};
use rx_core_operator_composite::CompositeOperator;

use crate::FinalizeOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFinalize: Operator + Sized {
	fn finalize<
		Callback: 'static
			+ Clone
			+ FnOnce(&mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
			+ Send
			+ Sync,
	>(
		self,
		callback: Callback,
	) -> CompositeOperator<Self, FinalizeOperator<Self::Out, Self::OutError, Callback, Self::Context>>
	{
		CompositeOperator::new(self, FinalizeOperator::new(callback))
	}
}

impl<T> CompositeOperatorExtensionFinalize for T where T: Operator {}
