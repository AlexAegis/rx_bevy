use std::marker::PhantomData;

use rx_bevy_observable::{Observable, Observer, Subscription};
use rx_bevy_operator::{Operator, OperatorSubscribe};

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

impl<Source, Op, PipeIn, PipeOut> Operator for Pipe<Source, Op, PipeIn, PipeOut>
where
	Op: OperatorSubscribe + Operator<In = PipeIn, Out = PipeOut>,
	Source: Clone,
	Op: Clone,
{
	type In = PipeIn;
	type Out = PipeOut;

	type Instance = Self;

	fn create_operator_instance(&self) -> Self::Instance {
		self.clone()
	}

	fn operate(&mut self, next: Self::In) -> Self::Out {
		self.operator.operate(next)
	}
}

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
	Op: OperatorSubscribe + Operator<In = PipeIn, Out = PipeOut>,
	Source: Observable<Out = Op::In>,
{
	type Out = PipeOut;

	fn subscribe<Destination: Observer<In = PipeOut>>(
		&mut self,
		destination: Destination,
	) -> Subscription<Destination> {
		self.operator
			.operator_subscribe(self.source_observable, destination)
	}
}
