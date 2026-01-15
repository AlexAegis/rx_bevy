use rx_core_common::{Observable, Operator};

use crate::operator::IsEmptyOperator;

pub trait ObservablePipeExtensionIsEmpty<'o>: 'o + Observable + Sized + Send + Sync {
	/// # [IsEmptyOperator]
	///
	/// The `is_empty` operator will emit a single boolean value indicating whether
	/// upstream emitted any items before completing:
	///
	/// - If the upstream completes without emitting any items, `is_empty` will emit
	///   `true` and then complete.
	/// - If the upstream emits any items, `is_empty` will immediately emit `false`
	///   and complete.
	#[allow(clippy::wrong_self_convention, reason = "This is an operator")]
	#[inline]
	fn is_empty(
		self,
	) -> <IsEmptyOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		IsEmptyOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionIsEmpty<'o> for O where O: 'o + Observable + Send + Sync {}
