use std::marker::PhantomData;

use rx_bevy_observable::{DynObserver, Observable, Observer, Subscriber, Subscription};
use rx_bevy_operator::Operator;

pub struct Pipe<Source, Op, PipeIn, PipeOut> {
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
	_phantom_data_in: PhantomData<PipeIn>,
	_phantom_data_out: PhantomData<PipeOut>,
}

impl<Source, Op, PipeIn, PipeOut> Clone for Pipe<Source, Op, PipeIn, PipeOut>
where
	Source: Clone,
	Op: Clone,
{
	fn clone(&self) -> Self {
		Self {
			operator: self.operator.clone(),
			source_observable: self.source_observable.clone(),
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}
/*
impl<Source, Op, PipeIn, PipeOut> Operator for Pipe<Source, Op, PipeIn, PipeOut>
where
	Op: Operator<In = PipeIn, Out = PipeOut>,
	Source: Clone,
	Op: Clone,
{
	type In = PipeIn;
	type Out = PipeOut;

	type InternalSubscriber = DynObserver<PipeIn>;

	fn operator_subscribe<Destination: 'static + Observer<Self::Out>>(
		&mut self,
		observer: Destination,
	) -> Self::InternalSubscriber {
		let a = self.operator.operator_subscribe(observer);
	}
}*/

impl<Source, Op, PipeIn, PipeOut> Pipe<Source, Op, PipeIn, PipeOut> {
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}

impl<Source, Op, PipeIn, PipeOut> Pipe<Source, Op, PipeIn, PipeOut> {
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp, PipeIn, NextOp::Out>
	where
		NextOp: Operator,
	{
		Pipe::<Self, NextOp, PipeIn, NextOp::Out>::new(self, operator)
	}
}

impl<Source, Op, PipeIn, PipeOut> Observable for Pipe<Source, Op, PipeIn, PipeOut>
where
	Op: Operator<Out = PipeOut>,
	Source: Observable<Out = Op::In>,
	<Op as Operator>::InternalSubscriber: 'static,
{
	type Out = PipeOut;

	type Subscription = <Source as Observable>::Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let operator_subscriber = self.operator.operator_subscribe(destination);
		self.source_observable.subscribe(operator_subscriber)
	}
}
