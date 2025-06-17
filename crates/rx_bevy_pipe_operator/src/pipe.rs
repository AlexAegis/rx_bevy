use rx_bevy_observable::{ClosableDestination, Observable, ObservableOutput, Observer, Operator};

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
			operator,
		}
	}
}

impl<Source, Op> Pipe<Source, Op>
where
	Op: Operator,
	Source: Observable<Out = Op::In, OutError = Op::InError>,
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
	Source: Observable<Out = Op::In, OutError = Op::InError>,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Op: Operator,
	Source: Observable<Out = Op::In, OutError = Op::InError>,
{
	type Subscription = Source::Subscription;

	#[inline]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let closable = ClosableDestination::new(destination);
		let operator_subscriber = self.operator.operator_subscribe(closable);
		self.source_observable.subscribe(operator_subscriber)
	}
}
