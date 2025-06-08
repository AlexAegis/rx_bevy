use rx_bevy_observable::{Observable, Observer, ObserverConnector};
use rx_bevy_operator::Operator;

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
	Source: Observable<Out = Op::In, Error = Op::InError>,
	<Op as Operator>::InternalSubscriber:
		ObserverConnector<In = Op::In, InError = Op::InError> + 'static,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp>
	where
		NextOp: Operator<In = Op::Out, InError = Op::OutError>,
	{
		Pipe::<Self, NextOp>::new(self, operator)
	}
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Op: Operator,
	Source: Observable<Out = Op::In, Error = Op::InError>,
	<Op as Operator>::InternalSubscriber:
		ObserverConnector<In = Op::In, InError = Op::InError> + 'static,
{
	type Out = Op::Out;
	type Error = Op::OutError;
	type Subscription = <Source as Observable>::Subscription;

	#[inline]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let operator_subscriber = self.operator.operator_subscribe(destination);
		self.source_observable.subscribe(operator_subscriber)
	}
}
