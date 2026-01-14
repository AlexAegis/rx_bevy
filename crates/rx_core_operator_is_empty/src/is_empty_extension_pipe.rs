use rx_core_common::{Observable, Operator};

use crate::operator::IsEmptyOperator;

pub trait ObservablePipeExtensionIsEmpty<'o>: 'o + Observable + Sized + Send + Sync {
	#[allow(clippy::wrong_self_convention, reason = "This is an operator")]
	#[inline]
	fn is_empty(
		self,
	) -> <IsEmptyOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		IsEmptyOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionIsEmpty<'o> for O where O: 'o + Observable + Send + Sync {}
