use rx_core_traits::{Observable, Operator};

use crate::operator::FirstOperator;

pub trait ObservablePipeExtensionFirst: Observable + Sized {
	#[inline]
	fn first(self) -> <FirstOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		FirstOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionFirst for O where O: Observable {}
