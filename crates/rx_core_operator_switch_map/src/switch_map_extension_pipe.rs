use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::SwitchMapOperator;

pub trait ObservablePipeExtensionSwitchMap: Observable + Sized {
	fn switch_map<
		NextInnerObservable: Observable<Context = Self::Context> + Signal,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
	) -> Pipe<Self, SwitchMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, SwitchMapOperator::new(switcher))
	}
}

impl<O> ObservablePipeExtensionSwitchMap for O where O: Observable {}
