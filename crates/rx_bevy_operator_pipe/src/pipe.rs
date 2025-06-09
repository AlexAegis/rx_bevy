use rx_bevy_observable::{Forwarder, Observable, Observer};
use rx_bevy_operator::{HigherOrderForwarder, HigherOrderOperator, Operator};

pub struct Pipe<Source, Op> {
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
}

impl<Source, Op> Clone for Pipe<Source, Op>
where
	Source: Observable + Clone,
	Op: Operator + Clone,
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
	Op: Operator,
	Source: Observable,
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
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
	/*
	#[inline]
	pub fn higher_pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, HigherOrderForwarder<NextOp>>
	where
		NextOp: HigherOrderOperator,
	{
		// let next_source = operator.source_on_next(next);

		let hof = HigherOrderForwarder::<NextOp> {
			higher_order_operator: operator,
		};
		let pipe = Pipe::<Self, NextOp>::new(self, hof);
	}*/
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
