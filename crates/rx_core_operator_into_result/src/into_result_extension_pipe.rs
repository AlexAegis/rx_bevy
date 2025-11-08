use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound, SubscriptionContext};

use crate::operator::IntoResultOperator;

/// Operator creator function
pub fn into_result<In, InError, Context>() -> IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	IntoResultOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTryCapture: Observable + Sized {
	fn into_result(
		self,
	) -> Pipe<Self, IntoResultOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, IntoResultOperator::default())
	}
}

impl<Obs> ObservableExtensionTryCapture for Obs where Obs: Observable {}
