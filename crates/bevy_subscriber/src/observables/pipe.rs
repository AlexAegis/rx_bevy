use std::{marker::PhantomData, process::Output};

use crate::{
	observers::Observer,
	operators::{MapOperator, OperatorIO, OperatorIntoObserver, OperatorSource, OperatorSubscribe},
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

impl<Op, PipeIn, PipeOut, Destination> Observable<Destination> for PipeBuilder<Op, PipeIn, PipeOut>
where
	Destination: Observer<In = PipeOut>,
	Op: OperatorSubscribe<Destination>,
{
	type Out = PipeOut;

	fn subscribe(self, destination: Destination) {
		self.operator.subscribe(destination);
	}
}

/// TODO: Could be part of a possible observable macro
impl<Op, PipeIn, PipeOut, Destination> ObservableWithOperators<Destination, PipeOut>
	for PipeBuilder<Op, PipeIn, PipeOut>
where
	Destination: Observer<In = PipeOut>,
	Op: OperatorSubscribe<Destination>,
{
}

pub trait ObservableWithOperators<Destination, Out>:
	Observable<Destination, Out = Out> + Sized
{
	fn pipe<NextOp>(self, mut operator: NextOp) -> PipeBuilder<NextOp, Out, NextOp::Out>
	where
		Self: Sized,
		NextOp: OperatorSource<Self> + OperatorIO,
	{
		operator.replace_source(self);
		PipeBuilder::new(operator)
	}

	fn map<NextOut, F: Fn(Out) -> NextOut>(
		self,
		transform: F,
	) -> MapOperator<Self, Out, NextOut, F> {
		MapOperator::new_with_source(self, transform)
	}
}
