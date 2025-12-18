use rx_core_traits::{Observable, Operator};

use crate::operator::EndWithOperator;

pub trait ObservablePipeExtensionEndWith: Observable + Sized {
	#[inline]
	fn end_with(
		self,
		end_with: Self::Out,
	) -> <EndWithOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		EndWithOperator::new(end_with).operate(self)
	}
}

impl<O> ObservablePipeExtensionEndWith for O where O: Observable {}
