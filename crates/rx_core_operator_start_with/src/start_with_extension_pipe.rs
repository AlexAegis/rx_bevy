use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::StartWithOperator;

pub trait ObservablePipeExtensionStartWith: Observable + Sized {
	fn start_with<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> Pipe<Self, StartWithOperator<OnSubscribe, Self::Out, Self::OutError>>
	where
		OnSubscribe: 'static + FnMut() -> Self::Out + Send + Sync,
	{
		Pipe::new(self, StartWithOperator::new(on_subscribe))
	}
}

impl<O> ObservablePipeExtensionStartWith for O where O: Observable {}
