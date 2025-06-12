use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, Operator};

// TODO: This isn't really usable, consider deleting
#[derive(Clone)]
pub enum OperatorChain<Prev, Op> {
	Root(Op),
	Next(Prev, Op),
}

impl<Prev, Op> OperatorChain<Prev, Op>
where
	Op: Operator,
	Op::Fw: 'static,
{
	pub fn new(operator: Op) -> Self {
		Self::Root(operator)
	}

	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> OperatorChain<Self, NextOp>
	where
		NextOp: Operator,
		NextOp::Fw: Forwarder<
				In = <Op as ObservableOutput>::Out,
				InError = <Op as ObservableOutput>::OutError,
			>,
	{
		OperatorChain::Next(self, operator)
	}
}

impl<Prev, Op> Operator for OperatorChain<Prev, Op>
where
	Op: Operator,
	Op::Fw: 'static,
{
	type Fw = <Op as Operator>::Fw;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> rx_bevy_observable::Subscriber<Self::Fw, Destination> {
		let operator = match self {
			OperatorChain::Root(op) => op,
			OperatorChain::Next(_prev, op) => op,
		};

		operator.operator_subscribe(destination)
	}
}

pub trait OperatorChainExtension<Op>
where
	Op: Operator,
{
	fn chain(self) -> OperatorChain<(), Op>;
}

impl<Op> OperatorChainExtension<Op> for Op
where
	Op: Operator,
{
	fn chain(self) -> OperatorChain<(), Op> {
		OperatorChain::Root(self)
	}
}
