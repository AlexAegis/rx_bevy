use rx_bevy_core::{Observable, Operator, ShareableSubscriber};
use rx_bevy_operator_composite::CompositeOperator;

use crate::SwitchMapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionSwitchMap: Operator + Sized {
	fn switch_map<
		Sharer: 'static
			+ ShareableSubscriber<
				In = NextInnerObservable::Out,
				InError = NextInnerObservable::OutError,
				Context = <Self as Operator>::Context,
			>,
		NextInnerObservable: 'static + Observable<Subscription = Sharer>,
		Switcher: 'static + Clone + Fn(Self::Out) -> NextInnerObservable,
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
