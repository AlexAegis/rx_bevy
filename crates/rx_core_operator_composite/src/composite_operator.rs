use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Subscriber, SubscriptionContext};

use crate::CompositeSubscriber;

#[derive(RxOperator)]
#[rx_in(PrevOp::In)]
#[rx_in_error(PrevOp::InError)]
#[rx_out(Op::Out)]
#[rx_out_error(Op::OutError)]
#[rx_context(Op::Context)]
pub struct CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError, Context = Op::Context>,
	Op: Operator,
{
	prev_op: PrevOp,
	op: Op,
}

impl<PrevOp, Op> CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError, Context = Op::Context>,
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
		NextOp: Operator<In = Op::Out, InError = Op::OutError, Context = Op::Context>,
	{
		CompositeOperator {
			prev_op: self,
			op: next_operator,
		}
	}
}

impl<PrevOp, Op> Operator for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError, Context = Op::Context>,
	Op: Operator,
{
	type Subscriber<Destination>
		= CompositeSubscriber<PrevOp::Subscriber<Op::Subscriber<Destination>>, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
		Op::Subscriber<Destination>:
			Subscriber<In = Op::In, InError = Op::InError, Context = Self::Context>,
		PrevOp::Subscriber<Op::Subscriber<Destination>>: Subscriber<Context = Self::Context>;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		CompositeSubscriber::new(
			self.prev_op
				.operator_subscribe(self.op.operator_subscribe(destination, context), context),
		)
	}
}
