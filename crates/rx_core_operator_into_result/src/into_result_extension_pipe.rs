use rx_core_common::{Observable, Operator};

use crate::operator::IntoResultOperator;

pub trait ObservablePipeExtensionTryCapture<'o>: 'o + Observable + Sized + Send + Sync {
	/// [IntoResultOperator]
	///
	/// Error handling operator. Captures upstream values and errors, and forwards
	/// them downstream as a `Result`.
	#[inline]
	fn into_result(
		self,
	) -> <IntoResultOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		IntoResultOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionTryCapture<'o> for O where O: 'o + Observable + Send + Sync {}
