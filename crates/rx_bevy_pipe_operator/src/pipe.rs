use rx_bevy_observable::{
	Forwarder, Observable, ObservableOutput, Observer, ObserverInput, Operator,
};

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
	Source: Observable<
			Out = <Op::Fw as ObserverInput>::In,
			OutError = <Op::Fw as ObserverInput>::InError,
		>,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp>
	where
		NextOp: Operator,
	{
		Pipe::<Self, NextOp>::new(self, operator)
	}
}

impl<Source, Op> ObservableOutput for Pipe<Source, Op>
where
	Op: Operator,
	Op::Fw: 'static,
	Source: Observable<
			Out = <Op::Fw as ObserverInput>::In,
			OutError = <Op::Fw as ObserverInput>::InError,
		>,
{
	type Out = <Op::Fw as ObservableOutput>::Out;
	type OutError = <Op::Fw as ObservableOutput>::OutError;
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Op: Operator,
	Op::Fw: 'static,
	Source: Observable<
			Out = <Op::Fw as ObserverInput>::In,
			OutError = <Op::Fw as ObserverInput>::InError,
		>,
{
	type Subscription = <Source as Observable>::Subscription;

	#[inline]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let operator_subscriber = self.operator.operator_subscribe::<Destination>(destination);
		self.source_observable.subscribe(operator_subscriber)
	}
}

// TODO: Do something with this
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
				In = <Op::Fw as ObservableOutput>::Out,
				InError = <Op::Fw as ObservableOutput>::OutError,
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
				In = <Self::Fw as ObservableOutput>::Out,
				InError = <Self::Fw as ObservableOutput>::OutError,
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
