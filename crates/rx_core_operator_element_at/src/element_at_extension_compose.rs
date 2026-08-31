use rx_core_common::{ComposableOperator, Provider};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::ElementAtOperator;

pub trait OperatorComposeExtensionElementAt: ComposableOperator + Sized {
	/// # [ElementAtOperator]
	///
	/// Emit the value at the given index then complete.
	///
	/// If the element at the specified index does not exist, because it had
	/// completed before reaching that index, the operator will either error
	/// with [IndexOutOfRange] or emit a default value if one was provided.
	///
	/// See `element_at_or_else` for providing a default value.
	///
	/// [IndexOutOfRange]: crate::operator::ElementAtOperatorError::IndexOutOfRange
	#[inline]
	fn element_at(
		self,
		index: usize,
	) -> CompositeOperator<Self, ElementAtOperator<Self::Out, Self::OutError>> {
		self.compose_with(ElementAtOperator::new(index))
	}

	/// # [ElementAtOperator]
	///
	/// Emit the value at the given index then complete.
	///
	/// If the element at the specified index does not exist, because it had
	/// completed before reaching that index, the operator will either error
	/// with [IndexOutOfRange] or emit a default value if one was provided.
	///
	/// [IndexOutOfRange]: crate::operator::ElementAtOperatorError::IndexOutOfRange
	#[inline]
	fn element_at_or_else<P>(
		self,
		index: usize,
		default_value: P,
	) -> CompositeOperator<Self, ElementAtOperator<Self::Out, Self::OutError>>
	where
		P: 'static + Provider<Provided = Self::Out> + Send + Sync,
	{
		self.compose_with(ElementAtOperator::new_with_default(index, default_value))
	}
}

impl<Op> OperatorComposeExtensionElementAt for Op where Op: ComposableOperator {}
