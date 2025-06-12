use rx_bevy_observable::{Forwarder, Observable};
use rx_bevy_operator::LiftingOperator;

use crate::LiftPipe;

pub trait ObservableExtensionLiftPipe: Observable + Sized {
	fn lift<LiftingOp>(self, operator: LiftingOp) -> LiftPipe<Self, LiftingOp>
	where
		Self: Sized
			+ Observable<
				Out = <LiftingOp::Fw as Forwarder>::In,
				Error = <LiftingOp::Fw as Forwarder>::InError,
			>,
		LiftingOp: LiftingOperator,
	{
		LiftPipe::new(self, operator)
	}
}

impl<T> ObservableExtensionLiftPipe for T where T: Observable {}
