use rx_core_traits::{Observable, Operator};

use crate::operator::IntoResultOperator;

pub trait ObservablePipeExtensionTryCapture: Observable + Sized {
	#[inline]
	fn into_result(
		self,
	) -> <IntoResultOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self> {
		IntoResultOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionTryCapture for O where O: Observable {}
