use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Subscriber};

use crate::CompositeSubscriber;

#[derive(RxOperator)]
#[rx_in(PrevOp::In)]
#[rx_in_error(PrevOp::InError)]
#[rx_out(Op::Out)]
#[rx_out_error(Op::OutError)]
pub struct CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	prev_op: PrevOp,
	op: Op,
}

impl<PrevOp, Op> CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	pub fn new(first_operator: PrevOp, second_operator: Op) -> Self {
		Self {
			prev_op: first_operator,
			op: second_operator,
		}
	}

	pub fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator<In = Op::Out, InError = Op::OutError>,
	{
		CompositeOperator {
			prev_op: self,
			op: next_operator,
		}
	}
}

impl<PrevOp, Op> Operator for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	type Subscriber<Destination>
		= CompositeSubscriber<PrevOp::Subscriber<Op::Subscriber<Destination>>, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		CompositeSubscriber::new(
			self.prev_op
				.operator_subscribe(self.op.operator_subscribe(destination)),
		)
	}
}
