use rx_core_common::{Observable, Operator};

use crate::operator::FinalizeOperator;

pub trait ObservablePipeExtensionFinalize<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn finalize<Callback: 'static + Clone + FnOnce() + Send + Sync>(
		self,
		teardown: Callback,
	) -> <FinalizeOperator<Self::Out, Self::OutError, Callback> as Operator<'o>>::OutObservable<Self>
	{
		FinalizeOperator::new(teardown).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionFinalize<'o> for O where O: 'o + Observable + Send + Sync {}
