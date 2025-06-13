use rx_bevy_observable::{Observable, ObservableOutput, Observer, ObserverInput, Operator};

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
	Op::Fw: 'static,
	Source: Observable<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
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
	Source: Observable<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
{
	type Out = <Op as ObservableOutput>::Out;
	type OutError = <Op as ObservableOutput>::OutError;
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Op: Operator,
	Op::Fw: 'static,
	Source: Observable<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
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
