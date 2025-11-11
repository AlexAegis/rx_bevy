use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::SwitchMapOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSwitchMap: Observable + Sized {
	fn switch_map<
		NextInnerObservable: 'static + Observable<Context = Self::Context> + Send + Sync,
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

impl<T> ObservableExtensionSwitchMap for T where T: Observable {}
