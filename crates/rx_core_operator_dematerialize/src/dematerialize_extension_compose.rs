use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Never, ObserverNotification, Signal};

use crate::operator::DematerializeOperator;

pub trait OperatorComposeExtensionDematerialize<In, InError>:
	ComposableOperator<Out = ObserverNotification<In, InError>, OutError = Never> + Sized
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn dematerialize(self) -> CompositeOperator<Self, DematerializeOperator<In, InError>> {
		self.compose_with(DematerializeOperator::<In, InError>::default())
	}
}

impl<Op, In, InError> OperatorComposeExtensionDematerialize<In, InError> for Op
where
	Op: ComposableOperator<Out = ObserverNotification<In, InError>, OutError = Never>,
	In: Signal,
	InError: Signal,
{
}
