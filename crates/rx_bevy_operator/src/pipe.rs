use std::marker::PhantomData;

use crate::{OperatorIO, OperatorSource, OperatorSubscribe};

use rx_bevy_observable::{Observable, Observer};

pub struct Pipe<Op, PipeIn, PipeOut> {
	pub(crate) operator: Op,
	_phantom_data_in: PhantomData<PipeIn>,
	_phantom_data_out: PhantomData<PipeOut>,
}

impl<Op, PipeIn, PipeOut> Pipe<Op, PipeIn, PipeOut> {
	pub fn new(operator: Op) -> Self {
		Self {
			operator,
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}

impl<Op, PipeIn, PipeOut> Pipe<Op, PipeIn, PipeOut> {
	pub fn pipe<NextOp>(self, mut operator: NextOp) -> Pipe<NextOp, PipeIn, NextOp::Out>
	where
		NextOp: OperatorSource<Op> + OperatorIO,
	{
		operator.replace_source(self.operator);
		Pipe::<NextOp, PipeIn, NextOp::Out>::new(operator)
	}
}

impl<Op, PipeIn, PipeOut> Observable for Pipe<Op, PipeIn, PipeOut>
where
	Op: OperatorSubscribe + OperatorIO<Out = PipeOut>,
{
	type Out = PipeOut;

	fn subscribe<Destination: Observer<In = Op::Out>>(self, destination: Destination) {
		self.operator.operator_subscribe(destination);
	}
}

pub trait ObservableExtensionPipe<Out>: Observable<Out = Out> + Sized {
	fn pipe<NextOp>(self, mut operator: NextOp) -> Pipe<NextOp, Out, NextOp::Out>
	where
		Self: Sized,
		NextOp: OperatorSource<Self> + OperatorIO,
	{
		operator.replace_source(self);
		Pipe::new(operator)
	}
}

impl<T, Out> ObservableExtensionPipe<Out> for T where T: Observable<Out = Out> {}
