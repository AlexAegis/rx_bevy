use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::SwitchMapOperator;

pub trait OperatorComposeExtensionSwitchMap: Operator + Sized {
	fn switch_map<
		NextInnerObservable: Observable<Context = Self::Context> + Signal,
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

impl<Op> OperatorComposeExtensionSwitchMap for Op where Op: Operator {}
