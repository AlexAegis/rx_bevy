use rx_bevy_core::{
	Observable, ObservableOutput, Operator, Subscriber, context::WithSubscriptionContext,
};

/// TODO: chance source to be a ref
pub struct Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
}

impl<Source, Op> Clone for Pipe<Source, Op>
where
	Source: 'static + Clone + Observable,
	Op: 'static
		+ Clone
		+ Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
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
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
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
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp>
	where
		NextOp: 'static
			+ Operator<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
				Context = <<Pipe<Source, Op> as Observable>::Subscription as WithSubscriptionContext>::Context,
			>,
	{
		Pipe::<Self, NextOp>::new(self, operator)
	}
}

impl<Source, Op> ObservableOutput for Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Source, Op> WithSubscriptionContext for Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	type Context = Source::Context;
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	type Subscription = Source::Subscription;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let operator_subscriber = self.operator.operator_subscribe(destination, context);
		self.source_observable
			.subscribe(operator_subscriber, context)
	}
}
