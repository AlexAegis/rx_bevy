use rx_bevy_core::{DestinationSharer, Observable, SignalBound, SubscriptionCollection};
use rx_bevy_ref_pipe::Pipe;

use crate::SwitchMapOperator;

/// Operator creator function
pub fn switch_map<In, InError, Switcher, Sharer, InnerObservable>(
	mapper: Switcher,
) -> SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable + Send + Sync,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	SwitchMapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSwitchMap: Observable + Sized {
	fn switch_map<
		Sharer: 'static
			+ DestinationSharer<
				In = NextInnerObservable::Out,
				InError = NextInnerObservable::OutError,
				Context = Self::Context,
			>
			+ SubscriptionCollection,
		NextInnerObservable: 'static + Observable<Context = Self::Context> + Send + Sync,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
		_use_sharer: impl Fn(Sharer),
	) -> Pipe<
		Self,
		SwitchMapOperator<Self::Out, Self::OutError, Switcher, Sharer, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, SwitchMapOperator::new(switcher))
	}
}

impl<T> ObservableExtensionSwitchMap for T where T: Observable {}
