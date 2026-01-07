use rx_core_common::{Observable, Operator};

use crate::operator::SkipOperator;

pub trait ObservablePipeExtensionSkip<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn skip(
		self,
		count: usize,
	) -> <SkipOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		SkipOperator::new(count).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionSkip<'o> for O where O: 'o + Observable + Send + Sync {}
