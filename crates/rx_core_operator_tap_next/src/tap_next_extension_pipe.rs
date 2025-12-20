use rx_core_traits::{Observable, Operator};

use crate::operator::TapNextOperator;

pub trait ObservablePipeExtensionTapNext: Observable + Sized {
	#[inline]
	fn tap_next<OnNext: 'static + FnMut(&Self::Out) + Clone + Send + Sync>(
		self,
		callback: OnNext,
	) -> <TapNextOperator<Self::Out, Self::OutError, OnNext> as Operator>::OutObservable<Self> {
		TapNextOperator::new(callback).operate(self)
	}
}

impl<O> ObservablePipeExtensionTapNext for O where O: Observable {}
