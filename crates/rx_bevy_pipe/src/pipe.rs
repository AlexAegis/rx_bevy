use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, Operator, Subscription,
	SubscriptionLike, subscribers::ObserverSubscriber,
};

pub struct Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
}

impl<Source, Op> Clone for Pipe<Source, Op>
where
	Source: 'static + Clone + Observable,
	Op: 'static + Clone + Operator<In = Source::Out, InError = Source::OutError>,
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
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
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
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp>
	where
		NextOp: 'static
			+ Operator<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	{
		Pipe::<Self, NextOp>::new(self, operator)
	}
}

impl<Source, Op> ObservableOutput for Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	type Subscriber<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>> =
		PipeSubscriber<Source, Op, Destination>;

	#[inline]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription<Self::Subscriber<Destination>> {
		let subscriber = ObserverSubscriber::new(destination);
		let operator_subscriber = self.operator.operator_subscribe(subscriber);
		let source_subscriber = self.source_observable.subscribe(operator_subscriber);

		let pipe_subscriber = PipeSubscriber::new(source_subscriber);
		Subscription::new(pipe_subscriber)
	}
}

pub struct PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	source_subscriber:
		Subscription<Source::Subscriber<Op::Subscriber<ObserverSubscriber<Destination>>>>,
}

impl<Source, Op, Destination> PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	pub fn new(
		source_subscriber: Subscription<
			Source::Subscriber<Op::Subscriber<ObserverSubscriber<Destination>>>,
		>,
	) -> Self {
		Self { source_subscriber }
	}
}

impl<Source, Op, Destination> ObservableOutput for PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Source, Op, Destination> ObserverInput for PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	type In = Op::Out;
	type InError = Op::OutError;
}

impl<Source, Op, Destination> Observer for PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	fn next(&mut self, _next: Self::In) {}

	fn error(&mut self, _error: Self::InError) {}

	fn complete(&mut self) {}
}

impl<Source, Op, Destination> SubscriptionLike for PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	fn is_closed(&self) -> bool {
		self.source_subscriber.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.source_subscriber.unsubscribe();
	}
}

impl<Source, Op, Destination> Operation for PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	type Destination = Destination;
}

impl<Source, Op, Destination> Drop for PipeSubscriber<Source, Op, Destination>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
	Destination: 'static + Observer<In = Op::Out, InError = Op::OutError>,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
