use rx_bevy_core::{Observable, Operator};
use rx_bevy_operator_composite::CompositeOperator;

use crate::SwitchMapOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionSwitchMap: Operator + Sized {
	fn switch_map<
		NextInnerObservable: 'static + Observable<Context = Self::Context> + Send + Sync,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
	) -> CompositeOperator<
		Self,
		SwitchMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, SwitchMapOperator::new(switcher))
	}
}

impl<T> CompositeOperatorExtensionSwitchMap for T where T: Operator {}
