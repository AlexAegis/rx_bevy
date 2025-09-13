use std::marker::PhantomData;

use rx_bevy_core::{Observable, ObservableOutput, Operator, SignalContext, Subscriber};

pub struct Pipe<'c, Source, Op>
where
	Source: 'static + Observable<'c>,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
	_phantom_data: PhantomData<&'c Source>,
}

impl<'c, Source, Op> Clone for Pipe<'c, Source, Op>
where
	Source: 'static + Clone + Observable<'c>,
	Op: 'static + Clone + Operator<In = Source::Out, InError = Source::OutError>,
{
	fn clone(&self) -> Self {
		Self {
			operator: self.operator.clone(),
			source_observable: self.source_observable.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<'c, Source, Op> Pipe<'c, Source, Op>
where
	Source: 'static + Observable<'c>,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
			_phantom_data: PhantomData,
		}
	}
}

impl<'c, Source, Op> Pipe<'c, Source, Op>
where
	Source: 'static + Observable<'c>,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<'c, Self, NextOp>
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

impl<'c, Source, Op> ObservableOutput for Pipe<'c, Source, Op>
where
	Source: 'static + Observable<'c>,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<'c, Source, Op> SignalContext for Pipe<'c, Source, Op>
where
	Source: 'static + Observable<'c>,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	type Context = Source::Context;
}

impl<'c, Source, Op> Observable<'c> for Pipe<'c, Source, Op>
where
	Source: 'static + Observable<'c>,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError>,
{
	type Subscription = Source::Subscription;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination as SignalContext>::Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let operator_subscriber = self.operator.operator_subscribe(destination, context);
		self.source_observable
			.subscribe(operator_subscriber, context)
	}
}
