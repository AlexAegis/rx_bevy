use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::SwitchMapOperator;

pub trait ObservablePipeExtensionSwitchMap: Observable + Sized {
	fn switch_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + FnMut(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, SwitchMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, SwitchMapOperator::new(mapper))
	}
}

impl<O> ObservablePipeExtensionSwitchMap for O where O: Observable {}
