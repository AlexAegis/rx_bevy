use rx_bevy_observable::{Forwarder, Observable, Observer};
use rx_bevy_operator::Operator;

pub struct Pipe<Source, PipeOp> {
	pub(crate) source_observable: Source,
	pub(crate) operator: PipeOp,
}

impl<Source, Op> Clone for Pipe<Source, Op>
where
	Source: Clone,
	Op: Clone,
{
	fn clone(&self) -> Self {
		Self {
			operator: self.operator.clone(),
			source_observable: self.source_observable.clone(),
		}
	}
}

impl<Source, Op> Pipe<Source, Op>
where
	Source: Observable,
	Op: Operator,
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator: operator,
		}
	}
}

impl<Source, Op> Pipe<Source, Op>
where
	Op: Operator,
	Op::Fw: 'static,
	Source: Observable<Out = <Op::Fw as Forwarder>::In, Error = <Op::Fw as Forwarder>::InError>,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp>
	where
		NextOp: Operator,
	{
		Pipe::<Self, NextOp>::new(self, operator)
	}
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Op: Operator,
	Op::Fw: 'static,
	Source: Observable<Out = <Op::Fw as Forwarder>::In, Error = <Op::Fw as Forwarder>::InError>,
{
	type Out = <Op::Fw as Forwarder>::Out;
	type Error = <Op::Fw as Forwarder>::OutError;
	type Subscription = <Source as Observable>::Subscription;

	#[inline]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let operator_subscriber = self.operator.operator_subscribe::<Destination>(destination);
		self.source_observable.subscribe(operator_subscriber)
	}
}

// TODO: Do something with this
#[derive(Clone)]
pub enum OperatorPipe<Prev, Op> {
	Root(Op),
	Next(Prev, Op),
}

impl<Prev, Op> OperatorPipe<Prev, Op>
where
	Op: Operator,
	Op::Fw: 'static,
{
	pub fn new(operator: Op) -> Self {
		Self::Root(operator)
	}

	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> OperatorPipe<Self, NextOp>
	where
		NextOp: Operator,
		NextOp::Fw:
			Forwarder<In = <Op::Fw as Forwarder>::Out, InError = <Op::Fw as Forwarder>::OutError>,
	{
		OperatorPipe::Next(self, operator)
	}
}

impl<Prev, Op> Operator for OperatorPipe<Prev, Op>
where
	Op: Operator,
	Op::Fw: 'static,
{
	type Fw = <Op as Operator>::Fw;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<In = <Self::Fw as Forwarder>::Out, Error = <Self::Fw as Forwarder>::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> rx_bevy_observable::Subscriber<Self::Fw, Destination> {
		let operator = match self {
			OperatorPipe::Root(op) => op,
			OperatorPipe::Next(_prev, op) => op,
		};

		operator.operator_subscribe(destination)
	}
}
