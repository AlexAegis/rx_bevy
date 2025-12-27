use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Subscriber};

use crate::CompositeSubscriber;

#[derive(RxOperator)]
#[rx_in(PrevOp::In)]
#[rx_in_error(PrevOp::InError)]
#[rx_out(Op::Out)]
#[rx_out_error(Op::OutError)]
pub struct CompositeOperator<PrevOp, Op>
where
	PrevOp: ComposableOperator<Out = Op::In, OutError = Op::InError>,
	Op: ComposableOperator,
{
	prev_op: PrevOp,
	op: Op,
}

impl<PrevOp, Op> CompositeOperator<PrevOp, Op>
where
	PrevOp: ComposableOperator<Out = Op::In, OutError = Op::InError>,
	Op: ComposableOperator,
{
	pub fn new(first_operator: PrevOp, second_operator: Op) -> Self {
		Self {
			prev_op: first_operator,
			op: second_operator,
		}
	}
}

impl<PrevOp, Op> ComposableOperator for CompositeOperator<PrevOp, Op>
where
	PrevOp: ComposableOperator<Out = Op::In, OutError = Op::InError>,
	Op: ComposableOperator,
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
