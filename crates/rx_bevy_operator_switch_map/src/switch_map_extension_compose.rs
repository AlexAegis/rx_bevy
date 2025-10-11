use rx_bevy_core::{DestinationSharer, Observable, Operator, SubscriptionCollection};
use rx_bevy_operator_composite::CompositeOperator;

use crate::SwitchMapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionSwitchMap: Operator + Sized {
	fn switch_map<
		Sharer: 'static
			+ DestinationSharer<
				In = NextInnerObservable::Out,
				InError = NextInnerObservable::OutError,
				Context = Self::Context,
			>
			+ SubscriptionCollection,
		NextInnerObservable: 'static + Observable<Subscription = Sharer, Context = Self::Context> + Send + Sync,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
	) -> CompositeOperator<
		Self,
		SwitchMapOperator<Self::Out, Self::OutError, Switcher, Sharer, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, SwitchMapOperator::new(switcher))
	}
}

impl<T> CompositeOperatorExtensionSwitchMap for T where T: Operator {}
