use std::marker::PhantomData;

use crate::{Operator, OperatorInstance, OperatorSubscribe};

use rx_bevy_observable::{Observable, Observer};

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

impl<Source, Op, PipeIn, PipeOut> OperatorInstance for Pipe<Source, Op, PipeIn, PipeOut>
where
	Op: OperatorSubscribe + Operator<In = PipeIn, Out = PipeOut>,
{
	type In = PipeIn;
	type Out = PipeOut;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	) {
		// self.
		//
		// self.operator
		// 	.operator_subscribe(self.source_observable, destination);
		// destination.on_push(value);
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

	fn subscribe<Destination: Observer<In = PipeOut>>(self, destination: Destination) {
		self.operator
			.operator_subscribe(self.source_observable, destination);
	}
}

pub trait ObservableExtensionPipe<Out>: Observable<Out = Out> + Sized {
	fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp, Out, NextOp::Out>
	where
		Self: Sized,
		NextOp: Operator,
	{
		Pipe::new(self, operator)
	}
}

impl<T, Out> ObservableExtensionPipe<Out> for T where T: Observable<Out = Out> {}
