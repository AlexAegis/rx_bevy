use rx_core_common::{Observable, Operator};

use crate::operator::FirstOperator;

pub trait ObservablePipeExtensionFirst<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn first(
		self,
	) -> <FirstOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		FirstOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionFirst<'o> for O where O: 'o + Observable + Send + Sync {}
