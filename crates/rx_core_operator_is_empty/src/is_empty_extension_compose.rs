use rx_core_common::ComposableOperator;
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::IsEmptyOperator;

pub trait OperatorComposeExtensionIsEmpty: ComposableOperator + Sized {
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
	fn is_empty(self) -> CompositeOperator<Self, IsEmptyOperator<Self::Out, Self::OutError>> {
		self.compose_with(IsEmptyOperator::default())
	}
}

impl<Op> OperatorComposeExtensionIsEmpty for Op where Op: ComposableOperator {}
