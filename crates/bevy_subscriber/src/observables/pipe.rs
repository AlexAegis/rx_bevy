use std::marker::PhantomData;

use crate::{
	observers::Observer,
	operators::{MapOperator, OperatorIO, OperatorSource, OperatorSubscribe},
};

use super::Observable;

pub struct PipeBuilder<Op, PipeIn, PipeOut> {
	pub(crate) operator: Op,
	_phantom_data_in: PhantomData<PipeIn>,
	_phantom_data_out: PhantomData<PipeOut>,
}

impl<Op, PipeIn, PipeOut> PipeBuilder<Op, PipeIn, PipeOut> {
	pub fn new(operator: Op) -> Self {
		Self {
			operator,
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}
impl<Op, PipeIn, PipeOut> PipeBuilder<Op, PipeIn, PipeOut> {
	pub fn pipe<NextOp>(self, mut operator: NextOp) -> PipeBuilder<NextOp, PipeIn, NextOp::Out>
	where
		NextOp: OperatorSource<Op> + OperatorIO,
	{
		operator.replace_source(self.operator);
		PipeBuilder::<NextOp, PipeIn, NextOp::Out>::new(operator)
	}
}

impl<Op, PipeIn, PipeOut> Observable for PipeBuilder<Op, PipeIn, PipeOut>
where
	Op: OperatorSubscribe + OperatorIO<Out = PipeOut>,
{
	type Out = PipeOut;

	fn subscribe<Destination: Observer<In = Op::Out>>(self, destination: Destination) {
		self.operator.operator_subscribe(destination);
	}
}

pub trait ObservableExtensionPipe<Out>: Observable<Out = Out> + Sized {
	fn pipe<NextOp>(self, mut operator: NextOp) -> PipeBuilder<NextOp, Out, NextOp::Out>
	where
		Self: Sized,
		NextOp: OperatorSource<Self> + OperatorIO,
	{
		operator.replace_source(self);
		PipeBuilder::new(operator)
	}
}

impl<T, Out> ObservableExtensionPipe<Out> for T where T: Observable<Out = Out> {}
