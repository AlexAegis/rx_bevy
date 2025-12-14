use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Observer};

use crate::operator::TapOperator;

pub trait ObservablePipeExtensionTap: Observable + Sized {
	fn tap<TapDestination>(
		self,
		tap_destination: TapDestination,
	) -> Pipe<Self, TapOperator<TapDestination>>
	where
		TapDestination:
			'static + Observer<In = Self::Out, InError = Self::OutError> + Clone + Send + Sync,
		Self::Out: Clone,
		Self::OutError: Clone,
	{
		Pipe::new(self, TapOperator::new(tap_destination))
	}
}

impl<O> ObservablePipeExtensionTap for O where O: Observable {}
