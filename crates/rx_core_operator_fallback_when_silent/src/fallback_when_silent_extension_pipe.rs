use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::FallbackWhenSilentOperator;

pub trait ObservablePipeExtensionInto: Observable + Sized {
	fn fallback_when_silent<Fallback: 'static + Fn() -> Self::Out + Clone + Send + Sync>(
		self,
		fallback: Fallback,
	) -> Pipe<Self, FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, Self::Context>>
	{
		Pipe::new(self, FallbackWhenSilentOperator::new(fallback))
	}
}

impl<O> ObservablePipeExtensionInto for O where O: Observable {}
