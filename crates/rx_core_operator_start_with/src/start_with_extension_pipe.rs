use rx_core_traits::{Observable, Operator};

use crate::operator::StartWithOperator;

pub trait ObservablePipeExtensionStartWith: Observable + Sized {
	#[inline]
	fn start_with(
		self,
		start_with: Self::Out,
	) -> <StartWithOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		StartWithOperator::new(start_with).operate(self)
	}
}

impl<O> ObservablePipeExtensionStartWith for O where O: Observable {}
