use rx_bevy_core::Observable;
use rx_bevy_pipe::Pipe;

use crate::TryCaptureOperator;

/// Operator creator function
pub fn try_capture<In, InError>() -> TryCaptureOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	TryCaptureOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTryCapture: Observable + Sized {
	fn try_capture(self) -> Pipe<Self, TryCaptureOperator<Self::Out, Self::OutError>> {
		Pipe::new(self, TryCaptureOperator::default())
	}
}

impl<Obs> ObservableExtensionTryCapture for Obs where Obs: Observable {}
