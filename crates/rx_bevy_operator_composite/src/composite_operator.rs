use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::CompositeSubscriber;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
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
	type Context = Op::Context;

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
		context: &mut Self::Context,
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

impl<PrevOp, Op> ObserverInput for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError, Context = Op::Context>,
	Op: Operator,
{
	type In = PrevOp::In;
	type InError = PrevOp::InError;
}

impl<PrevOp, Op> ObservableOutput for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError, Context = Op::Context>,
	Op: Operator,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}
