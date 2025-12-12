use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ExhaustMapOperator;

pub trait OperatorComposeExtensionExhaustMap: Operator + Sized {
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		exhauster: Switcher,
	) -> CompositeOperator<
		Self,
		ExhaustMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, ExhaustMapOperator::new(exhauster))
	}
}

impl<Op> OperatorComposeExtensionExhaustMap for Op where Op: Operator {}
