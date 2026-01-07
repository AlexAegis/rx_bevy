use rx_core_common::{Observable, Operator};

use crate::operator::EnumerateOperator;

pub trait ObservablePipeExtensionEnumerate<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn enumerate(
		self,
	) -> <EnumerateOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		EnumerateOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionEnumerate<'o> for O where O: 'o + Observable + Send + Sync {}
