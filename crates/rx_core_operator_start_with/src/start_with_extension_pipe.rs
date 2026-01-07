use rx_core_common::{Observable, Operator};

use crate::operator::StartWithOperator;

pub trait ObservablePipeExtensionStartWith<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn start_with(
		self,
		start_with: Self::Out,
	) -> <StartWithOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		StartWithOperator::new(start_with).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionStartWith<'o> for O where O: 'o + Observable + Send + Sync {}
