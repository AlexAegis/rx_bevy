use rx_bevy_core::{Observable, ObservableOutput, Operator, DropSubscription, UpgradeableObserver};

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
	#[inline]
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> DropSubscription {
		let subscriber = destination.upgrade();
		let operator_subscriber = self.operator.operator_subscribe(subscriber);
		self.source_observable.subscribe(operator_subscriber)
	}
}
