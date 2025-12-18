use rx_core_traits::{Observable, Operator};

use crate::operator::EnumerateOperator;

pub trait ObservablePipeExtensionEnumerate: Observable + Sized {
	#[inline]
	fn enumerate(
		self,
	) -> <EnumerateOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self> {
		EnumerateOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionEnumerate for O where O: Observable {}
