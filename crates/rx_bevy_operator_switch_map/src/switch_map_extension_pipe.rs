use rx_bevy_core::{Observable, ShareableSubscriber, SignalContext, SubscriptionCollection};
use rx_bevy_ref_pipe::Pipe;

use crate::SwitchMapOperator;

/// Operator creator function
pub fn switch_map<In, InError, Switcher, Sharer, InnerObservable>(
	mapper: Switcher,
) -> SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	SwitchMapOperator::new(mapper)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSwitchMap: Observable + Sized {
	fn switch_map<
		Sharer: 'static
			+ ShareableSubscriber<
				In = NextInnerObservable::Out,
				InError = NextInnerObservable::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>
			+ SubscriptionCollection,
		NextInnerObservable: 'static + Observable<Subscription = Sharer>,
		Switcher: 'static + Clone + Fn(Self::Out) -> NextInnerObservable,
	>(
		self,
		switcher: Switcher,
		_use_share: impl Fn(Sharer),
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
