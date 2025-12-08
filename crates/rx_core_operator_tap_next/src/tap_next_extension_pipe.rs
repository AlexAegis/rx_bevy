use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::TapNextOperator;

pub trait ObservablePipeExtensionTapNext: Observable + Sized {
	fn tap_next<OnNext: 'static + Fn(&Self::Out) + Clone + Send + Sync>(
		self,
		callback: OnNext,
	) -> Pipe<Self, TapNextOperator<Self::Out, Self::OutError, OnNext>> {
		Pipe::new(self, TapNextOperator::new(callback))
	}
}

impl<O> ObservablePipeExtensionTapNext for O where O: Observable {}
