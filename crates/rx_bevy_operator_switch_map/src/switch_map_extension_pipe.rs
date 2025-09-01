use rx_bevy_core::Observable;
use rx_bevy_pipe::Pipe;

use crate::SwitchMapOperator;

/// Operator creator function
pub fn switch_map<In, InError, Switcher, Out>(
	mapper: Switcher,
) -> SwitchMapOperator<In, InError, Switcher, Out>
where
	Switcher: Clone + Fn(In) -> Out,
{
	SwitchMapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSwitchMap: Observable + Sized {
	fn switch_map<
		NextInnerObservable: 'static + Observable,
		Switcher: 'static + Clone + Fn(Self::Out) -> NextInnerObservable,
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

impl<T> ObservableExtensionSwitchMap for T
where
	T: Observable,
	Self::Out: 'static,
{
}
