use rx_core_common::{Observable, Operator};

use crate::operator::EndWithOperator;

pub trait ObservablePipeExtensionEndWith<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn end_with(
		self,
		end_with: Self::Out,
	) -> <EndWithOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		EndWithOperator::new(end_with).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionEndWith<'o> for O where O: 'o + Observable + Send + Sync {}
