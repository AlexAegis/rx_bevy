use rx_core_traits::{Observable, Operator};

use crate::operator::SkipOperator;

pub trait ObservablePipeExtensionSkip: Observable + Sized {
	#[inline]
	fn skip(
		self,
		count: usize,
	) -> <SkipOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self> {
		SkipOperator::new(count).operate(self)
	}
}

impl<O> ObservablePipeExtensionSkip for O where O: Observable {}
