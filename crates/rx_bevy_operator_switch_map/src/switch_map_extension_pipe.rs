use rx_bevy_core::{
	DestinationSharer, Observable, SubscriptionCollection, SubscriptionLike, WithContext,
};
use rx_bevy_ref_pipe::Pipe;

use crate::SwitchMapOperator;

/// Operator creator function
pub fn switch_map<In, InError, Switcher, Sharer, InnerObservable>(
	mapper: Switcher,
) -> SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
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
				Context = <NextInnerObservable::Subscription as WithContext>::Context,
			>
			+ SubscriptionCollection,
		NextInnerObservable: 'static + Observable,
		Switcher: 'static + Clone + Fn(Self::Out) -> NextInnerObservable,
	>(
		self,
		switcher: Switcher,
		_use_sharer: impl Fn(Sharer),
	) -> Pipe<
		Self,
		SwitchMapOperator<Self::Out, Self::OutError, Switcher, Sharer, NextInnerObservable>,
	>
	where
		NextInnerObservable::Subscription:
			SubscriptionLike<Context = <Self::Subscription as WithContext>::Context>,
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, SwitchMapOperator::new(switcher))
	}
}

impl<T> ObservableExtensionSwitchMap for T where T: Observable {}
