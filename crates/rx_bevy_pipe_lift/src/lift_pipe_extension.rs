use rx_bevy_observable::{LiftingForwarder, Observable};
use rx_bevy_operator::LiftingOperator;

use crate::LiftPipe;

pub trait ObservableExtensionLiftPipe: Observable + Sized {
	fn lift<LiftingOp>(self, op: LiftingOp) -> LiftPipe<Self, LiftingOp>
	where
		Self: Sized
			+ Observable<
				Out = <LiftingOp::Fw as LiftingForwarder>::In,
				Error = <LiftingOp::Fw as LiftingForwarder>::InError,
			>,
		LiftingOp: LiftingOperator,
	{
		LiftPipe::new(self, op)
	}
}

impl<T> ObservableExtensionLiftPipe for T where T: Observable {}
