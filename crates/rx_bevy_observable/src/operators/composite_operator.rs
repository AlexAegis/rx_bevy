use std::marker::PhantomData;

use crate::{ObservableOutput, Observer, ObserverInput, Operator, Subscriber, Subscription};

#[derive(Clone)]
pub struct CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	prev_op: PrevOp,
	op: Op,
}

impl<PrevOp, Op> CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	pub fn new(first_operator: PrevOp, second_operator: Op) -> Self {
		Self {
			prev_op: first_operator,
			op: second_operator,
		}
	}

	pub fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator<In = Op::Out, InError = Op::OutError>,
	{
		CompositeOperator {
			prev_op: self,
			op: next_operator,
		}
	}
}

impl<PrevOp, Op> Operator for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	type Subscriber<D: Observer<In = Self::Out, InError = Self::OutError>> =
		CompositeSubscriber<PrevOp::Subscriber<Op::Subscriber<D>>, Op::Subscriber<D>, D>;

	fn operator_subscribe<
		Destination: Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		CompositeSubscriber {
			sub: self
				.prev_op
				.operator_subscribe(self.op.operator_subscribe(destination)),
			_phantom_data: PhantomData,
		}
	}
}

impl<PrevOp, Op> ObserverInput for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	type In = PrevOp::In;
	type InError = PrevOp::InError;
}

impl<PrevOp, Op> ObservableOutput for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator<Out = Op::In, OutError = Op::InError>,
	Op: Operator,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

#[derive(Clone)]
pub struct CompositeSubscriber<PrevSub, Sub, Destination>
where
	PrevSub: Subscriber<Destination = Sub>,
	Sub: Subscriber<Destination = Destination>,
	Destination: Observer<In = Sub::Out, InError = Sub::OutError>,
{
	pub sub: PrevSub,
	_phantom_data: PhantomData<Destination>,
}

impl<PrevSub, Sub, Destination> ObserverInput for CompositeSubscriber<PrevSub, Sub, Destination>
where
	PrevSub: Subscriber<Destination = Sub>,
	Sub: Subscriber<Destination = Destination>,
	Destination: Observer<In = Sub::Out, InError = Sub::OutError>,
{
	type In = PrevSub::In;
	type InError = PrevSub::InError;
}

impl<PrevSub, Sub, Destination> ObservableOutput for CompositeSubscriber<PrevSub, Sub, Destination>
where
	PrevSub: Subscriber<Destination = Sub>,
	Sub: Subscriber<Destination = Destination>,
	Destination: Observer<In = Sub::Out, InError = Sub::OutError>,
{
	type Out = Sub::Out;
	type OutError = Sub::OutError;
}

impl<PrevSub, Sub, Destination> Observer for CompositeSubscriber<PrevSub, Sub, Destination>
where
	PrevSub: Subscriber<Destination = Sub>,
	Sub: Subscriber<Destination = Destination>,
	Destination: Observer<In = Sub::Out, InError = Sub::OutError>,
{
	fn next(&mut self, next: Self::In) {
		self.sub.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.sub.error(error);
	}

	fn complete(&mut self) {
		self.sub.complete();
	}
}

impl<PrevSub, Sub, Destination> Subscriber for CompositeSubscriber<PrevSub, Sub, Destination>
where
	PrevSub: Subscriber<Destination = Sub>,
	Sub: Subscriber<Destination = Destination>,
	Destination: Observer<In = Sub::Out, InError = Sub::OutError>,
{
	type Destination = Destination;
}

impl<PrevSub, Sub, Destination> Subscription for CompositeSubscriber<PrevSub, Sub, Destination>
where
	PrevSub: Subscriber<Destination = Sub>,
	Sub: Subscriber<Destination = Destination>,
	Destination: Observer<In = Sub::Out, InError = Sub::OutError>,
{
	fn is_closed(&self) -> bool {
		self.sub.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.sub.unsubscribe();
	}
}

pub trait CompositeOperatorExtension: Operator + Sized {
	fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator<In = Self::Out, InError = Self::OutError>,
	{
		CompositeOperator {
			prev_op: self,
			op: next_operator,
		}
	}
}

impl<T> CompositeOperatorExtension for T where T: Operator {}
