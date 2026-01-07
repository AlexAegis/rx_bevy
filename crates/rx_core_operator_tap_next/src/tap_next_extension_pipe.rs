use rx_core_common::{Observable, Operator};

use crate::operator::TapNextOperator;

pub trait ObservablePipeExtensionTapNext<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn tap_next<OnNext: 'static + FnMut(&Self::Out) + Clone + Send + Sync>(
		self,
		callback: OnNext,
	) -> <TapNextOperator<Self::Out, Self::OutError, OnNext> as Operator<'o>>::OutObservable<Self>
	{
		TapNextOperator::new(callback).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionTapNext<'o> for O where O: 'o + Observable + Send + Sync {}
