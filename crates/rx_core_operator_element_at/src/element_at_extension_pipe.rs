use rx_core_common::{Observable, Operator, Provider};

use crate::operator::ElementAtOperator;

pub trait ObservablePipeExtensionElementAt<'o>: 'o + Observable + Sized + Send + Sync {
	/// # [ElementAtOperator]
	///
	/// Emit the value at the given index then complete.
	///
	/// If the element at the specified index does not exist, because it had
	/// completed before reaching that index, the operator will either error
	/// with [ElementAtOperatorError::IndexOutOfRange] or emit a default value
	/// if one was provided.
	///
	/// See `element_at_or_else` for providing a default value.
	#[inline]
	fn element_at(
		self,
		index: usize,
	) -> <ElementAtOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		ElementAtOperator::new(index).operate(self)
	}

	/// # [ElementAtOperator]
	///
	/// Emit the value at the given index then complete.
	///
	/// If the element at the specified index does not exist, because it had
	/// completed before reaching that index, the operator will either error
	/// with [ElementAtOperatorError::IndexOutOfRange] or emit a default value
	/// if one was provided.
	#[inline]
	fn element_at_or_else<P>(
		self,
		index: usize,
		default_value: P,
	) -> <ElementAtOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		P: 'static + Provider<Provided = Self::Out> + Send + Sync,
	{
		ElementAtOperator::new_with_default(index, default_value).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionElementAt<'o> for O where O: 'o + Observable + Send + Sync {}
