use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::ExhaustMapOperator;

pub trait ObservablePipeExtensionExhaustMap: Observable + Sized {
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Switcher: 'static + FnMut(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		exhauster: Switcher,
	) -> Pipe<Self, ExhaustMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, ExhaustMapOperator::new(exhauster))
	}
}

impl<O> ObservablePipeExtensionExhaustMap for O where O: Observable {}
