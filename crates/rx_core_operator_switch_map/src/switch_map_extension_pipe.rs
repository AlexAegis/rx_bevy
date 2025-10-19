use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound};

use crate::operator::SwitchMapOperator;

/// Operator creator function
pub fn switch_map<In, InError, Switcher, InnerObservable>(
	mapper: Switcher,
) -> SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable + Send + Sync,
{
	SwitchMapOperator::new(mapper)
}

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
