use rx_core_traits::{Observable, Operator};

use crate::operator::FinalizeOperator;

pub trait ObservablePipeExtensionFinalize: Observable + Sized {
	#[inline]
	fn finalize<Callback: 'static + Clone + FnOnce() + Send + Sync>(
		self,
		callback: Callback,
	) -> <FinalizeOperator<Self::Out, Self::OutError, Callback> as Operator>::OutObservable<Self> {
		FinalizeOperator::new(callback).operate(self)
	}
}

impl<O> ObservablePipeExtensionFinalize for O where O: Observable {}
