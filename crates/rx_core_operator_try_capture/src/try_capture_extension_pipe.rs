use rx_core_traits::{Observable, SignalBound};
use rx_core_observable_pipe::Pipe;

use crate::TryCaptureOperator;

/// Operator creator function
pub fn try_capture<In, InError>() -> TryCaptureOperator<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	TryCaptureOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTryCapture: Observable + Sized {
	fn try_capture(
		self,
	) -> Pipe<Self, TryCaptureOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, TryCaptureOperator::default())
	}
}

impl<Obs> ObservableExtensionTryCapture for Obs where Obs: Observable {}
